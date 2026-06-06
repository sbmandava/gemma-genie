//! End-to-end tests that run the `genie` binary. The model-backed tests are
//! `#[ignore]`d (they need the Gemma weights + GPU/CPU and are slow); run them
//! with `cargo test -- --ignored`. They use the sample corpus under
//! /opt/projects/unovie/dataingest/sample.
use std::path::Path;
use std::process::Command;

const SAMPLE: &str = "/opt/projects/unovie/dataingest/sample";

fn genie() -> Command {
    Command::new(env!("CARGO_BIN_EXE_genie"))
}

fn fixture(name: &str) -> String {
    format!("{}/tests/fixtures/{}", env!("CARGO_MANIFEST_DIR"), name)
}

// ---- fast (no model) ----

#[test]
fn version_prints() {
    let out = genie().arg("--version").output().unwrap();
    assert!(out.status.success());
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains("0.2"), "version output: {s}");
}

#[test]
fn help_lists_flags_and_subcommands() {
    let out = genie().arg("--help").output().unwrap();
    assert!(out.status.success());
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains("--ask"), "help missing --ask: {s}");
    assert!(s.contains("doctor"), "help missing doctor: {s}");
}

// ---- model-backed (slow; `cargo test -- --ignored`) ----

#[test]
#[ignore = "runs the model (weights + GPU/CPU); slow"]
fn doctor_reports_backend() {
    let out = genie().arg("doctor").output().unwrap();
    assert!(out.status.success());
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains("compute backend"), "doctor output: {s}");
}

#[test]
#[ignore = "runs the model; slow"]
fn txt_inline_answer_uses_source() {
    let f = fixture("notes.txt");
    let out = genie()
        .args(["--ask", "Who owns Apollo? Reply with just the name.", "--txt", &f])
        .output()
        .unwrap();
    assert!(out.status.success(), "stderr: {}", String::from_utf8_lossy(&out.stderr));
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.to_lowercase().contains("jane"), "expected 'Jane', got: {s}");
    assert!(s.contains("Sources:"), "missing Sources footer: {s}");
}

#[test]
#[ignore = "runs the full RAG path over a large PDF; slow"]
fn pdf_rag_answer_with_sources_no_noise() {
    let pdf = format!("{SAMPLE}/navrules.pdf");
    if !Path::new(&pdf).exists() {
        eprintln!("skipping: sample {pdf} not present");
        return;
    }
    let out = genie()
        .args(["--ask", "What do these rules govern? One sentence.", "--doc", &pdf])
        .output()
        .unwrap();
    assert!(out.status.success(), "stderr: {}", String::from_utf8_lossy(&out.stderr));
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(!s.trim().is_empty(), "empty answer");
    assert!(s.contains("Sources:"), "missing Sources footer");
    assert!(!s.contains("Loaded OpenCL"), "GPU noise leaked to stdout");
}

#[test]
#[ignore = "runs liteparse+soffice over a PPTX then the model; slow"]
fn pptx_answer() {
    let pptx = format!("{SAMPLE}/unovie-country.pptx");
    if !Path::new(&pptx).exists() {
        eprintln!("skipping: sample {pptx} not present");
        return;
    }
    let out = genie()
        .args(["--ask", "Summarize this deck in one sentence.", "--doc", &pptx])
        .output()
        .unwrap();
    assert!(out.status.success(), "stderr: {}", String::from_utf8_lossy(&out.stderr));
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(!s.trim().is_empty(), "empty answer");
}
