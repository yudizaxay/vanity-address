<div align="center">

<img src="assets/logo.svg" alt="Vanity Address logo" width="160" />

# vanity-address

**Fast, local, multi-chain vanity address generator**

Generate multi-chain keypairs whose public address matches your desired prefix and/or suffix — entirely on your machine. No servers. No tracking. Keys never leave your device.

<br />

<!-- Tech stack badges -->

![Rust](https://img.shields.io/badge/Rust-1.70+-orange?style=for-the-badge&logo=rust&logoColor=white)
![Solana](https://img.shields.io/badge/Solana-Supported-9945FF?style=for-the-badge&logo=solana&logoColor=white)
![Ethereum](https://img.shields.io/badge/EVM-Supported-3C3C3D?style=for-the-badge&logo=ethereum&logoColor=white)
![Bitcoin](https://img.shields.io/badge/Bitcoin-Supported-F7931A?style=for-the-badge&logo=bitcoin&logoColor=white)
![13+ Chains](https://img.shields.io/badge/Chains-13+-blue?style=for-the-badge)
![Rayon](https://img.shields.io/badge/Rayon-Parallel-DEA584?style=for-the-badge&logo=rust&logoColor=white)
![Clap](https://img.shields.io/badge/Clap-CLI-00C853?style=for-the-badge)
![Tauri](https://img.shields.io/badge/Tauri-Desktop-FFC131?style=for-the-badge&logo=tauri&logoColor=white)

<br />

<!-- Project status badges -->

[![CI](https://img.shields.io/github/actions/workflow/status/yudizaxay/vanity-address/ci.yml?style=for-the-badge&logo=githubactions&logoColor=white&label=CI)](https://github.com/yudizaxay/vanity-address/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/yudizaxay/vanity-address?style=for-the-badge&logo=github&label=Release)](https://github.com/yudizaxay/vanity-address/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue?style=for-the-badge)](LICENSE)
[![Issues](https://img.shields.io/github/issues/yudizaxay/vanity-address?style=for-the-badge&logo=github)](https://github.com/yudizaxay/vanity-address/issues)
[![Stars](https://img.shields.io/github/stars/yudizaxay/vanity-address?style=for-the-badge&logo=github&color=yellow)](https://github.com/yudizaxay/vanity-address/stargazers)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen?style=for-the-badge&logo=git&logoColor=white)](CONTRIBUTING.md)

<br />

[Features](#-features) ·
[Demo](#-demo) ·
[Install](#-install) ·
[Usage](#-usage) ·
[Architecture](#-architecture) ·
[Desktop App](#-desktop-app) ·
[Security](#-security) ·
[Contributing](#-contributing)

</div>

---

## ✨ Features

| Feature                     | Solana |   EVM    |
| --------------------------- | :----: | :------: |
| Prefix matching             |   ✅   |    ✅    |
| Suffix matching             |   ✅   |    ✅    |
| Case-insensitive mode       |   ✅   | ✅ (hex) |
| Exact case mode             |   ✅   |    —     |
| Parallel CPU grinding       |   ✅   |    ✅    |
| Live progress + ETA         |   ✅   |    ✅    |
| Multiple key export formats |   ✅   |    ✅    |
| 100% offline / local        |   ✅   |    ✅    |

<details>
<summary><strong>Why developers choose vanity-address</strong></summary>

<br />

|                      |                                                                             |
| -------------------- | --------------------------------------------------------------------------- |
| 🔒 **Privacy-first** | Keys generated locally — zero network calls                                 |
| ⚡ **Blazing fast**  | Multi-core parallel grinding via [rayon](https://github.com/rayon-rs/rayon) |
| 🔌 **Extensible**    | `ChainGrinder` trait — add chains without touching core                     |
| 🖥️ **Great CLI**     | Colors, spinner, ETA, quiet mode for scripts                                |
| 🪟 **Desktop app**   | **Vanity Address** — Tauri UI with the same wizard flow as the CLI          |
| 📦 **Open source**   | MIT licensed, contributions welcome                                         |
| 🛡️ **Transparent**   | Full source code — audit before you trust                                   |

</details>

---

## 🎬 Demo

<p align="center">
  <img src="assets/demo-terminal.svg" alt="Vanity Address terminal demo — interactive menu, live grind progress, and match output" width="720" />
</p>

<p align="center">
  <img src="assets/demo-desktop.svg" alt="Vanity Address desktop app — wizard summary and live grind progress" width="720" />
</p>

<p align="center">
  <sub>CLI interactive menu or <strong>Vanity Address</strong> desktop app — chain wizard → live benchmark → grind → export keys</sub>
</p>

### Solana

```bash
$ vanity-address --chain sol --suffix axay

vanity-address
Local multi-chain vanity address generator

  Chain     Solana
  Target    ending with 'axay'
  Mode      any case
  Expected  ~656.4M attempts (average)
  Hint      Base58 characters only. Invalid: 0, O, I, l

⠋ 12.4M keys | 2,198,421 keys/s | ~5 min remaining

 Match found!

  Address   7xKp...Qaxay
  Time      142.31s
  Attempts  312,847,291

 Private Keys
  Never share these with anyone.
```

### EVM (Ethereum)

```bash
$ vanity-address --chain evm --prefix dead --suffix beef

  Chain     EVM (Ethereum)
  Target    starting with '0xdead' and ending with 'beef'
  Expected  ~1.1T attempts (average)
  ...
```

---

## 📦 Install

### Download from GitHub Releases (recommended)

**Latest release:** [github.com/yudizaxay/vanity-address/releases/latest](https://github.com/yudizaxay/vanity-address/releases/latest)

No Rust or Node.js required — pick the file for your platform, download, extract, and run.

#### Which file do I need?

Go to **[Releases](https://github.com/yudizaxay/vanity-address/releases/latest)** and download **one** file:

| I want… | My computer | File to download |
| ------- | ----------- | ---------------- |
| **Desktop app** (easiest — window UI) | Mac M1 / M2 / M3 / M4 | `VanityAddress-*-Mac-AppleSilicon-Desktop.dmg` |
| **Terminal app** | Mac M1 / M2 / M3 / M4 | `VanityAddress-*-Mac-AppleSilicon-CLI.tar.gz` |
| **Terminal app** | Mac Intel | `VanityAddress-*-Mac-Intel-CLI.tar.gz` |
| **Terminal app** | Windows 10 / 11 | `VanityAddress-*-Windows-CLI.zip` |
| **Terminal app** | Linux | `VanityAddress-*-Linux-CLI.tar.gz` |

> **Tip:** `*` = version number (e.g. `0.3.0`). The Releases page shows the full filename.  
> **Checksum files** (`.sha256`) are optional — for security verification only; most users can skip them.

---

#### Linux (CLI)

```bash
# Replace 0.3.0 with the version on the Releases page if newer
curl -LO https://github.com/yudizaxay/vanity-address/releases/download/v0.3.0/VanityAddress-0.3.0-Linux-CLI.tar.gz
tar xzf VanityAddress-0.3.0-Linux-CLI.tar.gz
./vanity-address
```

Optional — install globally:

```bash
sudo cp vanity-address /usr/local/bin/
vanity-address --version
```

---

#### macOS — CLI (Terminal)

**Apple Silicon (M1/M2/M3/M4):**

```bash
curl -LO https://github.com/yudizaxay/vanity-address/releases/download/v0.3.0/VanityAddress-0.3.0-Mac-AppleSilicon-CLI.tar.gz
tar xzf VanityAddress-0.3.0-Mac-AppleSilicon-CLI.tar.gz
./vanity-address
```

**Intel Mac:** download `VanityAddress-*-Mac-Intel-CLI.tar.gz` instead.

If macOS blocks the binary (“unidentified developer” or **“damaged”**):

```bash
xattr -cr ./vanity-address
./vanity-address
```

Or: **System Settings → Privacy & Security → Open Anyway**

---

#### macOS — Desktop app (`.dmg`) — recommended for Mac users

1. Download **`VanityAddress-*-Mac-AppleSilicon-Desktop.dmg`** from [Releases](https://github.com/yudizaxay/vanity-address/releases/latest)
2. **Remove download quarantine** (required for unsigned open-source builds):
   ```bash
   xattr -cr ~/Downloads/VanityAddress-*-Mac-AppleSilicon-Desktop.dmg
   ```
3. Double-click the `.dmg` → drag **Vanity Address** to **Applications**
4. Clear quarantine on the installed app, then launch:
   ```bash
   xattr -cr "/Applications/Vanity Address.app"
   open -a "Vanity Address"
   ```

> **“Vanity Address is damaged and can’t be opened”?**  
> This is **not** a broken download — macOS Gatekeeper blocks apps that are not Apple-notarized.  
> Run the `xattr -cr` commands above, or: **right-click** the app → **Open** → **Open** again.  
> You can also use **System Settings → Privacy & Security → Open Anyway** after the first blocked launch.

> **Apple Silicon Macs only** (M1/M2/M3/M4). Intel Mac or Linux → use the **CLI** above or [build from source](#build-from-source).

---

#### Windows (CLI)

1. Download **`VanityAddress-*-Windows-CLI.zip`** from [Releases](https://github.com/yudizaxay/vanity-address/releases/latest)
2. Right-click → **Extract All**
3. Open the extracted folder and double-click **`vanity-address.exe`**, or in PowerShell:

```powershell
# After extracting the zip — run from the folder that contains vanity-address.exe
.\vanity-address.exe
```

4. Optional — add the folder to your PATH for terminal access from anywhere

Windows SmartScreen may warn on first run — click **More info → Run anyway** (unsigned open-source build).

---

#### Verify downloads (checksums)

Every archive on the Releases page ships with a `.sha256` sidecar file.

```bash
# Linux / macOS (optional)
shasum -a 256 -c VanityAddress-0.3.0-Linux-CLI.tar.gz.sha256
```

```powershell
# Windows (optional) — compare hash manually
Get-FileHash VanityAddress-0.3.0-Windows-CLI.zip -Algorithm SHA256
```

---

#### What's inside each archive?

| Archive | Contents |
| ------- | -------- |
| `*-CLI.tar.gz` / `*-CLI.zip` | `vanity-address` binary + docs |
| `*-Desktop.dmg` | Mac desktop app installer (double-click to install) |

**First run:** just execute the binary — interactive menu starts with no flags. See [Usage](#-usage).

---

### Other install methods

#### Homebrew (macOS / Linux)

```bash
brew install --build-from-source ./Formula/vanity-address.rb
```

See [RELEASING.md](RELEASING.md) for tap setup and formula hash updates.

### crates.io

```bash
cargo install vanity-address
```

Requires Rust 1.70+. Publishes `vanity-address` + `vanity-core` — see [RELEASING.md](RELEASING.md#publishing-to-cratesio).

### Build from source

**Requirements:** [Rust](https://rustup.rs/) 1.70+

```bash
git clone https://github.com/yudizaxay/vanity-address.git
cd vanity-address
cargo build --release
```

Binary:

```text
target/release/vanity-address
```

**Install globally:**

```bash
cargo install --path vanity-address
```

---

## 🚀 Usage

### Interactive mode (default)

Just run — no flags needed:

```bash
./target/release/vanity-address
```

```
╔══════════════════════════════════════════╗
║         vanity-address  v0.3.0           ║
╚══════════════════════════════════════════╝

  [1]  Start a new grind
  [2]  Help & pattern rules
  [3]  Exit

  Choose option [1-3]:
```

The wizard walks you through: **chain → prefix/suffix → pattern → estimate → confirm → grind**.

### Direct mode (power users / scripts)

```bash
vanity-address --chain sol --suffix axay
vanity-address --chain evm --prefix dead --suffix beef -q
```

### JSON output (automation)

Use `--json` with `--prefix` and/or `--suffix` for machine-readable stdout (errors go to stderr as JSON):

```bash
vanity-address --chain sol --suffix ax --json --no-benchmark --force
```

Example success payload:

```json
{
  "version": "0.3.0",
  "chain": "sol",
  "chain_name": "Solana",
  "pattern": {
    "prefix": "",
    "suffix": "ax",
    "description": "ending with 'ax'",
    "ignore_case": true
  },
  "address": "7xKp…Qax",
  "exports": [{ "label": "Secret Key (base58)", "value": "…" }],
  "stats": { "attempts": 1200, "elapsed_secs": 0.04, "keys_per_sec": 30000.0 },
  "measured_keys_per_sec": 2100000.0
}
```

> **Security:** JSON includes private keys on stdout. Use only in trusted environments. See [SECURITY.md](SECURITY.md).

Combine with `--save` / `--output` to persist keys; `saved_to` is included in the JSON when saving.

### Solana

```bash
vanity-address --chain sol --suffix axay
vanity-address --chain sol --prefix DeFi
vanity-address --chain sol --prefix DeFi --suffix axay
vanity-address --chain sol --suffix axay --exact
```

### EVM

```bash
vanity-address --chain evm --suffix beef
vanity-address --chain evm --prefix dead
vanity-address --chain evm --prefix dead --suffix beef
```

### CLI reference

| Flag                 | Description                                        | Default |
| -------------------- | -------------------------------------------------- | ------- |
| `--chain <ID>`       | Blockchain: `sol`, `evm`, `btc`, `ltc`, `doge`, `trx`, `cosmos`, `osmo`, `xrp`, `xlm`, `aptos`, `sui`, `near` | `sol`   |
| `--prefix <PATTERN>` | Address must start with pattern                    | —       |
| `--suffix <PATTERN>` | Address must end with pattern                      | —       |
| `--exact`            | Exact casing (base58 chains)                       | off     |
| `--save`             | Append match (incl. private keys) to `vanity-results.txt` | off |
| `--output <PATH>`    | Custom save file (with `--save` or interactive save) | `vanity-results.txt` |
| `--no-benchmark`     | Skip 2s speed calibration warm-up before grinding | off |
| `--force`            | Allow impractical patterns in CLI mode (blocked by default) | off |
| `--json`             | Machine-readable JSON on stdout (direct mode)      | off     |
| `-q, --quiet`        | Minimal plain-text output (script-friendly)        | off     |
| `--threads <N>`      | Override worker threads (auto-detected by default) | auto    |
| `-h, --help`         | Show help                                          | —       |
| `-V, --version`      | Show version                                       | —       |

> **Pattern rules**
>
> - **Base58 chains** (Solana, Bitcoin, Litecoin, Dogecoin, Tron, Ripple, Stellar): no `0`, `O`, `I`, `l` (Solana alphabet); Ripple uses its own alphabet
> - **Bech32** (Cosmos, Osmosis): `qpzry9x8gf2tvdw0s3jn54khce6mua7l` or full address with `cosmos1` / `osmo1` prefix
> - **Hex chains** (EVM, Aptos, Sui, NEAR): `0-9`, `a-f`; EVM/Aptos/Sui accept optional `0x` prefix

---

## 🏗 Architecture

```text
┌───────────────────┐   ┌───────────────────┐
│  vanity-address    │   │    vanity-app      │
│      (CLI)         │   │  (Tauri desktop UI)│
├───────────────────┴───┴───────────────────┤
│                vanity-core lib              │
│  ┌────────┐ ┌─────┐ ┌──────────┐ ┌───────┐ │
│  │ Solana │ │ EVM │ │ Bitcoin… │ │  +10  │ │
│  │Grinder │ │Grind│ │ Grinders │ │ more  │ │
│  └────────┘ └─────┘ └──────────┘ └───────┘ │
│              ChainGrinder trait             │
└─────────────────────────────────────────────┘
```

New chain = one file + trait implementation in `vanity-core/src/chains/`. Both the CLI and the desktop UI are thin frontends over the same `vanity-core` grinding engine — no chain logic is duplicated.

---

## 🖥 Desktop App

**Vanity Address** is the native desktop UI (`vanity-app/`), built with [Tauri 2](https://tauri.app/) and wired directly to `vanity-core` — no duplicated chain logic.

### Wizard flow (same as CLI)

```text
Home → Chain → Pattern (suffix/prefix/both) → Summary → Grind → Result
         ↑ Help              ↑ system info + warnings    ↑ stop anytime
```

| Feature | Desktop |
| ------- | ------- |
| 13 chains | ✅ |
| Live estimate + difficulty | ✅ |
| Impractical-pattern warning + double confirm | ✅ |
| 2s speed benchmark before grind | ✅ |
| Stop mid-grind | ✅ |
| Masked keys + reveal / copy all | ✅ |
| Native save dialog | ✅ |

### Requirements (build from source only)

Pre-built [Releases](https://github.com/yudizaxay/vanity-address/releases/latest) do **not** need Rust or Node.

To compile yourself:

- [Rust](https://rustup.rs/) 1.70+
- [Node.js](https://nodejs.org/) 18+ (desktop app only)
- macOS or Linux for `tauri build` (see [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/))

### Install from Releases (macOS desktop)

Download **`VanityAddress-*-Mac-AppleSilicon-Desktop.dmg`** → double-click → drag to Applications.  
Full steps in [Download from GitHub Releases](#download-from-github-releases-recommended).

### Run / build from source

```bash
cd vanity-app
npm install
npm run tauri dev      # development (hot reload)
npm run tauri build    # native .app / .dmg (macOS) or .deb / .AppImage (Linux)
```

Output lands in `target/release/bundle/` (workspace root) or `vanity-app/src-tauri/target/release/bundle/` when built outside the workspace.

Grinding runs on a background thread; keys stay masked until you click **Reveal**, and **Save** opens a native file picker (nothing is written automatically).

---

## ⚡ Performance

On startup, **vanity-address** (CLI and desktop app) probes your system (CPU cores, RAM) and runs a **2-second benchmark** when grinding starts for an honest ETA:

| Signal              | Behavior                                                                       |
| ------------------- | ------------------------------------------------------------------------------ |
| CPU cores           | Uses all cores on 1–2 core machines; reserves 1 core for OS on larger machines |
| Physical vs logical | Avoids oversubscribing physical cores when hyperthreading is present           |
| Available RAM       | Reduces workers on low-memory systems to prevent pressure                      |
| Progress interval   | Scales with worker count to keep UI smooth without overhead                    |

```bash
# Auto-detect (recommended)
vanity-address --chain sol --suffix axay

# Manual override
vanity-address --chain sol --suffix axay --threads 4
```

Example startup:

```text
  System    8 logical / 8 physical · 16.0 GB RAM (12.0 GB free) · 7 workers · memory: comfortable
  Chain     Solana
  Target    ending with 'axay'
```

| Pattern length | Solana (case-insensitive) | EVM (hex)     |
| -------------- | ------------------------- | ------------- |
| 4 chars        | ~7M attempts              | ~65K attempts |
| 6 chars        | ~656M attempts            | ~16M attempts |
| 8 chars        | ~58B attempts             | ~4B attempts  |

Longer patterns = exponentially harder. Start short, verify, then go longer.

---

## 🔐 Security

> **⚠️ Read before generating keys**
>
> | Rule         | Detail                                                  |
> | ------------ | ------------------------------------------------------- |
> | Local only   | Keys are generated on **your machine**                  |
> | No network   | This tool **never connects to the internet**            |
> | Never share  | **Do not** share private keys with anyone               |
> | Verify first | Always double-check the address before sending funds    |
> | Open source  | Audit the code — trust, but verify                      |
> | Risk         | Vanity grinding is probabilistic — use at your own risk |

---

## 🗺 Roadmap

- [x] Solana suffix / prefix grinding
- [x] EVM (Ethereum) support
- [x] Multi-chain `ChainGrinder` architecture
- [x] Polished CLI (colors, progress, `--chain`)
- [x] Auto system detection (CPU + memory tuned thread pool)
- [x] Desktop UI ([Tauri](https://tauri.app/)) — `vanity-app/`
- [x] Bitcoin, Litecoin, Dogecoin, Tron, Cosmos, Osmosis, Ripple, Stellar, Aptos, Sui, NEAR
- [ ] Cardano, TON (complex address formats)
- [ ] Regex patterns for power users

---

## 🤝 Contributing

We love contributions — **new blockchains, bug fixes, features, docs, and tests** are all welcome!

📖 **[CONTRIBUTING.md](CONTRIBUTING.md)** — full guide (add a chain, PR checklist, code style)

| Want to… | Start here |
|----------|------------|
| Add a blockchain | `vanity-core/src/chains/` + [contributing guide](CONTRIBUTING.md#adding-a-new-blockchain) |
| Fix a bug / UX issue | Fork → branch → PR with repro steps |
| Propose a feature | [Open a feature request](https://github.com/yudizaxay/vanity-address/issues/new?template=feature_request.yml) for big changes |

### Development checks

Before opening a PR, run the same checks as [CI](.github/workflows/ci.yml):

```bash
make check        # fmt + test + clippy + frontend build
make check-ci     # same, but fmt-check only (what CI runs)
```

Or manually:

```bash
# Format Rust (apply fixes)
cargo fmt --all

# Format check only — what CI runs (no file changes)
cargo fmt --all -- --check

# Tests
cargo test -p vanity-core -p vanity-address
cargo test -p vanity-app

# Lint
cargo clippy -p vanity-core -p vanity-address -- -D warnings
cargo clippy -p vanity-app -- -D warnings

# Desktop frontend (TypeScript + Vite)
cd vanity-app && npm ci && npm run build && cd ..
```

**One-liner** (quick pre-push):

```bash
cargo fmt --all && cargo test && cargo clippy -- -D warnings && cd vanity-app && npm ci && npm run build && cd ..
```

See [SECURITY.md](SECURITY.md) before reporting vulnerabilities publicly.

```bash
git checkout -b feat/my-feature
# … make changes, then run make check …
```

PRs use the [pull request template](.github/PULL_REQUEST_TEMPLATE.md) — fill it in so reviewers can merge faster.

1. Fork the repo
2. Create your branch (`feat/…`, `fix/…`, `docs/…`)
3. Commit with a clear message
4. Push and open a PR

---

## 📄 License

This project is licensed under the **[MIT License](LICENSE)** — free for personal and commercial use.

---

<div align="center">

**Built with 🦀 Rust** · Keys stay on your machine

<br />

[![GitHub](https://img.shields.io/badge/GitHub-yudizaxay%2Fvanity--address-181717?style=flat-square&logo=github)](https://github.com/yudizaxay/vanity-address)
[![Report Bug](https://img.shields.io/badge/Report-Bug-red?style=flat-square&logo=github)](https://github.com/yudizaxay/vanity-address/issues/new?template=bug_report.yml)
[![Request Feature](https://img.shields.io/badge/Request-Feature-blue?style=flat-square&logo=github)](https://github.com/yudizaxay/vanity-address/issues/new?template=feature_request.yml)

<br />

<sub>If this project helped you, consider giving it a ⭐ on GitHub!</sub>

</div>
