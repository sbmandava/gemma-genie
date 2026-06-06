# Pinned Versions

Gemma Genie resolves all of its runtime dependencies on demand via `uvx`. To
keep resolution reproducible — so a cache miss can't silently pull a newer,
possibly breaking release — every dependency is pinned to a known-good version.

This file is the single source of truth. When bumping a version, update the
pinned value **here and at every invocation site listed below**, then re-test.

## Versions

| Package        | Version  | Role                                              |
|----------------|----------|---------------------------------------------------|
| litert-lm      | 0.13.1   | On-device LLM runtime (Gemma inference)           |
| lancedb        | 0.33.0   | Vector store for RAG retrieval                    |
| model2vec      | 0.8.2    | Static embedder (minishlab/potion-retrieval-32M)  |
| numpy          | 2.4.6    | Numerics (RAG helper)                             |
| pyarrow        | 24.0.0   | LanceDB data layer                                |
| ladybug        | 0.17.1   | Embedded Cypher graph DB (entity correlation)     |
| liteparse      | 2.0.6    | Document extraction (PDF/DOCX/XLSX/PPTX/images)   |

## Where each pin lives

- **litert-lm** — `LITERT_VERSION="0.13.1"` in `genie` and `install.sh`; used as
  `litert-lm@${LITERT_VERSION}` (`genie:487`, `genie:489`, `install.sh:188`).
  Hardcoded as `litert-lm@0.13.1` in `genie_graph.py:82`. Not env-overridable.
- **lancedb** — `--with lancedb==0.33.0` in `genie` and `install.sh`;
  `dependencies` block in `genie_rag.py`.
- **model2vec** — `--with model2vec==0.8.2` in `genie` and `install.sh`;
  `dependencies` block in `genie_rag.py`.
- **numpy** / **pyarrow** — `dependencies` block in `genie_rag.py`.
- **ladybug** — `--with ladybug==0.17.1` in `genie` and `install.sh`;
  `dependencies` block in `genie_graph.py`.
- **liteparse** — `--from liteparse==2.0.6` in `genie`, `genie_rag.py`, and
  `install.sh`.
