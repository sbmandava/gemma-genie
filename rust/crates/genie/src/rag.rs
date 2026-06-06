//! Retrieval: model2vec-rs embeddings + lancedb vector store. M2 = single-doc
//! (--doc/--txt) with threshold gating + Sources footer. Directory mode and the
//! `cache` subcommand are M3.

use crate::cli::Cli;
use crate::config::{Config, EMBED_MODEL};
use crate::llm;
use crate::parse;
use anyhow::{bail, Result};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::Path;
use std::sync::Arc;

use arrow_array::cast::AsArray;
use arrow_array::types::Float32Type;
use arrow_array::{FixedSizeListArray, RecordBatch, StringArray};
use arrow_schema::{DataType, Field, Schema};
use futures::TryStreamExt;
use lancedb::query::{ExecutableQuery, QueryBase};
use model2vec_rs::model::StaticModel;

pub fn ask(question: &str, cli: &Cli, cfg: &Config) -> Result<()> {
    if cli.dir.is_some() {
        println!("genie (rust): directory mode (--dir) not yet implemented (RUST_PLAN.md M3).");
        return Ok(());
    }
    let rt = tokio::runtime::Runtime::new()?;
    let (prompt, sources) = rt.block_on(prepare(question, cli, cfg))?;
    drop(rt);
    llm::generate(cfg, prompt)?;
    print_sources(&sources);
    Ok(())
}

pub fn cache(_action: &str, _cfg: &Config) -> Result<()> {
    println!("genie (rust): cache subcommand not yet implemented (RUST_PLAN.md M3).");
    Ok(())
}

/// Build the model prompt (and source list) for a --doc/--txt ask.
async fn prepare(question: &str, cli: &Cli, cfg: &Config) -> Result<(String, Vec<String>)> {
    let path = cli
        .doc
        .as_ref()
        .or(cli.txt.as_ref())
        .ok_or_else(|| anyhow::anyhow!("no --doc/--txt path"))?;
    let src_disp = std::fs::canonicalize(path)
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|_| path.to_string_lossy().into_owned());

    let text = parse::extract(path, cli.pages.as_deref()).await?;

    // Small inputs go inline; large ones go through retrieval.
    if text.len() <= cfg.rag_threshold {
        let prompt = format!("{question}\n\nAnalyze the following document (\"{src_disp}\"):\n\n{text}");
        return Ok((prompt, vec![src_disp]));
    }

    eprintln!(
        "Large input ({} chars) — retrieving relevant chunks via LanceDB...",
        text.len()
    );
    let chunks = chunk_text(&text, cfg.chunk_size, 150);
    let model = StaticModel::from_pretrained(EMBED_MODEL, None, None, None)
        .map_err(|e| anyhow::anyhow!("failed to load embedder: {e}"))?;
    let cache_key = format!("{src_disp}|{}|cs={}", file_sig(path), cfg.chunk_size);
    let excerpts = embed_and_search(cfg, &cache_key, &src_disp, &chunks, question, &model).await?;

    let mut ctx = String::new();
    let mut sources: Vec<String> = Vec::new();
    for (source, chunk) in &excerpts {
        ctx.push_str(&format!("[source: {source}]\n{chunk}\n\n"));
        if !sources.contains(source) {
            sources.push(source.clone());
        }
    }
    let prompt =
        format!("{question}\n\nUse the following excerpts from \"{src_disp}\" to answer:\n\n{ctx}");
    Ok((prompt, sources))
}

/// Embed chunks into a per-document LanceDB table (reused if it already exists)
/// and return the top-k nearest chunks to the query.
async fn embed_and_search(
    cfg: &Config,
    cache_key: &str,
    src_disp: &str,
    chunks: &[String],
    query: &str,
    model: &StaticModel,
) -> Result<Vec<(String, String)>> {
    let uri = cfg.cache_db.to_string_lossy().into_owned();
    let db = lancedb::connect(&uri).execute().await?;
    let name = table_name(cache_key);

    let existing = db.table_names().execute().await?;
    let tbl = if existing.contains(&name) {
        db.open_table(&name).execute().await?
    } else {
        let embs = model.encode(&chunks.to_vec());
        let dim = embs.first().map(|v| v.len()).unwrap_or(0);
        if dim == 0 {
            bail!("embedder returned no vectors");
        }
        let batch = build_batch(chunks, src_disp, &embs, dim)?;
        db.create_table(&name, batch).execute().await?
    };

    let qv = model.encode_single(query);
    let results: Vec<RecordBatch> = tbl
        .query()
        .limit(cfg.rag_topk)
        .nearest_to(qv.as_slice())?
        .execute()
        .await?
        .try_collect()
        .await?;

    let mut out = Vec::new();
    for rb in &results {
        let texts = rb
            .column_by_name("text")
            .map(|c| c.as_string::<i32>())
            .ok_or_else(|| anyhow::anyhow!("missing text column"))?;
        let srcs = rb
            .column_by_name("source")
            .map(|c| c.as_string::<i32>())
            .ok_or_else(|| anyhow::anyhow!("missing source column"))?;
        for i in 0..rb.num_rows() {
            out.push((srcs.value(i).to_string(), texts.value(i).to_string()));
        }
    }
    Ok(out)
}

fn build_batch(chunks: &[String], src: &str, embs: &[Vec<f32>], dim: usize) -> Result<RecordBatch> {
    let schema = Arc::new(Schema::new(vec![
        Field::new("text", DataType::Utf8, false),
        Field::new("source", DataType::Utf8, false),
        Field::new(
            "vector",
            DataType::FixedSizeList(Arc::new(Field::new("item", DataType::Float32, true)), dim as i32),
            true,
        ),
    ]));
    let texts = StringArray::from(chunks.to_vec());
    let srcs = StringArray::from(vec![src.to_string(); chunks.len()]);
    let vectors = FixedSizeListArray::from_iter_primitive::<Float32Type, _, _>(
        embs.iter()
            .map(|e| Some(e.iter().map(|&x| Some(x)).collect::<Vec<_>>())),
        dim as i32,
    );
    Ok(RecordBatch::try_new(
        schema,
        vec![Arc::new(texts), Arc::new(srcs), Arc::new(vectors)],
    )?)
}

/// Overlapping char-window chunks (trimmed); paragraph-aware enough for retrieval.
pub fn chunk_text(text: &str, max_chars: usize, overlap: usize) -> Vec<String> {
    let chars: Vec<char> = text.chars().collect();
    if chars.len() <= max_chars {
        let t = text.trim();
        return if t.is_empty() { vec![] } else { vec![t.to_string()] };
    }
    let overlap = overlap.min(max_chars / 4);
    let step = max_chars.saturating_sub(overlap).max(1);
    let mut out = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        let end = (i + max_chars).min(chars.len());
        let s: String = chars[i..end].iter().collect();
        let t = s.trim();
        if !t.is_empty() {
            out.push(t.to_string());
        }
        if end == chars.len() {
            break;
        }
        i += step;
    }
    out
}

fn table_name(key: &str) -> String {
    let mut h = DefaultHasher::new();
    key.hash(&mut h);
    format!("doc_{:016x}", h.finish())
}

fn file_sig(path: &Path) -> String {
    match std::fs::metadata(path) {
        Ok(m) => {
            let mtime = m
                .modified()
                .ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_secs())
                .unwrap_or(0);
            format!("{}-{}", m.len(), mtime)
        }
        Err(_) => "0".to_string(),
    }
}

fn print_sources(sources: &[String]) {
    if sources.is_empty() {
        return;
    }
    println!("\nSources:");
    for s in sources {
        println!("  - {s}");
    }
}
