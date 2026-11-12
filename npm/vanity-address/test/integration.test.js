"use strict";

const test = require("node:test");
const assert = require("node:assert/strict");
const { generateAddress, VanityAddressError } = require("../src/index.js");

test("bitcoin: generates an address matching the prefix, with key exports", async () => {
  const wallet = await generateAddress({ chain: "btc", prefix: "1" });
  assert.match(wallet.address, /^1[1-9A-HJ-NP-Za-km-z]+$/);
  assert.ok(wallet.exports.length > 0);
  assert.ok(wallet.exports.some((e) => /hex/i.test(e.label)));
});

test("solana: generates an address matching the prefix, with key exports", async () => {
  const wallet = await generateAddress({ chain: "sol", prefix: "1" });
  assert.match(wallet.address, /^1[1-9A-HJ-NP-Za-km-z]+$/);
  assert.ok(wallet.exports.length > 0);
  assert.ok(wallet.exports.some((e) => /base58/i.test(e.label)));
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

test("estimateDifficulty / listChains / validatePattern helpers", () => {
  const { estimateDifficulty, listChains, validatePattern, isValidChain } = require("../src/index.js");
  const chains = listChains();
  assert.ok(chains.length >= 49, `expected >= 49 chains, got ${chains.length}`);
  const ids = new Set(chains.map((c) => c.id));
  // 0.8.0 wasm: prior menu + ZEC/BCH/STX/DCR/CKB (49).
  assert.equal(chains.length, 49);
  assert.ok(ids.has("rvn") && ids.has("btg") && ids.has("mona"));
  assert.ok(ids.has("btc-segwit") && ids.has("btc-taproot") && ids.has("inj") && ids.has("sei"));
  assert.ok(ids.has("zec") && ids.has("bch") && ids.has("stx") && ids.has("dcr") && ids.has("ckb"));
  assert.equal(isValidChain("eth"), true);
  assert.equal(isValidChain("nope"), false);
  const est = estimateDifficulty({ chain: "evm", prefix: "ab" });
  assert.ok(est.attempts > 0);
  assert.ok(est.timeLabel);
  const bad = validatePattern({ chain: "evm", prefix: "zzzz" });
  assert.equal(bad.ok, false);
});

test("listDeployModes + deploy aliases", () => {
  const { listDeployModes, isValidChain } = require("../src/index.js");
  const modes = listDeployModes();
  assert.equal(modes.length, 4);
  const ids = new Set(modes.map((m) => m.id));
  assert.ok(ids.has("evm-contract") && ids.has("trx-contract"));
  assert.ok(ids.has("sol-mint") && ids.has("create2"));
  const create2 = modes.find((m) => m.id === "create2");
  assert.equal(create2.needsFactory, true);
  assert.equal(
    modes.filter((m) => m.id !== "create2").every((m) => !m.needsFactory),
    true,
  );
  // Core aliases resolve through wasm is_valid_chain
  assert.equal(isValidChain("contract"), true);
  assert.equal(isValidChain("evm-create"), true);
  assert.equal(isValidChain("tron-contract"), true);
  assert.equal(isValidChain("mint"), true);
  assert.equal(isValidChain("create2"), true);
});

test("evm-contract: CREATE nonce-0 address matches prefix", async () => {
  const wallet = await generateAddress({
    chain: "evm-contract",
    prefix: "a",
    workers: 2,
  });
  assert.match(wallet.address, /^0x[0-9a-fA-F]{40}$/);
  assert.equal(wallet.address[2].toLowerCase(), "a");
  assert.ok(wallet.privateKey);
});

test("contract alias resolves to evm-contract grind", async () => {
  const wallet = await generateAddress({ chain: "contract", prefix: "b", workers: 2 });
  assert.match(wallet.address, /^0x[0-9a-fA-F]{40}$/);
  assert.equal(wallet.address[2].toLowerCase(), "b");
});

test("create2: salt grind with deployer + initCodeHash", async () => {
  const wallet = await generateAddress({
    chain: "create2",
    prefix: "a",
    deployer: "0x0000000000000000000000000000000000000001",
    initCodeHash: "0xc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470",
    workers: 2,
  });
  assert.match(wallet.address, /^0x[0-9a-fA-F]{40}$/);
  assert.equal(wallet.address[2].toLowerCase(), "a");
  assert.ok(wallet.privateKey);
});

test("create2 rejects without factory args", async () => {
  await assert.rejects(
    generateAddress({ chain: "create2", prefix: "a" }),
    (err) => err instanceof VanityAddressError && err.code === "INVALID_CHAIN",
  );
});

test("generateAddresses returns the requested count", async () => {
  const { generateAddresses } = require("../src/index.js");
  const wallets = await generateAddresses({ chain: "evm", prefix: "a", count: 2, workers: 2 });
  assert.equal(wallets.length, 2);
  for (const w of wallets) {
    assert.equal(w.address[2].toLowerCase(), "a");
    assert.ok(w.privateKey);
  }
});

test('Node ESM entry point (package exports "node" condition) works', async () => {
  // Import the bare package specifier (self-reference), not a file path, so
  // Node actually resolves through package.json "exports" — the same way a
  // real consumer's `import "vanity-address"` would — exercising the "node"
  // condition rather than bypassing it via direct filesystem resolution.
  const mod = await import("vanity-address");
  const wallet = await mod.generateAddress({ chain: "evm", prefix: "a" });
  assert.match(wallet.address, /^0x[0-9a-fA-F]{40}$/);
  assert.equal(typeof mod.listDeployModes, "function");
  assert.ok(mod.listDeployModes().some((m) => m.id === "evm-contract"));
});
