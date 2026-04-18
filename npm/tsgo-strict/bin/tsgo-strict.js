#!/usr/bin/env node
'use strict';

const { spawnSync } = require('node:child_process');
const { resolveBinary } = require('../lib/resolve');

try {
  const binary = resolveBinary();
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
