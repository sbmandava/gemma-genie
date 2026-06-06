//! litert-lm invocation. v1 subprocesses the prebuilt `litert-lm` binary with
//! noise filtering, live streaming, and CPU fallback. FFI is M6. TODO M1.
use crate::cli::Cli;
use anyhow::Result;
use std::path::Path;

pub fn describe_image(_path: &Path, _cli: &Cli) -> Result<()> {
    todo("--image", "M1")
}

pub fn transcribe_audio(_path: &Path, _cli: &Cli) -> Result<()> {
    todo("--audio", "M1")
}

fn todo(what: &str, milestone: &str) -> Result<()> {
    println!("genie (rust): {what} not yet implemented ({milestone}). See RUST_PLAN.md");
    Ok(())
}
