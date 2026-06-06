//! Correlation graph via lbug (ladybug): (:File)-[:Mentions]->(:Entity),
//! heuristic + LLM entity extraction, stats/query/correlate. TODO M4.
use anyhow::Result;

pub fn stats() -> Result<()> {
    println!("genie (rust): --graph-stats not yet implemented (RUST_PLAN.md M4).");
    Ok(())
}

pub fn query(_cypher: &str) -> Result<()> {
    println!("genie (rust): --graph-query not yet implemented (RUST_PLAN.md M4).");
    Ok(())
}
