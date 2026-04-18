#!/usr/bin/env node
'use strict';

const fs = require('node:fs');
const { spawnSync } = require('node:child_process');
const { resolveBinary } = require('../lib/resolve');

try {
  const binary = resolveBinary();
  // GitHub Actions' upload-artifact strips the executable bit, so the
  // published platform tarball can ship the binary at 0644. Ensure it's
  // runnable before we spawn it. Best-effort: skip silently on chmod
  // failure so we don't mask the real spawn error below.
  try {
    const st = fs.statSync(binary);
    if ((st.mode & 0o111) === 0) fs.chmodSync(binary, 0o755);
  } catch {}
  const result = spawnSync(binary, process.argv.slice(2), { stdio: 'inherit' });
  if (result.error) {
    process.stderr.write(`tsgo-strict: failed to launch native binary: ${result.error.message}\n`);
    process.exit(2);
  }
  process.exit(result.status == null ? 2 : result.status);
} catch (err) {
  process.stderr.write(`${err.message}\n`);
  process.exit(2);
}
