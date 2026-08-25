#!/usr/bin/env node
/*
 * Copyright 2026 Coralogix Ltd.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// Stamp a release version into every publishable package.json and rewrite
// the `workspace:*` optionalDependencies in the main launcher to concrete
// pins. `npm publish` (unlike `pnpm publish`) does not resolve the pnpm
// `workspace:` protocol, so shipping the manifest as-is would produce an
// install-time failure for end users.
//
// Also stamps `[workspace.package] version` in the root Cargo.toml, which is
// where `tsgo-strict --version` gets its string from (clap's `version`
// attribute reads CARGO_PKG_VERSION at compile time). Left unstamped the CLI
// reports the frozen in-repo 0.1.0 no matter what was published. That means
// the release workflow has to run this *before* the cargo builds, not just
// before `npm publish` -- see the `build` job in .github/workflows/release.yml.
//
// Cargo.lock is deliberately not rewritten: cargo refreshes the workspace
// members' entries itself on the next build, and the release builds don't pass
// `--locked`. If that flag is ever added, the build fails loudly rather than
// shipping a wrong version.
//
// Usage: node scripts/stamp-versions.mjs <version>
//   e.g. node scripts/stamp-versions.mjs 0.2.0

import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '..');

const version = (process.argv[2] ?? '').replace(/^v/, '').trim();
if (!/^\d+\.\d+\.\d+(?:-[\w.-]+)?(?:\+[\w.-]+)?$/.test(version)) {
  console.error(`stamp-versions: invalid version ${JSON.stringify(process.argv[2])}`);
  process.exit(1);
}

const mainPkgPath = path.join(repoRoot, 'npm', 'tsgo-strict', 'package.json');
const platformsDir = path.join(repoRoot, 'npm', 'platforms');
const platformPkgs = fs
  .readdirSync(platformsDir, { withFileTypes: true })
  .filter((e) => e.isDirectory())
  .map((e) => path.join(platformsDir, e.name, 'package.json'));

const updated = [];

function writeJson(filePath, data) {
  fs.writeFileSync(filePath, `${JSON.stringify(data, null, 2)}\n`, 'utf8');
  updated.push(path.relative(repoRoot, filePath));
}

// Platform packages: version field only.
for (const pkgPath of platformPkgs) {
  const pkg = JSON.parse(fs.readFileSync(pkgPath, 'utf8'));
  pkg.version = version;
  writeJson(pkgPath, pkg);
}

// Main package: version + rewrite every `workspace:*` optionalDep to
// the concrete version we're publishing. Any other protocol (e.g. a real
// semver range) is left untouched so this stays safe if someone later
// pins a platform dep explicitly.
const mainPkg = JSON.parse(fs.readFileSync(mainPkgPath, 'utf8'));
mainPkg.version = version;
if (mainPkg.optionalDependencies) {
  for (const [name, spec] of Object.entries(mainPkg.optionalDependencies)) {
    if (typeof spec === 'string' && spec.startsWith('workspace:')) {
      mainPkg.optionalDependencies[name] = version;
    }
  }
}
writeJson(mainPkgPath, mainPkg);

// Root Cargo.toml: the `version` under `[workspace.package]` only. Scoped to
// that table so a `version = "..."` in [workspace.dependencies] can never be
// hit by accident.
const cargoPath = path.join(repoRoot, 'Cargo.toml');
const cargoSrc = fs.readFileSync(cargoPath, 'utf8');
const tableStart = cargoSrc.indexOf('\n[workspace.package]\n');
if (tableStart === -1) {
  console.error(`stamp-versions: no [workspace.package] table in ${cargoPath}`);
  process.exit(1);
}
const bodyStart = tableStart + '\n[workspace.package]\n'.length;
const nextTable = cargoSrc.indexOf('\n[', bodyStart);
const tableEnd = nextTable === -1 ? cargoSrc.length : nextTable;
const table = cargoSrc.slice(bodyStart, tableEnd);
const stampedTable = table.replace(/^version = "[^"]*"$/m, `version = "${version}"`);
if (stampedTable === table) {
  console.error(`stamp-versions: no version key under [workspace.package] in ${cargoPath}`);
  process.exit(1);
}
fs.writeFileSync(
  cargoPath,
  cargoSrc.slice(0, bodyStart) + stampedTable + cargoSrc.slice(tableEnd),
  'utf8'
);

console.log(`stamp-versions: set version ${version} in ${updated.length} package.json file(s):`);
for (const p of updated) console.log(`  - ${p}`);
console.log('  - Cargo.toml ([workspace.package] version)');
