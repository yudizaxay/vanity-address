// Demo: vanity-address SDK usage (v0.5+)
//
// Run from this directory:  node examples/demo.js
// (or after `npm install vanity-address`, use `require("vanity-address")`)

const {
  generateAddress,
  generateAddresses,
  estimateDifficulty,
  validatePattern,
  listChains,
  isValidChain,
  VanityAddressError,
} = require("../src/index.js");

async function main() {
  console.log("0) Chains:", listChains().length, "| eth alias?", isValidChain("eth"));

  console.log("\n1) Estimate before grinding");
  const est = estimateDifficulty({ chain: "evm", prefix: "ab", workers: 4 });
  console.log("  ", est.attemptsLabel, "attempts ·", est.timeLabel, "·", est.difficulty, est.risk);

  console.log("\n2) Validate a bad pattern");
  console.log("  ", validatePattern({ chain: "evm", prefix: "zzzz" }));

  console.log("\n3) Basic usage — EVM prefix 'a' (multi-worker)");
  const wallet1 = await generateAddress({ chain: "evm", prefix: "a", workers: 2 });
  console.log("   address:", wallet1.address);
  console.log("   privateKey:", wallet1.privateKey);

  console.log("\n3b) Contains pattern — Solana with 'a'");
  const walletContains = await generateAddress({
    chain: "sol",
    contains: "a",
    workers: 2,
  });
  console.log("   address:", walletContains.address);

  console.log("\n3c) OR suffixes — EVM ending a or b");
  const walletOr = await generateAddress({
    chain: "evm",
    suffix: "a,b",
    workers: 2,
  });
  console.log("   address:", walletOr.address);

  console.log("\n4) Batch — two Bitcoin addresses starting with '1'");
  const batch = await generateAddresses({ chain: "btc", prefix: "1", count: 2, workers: 2 });
  console.log(
    "  ",
    batch.map((w) => w.address),
  );

  console.log("\n5) Progress + ETA");
  await generateAddress({
    chain: "evm",
    prefix: "aa",
    workers: 2,
    onProgress: (attempts, info) => {
      process.stdout.write(
        `\r   ${attempts} @ ${Math.round(info.keysPerSec)} keys/s` +
          (info.etaSeconds != null ? ` · eta ~${info.etaSeconds.toFixed(1)}s` : ""),
      );
    },
  }).then((w) => console.log(`\n   found: ${w.address}`));

  console.log("\n6) timeoutMs");
  try {
    await generateAddress({
      chain: "evm",
      prefix: "abcdef12",
      timeoutMs: 50,
      workers: 1,
    });
  } catch (err) {
    console.log("   timed out as expected:", err.name);
  }

  console.log("\n7) Error handling — invalid chain");
  try {
    await generateAddress({ chain: "not-a-real-chain", prefix: "a" });
  } catch (err) {
    if (err instanceof VanityAddressError) {
      console.log("   caught VanityAddressError:", err.code, "-", err.message);
    }
  }

  console.log("\nAll good — SDK is working end to end.");
}

main().catch((err) => {
  console.error("Unexpected error:", err);
  process.exit(1);
});
