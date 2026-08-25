# Programmatic SDK — setup & usage

`vanity-address` ships two things in one npm package: the CLI (`npx vanity-address`)
and a programmatic API you can call from your own Node.js code — `generateAddress()`.
It runs entirely in-process via WebAssembly: no subprocess, no native binary, works
the same on macOS/Linux/Windows and in bundlers/browsers.

Use this when your own app (a wallet generator, an onboarding flow, a backend
service) needs to grind a vanity address and get the private key back as data,
instead of shelling out to the CLI and parsing its output.

---

## Install

```bash
npm install vanity-address
```

That's it — no separate build step, no native compiler needed by *your*
project. The WASM binary ships pre-built inside the package.

**Requirements:** Node.js ≥ 18. Works with CommonJS (`require`) and ES modules
(`import`) out of the box, and with bundlers (Vite, webpack, esbuild) for
browser use.

---

## Quick start

```js
const { generateAddress } = require("vanity-address");

const wallet = await generateAddress({ chain: "evm", prefix: "abc" });

console.log(wallet.address);
// 0xabc1234...

console.log(wallet.exports);
// [{ label: "Private Key (hex)", value: "0x...", hint: null }]
```

ES modules work the same way:

```js
import { generateAddress } from "vanity-address";

const wallet = await generateAddress({ chain: "sui", prefix: "0xcafe" });
```

TypeScript picks up types automatically — no `@types` package needed.

---

## API reference

### `generateAddress(options)`

Returns a `Promise<Wallet>` that resolves once a matching address is found.

```ts
interface GenerateAddressOptions {
  chain: string;                         // required — see "Supported chains" below
  prefix?: string;                       // at least one of prefix/suffix required
  suffix?: string;
  caseSensitive?: boolean;                // default: false
  onProgress?: (attempts: number) => void;
  signal?: AbortSignal;
}

interface KeyExport {
  label: string;    // e.g. "Private Key (hex)", "Keypair (JSON)"
  value: string;
  hint?: string;
}

interface Wallet {
  address: string;
  exports: KeyExport[];   // one or more key formats — chain-dependent
}
```

Note there is no single `privateKey` field — some chains expose more than one
usable key format (e.g. Solana-family chains give both hex and base58), so
`exports` is always an array. Check `label` to find the format you need.

### Errors

Every failure — bad input or an internal grind failure — rejects with a
`VanityAddressError`:

```ts
class VanityAddressError extends Error {
  code: "INVALID_CHAIN" | "INVALID_PATTERN" | "INTERNAL";
}
```

```js
const { generateAddress, VanityAddressError } = require("vanity-address");

try {
  await generateAddress({ chain: "not-a-chain", prefix: "a" });
} catch (err) {
  if (err instanceof VanityAddressError) {
    console.error(err.code, err.message);
  }
}
```

---

## Progress reporting

Grinding a longer pattern can take a while. Pass `onProgress` to get a
running attempt count as the search continues:

```js
const wallet = await generateAddress({
  chain: "evm",
  prefix: "dead",
  onProgress: (attempts) => {
    console.log(`tried ${attempts} addresses so far...`);
  },
});
```

## Cancellation

Pass an `AbortSignal` to stop a grind in progress — the promise rejects with
a standard `AbortError` (same shape as `fetch`'s cancellation):

```js
const controller = new AbortController();

const promise = generateAddress({
  chain: "btc",
  prefix: "1love",
  signal: controller.signal,
});

// e.g. stop after 10 seconds
setTimeout(() => controller.abort(), 10_000);

try {
  const wallet = await promise;
} catch (err) {
  if (err.name === "AbortError") {
    console.log("cancelled");
  }
}
```

---

## Supported chains

The SDK supports every CLI chain **except Solana** (`sol`) — Solana's SDK
can't be compiled to WebAssembly (`solana-sdk` is incompatible with the
`wasm-bindgen` toolchain this package uses), so it stays CLI-only for now.

Use the same chain ids as the CLI's `--chain` flag:

| id | chain | id | chain |
|---|---|---|---|
| `algo` | Algorand | `icp` | Internet Computer |
| `aptos` | Aptos | `kaspa` | Kaspa |
| `btc` | Bitcoin | `ksm` | Kusama |
| `ada` | Cardano | `ltc` | Litecoin |
| `cosmos` | Cosmos | `erd` | MultiversX |
| `dash` | Dash | `near` | NEAR |
| `doge` | Dogecoin | `osmo` | Osmosis |
| `evm` | EVM (Ethereum, etc.) | `dot` | Polkadot |
| `fil` | Filecoin | `xrp` | Ripple |
| `hedera` | Hedera | `xlm` | Stellar |
| | | `sui` | Sui |
| | | `xtz` | Tezos |
| | | `ton` | TON |
| | | `trx` | Tron |

Need Solana? Use the CLI (`npx vanity-address --chain sol ...`) — see
[docs/USAGE.md](USAGE.md).

---

## How it works / is it safe?

Address generation happens **entirely inside your own process**, via a
WebAssembly build of the same Rust engine the CLI uses — nothing is sent
over the network, and the private key never leaves memory you control. The
grind runs in small chunks so your event loop stays responsive; `onProgress`
and `signal` are what let you observe and stop a long-running search.

Treat the returned `exports` values exactly like you would any other private
key: don't log them, don't send them anywhere you don't control, and store
them the same way you'd store any wallet secret.

---

## Full runnable example

See [`npm/vanity-address/examples/demo.js`](../npm/vanity-address/examples/demo.js) —
covers basic generation, progress reporting, cancellation, and error
handling in one file. Run it with:

```bash
cd npm/vanity-address
node examples/demo.js
```

## Related

- [docs/USAGE.md](USAGE.md) — CLI usage, all chains, JSON output
- [docs/NPM.md](NPM.md) — npm package internals, release process
- [npm/vanity-address/README.md](../npm/vanity-address/README.md) — package README shown on npmjs.com
