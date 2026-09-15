"use strict";

const { parentPort } = require("node:worker_threads");
const path = require("node:path");

// Load the same nodejs wasm build the main thread uses.
const wasm = require(path.join(__dirname, "..", "wasm", "nodejs", "vanity_wasm.js"));

// `running` guards against overlapping jobs on this worker; `cancelled` lets the
// pool stop this worker's current grind (when a sibling wins) without killing
// the thread, so the same worker can be reused for the next job in a batch.
let running = false;
let cancelled = false;

parentPort.on("message", (msg) => {
  if (!msg) return;

  if (msg.type === "cancel") {
    cancelled = true;
    return;
  }

  if (msg.type !== "start" || running) return;
  running = true;
  cancelled = false;
  const job = msg.job;
  const chunkSize = job.chunkSize || 5000;

  try {
    // eslint-disable-next-line no-constant-condition
    while (true) {
      if (cancelled) {
        running = false;
        cancelled = false;
        parentPort.postMessage({ type: "stopped" });
        return;
      }

      const chunk = wasm.grind_chunk(
        job.chain,
        job.prefix,
        job.suffix,
        job.ignoreCase,
        chunkSize,
      );

      // Report attempts for every chunk, including the winning one, so
      // onProgress fires even when a short pattern matches immediately.
      parentPort.postMessage({ type: "progress", attempts: chunk.attempts });

      if (chunk.found) {
        running = false;
        parentPort.postMessage({ type: "found", result: chunk.result });
        return;
      }
    }
  } catch (e) {
    running = false;
    parentPort.postMessage({
      type: "error",
      code: (e && e.code) || "INTERNAL",
      message: (e && e.message) || String(e),
    });
  }
});
