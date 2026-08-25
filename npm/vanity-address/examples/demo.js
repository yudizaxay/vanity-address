// Demo: vanity-address SDK usage
//
// Run from this directory:  node examples/demo.js
// (or after `npm install vanity-address` in your own project, swap the
//  require() path below for `require("vanity-address")`)

const { generateAddress, VanityAddressError } = require("../src/index.js");

async function main() {
  console.log("1) Basic usage — EVM address starting with 'a'");
  const wallet1 = await generateAddress({ chain: "evm", prefix: "a" });
  console.log("   address:", wallet1.address);
  console.log("   private key:", wallet1.exports[0].value);
  console.log();

  console.log("2) Bitcoin address starting with '1'");
  const wallet2 = await generateAddress({ chain: "btc", prefix: "1" });
  console.log("   address:", wallet2.address);
  console.log("   exports:", wallet2.exports.map((e) => e.label));
  console.log();

  console.log("3) With progress reporting (longer pattern, 4 hex chars)");
  await generateAddress({
    chain: "evm",
    prefix: "cafe",
    onProgress: (attempts) => process.stdout.write(`\r   tried ${attempts} so far...`),
  }).then((w) => console.log(`\n   found: ${w.address}`));
  console.log();

  console.log("4) Cancellation via AbortSignal (aborts after first progress tick)");
  const controller = new AbortController();
  try {
    await generateAddress({
      chain: "evm",
      prefix: "aaaaaaaa", // deliberately long/slow so we can cancel it
      signal: controller.signal,
      onProgress: () => controller.abort(),
    });
  } catch (err) {
    console.log("   cancelled as expected:", err.name);
  }
  console.log();

  console.log("5) Error handling — invalid chain");
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
