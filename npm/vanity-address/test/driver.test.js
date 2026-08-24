"use strict";

const test = require("node:test");
const assert = require("node:assert/strict");
const { createGenerateAddress, VanityAddressError } = require("../src/driver.js");

test("rejects synchronously when chain is missing", async () => {
  const generateAddress = createGenerateAddress(() => {
    throw new Error("should not be called");
  });
  await assert.rejects(
    generateAddress({ prefix: "a" }),
    (err) => err instanceof VanityAddressError && err.code === "INVALID_CHAIN",
  );
});

test("rejects when neither prefix nor suffix given", async () => {
  const generateAddress = createGenerateAddress(() => {
    throw new Error("should not be called");
  });
  await assert.rejects(
    generateAddress({ chain: "sol" }),
    (err) => err instanceof VanityAddressError && err.code === "INVALID_PATTERN",
  );
});

test("calls onProgress once per chunk and resolves on the found chunk", async () => {
  const calls = [];
  let chunkIndex = 0;
  const fakeGrindChunk = () => {
    chunkIndex += 1;
    if (chunkIndex < 3) {
      return { found: false, attempts: 5000 };
    }
    return {
      found: true,
      attempts: 5000,
      result: {
        address: "fakeAddr",
        exports: [{ label: "Private Key (hex)", value: "ab", hint: null }],
      },
    };
  };
  const generateAddress = createGenerateAddress(fakeGrindChunk);
  const wallet = await generateAddress({
    chain: "sol",
    prefix: "a",
    onProgress: (n) => calls.push(n),
  });
  assert.deepEqual(calls, [5000, 10000, 15000]);
  assert.equal(wallet.address, "fakeAddr");
  assert.equal(wallet.exports[0].label, "Private Key (hex)");
});

test("rejects with AbortError when aborted mid-grind", async () => {
  const controller = new AbortController();
  let calls = 0;
  const fakeGrindChunk = () => {
    calls += 1;
    if (calls === 2) controller.abort();
    return { found: false, attempts: 100 };
  };
  const generateAddress = createGenerateAddress(fakeGrindChunk);
  await assert.rejects(
    generateAddress({ chain: "sol", prefix: "a", signal: controller.signal }),
    (err) => err.name === "AbortError",
  );
});

test("rejects immediately if signal is already aborted", async () => {
  const controller = new AbortController();
  controller.abort();
  const generateAddress = createGenerateAddress(() => {
    throw new Error("should not be called");
  });
  await assert.rejects(
    generateAddress({ chain: "sol", prefix: "a", signal: controller.signal }),
    (err) => err.name === "AbortError",
  );
});

test("wraps a thrown wasm error into VanityAddressError", async () => {
  const fakeGrindChunk = () => {
    throw { code: "INVALID_CHAIN", message: "bad chain" };
  };
  const generateAddress = createGenerateAddress(fakeGrindChunk);
  await assert.rejects(
    generateAddress({ chain: "nope", prefix: "a" }),
    (err) => err instanceof VanityAddressError && err.code === "INVALID_CHAIN",
  );
});
