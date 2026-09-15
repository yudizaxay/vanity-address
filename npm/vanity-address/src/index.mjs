import * as wasm from "../wasm/bundler/vanity_wasm.js";
import { createSdk, VanityAddressError } from "./driver.js";

// Bundlers / browsers: single-thread grind (no worker_threads).
const sdk = createSdk(wasm, {});

export const generateAddress = sdk.generateAddress;
export const generateAddresses = sdk.generateAddresses;
export const estimateDifficulty = sdk.estimateDifficulty;
export const validatePattern = sdk.validatePattern;
export const listChains = sdk.listChains;
export const isValidChain = sdk.isValidChain;
export { VanityAddressError };
