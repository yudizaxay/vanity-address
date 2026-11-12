# AGENTS.md — Project memory & context

> **Purpose:** Resume development quickly. Read this at the start of AI or contributor sessions.  
> **Repo:** [yudizaxay/vanity-address](https://github.com/yudizaxay/vanity-address)  
> **License:** MIT

---

## What this project is

**vanity-address** — fast, local, multi-chain vanity cryptocurrency address generator.

- **49 menu chains:** prior set + Zcash (`zec` t1), Bitcoin Cash (`bch`), Stacks (`stx`), Decred (`dcr`), Nervos CKB (`ckb`); Bitcoin-family P2PKH forks (`btg`, `rvn`, `mona`, … — see `docs/BITCOIN_FORKS.md`); Bitcoin (P2PKH / SegWit / Taproot), Cardano, Celestia, Cosmos, Dash, Dogecoin, dYdX, EVM (+ aliases), Filecoin, Hedera, Injective, ICP, Kaspa, Kusama, Litecoin, MultiversX, NEAR, Osmosis, Polkadot, Ripple, Sei, Solana, Stellar, Sui, Tezos, TON, Tron
- **Two frontends:** CLI (`vanity-address`) + Tauri desktop app (`vanity-app`)
- **One engine:** `vanity-core` library (all chain logic lives here)
- **Patterns:** prefix / suffix / contains + `*` wildcards + comma OR + `--count` + `verify` + CREATE2 lite
- **Privacy:** 100% offline; keys never leave the device; no telemetry
- **OpenCL GPU:** EVM (incremental EC + batch inversion, ~46M keys/s ≈ 90× CPU, **measured on Apple M4 only**; NVIDIA/AMD untested); Solana experimental (~5× *slower* than CPU, so `--gpu auto` is EVM-only); CLI/desktop only; CPU re-verify

---

## Workspace layout

```text
vanity-address/ (workspace root)
├── vanity-core/          # Rust lib — grinders, pattern, estimate, system, gpu/
├── vanity-address/       # Rust CLI binary
├── vanity-app/           # Tauri 2 + Vite + TypeScript desktop UI
├── Formula/              # Homebrew formula (canonical)
├── homebrew-tap/         # README template for separate tap repo
├── scripts/              # Maintainer scripts (Homebrew, bench, etc.)
├── docs/                 # INSTALL.md, USAGE.md, HOMEBREW.md, BENCHMARKS.md, …
├── .github/workflows/    # ci.yml, release.yml
└── AGENTS.md             # ← this file
```

**Rule:** New chain = one file under `vanity-core/src/chains/` + `Chain` enum entry. Do **not** duplicate logic in CLI or desktop.

---

## Current version state (as of 2026-10-29)

| Package | Version | Published |
| ------- | ------- | --------- |
| vanity-core | **0.8.0** | ⏳ in tree — publish after tag |
| vanity-address (CLI) | **0.8.0** | ⏳ in tree — publish after tag |
| vanity-app (desktop) | **0.8.0** | ⏳ in tree — GitHub Release after tag |
| vanity-wasm / npm SDK | **0.8.0** | ⏳ in tree — publish after Release assets + prepare-npm |
| Web app (`landing/generate/`) | — | ✅ live at https://vanity-address.online/generate/ |

**Git tags on GitHub:** `v0.3.0` … `v0.7.0` (⏳ `v0.8.0` pending)  
**Latest GitHub Release:** [v0.6.6](https://github.com/yudizaxay/vanity-address/releases/tag/v0.6.6) — bump Formula/sha after `v0.8.0` exists

**Website deploy (Cloudflare Pages — GitHub `main` auto-deploys `landing/`):**

- Live: https://vanity-address.online/ (generator: [/generate/](https://vanity-address.online/generate/))
- Build settings: Framework `None`, build command empty, **output directory `landing`**, root directory empty, branch `main`
- SEO pages (per-chain guides, `/chains/`, difficulty calculator, `sitemap.xml`) are generated: edit `scripts/gen-seo-pages.py`, run `python3 scripts/gen-seo-pages.py`, commit output. `landing/_worker.js` 301s `www` → apex and serves `404.html` for missing paths
- Headers/CSP: `landing/_headers` (Cloudflare). `landing/netlify.toml` is legacy/optional only
- After `vanity-core` / `vanity-wasm` changes: `./scripts/build-web.sh`, commit `landing/generate/wasm/`, push — Pages serves Git as-is (no Rust build on CF)

**crates.io publish order (critical):**

```bash
cargo publish -p vanity-core   # FIRST — no # comments on same line!
# wait ~1 min for index
cargo publish -p vanity-address
```

---

## Distribution channels

| Channel | Status | User command |
| ------- | ------ | ------------ |
| **GitHub Releases** | ✅ v0.6.6 (⏳ v0.8.0 pending) | Download `.dmg`, `.exe`, CLI archives |
| **crates.io** | ✅ v0.6.6 (⏳ v0.8.0 pending) | `cargo install vanity-address` |
| **Homebrew tap** | ✅ v0.6.6 (⏳ v0.8.0 pending) | `brew tap yudizaxay/tap && brew trust yudizaxay/tap && brew install vanity-address` |
| **Homebrew local** | ✅ Works | `brew install --build-from-source ./Formula/vanity-address.rb` |
| **npm** | ✅ v0.7.0 / 0.7.1 (⏳ 0.8.0 pending) | `npx vanity-address` / `npm i vanity-address` |
| **Web app** | ✅ Live | https://vanity-address.online/generate/ |
| **Winget / Scoop / AUR** | ❌ Not yet | Future optional channels |

### Homebrew user install (Homebrew 6+)

```bash
brew tap yudizaxay/tap
brew trust yudizaxay/tap   # required once (third-party tap)
brew install vanity-address   # builds from source; 3–8 min first time
```

Or trust only the formula: `brew trust --formula yudizaxay/tap/vanity-address`

### Homebrew maintainer flow

```bash
./scripts/update-homebrew-formula.sh X.Y.Z   # tag must exist on GitHub first
git add Formula/vanity-address.rb && git commit && git push
./scripts/sync-homebrew-tap.sh --push "vanity-address X.Y.Z"
```

Tap repo: https://github.com/yudizaxay/homebrew-tap (already created).  
Canonical formula: `Formula/vanity-address.rb` → sync copies into tap.  
Override tap path: `HOMEBREW_TAP_DIR=/path/to/homebrew-tap`.

See [docs/HOMEBREW.md](docs/HOMEBREW.md).

### npm maintainer flow

```bash
./scripts/build-wasm.sh          # required for SDK wasm artifacts before npm pack
./scripts/prepare-npm.sh X.Y.Z   # downloads Release CLI assets into npm/*/bin
./scripts/publish-npm.sh --dry-run
./scripts/publish-npm.sh         # requires npm login
```

Layout: `npm/vanity-address` (CLI shim + WASM SDK) + `npm/vanity-address-{darwin-arm64,darwin-x64,linux-x64,win32-x64}`.  
`vanity-app` stays private (desktop only). See [docs/NPM.md](docs/NPM.md) and [docs/SDK.md](docs/SDK.md).

---

## Release checklist (maintainers)

1. `make check-ci` green on `main`
2. Bump version in **all** (keep in sync):
   - `vanity-core/Cargo.toml`
   - `vanity-address/Cargo.toml`
   - `vanity-app/package.json`
   - `vanity-app/src-tauri/Cargo.toml`
   - `vanity-app/src-tauri/tauri.conf.json`
   - `vanity-address/src/banner.rs`, `vanity-app/index.html`, demo SVGs
3. `CHANGELOG.md` new section
4. `./scripts/update-homebrew-formula.sh X.Y.Z` (after tag exists — or after tagging, in a follow-up commit)
5. Commit + push `main`
6. `git tag -a vX.Y.Z -m "vX.Y.Z" && git push origin vX.Y.Z`
7. Watch [Release workflow](.github/workflows/release.yml)
8. `cargo publish -p vanity-core` then `cargo publish -p vanity-address`
9. `./scripts/sync-homebrew-tap.sh --push "vanity-address X.Y.Z"`
10. `./scripts/prepare-npm.sh X.Y.Z` then `./scripts/publish-npm.sh`

Full detail: [RELEASING.md](RELEASING.md)

---

## Architecture (runtime)

```text
vanity-address (CLI) ──┐
vanity-app (Tauri)  ──┼──► vanity-core
                      │      ├── grinder.rs (rayon parallel)
                      │      ├── gpu/ (OpenCL · sol + evm · feature `opencl`)
                      │      ├── pattern.rs
                      │      ├── estimate.rs
                      │      ├── system.rs (CPU/memory tuning)
                      │      └── chains/ (chain impls)
```

CLI modules: `main.rs`, `menu.rs`, `terminal.rs`, `banner.rs`, `json_output.rs`, `warnings.rs`  
Desktop: `commands.rs`, `state.rs` — thin wrappers over `vanity-core`

---

## Notable fixes & gotchas

### Windows CLI (0.3.2+)

- **Double typing:** `terminal.rs` — only handle `KeyEventKind::Press`
- **Screen clear:** `menu.rs` — crossterm WinAPI, not ANSI alone
- **Enter:** handle `\r` on Windows

### crates.io README

- Images: absolute `raw.githubusercontent.com` URLs (not `assets/`)
- Links: absolute GitHub URLs (relative paths 404 from crate path)

### Build profiles

- **Workspace** `[profile.release]`: `lto=true`, `codegen-units=1` — GitHub Release binaries
- **vanity-address crate** `[profile.release]`: `lto=false`, `codegen-units=16` — faster `cargo install`
- Warning during workspace publish: non-root profiles ignored during verify; published crate tarball still includes install profile

### cargo install / brew compile slowness

Solana SDK compile is heavy (3–8 min first time). Documented in `docs/INSTALL.md`. Pre-built GitHub binaries are faster (~30s).

### Homebrew (critical)

- **Formula API:** use `std_cargo_args(path: "vanity-address")` — **not** `std_cargo_install_args` (removed; causes `NoMethodError` on modern Brew)
- **Tap trust (Homebrew 6+):** users must `brew trust yudizaxay/tap` (or `--formula …`) before install
- **Other untrusted taps:** if user’s Mac has untrusted taps (e.g. `mongodb/brew`), any `brew install` may fail until those taps are trusted/untapped — not a vanity-address bug. Docs: [docs/HOMEBREW.md](docs/HOMEBREW.md)
- **Push tap via SSH** if HTTPS 403 (org/auth mismatch)
- **"No available formula with the name vanity-address"**: plain `brew install` only searches homebrew-core. The user has not added the tap on that Mac — run `brew tap yudizaxay/tap && brew trust yudizaxay/tap` first (or `brew install yudizaxay/tap/vanity-address`)

### OpenCL (0.7+)

- Feature `opencl` enabled by default for CLI + desktop; Apple uses `CommandQueue::create` (not `create_with_properties`)
- EVM speed claims must name the hardware (**Apple M4 only** measured); Solana GPU stays “experimental, slower than CPU”; gate via `gpu_should_use` (CLI + desktop share it)
- GPU candidates always CPU-reverified before export
- Kernels: `vanity-core/src/gpu/kernels/{evm,solana}.cl`

### Dependabot

- `sha3` ignored for major/minor in `.github/dependabot.yml` (manual crypto updates)

---

## CI & quality

```bash
make check-ci     # fmt-check + test + clippy + frontend build
make test-cli     # vanity-core + vanity-address
make test-app     # Tauri crate tests
make homebrew-formula VER=X.Y.Z
```

**CI matrix:** Linux CLI, macOS desktop, Windows CLI + desktop

---

## Security

- Keys generated locally only; tool does not network
- `SECURITY.md` + `docs/SECURITY-MODEL.md` for threat model / reporting
- Never commit secrets; warn on `vanity-results.txt` (private keys)

---

## Pending / optional next steps

- [ ] Tag + publish **v0.8.0** (GitHub Release → crates.io → Homebrew → npm)
- [ ] Optional: ICP explorer link on the web result
- [ ] Winget / Scoop manifests
- [ ] Code signing for macOS Gatekeeper / Windows SmartScreen
- [ ] Submit to homebrew-core when notability criteria met
- [ ] Cloudflare Pages: disable Web Analytics (Metrics) so edge stops injecting `beacon.min.js`
- [x] Faster EVM GPU (incremental EC + batch inversion) — Solana GPU still slower than CPU; NVIDIA/AMD untested

### Recently completed (2026-10-20)

- [x] **0.7.0** in tree: Phases 1–3 (forks, puzzle, split-key, security, OpenCL EVM+Solana GPU, benchmarks)
- [x] Phase 2: split-key + SECURITY-MODEL + export/Safety warnings
- [x] Phase 1: Bitcoin-family forks + puzzle mode

---

## Conventions for AI / contributors

1. **Minimize scope** — smallest correct diff; no unrelated refactors
2. **Match existing style** — naming, error handling, module layout
3. **vanity-core first** for chain logic; keep CLI/desktop thin
4. **Don't commit** unless user asks
5. **Verify** with `make test` / `cargo package` before claiming publish-ready
6. **Read** `RELEASING.md` before version bumps or publishes
7. **After Homebrew formula changes** — sync tap with `./scripts/sync-homebrew-tap.sh --push`
8. **After npm package changes** — `./scripts/prepare-npm.sh` then `./scripts/publish-npm.sh` (do not commit binaries)
9. **After `vanity-core` / `vanity-wasm` changes** — `./scripts/build-web.sh` and commit `landing/generate/wasm/` so the web generator matches

---

*Last updated: 2026-11-12 — **0.8.0** in tree (+ ZEC/BCH/STX/DCR/CKB); tag/publish pending.*
