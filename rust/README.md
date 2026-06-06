# Gemma Genie — Rust rewrite

Single-binary Rust port of the bash + Python implementation in [`../python/`](../python/).
Design and milestones: [`../RUST_PLAN.md`](../RUST_PLAN.md). Dependency wiring and
build prerequisites: [`../CLAUDE.md`](../CLAUDE.md).

**Status: M0 scaffold.** The CLI parses every flag/subcommand of the bash `genie`
and dispatches to module stubs that print which milestone implements them. No
behaviour yet.

## Build & run

```sh
cd rust
cargo run -- --help
cargo run -- --ask "why is the sky blue"
cargo run -- doctor
```

## Layout

```
rust/
├── Cargo.toml            # workspace (+ commented [patch] for the lance core)
└── crates/genie/
    ├── Cargo.toml        # binary crate (clap; deps added per milestone)
    └── src/
        ├── main.rs       # dispatch
        ├── cli.rs        # clap CLI surface
        ├── config.rs     # paths + env overrides            (M1)
        ├── backend.rs    # GPU resolve/verify + CPU fallback (M1)
        ├── llm.rs        # litert-lm subprocess + streaming  (M1)
        ├── parse.rs      # liteparse extraction              (M2)
        ├── rag.rs        # model2vec-rs + lancedb            (M2/M3)
        ├── graph.rs      # lbug correlation graph            (M4)
        ├── models.rs     # HF download/verify, lifecycle     (M5)
        └── doctor.rs     # dependency + backend report       (M1)
```

## Dependencies

Added per milestone from **local** `research/` sources — see `../CLAUDE.md`.
Native prerequisites: CMake + C++ (lbug), PDFium and Tesseract (liteparse). The
`litert-lm` runtime is subprocessed from the prebuilt binary in v1.
