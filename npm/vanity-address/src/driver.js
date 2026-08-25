"use strict";

const CHUNK_SIZE = 5000;

class VanityAddressError extends Error {
  constructor(message, code) {
    super(message);
    this.name = "VanityAddressError";
    this.code = code;
  }
}

function abortError() {
  const err = new Error("aborted");
  err.name = "AbortError";
  return err;
}

function createGenerateAddress(grindChunk) {
  return function generateAddress(options) {
    const {
      chain,
      prefix = "",
      suffix = "",
      caseSensitive = false,
      onProgress,
      signal,
    } = options || {};

    if (!chain || typeof chain !== "string") {
      return Promise.reject(new VanityAddressError("options.chain is required", "INVALID_CHAIN"));
    }
    if (!prefix && !suffix) {
      return Promise.reject(
        new VanityAddressError("options.prefix or options.suffix is required", "INVALID_PATTERN"),
      );
    }
    if (signal && signal.aborted) {
      return Promise.reject(abortError());
    }

    return new Promise((resolve, reject) => {
      let totalAttempts = 0;

      function step() {
        if (signal && signal.aborted) {
          reject(abortError());
          return;
        }

        let chunk;
        try {
          chunk = grindChunk(chain, prefix, suffix, !caseSensitive, CHUNK_SIZE);
        } catch (e) {
          const code = (e && e.code) || "INTERNAL";
          const message = (e && e.message) || String(e);
          reject(new VanityAddressError(message, code));
          return;
        }

        totalAttempts += chunk.attempts;
        if (typeof onProgress === "function") {
          onProgress(totalAttempts);
        }

        if (chunk.found) {
          resolve({ address: chunk.result.address, exports: chunk.result.exports });
          return;
        }

        setTimeout(step, 0);
      }

      step();
    });
  };
}

module.exports = { createGenerateAddress, VanityAddressError };
