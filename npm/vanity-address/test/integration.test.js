"use strict";

const test = require("node:test");
const assert = require("node:assert/strict");
const { generateAddress, VanityAddressError } = require("../src/index.js");

// NOTE: Solana ("sol") is excluded from the wasm build (wasm-bindgen
// incompatibility fixed by excluding Solana specifically while keeping the
// other 24 chains), so this base58 case uses Bitcoin ("btc") instead of the
// Solana example from the task brief.
test("bitcoin: generates an address matching the prefix, with key exports", async () => {
  const wallet = await generateAddress({ chain: "btc", prefix: "1" });
  assert.match(wallet.address, /^1[1-9A-HJ-NP-Za-km-z]+$/);
  assert.ok(wallet.exports.length > 0);
  assert.ok(wallet.exports.some((e) => /hex/i.test(e.label)));
});

test("evm: generates a 0x-prefixed address matching the pattern", async () => {
  const wallet = await generateAddress({ chain: "evm", prefix: "a" });
  assert.match(wallet.address, /^0x[0-9a-fA-F]{40}$/);
  assert.equal(wallet.address[2].toLowerCase(), "a");
});

test("rejects an unknown chain id", async () => {
  await assert.rejects(
    generateAddress({ chain: "not-a-real-chain", prefix: "a" }),
    (err) => err instanceof VanityAddressError && err.code === "INVALID_CHAIN",
  );
});

test("rejects when no prefix or suffix is given", async () => {
  await assert.rejects(
    generateAddress({ chain: "evm" }),
    (err) => err instanceof VanityAddressError && err.code === "INVALID_PATTERN",
  );
});

test("supports cancellation via AbortSignal", async () => {
  const controller = new AbortController();
  const promise = generateAddress({
    chain: "evm",
    // an unreachable-in-practice 8-char prefix keeps this grinding long enough to abort
    prefix: "aaaaaaaa",
    signal: controller.signal,
    onProgress: () => controller.abort(),
  });
  await assert.rejects(promise, (err) => err.name === "AbortError");
});
