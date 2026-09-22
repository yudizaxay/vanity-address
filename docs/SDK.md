# Programmatic SDK — setup & usage

`vanity-address` ships two things in one npm package: the CLI (`npx vanity-address`)
and a programmatic API you can call from your own Node.js code. It runs entirely
in-process via WebAssembly: no subprocess, no native compiler for *your* app,
works the same on macOS/Linux/Windows and in bundlers/browsers.

Use this when your own app needs to grind a vanity address and get the private
key back as data, instead of shelling out to the CLI.

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
const { generateAddress } = require("vanity-address");

const wallet = await generateAddress({ chain: "evm", prefix: "abc" });

console.log(wallet.address);     // 0xabc...
console.log(wallet.privateKey);  // first export value (convenience)
console.log(wallet.exports);     // full list of key formats
```

```js
import { generateAddress } from "vanity-address";

const wallet = await generateAddress({ chain: "sui", prefix: "0xcafe", workers: 4 });
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

### `listChains()` / `isValidChain(id)`

```js
listChains();          // [{ id: "algo", name: "Algorand (base32)" }, ...]
isValidChain("eth");   // true (alias → EVM)
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

All **31** chains — same ids as the CLI `--chain` flag:

| id | chain | id | chain |
|---|---|---|---|
| `algo` | Algorand | `ksm` | Kusama |
| `aptos` | Aptos | `ltc` | Litecoin |
| `btc` | Bitcoin (P2PKH) | `erd` | MultiversX |
| `btc-segwit` | Bitcoin SegWit (`bc1q`) | `near` | NEAR |
| `btc-taproot` | Bitcoin Taproot (`bc1p`) | `osmo` | Osmosis |
| `ada` | Cardano | `dot` | Polkadot |
| `tia` | Celestia | `xrp` | Ripple |
| `cosmos` | Cosmos | `sei` | Sei |
| `dash` | Dash | `sol` | Solana |
| `doge` | Dogecoin | `xlm` | Stellar |
| `dydx` | dYdX | `sui` | Sui |
| `evm` | EVM (+ Base, Robinhood, …) | `xtz` | Tezos |
| `fil` | Filecoin | `ton` | TON |
| `hedera` | Hedera | `trx` | Tron |
| `inj` | Injective | | |
| `icp` | Internet Computer | | |
| `kaspa` | Kaspa | | |

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
