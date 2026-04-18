'use strict';

const path = require('node:path');
const { familySync, MUSL } = require('detect-libc');

function pickPackage() {
  const platform = process.platform;
  const arch = process.arch;
  const family = platform === 'linux' ? familySync() : null;

  if (platform === 'linux' && arch === 'x64') {
    return family === MUSL ? '@tsgo-strict/linux-x64-musl' : '@tsgo-strict/linux-x64-gnu';
  }
  if (platform === 'linux' && arch === 'arm64') {
    if (family === MUSL) return null;
    return '@tsgo-strict/linux-arm64-gnu';
  }
  if (platform === 'darwin' && arch === 'x64') return '@tsgo-strict/darwin-x64';
  if (platform === 'darwin' && arch === 'arm64') return '@tsgo-strict/darwin-arm64';
  if (platform === 'win32' && arch === 'x64') return '@tsgo-strict/win32-x64-msvc';
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
  const { pkg, packagePath } = platformPackageJson();
  const binField = pkg.bin;
  const relative = typeof binField === 'string' ? binField : binField && binField['tsgo-strict'];
  if (!relative) {
    throw new Error(`tsgo-strict: platform package has no bin entry`);
  }
  return path.resolve(path.dirname(packagePath), relative);
}

function resolveNativeAddon() {
  const { pkg, packagePath } = platformPackageJson();
  const relative = pkg.main || 'native/tsgo-strict.node';
  return path.resolve(path.dirname(packagePath), relative);
}

let cachedAddon = null;
function loadAddon() {
  if (cachedAddon) return cachedAddon;
  const addonPath = resolveNativeAddon();
  cachedAddon = require(addonPath);
  return cachedAddon;
}

async function run(options = {}) {
  const addon = loadAddon();
  return addon.run(options);
}

module.exports = { resolveBinary, resolveNativeAddon, pickPackage, run };
