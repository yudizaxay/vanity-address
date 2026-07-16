.PHONY: help fmt fmt-check test test-cli test-app clippy clippy-cli clippy-app build build-cli build-app check check-ci desktop-dev desktop-build install clean

CARGO ?= cargo
NPM ?= npm

help:
	@echo "vanity-address — development targets"
	@echo ""
	@echo "  make fmt          Apply Rust formatting (cargo fmt --all)"
	@echo "  make fmt-check    Check formatting (CI)"
	@echo "  make test         Run all Rust tests"
	@echo "  make test-cli     Test vanity-core + vanity-address"
	@echo "  make test-app     Test vanity-app (Tauri)"
	@echo "  make clippy       Lint all Rust crates"
	@echo "  make build        Build CLI release binary"
	@echo "  make build-app    Build desktop frontend (npm run build)"
	@echo "  make check        fmt + test + clippy + frontend build"
	@echo "  make check-ci     Same as check but fmt-check instead of fmt"
	@echo "  make desktop-dev  Run Tauri dev server"
	@echo "  make desktop-build  Build native desktop bundle (.dmg / etc.)"
	@echo "  make install      cargo install --path vanity-address"
	@echo "  make homebrew-formula VER=0.3.5  Update Formula sha256 for a release tag"
	@echo "  make clean        cargo clean"

fmt:
	$(CARGO) fmt --all

fmt-check:
	$(CARGO) fmt --all -- --check

test: test-cli test-app

test-cli:
	$(CARGO) test -p vanity-core -p vanity-address

test-app:
	$(CARGO) test -p vanity-app

clippy: clippy-cli clippy-app

clippy-cli:
	$(CARGO) clippy -p vanity-core -p vanity-address -- -D warnings

clippy-app:
	$(CARGO) clippy -p vanity-app -- -D warnings

build:
	$(CARGO) build --release -p vanity-address

build-app:
	cd vanity-app && $(NPM) ci && $(NPM) run build

check: fmt test clippy build-app

check-ci: fmt-check test clippy build-app

desktop-dev:
	cd vanity-app && $(NPM) run tauri dev

desktop-build:
	cd vanity-app && $(NPM) ci && $(NPM) run tauri build

install:
	$(CARGO) install --path vanity-address

homebrew-formula:
	@test -n "$(VER)" || (echo "Usage: make homebrew-formula VER=0.3.5" && exit 1)
	./scripts/update-homebrew-formula.sh $(VER)

clean:
	$(CARGO) clean
