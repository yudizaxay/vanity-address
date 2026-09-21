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

function timeoutError() {
  const err = new Error("timed out");
  err.name = "TimeoutError";
  return err;
}

function wrapWallet(result) {
  const exports = (result && result.exports) || [];
  const privateKey = exports.length > 0 ? exports[0].value : undefined;
  return {
    address: result.address,
    exports,
    privateKey,
  };
}

function callProgress(onProgress, info) {
  if (typeof onProgress !== "function") return;
  // First arg stays a number for backward compatibility; second is the rich info object.
  onProgress(info.attempts, info);
}

function normalizeOptions(options) {
  const opts = options || {};
  return {
    chain: opts.chain,
    prefix: opts.prefix || "",
    suffix: opts.suffix || "",
    contains: opts.contains || "",
    caseSensitive: Boolean(opts.caseSensitive),
    onProgress: opts.onProgress,
    signal: opts.signal,
    timeoutMs: opts.timeoutMs,
    workers: opts.workers,
    keysPerSec: opts.keysPerSec,
    count: opts.count,
  };
}

function requireChainPattern(opts) {
  if (!opts.chain || typeof opts.chain !== "string") {
    throw new VanityAddressError("options.chain is required", "INVALID_CHAIN");
  }
  if (!opts.prefix && !opts.suffix && !opts.contains) {
    throw new VanityAddressError(
      "options.prefix, options.suffix, or options.contains is required",
      "INVALID_PATTERN",
    );
  }
}

function attachTimeout(signal, timeoutMs) {
  if (timeoutMs == null) {
    return { signal, cleanup: () => {} };
  }
  const ms = Number(timeoutMs);
  if (!Number.isFinite(ms) || ms <= 0) {
    throw new VanityAddressError("options.timeoutMs must be a positive number", "INVALID_PATTERN");
  }
  const controller = new AbortController();
  const onAbort = () => controller.abort();
  if (signal) {
    if (signal.aborted) {
      controller.abort();
    } else {
      signal.addEventListener("abort", onAbort, { once: true });
    }
  }
  let timedOut = false;
  const timer = setTimeout(() => {
    timedOut = true;
    controller.abort();
  }, ms);
  return {
    signal: controller.signal,
    timedOut: () => timedOut,
    cleanup: () => {
      clearTimeout(timer);
      if (signal) signal.removeEventListener("abort", onAbort);
    },
  };
}

function grindSingleThreaded(grindChunk, opts, expectedAttempts) {
  return new Promise((resolve, reject) => {
    let totalAttempts = 0;
    const started = Date.now();

    function step() {
      if (opts.signal && opts.signal.aborted) {
        reject(abortError());
        return;
      }

      let chunk;
      try {
        chunk = grindChunk(
          opts.chain,
          opts.prefix,
          opts.suffix,
          !opts.caseSensitive,
          CHUNK_SIZE,
          opts.contains || undefined,
        );
      } catch (e) {
        const code = (e && e.code) || "INTERNAL";
        const message = (e && e.message) || String(e);
        reject(new VanityAddressError(message, code));
        return;
      }

      totalAttempts += chunk.attempts;
      const elapsed = Math.max((Date.now() - started) / 1000, 0.001);
      const keysPerSec = totalAttempts / elapsed;
      const remaining = Math.max((expectedAttempts || 0) - totalAttempts, 0);
      const etaSeconds =
        expectedAttempts && keysPerSec > 0 ? remaining / keysPerSec : undefined;

      callProgress(opts.onProgress, {
        attempts: totalAttempts,
        keysPerSec,
        etaSeconds,
      });

      if (chunk.found) {
        resolve(wrapWallet(chunk.result));
        return;
      }

      setTimeout(step, 0);
    }

    step();
  });
}

/**
 * @param {object} wasm — wasm module exports
 * @param {{ grindWithWorkers?: Function, createWorkerPool?: Function }} [extras]
 */
