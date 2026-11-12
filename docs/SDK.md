# Programmatic SDK — setup & usage

`vanity-address` ships two things in one npm package: the CLI (`npx vanity-address`)
and a programmatic API you can call from your own Node.js code. It runs entirely
in-process via WebAssembly: no subprocess, no native compiler for *your* app,
works the same on macOS/Linux/Windows and in bundlers/browsers.

Use this when your own app needs to grind a vanity address and get the private
key back as data, instead of shelling out to the CLI.

**Surface limits:** the WASM SDK and web generator are CPU-only. OpenCL GPU,
Bitcoin puzzle mode, and split-key live on the **CLI** and **desktop app** only.

---

## Install

```bash
npm install vanity-address
```

**Requirements:** Node.js ≥ 18. CommonJS + ESM. Browser/bundler builds use a
single-thread grind; **Node uses a multi-core `worker_threads` pool by default**.

---

## Quick start

```js
const { generateAddress, listDeployModes } = require("vanity-address");

const wallet = await generateAddress({ chain: "evm", prefix: "abc" });

console.log(wallet.address);     // 0xabc...
console.log(wallet.privateKey);  // first export value (convenience)
console.log(wallet.exports);     // full list of key formats

listDeployModes(); // evm-contract, trx-contract, sol-mint, create2
```

```js
import { generateAddress } from "vanity-address";

const wallet = await generateAddress({ chain: "sui", prefix: "0xcafe", workers: 4 });

// Contract vanity (CREATE · nonce 0) — privateKey is the deployer key
const c = await generateAddress({ chain: "evm-contract", prefix: "cafe" });

// CREATE2 — privateKey is the salt; pass factory params
const { address, privateKey: salt } = await generateAddress({
  chain: "create2",
  prefix: "cafe",
  deployer: "0xYourFactory…",
  initCodeHash: "0x…",
});
```

TypeScript types ship in the package — no `@types` needed. Prefer `ChainId` for
autocomplete (`"sol" | "evm" | …`).

---

## API reference

### `generateAddress(options)`

```ts
interface GenerateAddressOptions {
  chain: string;              // required — see Supported chains
  prefix?: string;            // at least one of prefix/suffix/contains required
  suffix?: string;
  contains?: string;          // substring anywhere; `*` wildcards OK
  caseSensitive?: boolean;    // default: false
  onProgress?: (attempts: number, info: ProgressInfo) => void;
  signal?: AbortSignal;
  timeoutMs?: number;         // rejects with TimeoutError
  workers?: number;           // Node: default = CPU count; set 1 to disable pool
  keysPerSec?: number;        // override ETA heuristic
  count?: number;             // generateAddresses batch size
  deployer?: string;          // CREATE2: 20-byte hex deployer / factory
  initCodeHash?: string;      // CREATE2: 32-byte hex keccak256(init code)
}

interface ProgressInfo {
  attempts: number;
  keysPerSec: number;
  etaSeconds?: number;
}

interface Wallet {
  address: string;
  exports: KeyExport[];
  privateKey?: string;        // === exports[0].value when present
}
```

### `generateAddresses(options)`

Same as `generateAddress`, plus required `count: number`. Returns `Promise<Wallet[]>`.
Wallets are generated **sequentially** (safe for key handling).

### `estimateDifficulty(options)`

Returns expected attempts, human labels, risk (`none` | `caution` | `long` | `impractical`),
and ETA. Pass `workers` to scale the single-thread heuristic.

```js
const est = estimateDifficulty({ chain: "sol", suffix: "moon", workers: 8 });
console.log(est.attemptsLabel, est.timeLabel, est.risk);
```

### `validatePattern(options)`

Cheap check before grinding — charset / empty pattern / unknown chain:

```js
const v = validatePattern({ chain: "evm", prefix: "zzzz" });
// { ok: false, error: "'prefix' contains 'z' — must be hex ..." }
```

### `listChains()` / `listDeployModes()` / `isValidChain(id)`

```js
listChains();          // [{ id: "algo", name: "Algorand (base32)" }, ...]
listDeployModes();     // [{ id: "evm-contract", name: "…", needsFactory: false }, …]
isValidChain("eth");   // true (alias → EVM)
isValidChain("create2"); // true (still needs deployer + initCodeHash at grind time)
```

