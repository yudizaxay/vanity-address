import { createRequire } from "node:module";
import { createSdk, VanityAddressError } from "./driver.js";

const require = createRequire(import.meta.url);
const wasm = require("../wasm/nodejs/vanity_wasm.js");

let grindWithWorkers;
let createWorkerPool;
try {
  const pool = require("./pool.js");
  grindWithWorkers = pool.grindWithWorkers;
  createWorkerPool = pool.createWorkerPool;
} catch {
  grindWithWorkers = undefined;
  createWorkerPool = undefined;
}

const sdk = createSdk(wasm, { grindWithWorkers, createWorkerPool });

export const generateAddress = sdk.generateAddress;
export const generateAddresses = sdk.generateAddresses;
export const estimateDifficulty = sdk.estimateDifficulty;
export const validatePattern = sdk.validatePattern;
export const listChains = sdk.listChains;
export const isValidChain = sdk.isValidChain;
export { VanityAddressError };
