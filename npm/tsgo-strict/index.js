'use strict';

const { resolveNativeAddon } = require('./lib/resolve');

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

module.exports = { run };
