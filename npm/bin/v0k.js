#!/usr/bin/env node

const { execFileSync, spawn } = require("child_process");
const path = require("path");
const os = require("os");
const fs = require("fs");

const BINARY_NAME = "v0k";

/**
 * Resolve the path to the native Rust binary.
 * Search order:
 *   1. Right next to this script (downloaded by postinstall)
 *   2. Sibling to npm package (build output: ../target/release/v0k or ../target/debug/v0k)
 *   3. In system PATH
 */
function findBinary() {
  const ext = os.platform() === "win32" ? ".exe" : "";

  // 1. Check relative to this script (downloaded via npm install)
  const downloadedBinary = path.join(__dirname, BINARY_NAME + ext);
  if (fs.existsSync(downloadedBinary) && downloadedBinary !== __filename) {
    return downloadedBinary;
  }

  // 2. Check relative to this package (development / local build)
  const projectRoot = path.resolve(__dirname, "..", "..");
  for (const profile of ["release", "debug"]) {
    const candidate = path.join(projectRoot, "target", profile, BINARY_NAME + ext);
    if (fs.existsSync(candidate)) {
      return candidate;
    }
  }

  // 2. Check system PATH
  try {
    const which = os.platform() === "win32" ? "where" : "which";
    const result = execFileSync(which, [BINARY_NAME], { encoding: "utf-8" }).trim();
    if (result) return result.split("\n")[0];
  } catch {
    // not in PATH
  }

  console.error(
    `Error: Could not find the '${BINARY_NAME}' binary.\n` +
    `Please build the Rust core first:\n` +
    `  cd ${projectRoot} && cargo build --release\n`
  );
  process.exit(1);
}

// Forward all arguments to the native binary, inheriting stdio.
const binary = findBinary();
const args = process.argv.slice(2);

const child = spawn(binary, args, {
  stdio: "inherit",
});

child.on("error", (err) => {
  console.error(`Failed to start ${BINARY_NAME}: ${err.message}`);
  process.exit(1);
});

child.on("exit", (code) => {
  process.exit(code ?? 1);
});