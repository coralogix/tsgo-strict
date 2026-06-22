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

'use strict';

// Internal platform-binary + N-API addon resolver. Not part of the public
// package surface; the bin launcher and `index.js` import from here
// directly.

const path = require('node:path');
const { familySync, MUSL } = require('detect-libc');

function pickPackage() {
  const platform = process.platform;
  const arch = process.arch;
  const family = platform === 'linux' ? familySync() : null;

  if (platform === 'linux' && arch === 'x64') {
    return family === MUSL
      ? '@cx/tsgo-strict-linux-x64-musl'
      : '@cx/tsgo-strict-linux-x64-gnu';
  }
  if (platform === 'linux' && arch === 'arm64') {
    if (family === MUSL) return null;
    return '@cx/tsgo-strict-linux-arm64-gnu';
  }
  if (platform === 'darwin' && arch === 'arm64') return '@cx/tsgo-strict-darwin-arm64';
  if (platform === 'win32' && arch === 'x64') return '@cx/tsgo-strict-win32-x64-msvc';
  return null;
}

function platformPackageJson() {
  const name = pickPackage();
  if (!name) {
    throw new Error(
      `tsgo-strict: unsupported platform ${process.platform}-${process.arch} (libc family: ${familySync() || 'n/a'})`
    );
  }
  let packagePath;
  try {
    packagePath = require.resolve(`${name}/package.json`);
  } catch {
    throw new Error(
      `tsgo-strict: expected platform package '${name}' is not installed. ` +
        `This usually means 'optionalDependencies' was not installed (check npm/pnpm/yarn install logs).`
    );
  }
  return { name, packagePath, pkg: require(packagePath) };
}

function resolveBinary() {
  // Platform packages don't declare `bin` (that would collide with this
  // launcher's own `bin` entry on hoist). Hard-code the staged path —
  // it matches what the release workflow writes into
  // `npm/platforms/<triple>/bin/`.
  const { packagePath } = platformPackageJson();
  const relative = process.platform === 'win32' ? 'bin/tsgo-strict.exe' : 'bin/tsgo-strict';
  return path.resolve(path.dirname(packagePath), relative);
}

function resolveNativeAddon() {
  const { pkg, packagePath } = platformPackageJson();
  const relative = pkg.main || 'native/tsgo-strict.node';
  return path.resolve(path.dirname(packagePath), relative);
}

module.exports = { pickPackage, resolveBinary, resolveNativeAddon };
