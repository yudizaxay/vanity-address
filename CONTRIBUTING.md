# Contributing to vanity-address

Thank you for considering a contribution! This project is open source and community-driven.

## Getting started

```bash
git clone https://github.com/yudizaxay/vanity-address.git
cd vanity-address
cargo build --release
cargo test
```

## Project structure

```text
vanity-address/
├── vanity-core/     # Chain grinders + grinding engine (library)
├── vanity-address/     # CLI binary
├── Cargo.toml       # Workspace root
└── README.md
```

## Adding a new chain

1. Create `vanity-core/src/chains/your_chain.rs`
2. Implement the `ChainGrinder` trait
3. Register it in `vanity-core/src/chains/mod.rs`
4. Add a variant to the `Chain` enum and `ChainArg` in the CLI
5. Update README and add tests

## Pull request guidelines

- Keep PRs focused — one feature or fix per PR
- Run `cargo fmt`, `cargo clippy`, and `cargo test` before submitting
- Update README if behavior or flags change
- Never commit private keys, `.env` files, or `target/`

## Security

If you discover a security vulnerability, please open a private security advisory on GitHub rather than a public issue.
