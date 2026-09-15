"use strict";

const wasm = require("../wasm/nodejs/vanity_wasm.js");
const { createSdk, createGenerateAddress, VanityAddressError } = require("./driver.js");

let grindWithWorkers;
let createWorkerPool;
try {
  // worker_threads is Node-only; browsers / bundlers fall back to single-thread.
  const pool = require("./pool.js");
  grindWithWorkers = pool.grindWithWorkers;
  createWorkerPool = pool.createWorkerPool;
} catch {
  grindWithWorkers = undefined;
  createWorkerPool = undefined;
}

const sdk = createSdk(wasm, { grindWithWorkers, createWorkerPool });

module.exports.generateAddress = sdk.generateAddress;
module.exports.generateAddresses = sdk.generateAddresses;
module.exports.estimateDifficulty = sdk.estimateDifficulty;
module.exports.validatePattern = sdk.validatePattern;
module.exports.listChains = sdk.listChains;
module.exports.isValidChain = sdk.isValidChain;
module.exports.VanityAddressError = VanityAddressError;
module.exports.createGenerateAddress = createGenerateAddress;
