"use strict";

/**
 * Node worker_threads pool for parallel WASM grinding.
 * First worker to find a match wins; siblings are cancelled (not killed) so
 * the same worker threads can be reused across a batch (generateAddresses).
 */

const { Worker } = require("node:worker_threads");
const path = require("node:path");

const workerPath = path.join(__dirname, "grind-worker.js");

/**
 * A persistent set of worker threads. Call `.run()` once per grind job
 * (reuses the same threads) and `.destroy()` once the pool is no longer needed.
 */
class WorkerPool {
  constructor(workerCount) {
    this.workers = [];
    for (let i = 0; i < workerCount; i += 1) {
      this.workers.push(new Worker(workerPath));
    }
  }

  /** Run a single grind job across all workers in this pool. */
  run(opts, ctx) {
    const {
      expectedAttempts,
      chunkSize,
      callProgress,
      wrapWallet,
      VanityAddressError,
      abortError,
    } = ctx;
    const workers = this.workers;

    return new Promise((resolve, reject) => {
      let settled = false;
      let totalAttempts = 0;
      const started = Date.now();
      let onAbort = null;
      const listeners = [];

      function detach() {
        if (opts.signal && onAbort) {
          opts.signal.removeEventListener("abort", onAbort);
        }
        for (const { worker, onMessage, onError, onExit } of listeners) {
          worker.off("message", onMessage);
          worker.off("error", onError);
          worker.off("exit", onExit);
        }
      }

      function cancelSiblings(except) {
        for (const w of workers) {
          if (w !== except) {
            try {
              w.postMessage({ type: "cancel" });
            } catch {
              // ignore — worker may already be exiting
            }
          }
        }
      }

      function fail(err) {
        if (settled) return;
        settled = true;
        detach();
        cancelSiblings(null);
        reject(err);
      }

      function succeed(result, winner) {
        if (settled) return;
        settled = true;
        detach();
        cancelSiblings(winner);
        resolve(wrapWallet(result));
      }

      if (opts.signal) {
        if (opts.signal.aborted) {
          fail(abortError());
          return;
        }
        onAbort = () => fail(abortError());
        opts.signal.addEventListener("abort", onAbort, { once: true });
      }

      const job = {
        chain: opts.chain,
        prefix: opts.prefix,
        suffix: opts.suffix,
        contains: opts.contains || "",
        ignoreCase: !opts.caseSensitive,
        chunkSize,
      };

      for (const worker of workers) {
        const onMessage = (msg) => {
          if (settled) return;
          if (!msg || typeof msg !== "object") return;

          if (msg.type === "progress") {
            totalAttempts += msg.attempts || 0;
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
            return;
          }

          if (msg.type === "found") {
            succeed(msg.result, worker);
            return;
          }

          if (msg.type === "error") {
            fail(
              new VanityAddressError(
                msg.message || "worker error",
                msg.code || "INTERNAL",
              ),
            );
          }
        };

        const onError = (err) => {
          fail(new VanityAddressError(err.message || String(err), "INTERNAL"));
        };

        const onExit = (code) => {
          if (settled) return;
          if (code !== 0) {
            fail(new VanityAddressError(`worker exited with code ${code}`, "INTERNAL"));
          }
        };

        worker.on("message", onMessage);
        worker.on("error", onError);
        worker.on("exit", onExit);
        listeners.push({ worker, onMessage, onError, onExit });

        worker.postMessage({ type: "start", job });
      }
    });
  }

  destroy() {
    for (const w of this.workers) {
      try {
        w.terminate();
      } catch {
        // ignore
      }
    }
    this.workers.length = 0;
  }
}

function createWorkerPool(workerCount) {
  return new WorkerPool(workerCount);
}

/** One-shot helper: spin up a pool for a single job, then tear it down. */
async function grindWithWorkers(opts, ctx) {
  const pool = createWorkerPool(ctx.workerCount);
  try {
    return await pool.run(opts, ctx);
  } finally {
    pool.destroy();
  }
}

module.exports = { createWorkerPool, grindWithWorkers };
