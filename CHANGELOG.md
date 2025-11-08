# 📜 Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/).

---

## [0.2.0] - 2025-11-03
### Added
- Workspace-wide version management (`--only`, `--list`, auto-detect `[workspace]`).
- Automatic `[package].version` creation if missing.
- `--set <version>` option for manual version assignment.
- `--auto` flag for CI/CD automation.
- MIT open-source license and GitHub Actions CI workflow.
- Initial public release of `cargo-verctl`.

---

## [0.1.0] - 2025-10-30
### Added
- Basic version bump functionality for single Cargo.toml (`--bump patch|minor|major`).
- Interactive prompt mode for local builds.
