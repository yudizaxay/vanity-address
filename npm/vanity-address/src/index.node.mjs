import { createRequire } from "node:module";
import { createGenerateAddress, VanityAddressError } from "./driver.js";

const wasm = createRequire(import.meta.url)("../wasm/nodejs/vanity_wasm.js");

export const generateAddress = createGenerateAddress(wasm.grind_chunk);
export { VanityAddressError };
