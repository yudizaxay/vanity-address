# AGENTS.md — Project memory & context

> **Purpose:** Resume development quickly. Read this at the start of AI or contributor sessions.  
> **Repo:** [yudizaxay/vanity-address](https://github.com/yudizaxay/vanity-address)  
> **License:** MIT

---

## What this project is

**vanity-address** — fast, local, multi-chain vanity cryptocurrency address generator.

- **13 chains:** Solana, EVM, Bitcoin, Litecoin, Dogecoin, Tron, Cosmos, Osmosis, Ripple, Stellar, Aptos, Sui, NEAR
- **Two frontends:** CLI (`vanity-address`) + Tauri desktop app (`vanity-app`)
- **One engine:** `vanity-core` library (all chain logic lives here)
- **Privacy:** 100% offline; keys never leave the device; no telemetry

---

## Workspace layout

```text
vanity-address/          (workspace root)
├── vanity-core/         # Rust lib — grinders, pattern, estimate, system
├── vanity-address/      # Rust CLI binary
├── vanity-app/          # Tauri 2 + Vite + TypeScript desktop UI
├── Formula/             # Homebrew formula (canonical)
├── homebrew-tap/        # README template for separate tap repo
├── scripts/             # Maintainer scripts (Homebrew, etc.)
├── docs/                # INSTALL.md, USAGE.md, HOMEBREW.md
├── .github/workflows/   # ci.yml, release.yml
└── AGENTS.md            # ← this file
```

**Rule:** New chain = one file under `vanity-core/src/chains/` + `Chain` enum entry. Do **not** duplicate logic in CLI or desktop.

---

## Current version state (as of 2026-07-16)

| Package | Version | Published |
| ------- | ------- | --------- |
| vanity-core | **0.3.5** | ✅ crates.io |
| vanity-address (CLI) | **0.3.5** | ✅ crates.io |
| vanity-app (desktop) | **0.3.5** | GitHub Releases (tag-dependent) |

**Git tags on GitHub:** `v0.3.0` … `v0.3.4` — **`v0.3.5` tag not pushed yet** (crates.io 0.3.5 is live; push tag + run `./scripts/update-homebrew-formula.sh 0.3.5` for Homebrew)

**crates.io publish order (critical):**

```bash
cargo publish -p vanity-core    # FIRST — no # comments on same line!
# wait ~1 min for index
cargo publish -p vanity-address
```

---

## Distribution channels

| Channel | Status | User command |
| ------- | ------ | ------------ |
| **GitHub Releases** | ✅ Active | Download `.dmg`, `.exe`, CLI archives |
| **crates.io** | ✅ v0.3.5 | `cargo install vanity-address` |
| **Homebrew tap** | 🟡 Repo created: [yudizaxay/homebrew-tap](https://github.com/yudizaxay/homebrew-tap) — run sync script to push formula | `brew tap yudizaxay/tap && brew install vanity-address` |
| **Homebrew local** | ✅ Works now | `brew install --build-from-source ./Formula/vanity-address.rb` |
| **npm** | ❌ Not for end users | `vanity-app/package.json` is `private` (build only) |
| **Winget / Scoop / AUR** | ❌ Not yet | Future optional channels |

### Homebrew maintainer flow

```bash
./scripts/update-homebrew-formula.sh 0.3.5
./scripts/sync-homebrew-tap.sh --push "vanity-address 0.3.5"
```

See [docs/HOMEBREW.md](docs/HOMEBREW.md).

**One-time:** Create GitHub repo `yudizaxay/homebrew-tap`, clone to `../homebrew-tap`, run sync script.

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
4. `./scripts/update-homebrew-formula.sh X.Y.Z`
5. Commit + push `main`
6. `git tag -a vX.Y.Z -m "vX.Y.Z" && git push origin vX.Y.Z`
7. Watch [Release workflow](.github/workflows/release.yml)
8. `cargo publish -p vanity-core` then `cargo publish -p vanity-address`
9. `./scripts/sync-homebrew-tap.sh --push "vanity-address X.Y.Z"`

Full detail: [RELEASING.md](RELEASING.md)

---

## Architecture (runtime)

```text
vanity-address (CLI)  ──┐
vanity-app (Tauri)    ──┼──► vanity-core
                        │      ├── grinder.rs   (rayon parallel)
                        │      ├── pattern.rs
                        │      ├── estimate.rs
                        │      ├── system.rs    (CPU/memory tuning)
                        │      └── chains/      (13 chain impls)
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

### cargo install slowness

Solana SDK compile is heavy (3–8 min first time). Documented in `docs/INSTALL.md`. Pre-built GitHub binaries are faster (~30s).

### Dependabot

- `sha3` ignored for major/minor in `.github/dependabot.yml` (manual crypto updates)

---

## CI & quality

```bash
make check-ci    # fmt-check + test + clippy + frontend build
make test-cli    # vanity-core + vanity-address
make test-app    # Tauri crate tests
```

**CI matrix:** Linux CLI, macOS desktop, Windows CLI + desktop

---

## Security

- Keys generated locally only; tool does not network
- `SECURITY.md` for vulnerability reporting
- Never commit secrets; warn on `vanity-results.txt` (private keys)

---

## Completed work (history summary)

| Area | Done |
| ---- | ---- |
| README trim + `docs/INSTALL.md`, `docs/USAGE.md` | ✅ |
| Windows CLI fixes (typing, clear, Enter) | ✅ |
| Windows desktop NSIS installer in release workflow | ✅ |
| Windows CI jobs | ✅ |
| crates.io publish + README link/image fixes | ✅ 0.3.2–0.3.5 |
| Install speed docs + faster cargo install profile | ✅ 0.3.5 |
| Uninstall docs | ✅ 0.3.5 |
| Homebrew formula + scripts + tap docs | ✅ (tap repo push pending) |
| Dependabot PRs #22/#23 applied locally | ✅ |

---

## Pending / optional next steps

- [ ] Push formula to tap: `git clone …/homebrew-tap ../homebrew-tap && ./scripts/sync-homebrew-tap.sh --push "initial formula"`
- [ ] Push `v0.3.5` tag to GitHub, then `./scripts/update-homebrew-formula.sh 0.3.5` and sync tap again
- [ ] README quick-start curl examples still say `v0.3.2` — update to `latest` or current tag
- [ ] Winget / Scoop manifests (Windows package managers)
- [ ] npm binary wrapper for `npx vanity-address` (optional)
- [ ] Code signing for macOS Gatekeeper / Windows SmartScreen (unsigned warnings documented)
- [ ] Submit to homebrew-core when notability criteria met

---

## Key file index

| File | Role |
| ---- | ---- |
| `vanity-core/src/chains/mod.rs` | Chain enum + menu IDs |
| `vanity-core/src/grinder.rs` | Parallel grind + benchmark |
| `vanity-address/src/main.rs` | CLI entry + clap |
| `vanity-address/src/terminal.rs` | Interactive input (Windows-sensitive) |
| `vanity-app/src-tauri/src/commands.rs` | Desktop Tauri commands |
| `Formula/vanity-address.rb` | Homebrew formula |
| `scripts/update-homebrew-formula.sh` | Bump formula hash |
| `scripts/sync-homebrew-tap.sh` | Push formula to tap repo |
| `.github/workflows/release.yml` | Release binaries on tag |
| `.github/workflows/ci.yml` | CI on push/PR |

---

## Conventions for AI / contributors

1. **Minimize scope** — smallest correct diff; no unrelated refactors
2. **Match existing style** — naming, error handling, module layout
3. **vanity-core first** for chain logic; keep CLI/desktop thin
4. **Don't commit** unless user asks
5. **Verify** with `make test` / `cargo package` before claiming publish-ready
6. **Read** `RELEASING.md` before version bumps or publishes

---

*Last updated: 2026-07-16 — Homebrew tap setup + project memory file added.*
