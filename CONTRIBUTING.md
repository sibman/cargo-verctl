# 🤝 Contributing to cargo-verctl

Thank you for your interest in contributing! ❤️  
This project is open to contributions from developers of all levels.

## 🧰 Development Setup

```bash
git clone https://github.com/sibman/cargo-verctl.git
cd cargo-verctl
cargo build
cargo run -- --list
cargo fmt
cargo clippy
```

## 🧪 Testing Guidelines

- Add tests for all new features.
- Use temporary directories for integration tests.
- Run `cargo test` before pushing.

## 🧱 Branching Model

- `main` — stable branch  
- `develop` — integration branch  
- `feature/<name>` — new features or fixes

## 💬 Submitting Pull Requests

- Keep PRs focused and small.
- Update `CHANGELOG.md` for new changes.
- Ensure CI passes before requesting review.

## 🧑‍💻 Code Style

Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/):
- Use `snake_case` for functions and variables.
- Use `Clippy` for lint checks.
- Use `anyhow` for uniform error handling.

## 🪪 License

Contributions are licensed under the **MIT License**.
