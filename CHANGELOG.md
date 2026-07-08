# Changelog

All notable changes to this project are documented here.

## [0.2.0] - 2026-07-08

### Added

- **13 blockchains:** Solana, EVM, Bitcoin, Litecoin, Dogecoin, Tron, Cosmos, Osmosis, Ripple, Stellar, Aptos, Sui, NEAR
- Interactive menu wizard with pattern estimates, difficulty bars, and impractical-pattern warnings
- Live **2-second speed benchmark** before grinding for honest ETA
- CLI flags: `--output`, `--no-benchmark`, `--force`
- Block impractical patterns in CLI unless `--force` is passed
- Optional save to `vanity-results.txt` (or custom path) with formatted key export
- Polished match screen: highlighted pattern, copy block, generate-another flow
- Contributor docs and PR template

### Changed

- Shared `grind_estimate` module for menu + CLI warnings
- Auto system detection tunes rayon workers from CPU + RAM

### Security

- Keys are generated locally; no network calls
- `vanity-results.txt` is gitignored — never commit private keys

[0.2.0]: https://github.com/yudizaxay/vanity-address/releases/tag/v0.2.0
