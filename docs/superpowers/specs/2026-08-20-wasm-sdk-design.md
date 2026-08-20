# Programmatic WASM SDK for `vanity-address`

Status: approved (design), pending implementation plan
Branch: `feature/wasm-sdk`

## Problem

`vanity-address` currently ships only as a CLI: the npm package
(`npm/vanity-address`) spawns a per-platform native binary
(`vanity-address-{darwin,linux,win32}-*`) via `bin/cli.js` and parses its
output. There is no way for a Node/browser application to call into the
generator programmatically — e.g. a wallet app that wants to grind a vanity
address and private key in-process, in response to user action, without
shelling out to a subprocess.

## Goals

- `npm i vanity-address` already gives CLI (`bin/cli.js`, unchanged).
- Same package additionally exposes a programmatic JS/TS API:
  `generateAddress(options) => Promise<Wallet>`.
- Works in Node **and** bundler/browser environments (no native binary
  dependency for the SDK path).
- Supports all chains `vanity-core` already supports (20+: Solana, EVM,
  Bitcoin/Litecoin/Dogecoin/Dash, Tron, Cosmos/Osmosis, Ripple, Near,
  Polkadot/Kusama, Algorand, Aptos, Cardano, Filecoin, Hedera, ICP, Kaspa,
  MultiversX, Stellar, Sui, Tezos, TON).
- Progress reporting and cancellation for potentially long-running grinds.
- Private keys returned in each chain's native format (matching what the
  CLI already outputs via `json_output.rs`).

## Non-goals

- No change to existing CLI behavior or its native-binary distribution.
- No multi-threaded WASM (SharedArrayBuffer/Web Workers) in this iteration
  — single-threaded WASM with a chunked JS-driven loop is sufficient and
  keeps the build/deploy story simple.
- No key storage, persistence, or transmission — the SDK only generates
  and returns values; what the caller does with them is out of scope.

## Architecture

### New Rust crate: `vanity-wasm`

A thin `wasm-bindgen` binding layer around the existing `vanity-core`
crate (which already implements `ChainGrinder` for every supported
chain — see `vanity-core/src/chains/mod.rs`). It does not reimplement any
grinding logic.

Exposes a single chunked entry point, roughly:

```rust
#[wasm_bindgen]
pub fn grind_chunk(chain: &str, pattern_json: &str, attempts: u32) -> Result<JsValue, JsValue>;
```

- `attempts` bounds how much work one call does (e.g. a few thousand
  tries) so control returns to JS between chunks — this is what makes
  progress reporting and cancellation possible without WASM threads.
- Returns either `{ found: false, attempts: N }` or
  `{ found: true, attempts: N, result: { address, privateKey, publicKey? } }`
  as a JSON-serializable value (`serde-wasm-bindgen`).
- Invalid chain name or malformed pattern returns a `Result::Err` that
  wasm-bindgen surfaces as a JS exception with a stable `code` field.

Built via `wasm-pack build --target bundler` (works for both Node ESM and
bundler/browser consumers) into `npm/vanity-address/wasm/`.

### JS driver: `npm/vanity-address/src/index.js` (+ `index.d.ts`)

Owns the loop that repeatedly calls `grind_chunk`, and is the only part
that knows about progress/cancellation/errors:

```ts
export interface GenerateAddressOptions {
  chain: string;
  prefix?: string;
  suffix?: string;
  caseSensitive?: boolean;
  onProgress?: (attempts: number) => void;
  signal?: AbortSignal;
}

export interface Wallet {
  address: string;
  privateKey: string;   // chain-native format (base58 / hex / WIF, per chain)
  publicKey?: string;
}

export function generateAddress(options: GenerateAddressOptions): Promise<Wallet>;
```

Loop sketch:
1. Validate `options.chain` / pattern up front (reject synchronously-fast
   with a typed error before any grinding starts).
2. Loop: call `grind_chunk`, accumulate attempts, invoke `onProgress`,
   check `signal.aborted` between chunks (reject with `AbortError` if
   set), until `found: true`.
3. Map the wasm result into the `Wallet` shape and resolve.

### Package wiring

`npm/vanity-address/package.json`:
- Add `"exports"`: `"."` → `./src/index.js` (programmatic API), keep
  `"bin"` as-is for the CLI.
- Add `"types"` → `./src/index.d.ts`.
- Add `"files"` entries for `src/`, `wasm/`.
- `wasm/` artifacts are prebuilt at release time (via the existing
  release scripts under `scripts/`), not compiled on `npm install` —
  consistent with how the native CLI binaries are already distributed as
  prebuilt optional deps.

## Error handling

- Synchronous validation errors (bad chain name, empty/invalid pattern)
  reject immediately with `VanityAddressError` (`code: 'INVALID_CHAIN'
  | 'INVALID_PATTERN'`).
- Cancellation via `AbortSignal` rejects with a DOMException-style
  `AbortError` (matches `fetch` semantics, no custom cancellation type
  to learn).
- Rust panics inside the wasm boundary are caught by wasm-bindgen and
  surfaced as JS exceptions, wrapped into `VanityAddressError` with
  `code: 'INTERNAL'` so callers get one consistent error shape.

## Testing

- `wasm-bindgen-test` for the Rust↔WASM boundary (`grind_chunk` chunking,
  found/not-found shapes, error surfacing).
- JS integration tests (project's existing test runner) exercising
  `generateAddress` per chain family with short/fast patterns (1
  character or empty prefix, so grinding finishes near-instantly in CI):
  - returned `address` matches the requested pattern and the chain's
    address-format regex,
  - returned `privateKey`, when re-derived through the same chain logic,
    reproduces the same `address` (round-trip correctness),
  - `onProgress` is called at least once for a multi-chunk grind,
  - an `AbortSignal` aborted mid-grind rejects with `AbortError`,
  - an invalid chain name / pattern rejects synchronously with the
    expected error `code`.

## Open items for the implementation plan

- Exact wasm-pack target/output layout and how it plugs into the
  existing release scripts (`scripts/`, `RELEASING.md`).
- Whether `index.js` ships as CJS, ESM, or dual — needs to match what
  `bin/cli.js` and the package's `"type"` field currently assume.
