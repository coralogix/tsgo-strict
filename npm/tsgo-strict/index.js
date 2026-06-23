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
