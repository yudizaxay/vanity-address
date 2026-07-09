# Contributing to vanity-address

Thank you for helping improve **vanity-address**! Contributions of all kinds are welcome — new blockchains, bug fixes, UX polish, docs, and performance improvements.

## Quick start

```bash
git clone https://github.com/yudizaxay/vanity-address.git
cd vanity-address
cargo build --release
cargo test
cargo clippy -- -D warnings
```

Before opening a PR, run:

```bash
make check
# or CI-equivalent:
make check-ci
```

Equivalent manual steps:

```bash
cargo fmt --all
cargo test
cargo clippy -- -D warnings
cd vanity-app && npm ci && npm run build && cd ..
```

See [SECURITY.md](SECURITY.md) for vulnerability reporting — do not open public issues for sensitive findings.

---

## What you can contribute

| Type | Examples |
|------|----------|
| **New blockchain** | Cardano, TON, Bitcoin SegWit (`bc1…`), more Cosmos hubs |
| **Bug fixes** | Menu input, wrong address derivation, pattern matching |
| **Features** | `--output` path, regex patterns, benchmark mode, Tauri UI |
| **Docs** | README, chain-specific wallet import notes |
| **Tests** | Address derivation unit tests, pattern validation |

Not sure where to start? Open an [issue](https://github.com/yudizaxay/vanity-address/issues/new) first — we can align on approach before you code.

---

## Project structure

```text
vanity-address/
├── vanity-core/              # Library: grinding engine + chains
│   └── src/
│       ├── chain.rs          # ChainGrinder trait, GrindAttempt, KeypairResult
│       ├── chains/             # One module per chain (or chain family)
│       │   ├── util.rs         # Shared crypto (base58, hash160, patterns…)
│       │   ├── mod.rs          # Chain enum, from_id(), MENU_CHAINS
│       │   ├── solana.rs
│       │   ├── evm.rs
│       │   └── …
│       ├── grinder.rs          # Parallel grind loop (rayon)
│       └── system.rs           # CPU/RAM detection
├── vanity-address/           # CLI binary
│   └── src/
│       ├── main.rs             # Direct CLI + grind output
│       ├── menu.rs             # Interactive wizard
│       └── terminal.rs         # Keypress input
└── README.md
```

**Rule of thumb:** chain logic lives in `vanity-core`; UI/CLI lives in `vanity-address`.

---

## Adding a new blockchain

### 1. Implement `ChainGrinder`

Create `vanity-core/src/chains/your_chain.rs` and implement the trait in `chain.rs`:

```rust
pub trait ChainGrinder: Send + Sync + Clone {
    fn id(&self) -> &'static str;
    fn display_name(&self) -> &'static str;
    fn grind_attempt(&self) -> (String, GrindAttempt);
    fn finalize(&self, attempt: GrindAttempt) -> KeypairResult;
    fn build_pattern(...) -> Result<Pattern, String>;
    fn expected_attempts(&self, pattern: &Pattern) -> f64;
    fn matches(&self, address: &str, pattern: &Pattern) -> bool;
    fn supports_exact_case(&self) -> bool;
    fn pattern_hint(&self) -> &'static str;
}
```

**GrindAttempt variants:**

- `GrindAttempt::Solana(Keypair)` — Solana only
- `GrindAttempt::Secret32([u8; 32])` — most other chains (secp256k1 or ed25519 seed)

Reuse helpers in `chains/util.rs` when possible:

- **secp256k1** (BTC, LTC, DOGE, TRX, Cosmos, XRP): `grind_secp256k1`, `p2pkh_address`, `build_base58_pattern`
- **ed25519** (Stellar, Aptos, Sui, NEAR): `grind_ed25519` + Solana `Keypair` (avoids `ed25519-dalek` / `zeroize` conflicts with `solana-sdk`)
- **hex** (EVM, Aptos, Sui): `build_hex_pattern`, `hex_combinations`
- **bech32** (Cosmos): `bech32` crate + `BECH32_CHARSET`

Look at an existing chain similar to yours — e.g. `tron.rs` for base58+keccak, `cosmos.rs` for bech32.

### 2. Register the chain

In `vanity-core/src/chains/mod.rs`:

1. `mod your_chain;` + `pub use`
2. Add variant to `enum Chain`
3. Add entry to `MENU_CHAINS` (interactive menu label)
4. Add arm in `from_menu_index()` and `from_id()` (include aliases, e.g. `eth` → `evm`)
5. Add arm in the `dispatch!` macro for `ChainGrinder` impl
6. Update `all_ids()` if applicable

### 3. Dependencies

Add crates only to `vanity-core/Cargo.toml` if needed. **Avoid** pulling in `ed25519-dalek` v2 — it conflicts with `solana-sdk`'s `zeroize`. Use Solana's `Keypair` for ed25519 chains instead.

### 4. Tests

Add unit tests in your chain module (see `bitcoin_like.rs` or `evm.rs`):

- Address format (prefix char, length)
- Pattern validation (reject invalid chars)
- `matches()` for a known address + pattern

### 5. Docs

- Update **README** — chain list, `--chain` flag, pattern rules
- Add wallet import hint in `finalize()` → `KeyExport.hint`

### 6. Verify manually

```bash
cargo run -- --chain your_id --suffix ab
# Interactive: pick chain from menu, short pattern, confirm address in a testnet explorer
```

---

## Bug fixes & small changes

- One logical fix per PR when possible
- Add a test if the bug could regress
- Mention steps to reproduce in the PR description

## New features

- Open an issue first for large changes (UI rewrite, regex engine, new output formats)
- Keep CLI flags consistent with existing style (`--long-flag`)
- Interactive menu changes should stay keyboard-first (no Enter for simple choices)

---

## Pull request checklist

Copy into your PR (or use the [PR template](.github/PULL_REQUEST_TEMPLATE.md)):

- [ ] `cargo fmt --all` applied (CI runs `cargo fmt --all -- --check`)
- [ ] `cargo test` passes
- [ ] `cargo clippy -- -D warnings` passes
- [ ] `vanity-app`: `npm ci && npm run build` passes (if UI changed)
- [ ] README updated (if chains, flags, or UX changed)
- [ ] No private keys, `vanity-results.txt`, `.env`, or `target/` committed
- [ ] New chain: address derivation verified against official wallet/docs
- [ ] PR title is clear (e.g. `Add Cardano grinder`, `Fix menu selection for option 13`)

### PR title examples

- `feat: add TON vanity grinder`
- `fix: wait longer for multi-digit menu selection`
- `docs: add Cosmos pattern examples to README`
- `perf: reduce allocations in grind loop`

---

## Code style

- Match existing naming and module layout
- Prefer extending `util.rs` over duplicating base58/hex logic
- Comments only for non-obvious crypto or protocol details
- No drive-by refactors unrelated to your change

---

## Security

- **Never** commit real private keys or `vanity-results.txt`
- Vanity grinding is probabilistic — document limitations honestly
- If you find a security vulnerability, use [GitHub Security Advisories](https://github.com/yudizaxay/vanity-address/security/advisories) (private) — not a public issue

---

## License

By contributing, you agree that your contributions will be licensed under the [MIT License](LICENSE).
