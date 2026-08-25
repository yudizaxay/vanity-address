import * as wasm from "../wasm/bundler/vanity_wasm.js";
import { createGenerateAddress, VanityAddressError } from "./driver.js";

export const generateAddress = createGenerateAddress(wasm.grind_chunk);
export { VanityAddressError };
