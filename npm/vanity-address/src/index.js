"use strict";

const wasm = require("../wasm/nodejs/vanity_wasm.js");
const { createGenerateAddress, VanityAddressError } = require("./driver.js");

module.exports.generateAddress = createGenerateAddress(wasm.grind_chunk);
module.exports.VanityAddressError = VanityAddressError;
