//! Retrieval: model2vec-rs embeddings + lancedb store/search, chunking, TTL
//! eviction, single-doc / dir / search-all modes, Sources footer. TODO M2/M3.
use crate::cli::Cli;
use anyhow::Result;

pub fn ask(_question: &str, _cli: &Cli) -> Result<()> {
    println!("genie (rust): --ask not yet implemented (RUST_PLAN.md M1/M2). See RUST_PLAN.md");
    Ok(())
}

pub fn cache(action: &str) -> Result<()> {
    println!("genie (rust): cache {action} not yet implemented (RUST_PLAN.md M3).");
    Ok(())
}
