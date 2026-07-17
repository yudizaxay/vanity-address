#!/usr/bin/env node
"use strict";

const { spawnSync } = require("node:child_process");
const fs = require("node:fs");
const path = require("node:path");

const PACKAGE_BY_PLATFORM = {
  "darwin-arm64": "vanity-address-darwin-arm64",
  "darwin-x64": "vanity-address-darwin-x64",
  "linux-x64": "vanity-address-linux-x64",
  "win32-x64": "vanity-address-win32-x64",
};

function binaryName() {
  return process.platform === "win32" ? "vanity-address.exe" : "vanity-address";
}

function resolvePkgJson(pkg) {
  // Prefer paths next to this package (node_modules/ or npm/ siblings for local file: installs).
  const searchRoots = [
    path.join(__dirname, "..", ".."),
    process.cwd(),
    path.join(process.cwd(), "node_modules"),
  ];
  try {
    return require.resolve(`${pkg}/package.json`, { paths: searchRoots });
  } catch {
    return require.resolve(`${pkg}/package.json`);
  }
}

function resolveBinary() {
  const key = `${process.platform}-${process.arch}`;
  const pkg = PACKAGE_BY_PLATFORM[key];
  if (!pkg) {
    console.error(
      `vanity-address: unsupported platform ${key}.\n` +
        `Supported: ${Object.keys(PACKAGE_BY_PLATFORM).join(", ")}\n` +
        `Download a pre-built CLI from https://github.com/yudizaxay/vanity-address/releases/latest`,
    );
    process.exit(1);
  }

  let pkgJson;
  try {
    pkgJson = resolvePkgJson(pkg);
  } catch {
    console.error(
      `vanity-address: optional package "${pkg}" is not installed.\n` +
        `Try: npm install -g vanity-address\n` +
        `Or download from https://github.com/yudizaxay/vanity-address/releases/latest`,
    );
    process.exit(1);
  }

  const binPath = path.join(path.dirname(pkgJson), "bin", binaryName());
  if (!fs.existsSync(binPath)) {
    console.error(`vanity-address: binary missing at ${binPath}`);
    process.exit(1);
  }
  return binPath;
}

const result = spawnSync(resolveBinary(), process.argv.slice(2), {
  stdio: "inherit",
});

if (result.error) {
  console.error(result.error.message);
  process.exit(1);
}

process.exit(result.status === null ? 1 : result.status);
