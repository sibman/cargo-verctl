# 🦀 cargo-verctl

[![CI](https://github.com/sibman/cargo-verctl/actions/workflows/ci.yml/badge.svg)](https://github.com/sibman/cargo-verctl/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Crates.io](https://img.shields.io/badge/cargo-subcommand-orange.svg)]()

**Cargo subcommand for managing versions across single crates and Rust workspaces.**  
Supports `bump`, `set`, `list`, and workspace-wide version management.

## ✨ Features

- ✅ Bump patch/minor/major versions interactively or automatically  
- ✅ Set explicit versions (`--set 1.0.0`)  
- ✅ Detect and manage workspace subprojects independently  
- ✅ Create `[package].version` if missing  
- ✅ Cross-platform (Windows, macOS, Linux)  
- ✅ Perfect for CI/CD pipelines and Makefiles  

## 📦 Installation

```bash
cargo install --git https://github.com/sibman/cargo-verctl
```

Then use it as a Cargo subcommand:

```bash
cargo verctl --help
```

## 🚀 Usage Examples

### List crate versions
```bash
cargo verctl --list
```

### Bump versions
```bash
cargo verctl --bump patch
cargo verctl --bump minor
cargo verctl --bump major
```

### Set a specific version
```bash
cargo verctl --set 1.0.0
```

### Manage workspace members
```bash
cargo verctl --bump patch
cargo verctl --only crate-a --set 1.0.0
cargo verctl --list
```

## 🧰 CI/CD Example

```yaml
- uses: actions/checkout@v4
- uses: actions-rs/toolchain@v1
  with:
    toolchain: stable
- run: cargo install --path .
- run: cargo verctl --auto --bump patch
```

## ⚖️ License

MIT License © 2025 [Andrey Yelabugin (sibman)](https://github.com/sibman)
