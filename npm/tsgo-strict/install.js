#!/usr/bin/env node
'use strict';

// Postinstall optimizer (esbuild-style). The JS shim at `bin/tsgo-strict`
// always works, but spawning a separate Node process per invocation adds
// ~30-50ms of startup. On *nix, we hard-link the platform-specific native
// binary directly over the shim so `node_modules/.bin/tsgo-strict` (which
// symlinks to this file) resolves to the native ELF/Mach-O directly.
//
// This is a pure optimization — if it fails for any reason, the JS shim
// is preserved and everything still works. On Windows, yarn, or when
// installed with `--ignore-scripts`, the shim stays in place.

const fs = require('node:fs');
const path = require('node:path');

function isYarn() {
  const ua = process.env.npm_config_user_agent;
  return !!ua && /\byarn\//.test(ua);
}

function main() {
  // Windows: keep the JS shim. npm on Windows generates `.cmd`/`.ps1`
  // wrappers next to the shim — replacing it with a PE executable would
  // break those wrappers.
  if (process.platform === 'win32') return;
  if (isYarn()) return;

  let resolveBinary;
  try {
    ({ resolveBinary } = require('./lib/resolve'));
  } catch {
    return;
  }

  let binary;
  try {
    binary = resolveBinary();
  } catch {
    // Platform not supported, or the optional dep didn't install. The
    // shim will surface a helpful error the first time it runs.
    return;
  }

  if (!fs.existsSync(binary)) return;

  try {
    const st = fs.statSync(binary);
    if ((st.mode & 0o111) === 0) fs.chmodSync(binary, 0o755);
  } catch {}

  const shim = path.join(__dirname, 'bin', 'tsgo-strict');
  const tmp = path.join(__dirname, 'bin', 'tsgo-strict.tmp');
  try {
    try { fs.unlinkSync(tmp); } catch {}
    fs.linkSync(binary, tmp);
    fs.renameSync(tmp, shim);
  } catch {
    // Cross-device link, read-only fs, whatever — shim stays put.
    try { fs.unlinkSync(tmp); } catch {}
  }
}

main();
