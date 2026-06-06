# Changelog

All notable changes to Gemma Genie are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.2]

### Changed
- Pinned all runtime dependencies to known-good versions for reproducible
  `uvx` resolution — see [VERSIONS.md](VERSIONS.md):
  litert-lm 0.13.1, lancedb 0.33.0, model2vec 0.8.2, numpy 2.4.6,
  pyarrow 24.0.0, ladybug 0.17.1, liteparse 2.0.6.
- Switched the embedder to `minishlab/potion-retrieval-32M` (retrieval-tuned)
  for better RAG quality, replacing `minishlab/potion-base-8M`.

## [0.2.1]

### Added
- `genie --verify-models` to check model integrity.

### Changed
- Verify model integrity using `hf_hub_download` as the source of truth.
