<div align="center">

<img src="https://raw.githubusercontent.com/yudizaxay/vanity-address/main/assets/logo.svg" alt="Vanity Address logo" width="120" />

# vanity-address

**Generate custom crypto wallet addresses that start, end, or contain YOUR word — 49 wallet chains including Solana, Ethereum, Bitcoin SegWit/Taproot, Zcash, BCH, Stacks. Plus vanity contract addresses and Solana token mints.**

[![npm](https://img.shields.io/npm/v/vanity-address?style=flat-square&logo=npm&color=cb3837)](https://www.npmjs.com/package/vanity-address)
[![downloads](https://img.shields.io/npm/dm/vanity-address?style=flat-square&color=blue)](https://www.npmjs.com/package/vanity-address)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue?style=flat-square)](https://github.com/yudizaxay/vanity-address/blob/main/LICENSE)
[![GitHub](https://img.shields.io/github/stars/yudizaxay/vanity-address?style=flat-square&logo=github)](https://github.com/yudizaxay/vanity-address)

**100% offline · keys never leave your machine · no telemetry · open source**

No terminal handy? [Try it in your browser](https://vanity-address.online/generate/) — same engine, nothing to install.

Guides: [all supported chains](https://vanity-address.online/chains/) · [difficulty calculator](https://vanity-address.online/vanity-address-difficulty/) · [Solana](https://vanity-address.online/solana-vanity-address/) · [Ethereum](https://vanity-address.online/ethereum-vanity-address/) · [Bitcoin](https://vanity-address.online/bitcoin-vanity-address/)

</div>

---

## What is a vanity address?

A normal crypto wallet address is random gibberish:

```text
7xKpN4qwRnLB2fGhy9uMcVTAeWDs38ZjQaU5vXrE6bYt
```

A **vanity address** is a real, fully working wallet address that contains a word **you** choose — at the start or the end:

```text
7xKpN4qwRnLB2fGhy9uMcVTAeWDs38ZjQaU5vXrmoon   ← ends with "moon"
DeFiN4qwRnLB2fGhy9uMcVTAeWDs38ZjQaU5vXrE6bY   ← starts with "DeFi"
0xdead...cafe                                  ← Ethereum with dead / cafe
```

### Why would I want one?

- **Branding** — projects, DAOs and creators use recognizable deposit addresses (`...pump`, `...dao`, `...shop`)
- **Personal identity** — your name or handle in your wallet address
- **Easy verification** — you (and your users) can spot the right address at a glance and avoid copy-paste scams

### How does this tool make one?

There is no shortcut in cryptography — the tool rapidly generates **millions of random keypairs per second** on your CPU and keeps the first one whose address matches your pattern. That keypair is 100% standard: import the private key into Phantom, MetaMask, or any normal wallet and use it like any other account.

Everything runs **locally on your machine**. Nothing is sent anywhere.

---

## Quick start

```bash
# No install needed — runs instantly
npx vanity-address
```

This opens an interactive wizard: **pick a chain → type your word → see the time estimate → grind → get your keys.**

Or install globally:

```bash
npm install -g vanity-address
vanity-address
```

> Requires **Node.js 18+**. Works on macOS (Intel & Apple Silicon), Linux x64, Windows x64.

---

## Examples

```bash
# Solana address ending in "moon"
npx vanity-address --chain sol --suffix moon

# Ethereum address starting with 0xdead
npx vanity-address --chain evm --prefix dead

# Both prefix AND suffix
npx vanity-address --chain sol --prefix Cool --suffix xyz

# Save the keys to a file (vanity-results.txt)
npx vanity-address --chain sol --suffix moon --save

# JSON output for scripts / automation
npx vanity-address --chain sol --suffix ax --json --no-benchmark

# EVM contract starting with 0xcafe (deployer's first deploy, nonce 0)
npx vanity-address --chain evm-contract --prefix cafe

# Tron contract starting with TCafe…
npx vanity-address --chain trx-contract --prefix Cafe

# CREATE2 salt for a known factory
npx vanity-address --create2 --deployer 0x… --init-code-hash 0x… --prefix cafe

# Solana token mint ending in "pump" → spl-token create-token mint.json
npx vanity-address --chain sol-mint --suffix pump
```

Example output:

```text
  Chain     Solana
  Target    ending with 'moon'
  Expected  ~656M attempts (average)

⠋ 12.4M keys | 2,198,421 keys/s | ~5 min remaining

 Match found!

  Address   7xKpN4qwRnLB2fGhy9uMcVTAeWDs38ZjQaU5vXrmoon
  Time      142.31s

 Private Keys
  Never share these with anyone.
```

> ⏱ **Tip:** short patterns (3–4 characters) take seconds-to-minutes. Every extra character multiplies the time by ~30–60×. The tool shows an honest estimate **before** starting, so you can decide.

---

## Supported chains (44)

| Chain | `--chain` | Address style | Works with |
| ----- | --------- | ------------- | ---------- |
| Algorand | `algo` | base32 | Pera, Defly |
| Aptos | `aptos` | `0x` hex | Petra |
| Bitcoin | `btc` | base58 (P2PKH) | Electrum, Sparrow |
| Bitcoin Gold | `btg` | base58 (P2PKH `G…`) | — |
| Bitcoin SegWit | `btc-segwit` | `bc1q…` | Sparrow, Electrum |
| Bitcoin Taproot | `btc-taproot` | `bc1p…` | Sparrow |
| Canada eCoin | `cdn` | base58 (P2PKH `C…`) | — |
| Cardano | `ada` | `addr1…` enterprise | — |
| Celestia | `tia` | bech32 (`celestia1…`) | Keplr |
| Cosmos | `cosmos` | bech32 (`cosmos1…`) | Keplr |
| Dash | `dash` | base58 (P2PKH `X…`) | Dash Core |
| DigiByte | `dgb` | base58 (P2PKH `D…`) | — |
| Dogecoin | `doge` | base58 | — |
| dYdX | `dydx` | bech32 | — |
| Einsteinium | `emc2` | base58 (P2PKH `E…`) | — |
| Ethereum + all EVM | `evm` | `0x` hex | MetaMask, Rabby, Base, Hyperliquid, Sonic, Robinhood… |
| Filecoin | `fil` | `f1…` | Lotus, Glif |
| Hedera | `hedera` | ed25519 pubkey hex | HashPack (0.0.N on-chain) |
| Injective | `inj` | bech32 (`inj1…`) | Keplr |
| Internet Computer | `icp` | principal | dfx / NNS |
| Kaspa | `kaspa` | `kaspa:…` | Kaspium |
| Kusama | `ksm` | SS58 ed25519 | Polkadot.js |
| Litecoin | `ltc` | base58 | — |
| Monacoin | `mona` | base58 (P2PKH `M…`) | — |
| MultiversX | `erd` | bech32 (`erd1…`) | xPortal |
| NEAR | `near` | hex implicit | — |
| Novacoin | `nvc` | base58 (P2PKH `4…`) | — |
| Osmosis | `osmo` | bech32 (`osmo1…`) | Keplr |
| Peercoin | `ppc` | base58 (P2PKH `P…`) | — |
| Polkadot | `dot` | SS58 ed25519 | Polkadot.js |
| Primecoin | `xpm` | base58 (P2PKH `A…`) | — |
| Qtum | `qtum` | base58 (P2PKH `Q…`) | — |
| Ravencoin | `rvn` | base58 (P2PKH `R…`) | — |
| Ripple | `xrp` | base58 (`r…`) | — |
| Sei | `sei` | bech32 (`sei1…`) | Compass |
| Solana | `sol` | base58 | Phantom, Solflare |
| Stellar | `xlm` | strkey (`G…`) | — |
| Sui | `sui` | `0x` hex | Sui Wallet |
| Syscoin | `sys` | base58 (P2PKH `S…`) | — |
| Tezos | `xtz` | `tz1…` | Temple, Kukai |
| TON | `ton` | `UQ…` V4R2 | Tonkeeper |
| Tron | `trx` | base58 (`T…`) | TronLink |
| Vertcoin | `vtc` | base58 (P2PKH `V…`) | — |
| Viacoin | `via` | base58 (P2PKH `V…`) | — |

### Deploy modes

| Mode | How to run | What you get |
| ---- | ---------- | ------------ |
| EVM contract (CREATE) | `--chain evm-contract` | Deployer key whose **first** deploy (nonce 0) lands on your vanity contract |
| Tron contract (CREATE) | `--chain trx-contract` | Same idea, Tron `T…` contract address |
| EVM CREATE2 | `--create2 --deployer … --init-code-hash …` | Salt for a factory deploy (not a private key) |
| Solana token mint | `--chain sol-mint` | Mint keypair JSON for `spl-token create-token mint.json` |

---

## All flags

| Flag | What it does | 
| ---- | ------------ |
| `--chain <ID>` | Which blockchain or deploy mode (see tables above) |
| `--prefix <WORD>` | Address must **start** with this (`*` OK) |
| `--suffix <WORD>` | Address must **end** with this (`*` OK) |
| `--contains <WORD>` | Address must **contain** this (`*` OK) |
| `--count <N>` | Find N matches |
| `--exact` | Match upper/lowercase exactly |
| `--save` | Save keys to `vanity-results.txt` (owner-only on macOS/Linux) |
| `--output <PATH>` | Custom file for saved keys |
| `--json` | Machine-readable output |
| `--threads <N>` | Limit CPU threads used |
| `--no-benchmark` | Skip the 2s speed warm-up |
| `-q, --quiet` | Minimal output |

Full guide with pattern rules per chain: [docs/USAGE.md](https://github.com/yudizaxay/vanity-address/blob/main/docs/USAGE.md)

---

## Programmatic API

```js
const {
  generateAddress,
  estimateDifficulty,
  listChains,
  listDeployModes,
} = require("vanity-address");

listDeployModes(); // evm-contract, trx-contract, sol-mint, create2
// Aliases: contract → evm-contract, mint → sol-mint, tron-contract → trx-contract

console.log(estimateDifficulty({ chain: "evm", prefix: "abc", workers: 4 }));

const wallet = await generateAddress({
  chain: "evm",
  prefix: "abc",
  workers: 4, // Node: multi-core by default
  onProgress: (attempts, { keysPerSec, etaSeconds }) => {
    /* ... */
  },
});
console.log(wallet.address, wallet.privateKey);

// CREATE2 — pass deployer + initCodeHash; privateKey is the salt
await generateAddress({
  chain: "create2",
  prefix: "cafe",
  deployer: "0xYourFactory…",
  initCodeHash: "0x…",
});
```

Also: `generateAddresses({ count })`, `validatePattern()`, `isValidChain()`, `timeoutMs`, AbortSignal.

Supports `import` (ESM) and `require` (CJS). Node uses a **worker_threads** pool by default; browsers stay single-threaded.

📖 **Full guide:** [docs/SDK.md](https://github.com/yudizaxay/vanity-address/blob/main/docs/SDK.md)

---

## Is it safe?

| Question | Answer |
| -------- | ------ |
| Does it send my keys anywhere? | **No.** The tool makes zero network connections. |
| Are the wallets real? | Yes — standard keypairs, importable into Phantom / MetaMask / etc. |
| Is it open source? | Yes, MIT licensed — [read the code](https://github.com/yudizaxay/vanity-address) |
| Telemetry / analytics? | None. |
| Saved key files? | Created owner-only (`0600`) on macOS/Linux; secrets wiped from memory after use. |

⚠️ **Golden rule:** whoever has the private key controls the funds. Never share it, and protect `vanity-results.txt` if you use `--save`.

---

## How the npm package works

This package doesn't compile anything on your machine. It ships a **pre-built native binary** (written in Rust) for your platform via `optionalDependencies` — the same approach used by esbuild and Biome:

| Platform package | For |
| ---------------- | --- |
| `vanity-address-darwin-arm64` | macOS Apple Silicon (M1–M4) |
| `vanity-address-darwin-x64` | macOS Intel |
| `vanity-address-linux-x64` | Linux x86_64 |
| `vanity-address-win32-x64` | Windows x64 |

npm installs only the one matching your OS. No `postinstall` scripts. Binaries are built by [GitHub Actions](https://github.com/yudizaxay/vanity-address/actions) and are identical to the [GitHub Release](https://github.com/yudizaxay/vanity-address/releases) assets.

**Speed:** grinds on all CPU cores in parallel — millions of keys per second on modern hardware.

---

## Prefer another install method?

| Method | Command |
| ------ | ------- |
| Homebrew (macOS/Linux) | `brew tap yudizaxay/tap && brew trust yudizaxay/tap && brew install vanity-address` |
| Cargo (Rust) | `cargo install vanity-address` |
| Direct download | [GitHub Releases](https://github.com/yudizaxay/vanity-address/releases/latest) |
| **Desktop app (GUI)** | [.dmg / .exe downloads](https://github.com/yudizaxay/vanity-address/releases/latest) — same engine, point-and-click |

---

## Links

- 📖 [Full documentation](https://github.com/yudizaxay/vanity-address#readme)
- 🚀 [Usage guide (all chains + JSON)](https://github.com/yudizaxay/vanity-address/blob/main/docs/USAGE.md)
- 📦 [Install guide](https://github.com/yudizaxay/vanity-address/blob/main/docs/INSTALL.md)
- 🐛 [Report an issue](https://github.com/yudizaxay/vanity-address/issues)
- 🔐 [Security policy](https://github.com/yudizaxay/vanity-address/blob/main/SECURITY.md)

---

<div align="center">

**MIT © [yudizaxay](https://github.com/yudizaxay)**

If this saved you time, [star the repo ⭐](https://github.com/yudizaxay/vanity-address)

</div>
