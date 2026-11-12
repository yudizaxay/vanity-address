# Changelog

All notable changes to this project are documented here.

## [Unreleased]

## [0.8.0] - 2026-11-12

### Added

- **Zcash** (`zec`): transparent mainnet P2PKH `t1…` (2-byte version); shielded/Unified out of scope
- **Bitcoin Cash** (`bch`): CashAddr P2PKH `q…` / `bitcoincash:q…`
- **Stacks** (`stx`): mainnet c32check `SP…`
- **Decred** (`dcr`): mainnet P2PKH `Ds…` (BLAKE256 hash160 + double-BLAKE256 checksum)
- **Nervos CKB** (`ckb`): short secp256k1 address `ckb1qyq…` (blake160 + bech32)
- Menu / web / SEO guides for the five new chains — **49** wallet chains total

### Changed

- Version bump across core, CLI, desktop, wasm, and npm packages to **0.8.0**

## [0.7.1] - 2026-10-29

### Changed

- docs: canonical site vanity-address.online — npm `homepage` was still the old `pages.dev` URL on 0.7.0; npm keywords expanded for search. npm main package only (platform binaries stay 0.7.0; `publish-npm.sh --main-only`)
- **Canonical website** is now [vanity-address.online](https://vanity-address.online/) (Cloudflare Pages custom domain). `landing/_worker.js` 301s `www` to the apex. Docs, npm `homepage`, and landing canonical/OG/schema URLs updated.
- Crate `homepage` (`vanity-core`, `vanity-address`) and the Homebrew formula now point to the website; README, npm README, `docs/INSTALL.md`, `docs/USAGE.md`, `docs/NPM.md` link the new guides and difficulty calculator.
- SEO cluster: per-chain guides, `/chains/` hub, difficulty calculator, `404.html`, PNG Open Graph image, sitemap with `lastmod` (see `scripts/gen-seo-pages.py` + `docs/GSC.md`)

## [0.7.0] - 2026-10-20

### Added

- **Bitcoin-family P2PKH forks** (Phase 1): `btg`, `rvn`, `mona`, `vtc`, `dgb`, `ppc`, `qtum`, `via`, `xpm`, `emc2`, `sys`, `nvc`, `cdn` — verified version + WIF bytes; see `docs/BITCOIN_FORKS.md`
- **Bitcoin puzzle mode** (CLI): `--puzzle <bits>` (+ optional `--puzzle-start <hex>`) — restricted secp256k1 keyspace grind on BTC P2PKH
- **Split-key vanity** (Phase 2): `split-key gen` / `--split-pubkey` (`-P`) / `split-key combine` for Bitcoin-family P2PKH; desktop `split_key_gen` / `split_key_combine`
- **Security pack**: `docs/SECURITY-MODEL.md` (threat model, not-a-wallet, not formally audited); stronger `--save` / export warnings; Safety UX on landing + `/generate/`
- **OpenCL GPU** (Phase 3): feature `opencl` on CLI/desktop; `--gpu auto|require|off`; `gpu devices` / `gpu bench`; mandatory CPU re-verify. **EVM** uses incremental EC with batch inversion (~46M keys/s, ~90× CPU on Apple M4; other GPUs untested). **Solana** is experimental and slower than CPU, so `--gpu auto` leaves it on CPU. Web/npm WASM remain CPU-only. See `docs/BENCHMARKS.md` + `scripts/bench.sh`
- **0.7.0 roadmap** in `docs/ROADMAP-0.7.md`

## [0.6.7] - 2026-10-19

### Added

- **Grind power presets** (`eco` · `balanced` · `fast` · `extreme`) in `vanity-core`, CLI (`--preset` + interactive Power step), desktop summary picker, and web generator — memory caps still apply; `--threads` wins over preset

### Changed

- Landing PageSpeed / AI crawl hygiene: drop invalid `Agentmap` from `robots.txt`, schema-valid empty `ai-catalog.json`, homepage CSP to block Cloudflare Insights beacon, softer hero layout reads

## [0.6.6] - 2026-10-16

### Added

- **Tron contract vanity (`trx-contract`)** — CREATE / nonce 0, same idea as `evm-contract`, base58 `T…` address
- **CREATE2 in the npm SDK** — `chain: "create2"` with `deployer` + `initCodeHash`; `listDeployModes()` for discovery
- Deploy modes in the **desktop** chain picker (including CREATE2 factory fields) and CLI interactive digit menu
- **Web generator** Address | Contract header toggle — `evm-contract`, `trx-contract`, `sol-mint`, CREATE2 (factory fields); random starting chain on each load
- WASM exports: `list_deploy_modes`, optional factory args on grind / estimate / validate / chain_details
- npm SDK: export `listDeployModes` from ESM entry points; contract/mint aliases covered in demo + tests

## [0.6.5] - 2026-10-15

### Fixed

- **Wait-time estimates** match real grind odds for Base58 and TON when ignore-case is on (prefix / suffix / contains) — alphabet-aware combinations instead of a blind per-character factor
- Web generator grind visor HUD no longer clips the last letter of short search words

### Changed

- Landing site hosts on **Cloudflare Pages**; CSP via `landing/_headers`. Docs and package homepages point at the site URL

## [0.6.4] - 2026-10-12

### Added

- **Web generator** at `/generate/` on the landing site — pick a chain, type a word, generate, save; runs the same Rust engine as WASM on all CPU cores in the browser. Keys never leave the tab and are never stored: no cookies, no third-party requests, strict CSP, safety notice before first use (remembered on the device, reopen via "Safety"), keys blurred until shown, dark and light themes
- `scripts/build-web.sh` builds the browser engine into `landing/generate/wasm/`

### Changed

- WASM engine (web app + npm SDK) re-derives every found key and discards it unless it reproduces the matched address
- Dependency updates: Tauri 2.12 (dialog 2.8, opener 2.7), clap 4.6, rand 0.8.8, base64 0.23, crc32fast 1.5, Vite 8.3

## [0.6.3] - 2026-10-08

### Changed

- **Difficulty is judged by time, not length** — warnings now come from the estimated time on your machine (hours / a week or more / not practical), so 8 hex characters and 8 base58 characters are no longer treated alike
- Interactive summary measures real speed (1s) and shows a per-character guide: verdict plus "each character ≈ N× longer · one fewer ≈ …"; the grind reuses that speed instead of warming up again
- Progress line shows "% of average · avg …" instead of a misleading "minutes remaining" countdown
- Desktop app uses the same warnings and pattern guide

### Fixed

- CLI direct mode calibrates speed before warning or blocking, so the warning matches the shown estimate
- npm 0.6.2 shipped the 0.6.1 CLI binary; `publish-npm.sh` now refuses to publish a stale binary

## [0.6.2] - 2026-10-05

### Fixed

- **Impossible prefixes are rejected** instead of grinding forever — e.g. Kaspa `ax` (always `kaspa:q` + `p/q/r/z`), Cardano, TON, Stellar, Litecoin, Dogecoin, Dash, Tron, Tezos, Kusama. The error names the characters that can appear there
- **Accurate difficulty estimates** — exact odds for base58 address starts (they are not uniform), and fixed starts (`0x`, `kaspa:q`, `cosmos1`, `1`, `D`, `tz1`…) no longer count as difficulty, so "Easy" and the long-pattern warning agree
- Fixed starts are added automatically (`--chain ada --prefix 8` → `addr1v8…`); ambiguous input picks the reading that can exist (Doge `D1` → `DD1…`)
- Stellar patterns validated as base32 (was base58)
- Interactive wizard shows the selected chain on every step and the target before grinding

## [0.6.1] - 2026-10-05

### Added

- **EVM contract vanity (`--chain evm-contract`)** — grind a deployer key so its first deployment (CREATE, nonce 0) lands on a matching contract address; no factory or init code hash needed
- **Solana token mint vanity (`--chain sol-mint`)** — mint keypair JSON ready for `spl-token create-token mint.json`
- **EVM aliases** — `hyperliquid` / `hyperevm`, `sonic`
- `verify` works with `evm-contract` and `sol-mint`

### Security

- Saved key files (CLI `--save` and desktop Save) are created as owner-only `0600` on macOS/Linux; existing files are tightened
- Secret buffers and exported key strings are zeroized when dropped

## [0.6.0] - 2026-09-21

### Added

- **Contains + wildcards** — `--contains cafe`, `Cool*xyz` in prefix/suffix/contains
- **Multi-pattern OR** — `--suffix moon,pump,dao` (comma alternatives)
- **Find N** — `--count 5`
- **Verify** — `vanity-address verify --chain sol --address … --key …`
- **Bitcoin SegWit + Taproot** — `btc-segwit` (`bc1q…`), `btc-taproot` (`bc1p…`)
- **Trending Cosmos HRPs** — Sei, Injective, Celestia, dYdX
- **EVM aliases** — `robinhood`, `base`, `arb`, `optimism`, `polygon`, `avax`, `bnb`, `monad`, …
- **CREATE2 lite** — `--create2 --deployer … --init-code-hash …` salt grind for contract vanity
- Wallet export polish — Solana Phantom JSON byte array; clearer MetaMask/ethers hints
- npm SDK `contains` option (wasm + worker pool)

### Changed

- **31 chains** in the interactive menu (was 25)
- Pattern engine shared across all grinders

## [0.5.0] - 2026-09-15

### Added

- **Powerful npm SDK** — `estimateDifficulty()`, `validatePattern()`, `listChains()`, `isValidChain()`, `generateAddresses({ count })`
- **Multi-core Node grind** via `worker_threads` (default = CPU count; `workers: 1` to disable)
- Richer progress: `onProgress(attempts, { keysPerSec, etaSeconds })` (first arg still a number)
- `wallet.privateKey` convenience field + `timeoutMs` option
- Typed `ChainId` union in TypeScript definitions
- WASM exports: `estimate_difficulty`, `validate_pattern`, `list_chains`, `is_valid_chain`

### Changed

- Package / crates descriptions emphasize offline + no telemetry
- SDK docs and npm README updated for the 0.5 API

## [0.4.1] - 2026-08-25

### Changed

- **WASM SDK now includes Solana** — all **25 chains** work in CLI, desktop, and the npm `generateAddress()` API
- Removed `solana-sdk` as a normal `vanity-core` dependency (dev-dep regression test kept); unified `MENU_CHAINS` for native + wasm

## [0.4.0] - 2026-08-25

### Added

- **Programmatic WASM SDK** — `generateAddress()` in the `vanity-address` npm package (Node + bundlers/browsers); see [docs/SDK.md](docs/SDK.md)
- `vanity-wasm` crate + `./scripts/build-wasm.sh` for shipping prebuilt wasm
- Runnable SDK demo: `npm/vanity-address/examples/demo.js`
- CI `wasm-sdk` job (build + publint)

### Changed

- Native Solana grinding remains CLI/desktop-only; wasm builds exclude Solana (24 chains in browser/Node SDK)
- npm package now exports JS/TS API entry points alongside the CLI shim

## [0.3.8] - 2026-07-28

### Added

- **3 new chains** (25 total): Dash P2PKH (`dash`), Kusama SS58 ed25519 (`ksm`), MultiversX `erd1` bech32 (`erd` / `mvx` / `elrond`)
- Known-vector tests: MultiversX Alice address, Kusama SS58 prefix 2

## [0.3.7] - 2026-07-20

### Added

- **9 new chains** (22 total): Algorand (`algo`), Tezos tz1 (`xtz`), Internet Computer (`icp`), Kaspa (`kaspa`), TON Wallet V4R2 (`ton`), Filecoin f1 (`fil`), Polkadot SS58 ed25519 (`dot`), Cardano enterprise (`ada`), Hedera ed25519 pubkey (`hedera`)
- Known-vector tests vs third-party refs (rusty-kaspa, pytezos, substrate, bip_utils, go-address) for Kaspa, Tezos, Polkadot, Algorand, Filecoin (+ existing TON)

### Fixed

- **Ripple (XRP) alphabet** — corrected corrupted base58 charset (was 57 chars with duplicates; could panic mid-grind). Now full 58-char XRPL alphabet.
- **Kaspa checksum** — was BIP-173 bech32; now CashAddr-style 8-char checksum (rusty-kaspa vectors)
- **Tezos `edsk` export** — used expanded-key prefix on a 32-byte seed; fixed to seed prefix `[13,15,58,7]` (pytezos vector)

### Changed

- Chain menu / docs ordered **A–Z** by display name
- Shared `der_ed25519_spki()` helper for ICP + Hedera

### Notes

- **Hedera:** vanities the ed25519 public key hex — account IDs (`0.0.N`) are assigned on-chain when you create the account
- **Polkadot:** ed25519 SS58 (prefix 0); sr25519 wallets may differ
- **Cardano:** Shelley enterprise (payment-only) `addr1…` addresses
- **TON:** Wallet V4R2 non-bounceable `UQ…` (Tonkeeper-style)
- **ICP:** prefer `--prefix` for vanity; encoded principals always end with `e` (self-authenticating tag)

## [0.3.6] - 2026-07-17

### Added

- **npm distribution live** — `npx vanity-address` / `npm install -g vanity-address` via platform packages with `optionalDependencies` (no postinstall, no Rust needed)
- **npm README** — full explainer: what a vanity address is, examples, chains table, safety Q&A

### Changed

- Version unified to **0.3.6** across crates.io, GitHub Releases, Homebrew tap, and npm

## [0.3.5] - 2026-07-16

### Added

- **Install docs:** why `cargo install` is slow, faster pre-built binary path, and how to uninstall
- **Homebrew tap workflow:** formula v0.3.5, `scripts/update-homebrew-formula.sh`, `scripts/sync-homebrew-tap.sh`, [docs/HOMEBREW.md](docs/HOMEBREW.md), [AGENTS.md](AGENTS.md) project memory
- **npm CLI wrapper:** `npm/vanity-address` + platform packages, [docs/NPM.md](docs/NPM.md), `scripts/prepare-npm.sh` / `publish-npm.sh`

### Changed

- **`cargo install` compile time:** release profile in the published CLI crate uses `lto = false` and higher `codegen-units` (faster first-time compile; GitHub Release binaries still use workspace LTO)
- **Docs / memory:** README + INSTALL curl examples → v0.3.5; `AGENTS.md` synced (Homebrew live, `std_cargo_args`, tap trust)

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

[0.6.0]: https://github.com/yudizaxay/vanity-address/releases/tag/v0.6.0
[0.5.0]: https://github.com/yudizaxay/vanity-address/releases/tag/v0.5.0
[0.4.1]: https://github.com/yudizaxay/vanity-address/releases/tag/v0.4.1
[0.4.0]: https://github.com/yudizaxay/vanity-address/releases/tag/v0.4.0
[0.3.8]: https://github.com/yudizaxay/vanity-address/releases/tag/v0.3.8
[0.3.7]: https://github.com/yudizaxay/vanity-address/releases/tag/v0.3.7
[0.3.6]: https://github.com/yudizaxay/vanity-address/releases/tag/v0.3.6
[0.3.5]: https://github.com/yudizaxay/vanity-address/releases/tag/v0.3.5
[0.3.4]: https://github.com/yudizaxay/vanity-address/releases/tag/v0.3.4
[0.3.3]: https://github.com/yudizaxay/vanity-address/releases/tag/v0.3.3
[0.3.2]: https://github.com/yudizaxay/vanity-address/releases/tag/v0.3.2
[0.3.0]: https://github.com/yudizaxay/vanity-address/releases/tag/v0.3.0
[0.2.0]: https://github.com/yudizaxay/vanity-address/releases/tag/v0.2.0
