<div align="center">

<img src="assets/logo.png" alt="vanity-address logo" width="160" />

# vanity-address

**Fast, local, multi-chain vanity address generator**

Generate Solana and EVM keypairs whose public address matches your desired prefix and/or suffix — entirely on your machine. No servers. No tracking. Keys never leave your device.

<br />

<!-- Tech stack badges -->

![Rust](https://img.shields.io/badge/Rust-1.70+-orange?style=for-the-badge&logo=rust&logoColor=white)
![Solana](https://img.shields.io/badge/Solana-Supported-9945FF?style=for-the-badge&logo=solana&logoColor=white)
![Ethereum](https://img.shields.io/badge/EVM-Supported-3C3C3D?style=for-the-badge&logo=ethereum&logoColor=white)
![Rayon](https://img.shields.io/badge/Rayon-Parallel-DEA584?style=for-the-badge&logo=rust&logoColor=white)
![Clap](https://img.shields.io/badge/Clap-CLI-00C853?style=for-the-badge)

<br />

<!-- Project status badges -->

[![CI](https://img.shields.io/github/actions/workflow/status/yudizaxay/vanity-address/ci.yml?style=for-the-badge&logo=githubactions&logoColor=white&label=CI)](https://github.com/yudizaxay/vanity-address/actions/workflows/ci.yml)
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
| 📦 **Open source**   | MIT licensed, contributions welcome                                         |
| 🛡️ **Transparent**   | Full source code — audit before you trust                                   |

</details>

---

## 🎬 Demo

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
| `--chain <sol\|evm>` | Blockchain to grind for                            | `sol`   |
| `--prefix <PATTERN>` | Address must start with pattern                    | —       |
| `--suffix <PATTERN>` | Address must end with pattern                      | —       |
| `--exact`            | Exact casing (Solana only)                         | off     |
| `-q, --quiet`        | Minimal output (script-friendly)                   | off     |
| `--threads <N>`      | Override worker threads (auto-detected by default) | auto    |
| `-h, --help`         | Show help                                          | —       |
| `-V, --version`      | Show version                                       | —       |

> **Pattern rules**
>
> - **Solana:** base58 only. Invalid chars: `0`, `O`, `I`, `l`
> - **EVM:** hex (`0-9`, `a-f`). Optional `0x` prefix

---

## 🏗 Architecture

```text
┌─────────────────────────────────────┐
│          vanity-address CLI         │
├─────────────────────────────────────┤
│           vanity-core lib           │
│  ┌─────────┐ ┌─────────┐ ┌───────┐ │
│  │ Solana  │ │   EVM   │ │ Future│ │
│  │ Grinder │ │ Grinder │ │ Chain │ │
│  └─────────┘ └─────────┘ └───────┘ │
│         ChainGrinder trait          │
└─────────────────────────────────────┘
```

New chain = one file + trait implementation in `vanity-core/src/chains/`.

---

## ⚡ Performance

On startup, **vanity-address probes your system** (CPU cores, RAM) and configures an optimized rayon thread pool before grinding:

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
- [ ] Desktop UI ([Tauri](https://tauri.app/))
- [ ] Bitcoin, Cosmos, and more chains
- [ ] Regex patterns for power users

---

## 🤝 Contributing

We love contributions! See [CONTRIBUTING.md](CONTRIBUTING.md).

```bash
git checkout -b feat/my-feature
cargo build --release
cargo test
cargo clippy -- -D warnings
```

1. Fork the repo
2. Create your branch
3. Commit changes
4. Push and open a PR

---

## 📄 License

This project is licensed under the **[MIT License](LICENSE)** — free for personal and commercial use.

---

<div align="center">

**Built with 🦀 Rust** · Keys stay on your machine

<br />

[![GitHub](https://img.shields.io/badge/GitHub-yudizaxay%2Fvanity--address-181717?style=flat-square&logo=github)](https://github.com/yudizaxay/vanity-address)
[![Report Bug](https://img.shields.io/badge/Report-Bug-red?style=flat-square&logo=github)](https://github.com/yudizaxay/vanity-address/issues/new)
[![Request Feature](https://img.shields.io/badge/Request-Feature-blue?style=flat-square&logo=github)](https://github.com/yudizaxay/vanity-address/issues/new)

<br />

<sub>If this project helped you, consider giving it a ⭐ on GitHub!</sub>

</div>