### Errors

```ts
class VanityAddressError extends Error {
  code: "INVALID_CHAIN" | "INVALID_PATTERN" | "INTERNAL";
}
```

Cancellation uses a standard `AbortError`. `timeoutMs` rejects with `TimeoutError`.

---

## Progress reporting

```js
await generateAddress({
  chain: "evm",
  prefix: "dead",
  onProgress: (attempts, { keysPerSec, etaSeconds }) => {
    console.log(attempts, Math.round(keysPerSec), "keys/s", etaSeconds);
  },
});
```

The first argument remains a **number** so older `(attempts) => …` callbacks keep working.

## Cancellation & timeout

```js
const controller = new AbortController();
setTimeout(() => controller.abort(), 10_000);

await generateAddress({
  chain: "btc",
  prefix: "1love",
  signal: controller.signal,
  timeoutMs: 60_000, // optional belt-and-suspenders
});
```

---

## Supported chains

All **49** chains — same ids as the CLI `--chain` flag:

| id | chain | id | chain |
|---|---|---|---|
| `algo` | Algorand | `ksm` | Kusama |
| `aptos` | Aptos | `ltc` | Litecoin |
| `btc` | Bitcoin (P2PKH) | `mona` | Monacoin |
| `btg` | Bitcoin Gold | `erd` | MultiversX |
| `btc-segwit` | Bitcoin SegWit (`bc1q`) | `near` | NEAR |
| `btc-taproot` | Bitcoin Taproot (`bc1p`) | `nvc` | Novacoin |
| `cdn` | Canada eCoin | `osmo` | Osmosis |
| `ada` | Cardano | `ppc` | Peercoin |
| `tia` | Celestia | `dot` | Polkadot |
| `cosmos` | Cosmos | `xpm` | Primecoin |
| `dash` | Dash | `qtum` | Qtum |
| `dgb` | DigiByte | `rvn` | Ravencoin |
| `doge` | Dogecoin | `xrp` | Ripple |
| `dydx` | dYdX | `sei` | Sei |
| `emc2` | Einsteinium | `sol` | Solana |
| `evm` | EVM (+ Base, Hyperliquid, Sonic, Robinhood, …) | `xlm` | Stellar |
| `fil` | Filecoin | `sui` | Sui |
| `hedera` | Hedera | `sys` | Syscoin |
| `inj` | Injective | `xtz` | Tezos |
| `icp` | Internet Computer | `ton` | TON |
| `kaspa` | Kaspa | `trx` | Tron |
| | | `vtc` | Vertcoin |
| | | `via` | Viacoin |

Deploy modes (also from `listDeployModes()`; accepted as `chain`):

| id | what you get | `privateKey` / first export |
|---|---|---|
| `evm-contract` | contract address from the deployer's first deploy (CREATE, nonce 0) | deployer private key (hex) |
| `trx-contract` | Tron contract address (`T…`) from CREATE / nonce 0 | deployer private key (hex) |
| `sol-mint` | SPL token mint address | mint secret key (base58); `exports[1]` is the keypair JSON for `spl-token` |
| `create2` | CREATE2 contract address | salt (hex) — pass `deployer` + `initCodeHash` |

Aliases (same grind): `contract` / `evm-create` → `evm-contract`, `tron-contract` → `trx-contract`, `mint` / `spl-mint` → `sol-mint`.

```js
const { address, privateKey: salt } = await generateAddress({
  chain: "create2",
  prefix: "cafe",
  deployer: "0xYourFactory…",
  initCodeHash: "0x…", // keccak256(init code)
});
```

---

## How it works / safety

Generation runs **inside your process** via a WebAssembly build of the same Rust
engine as the CLI. Nothing is sent over the network.

On Node, multiple `worker_threads` each load WASM and grind in parallel; the first
match wins and other workers are stopped. Browsers stay single-threaded (use
`workers: 1` anywhere to force that).

Treat `privateKey` / `exports` like any wallet secret: never log or ship them.

---

## Full runnable example

```bash
cd npm/vanity-address
node examples/demo.js
```

## Related

- [docs/USAGE.md](USAGE.md) — CLI usage
- [docs/NPM.md](NPM.md) — package internals / release
- [npm/vanity-address/README.md](../npm/vanity-address/README.md) — npmjs.com README
