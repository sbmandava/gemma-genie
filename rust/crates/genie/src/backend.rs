//! Compute-backend selection: resolve (env -> cache -> probe), verify GPU with
//! litert-lm, and runtime CPU fallback. Ports the 0.2.4 bash logic. TODO M1.
use anyhow::Result;

/// Returns "gpu" or "cpu". TODO M1.
pub fn resolve() -> Result<String> {
    anyhow::bail!("backend::resolve not yet implemented (RUST_PLAN.md M1)")
}

/// Real litert-lm GPU probe (tiny generation). TODO M1.
pub fn verify_gpu() -> bool {
    false
}