function createSdk(wasm, extras) {
  const grindChunk = wasm.grind_chunk.bind(wasm);
  const listChainsRaw = wasm.list_chains ? wasm.list_chains.bind(wasm) : null;
  const estimateRaw = wasm.estimate_difficulty
    ? wasm.estimate_difficulty.bind(wasm)
    : null;
  const validateRaw = wasm.validate_pattern ? wasm.validate_pattern.bind(wasm) : null;
  const grindWithWorkers = extras && extras.grindWithWorkers;
  const createWorkerPool = extras && extras.createWorkerPool;

  function listChains() {
    if (!listChainsRaw) {
      throw new VanityAddressError("list_chains is unavailable in this build", "INTERNAL");
    }
    return listChainsRaw();
  }

  function isValidChain(id) {
    if (!id || typeof id !== "string") return false;
    if (typeof wasm.is_valid_chain === "function") {
      return Boolean(wasm.is_valid_chain(id));
    }
    try {
      const chains = listChains();
      const needle = id.toLowerCase();
      return chains.some((c) => c.id === needle);
    } catch {
      return false;
    }
  }

  function estimateDifficulty(options) {
    const opts = normalizeOptions(options);
    requireChainPattern(opts);
    if (!estimateRaw) {
      throw new VanityAddressError("estimate_difficulty is unavailable in this build", "INTERNAL");
    }
    const workers = Math.max(1, Number(opts.workers) || 1);
    const userKps = opts.keysPerSec && opts.keysPerSec > 0 ? opts.keysPerSec : 0;

    let est = estimateRaw(
      opts.chain,
      opts.prefix,
      opts.suffix,
      !opts.caseSensitive,
      userKps,
      opts.contains || undefined,
    );

    // Re-run through the same wasm estimator at the scaled throughput so
    // risk/difficulty/timeLabel all stay consistent with the faster ETA,
    // instead of scaling avgSecs/keysPerSec in JS while reusing single-thread labels.
    if (workers > 1 && !userKps) {
      const scaledKps = est.keys_per_sec * workers;
      est = estimateRaw(
        opts.chain,
        opts.prefix,
        opts.suffix,
        !opts.caseSensitive,
        scaledKps,
        opts.contains || undefined,
      );
    }

    return {
      attempts: est.attempts,
      attemptsLabel: est.attempts_label,
      avgSecs: est.avg_secs,
      timeLabel: est.time_label,
      difficulty: est.difficulty,
      difficultyBars: est.difficulty_bars,
      risk: est.risk,
      patternChars: est.pattern_chars,
      keysPerSec: est.keys_per_sec,
      workers,
    };
  }

  function validatePattern(options) {
    const opts = normalizeOptions(options);
    if (!opts.chain || typeof opts.chain !== "string") {
      return {
        ok: false,
        error: "options.chain is required",
      };
    }
    if (!opts.prefix && !opts.suffix && !opts.contains) {
      return {
        ok: false,
        error: "options.prefix, options.suffix, or options.contains is required",
      };
    }
    if (!validateRaw) {
      throw new VanityAddressError("validate_pattern is unavailable in this build", "INTERNAL");
    }
    const raw = validateRaw(
      opts.chain,
      opts.prefix,
      opts.suffix,
      !opts.caseSensitive,
      opts.contains || undefined,
    );
    return {
      ok: Boolean(raw.ok),
      error: raw.error || undefined,
      risk: raw.risk || undefined,
      attempts: raw.attempts != null ? raw.attempts : undefined,
      attemptsLabel: raw.attempts_label || undefined,
      timeLabel: raw.time_label || undefined,
    };
  }

  /**
   * Shared grind implementation for one address. When `pool` is given (an
   * already-running WorkerPool), its worker threads are reused instead of
   * spinning up a fresh set — used by generateAddresses to avoid respawning
   * workers per item in a batch.
   */
  async function grindOnce(opts, pool) {
    const timeout = attachTimeout(opts.signal, opts.timeoutMs);
    opts.signal = timeout.signal;

    let expectedAttempts = 0;
    try {
      if (estimateRaw) {
        const est = estimateRaw(
          opts.chain,
          opts.prefix,
          opts.suffix,
          !opts.caseSensitive,
          0,
          opts.contains || undefined,
        );
        expectedAttempts = est.attempts || 0;
      }
    } catch {
      // estimate is best-effort for progress ETA
    }

    try {
      if (opts.signal && opts.signal.aborted) {
        if (timeout.timedOut && timeout.timedOut()) {
          throw timeoutError();
        }
        throw abortError();
      }

      const grindCtx = {
        expectedAttempts,
        chunkSize: CHUNK_SIZE,
        callProgress,
        wrapWallet,
        VanityAddressError,
        abortError,
      };

      let wallet;
      if (pool) {
        wallet = await pool.run(opts, grindCtx);
      } else {
        const workerCount = resolveWorkerCount(opts.workers);
        if (workerCount > 1 && typeof grindWithWorkers === "function") {
          wallet = await grindWithWorkers(opts, { ...grindCtx, workerCount });
        } else {
          wallet = await grindSingleThreaded(grindChunk, opts, expectedAttempts);
        }
      }
      return wallet;
    } catch (err) {
      if (err && err.name === "AbortError" && timeout.timedOut && timeout.timedOut()) {
        throw timeoutError();
      }
      throw err;
    } finally {
      timeout.cleanup();
    }
  }

  async function generateAddress(options) {
    const opts = normalizeOptions(options);
    try {
      requireChainPattern(opts);
    } catch (e) {
      return Promise.reject(e);
    }
    return grindOnce(opts, null);
  }

  async function generateAddresses(options) {
    const opts = normalizeOptions(options);
    try {
      requireChainPattern(opts);
    } catch (e) {
      return Promise.reject(e);
    }
    const count = Math.floor(Number(opts.count));
    if (!Number.isFinite(count) || count < 1) {
      throw new VanityAddressError(
        "options.count must be a positive integer",
        "INVALID_PATTERN",
      );
    }

    const workerCount = resolveWorkerCount(opts.workers);
    const pool =
      workerCount > 1 && typeof createWorkerPool === "function"
        ? createWorkerPool(workerCount)
        : null;

    try {
      const out = [];
      for (let i = 0; i < count; i += 1) {
        out.push(await grindOnce({ ...opts, count: undefined }, pool));
      }
      return out;
    } finally {
      if (pool) pool.destroy();
    }
  }

  // Back-compat factory used by unit tests with a fake grind_chunk.
  function generateAddressCompat(options) {
    return generateAddress(options);
  }

  return {
    generateAddress: generateAddressCompat,
    generateAddresses,
    estimateDifficulty,
    validatePattern,
    listChains,
    isValidChain,
    VanityAddressError,
    // test helper
    _createGenerateAddress: (fakeGrind) => createGenerateAddress(fakeGrind),
  };
}

