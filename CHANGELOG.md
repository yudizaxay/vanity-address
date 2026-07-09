# Changelog

All notable changes to this project are documented here.

## [0.2.0] - 2026-07-09

### Added

- **13 blockchains:** Solana, EVM, Bitcoin, Litecoin, Dogecoin, Tron, Cosmos, Osmosis, Ripple, Stellar, Aptos, Sui, NEAR
- **Vanity Address** desktop app (`vanity-app/`) — Tauri 2 UI sharing `vanity-core` with the CLI
  - Wizard: home → chain → pattern → summary → grind → result
  - Live estimates, system profile, impractical-pattern warnings + double confirm
  - Background grinding with stop, masked keys, copy-all, native save dialog
  - Branded window title and custom app icons from `assets/logo.svg`
- Interactive CLI menu wizard with pattern estimates, difficulty bars, and impractical-pattern warnings
- Live **2-second speed benchmark** before grinding for honest ETA
- CLI flags: `--output`, `--no-benchmark`, `--force`
- Block impractical patterns in CLI unless `--force` is passed
- Optional save to `vanity-results.txt` (or custom path) with formatted key export
- Polished match screen: highlighted pattern, copy block, generate-another flow
- GitHub Release workflow (Linux x86_64 + macOS arm64 CLI binaries)
- GitHub issue templates (bug report, feature request)
- Contributor docs and PR template

### Changed

- Shared `grind_estimate` module for menu, CLI, and desktop warnings
- `CancelToken` on `vanity-core` grinder for cooperative cancel (desktop stop button)
- Auto system detection tunes rayon workers from CPU + RAM
- README: architecture diagram, desktop app section, pre-built CLI install notes

### Security

- Keys are generated locally; no network calls
- `vanity-results.txt` is gitignored — never commit private keys

[0.2.0]: https://github.com/yudizaxay/vanity-address/releases/tag/v0.2.0
