# Changelog

All notable changes to this project are documented here.

## [0.3.5] - 2026-07-16

### Added

- **Install docs:** why `cargo install` is slow, faster pre-built binary path, and how to uninstall
- **Homebrew tap workflow:** formula v0.3.5, `scripts/update-homebrew-formula.sh`, `scripts/sync-homebrew-tap.sh`, [docs/HOMEBREW.md](docs/HOMEBREW.md), [AGENTS.md](AGENTS.md) project memory

### Changed

- **`cargo install` compile time:** release profile in the published CLI crate uses `lto = false` and higher `codegen-units` (faster first-time compile; GitHub Release binaries still use workspace LTO)

## [0.3.4] - 2026-07-14

### Fixed

- **crates.io README links** — docs/SECURITY/CONTRIBUTING/LICENSE use absolute GitHub URLs (relative links 404 from the `vanity-address/` crate path)

### Changed

- Workspace version unified to **0.3.4**

## [0.3.3] - 2026-07-14

### Fixed

- **crates.io README images** — logo/demo use absolute GitHub raw URLs (relative `assets/` paths break on crates.io)

### Changed

- Workspace version unified to **0.3.3**

## [0.3.2] - 2026-07-10

### Added

- **Windows desktop installer** — `VanityAddress-*-Windows-Desktop.exe` (NSIS) in GitHub Releases
- **CI on Windows** — CLI + desktop app jobs so Windows regressions are caught without a local Windows machine
- **crates.io ready** — `vanity-core` + `vanity-address` at **0.3.2** (`cargo install vanity-address` after publish)

### Fixed

- **Windows CLI:** typing prefix/suffix no longer doubles every character (crossterm Press+Release)
- **Windows CLI:** Enter / `\r` handled correctly when confirming text input
- **Windows CLI:** screen clear uses crossterm WinAPI (ANSI escapes alone can fail in older consoles)
- **Windows CLI:** Backspace on empty input no longer underflows the buffer

### Changed

- Workspace version unified to **0.3.2**
- README / install docs point at **v0.3.2** release assets
- Dependencies: `sha3` 0.12, `rayon` 1.12, `rand` 0.8.7, `clap` 4.6, `serde_json` 1.0.150
- Dependabot: ignore `sha3` major/minor bumps (manual crypto updates only)

## [0.3.0] - 2026-07-09

### Added

- **`--json` CLI flag** — machine-readable stdout for scripts and automation (errors on stderr as JSON)
- **Makefile** — `make fmt`, `make test`, `make check`, `make check-ci`, `make desktop-build`
- **SECURITY.md** — vulnerability reporting and private-key handling policy
- **Dependabot** — weekly Rust/npm and monthly GitHub Actions updates
- **Homebrew formula** — `Formula/vanity-address.rb` (build from source or tap)
- **Desktop demo asset** — `assets/demo-desktop.svg` for README
- **Release binaries:**
  - Windows x86_64 (`.zip`)
  - macOS Intel x86_64 (`.tar.gz`)
  - macOS desktop `.dmg` bundle (arm64, in release tarball)
- **crates.io metadata** on `vanity-core` and `vanity-address` for publishing

### Changed

- Workspace version unified to **0.3.0** (`vanity-core`, CLI, desktop app)
- README: install options (Homebrew, cargo install, JSON mode), desktop demo, `make check`
- Release workflow builds CLI for 4 platforms + desktop `.dmg`

### Security

- Documented JSON output may include private keys on stdout — use in trusted environments only

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

[0.3.5]: https://github.com/yudizaxay/vanity-address/releases/tag/v0.3.5
[0.3.4]: https://github.com/yudizaxay/vanity-address/releases/tag/v0.3.4
[0.3.3]: https://github.com/yudizaxay/vanity-address/releases/tag/v0.3.3
[0.3.2]: https://github.com/yudizaxay/vanity-address/releases/tag/v0.3.2
[0.3.0]: https://github.com/yudizaxay/vanity-address/releases/tag/v0.3.0
[0.2.0]: https://github.com/yudizaxay/vanity-address/releases/tag/v0.2.0
