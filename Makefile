VERCTL := $(shell command -v cargo-verctl 2>/dev/null)

check-verctl:
ifndef VERCTL
	@echo "❌ cargo-verctl not found."
	@echo "👉 Install it via:"
	@echo "   cargo install --git https://github.com/sibman/cargo-verctl"
	@exit 1
endif

build:
	cargo build --release

test:
	cargo test

install:
	cargo install --path .

version-list: check-verctl
	cargo verctl --list

version-bump: check-verctl
	cargo verctl --bump patch

version-set: check-verctl
	cargo verctl --set 0.1.0