/** Legacy factory kept for unit tests that inject a fake grind_chunk. */
function createGenerateAddress(grindChunk) {
  return function generateAddress(options) {
    const opts = normalizeOptions(options);
    try {
      requireChainPattern(opts);
    } catch (e) {
      return Promise.reject(e);
    }
    if (opts.signal && opts.signal.aborted) {
      return Promise.reject(abortError());
    }

    const timeout = attachTimeout(opts.signal, opts.timeoutMs);
    opts.signal = timeout.signal;

    return grindSingleThreaded(grindChunk, opts, 0)
      .catch((err) => {
        if (err && err.name === "AbortError" && timeout.timedOut && timeout.timedOut()) {
          throw timeoutError();
        }
        throw err;
      })
      .finally(() => timeout.cleanup());
  };
}

function resolveWorkerCount(workers) {
  if (workers === 0 || workers === 1) return 1;
  if (workers != null && Number(workers) > 0) {
    return Math.min(32, Math.floor(Number(workers)));
  }
  try {
    const os = require("node:os");
    const n = os.cpus() && os.cpus().length;
    return n && n > 1 ? Math.min(n, 16) : 1;
  } catch {
    return 1;
  }
}

module.exports = {
  createSdk,
  createGenerateAddress,
  VanityAddressError,
  wrapWallet,
  callProgress,
  CHUNK_SIZE,
};
