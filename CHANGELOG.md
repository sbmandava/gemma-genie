# Changelog

All notable changes to Gemma Genie are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.3]

### Fixed
- `genie --ask` no longer exits non-zero after printing a correct answer — an
  empty "Sources" footer made `grep` fail and tripped `set -euo pipefail`.

### Added
- `--doc` on Office formats (DOCX/XLSX/PPTX and ODF) now exits early with an
  OS-specific install command when LibreOffice's `soffice` is missing, instead
  of failing deep inside the parser.

### Changed
- Suppressed harmless GPU/OpenCL backend log noise from the on-device runtime
  (the `maxDynamic…` warnings and the `Loaded OpenCL library` line), and
  tolerate a non-zero exit from the litert-lm GPU backend after a successful
  generation.
- `genie doctor` now reports the compute backend genie will actually use
  (GPU/CPU) and actively verifies the GPU runs with litert-lm, falling back to
  CPU — and correcting the cached choice — when it doesn't.
- LibreOffice install hints are OS-aware (Homebrew / apt / dnf / yum / pacman /
  zypper) rather than always suggesting Homebrew, and `install.sh` installs only
  the minimal headless components instead of the full GUI suite.

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
