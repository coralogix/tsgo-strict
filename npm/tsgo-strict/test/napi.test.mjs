// Integration tests for the N-API `run()` surface exposed by
// `tsgo-strict`. Require the platform subpackage's native addon to be staged
// at `npm/platforms/<triple>/native/tsgo-strict.node` (the release workflow
// does this automatically; locally, run `scripts/stage-local-addon.sh`).

import test from 'node:test';
import assert from 'node:assert/strict';
import path from 'node:path';
import os from 'node:os';
import { fileURLToPath } from 'node:url';
import { existsSync, mkdirSync, mkdtempSync, rmSync, writeFileSync } from 'node:fs';

import { run } from '../index.js';
import { pickPackage, resolveNativeAddon } from '../lib/resolve.js';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const FIXTURE = path.join(__dirname, 'fixtures', 'basic');
const PRAGMA_FIXTURE = path.join(__dirname, 'fixtures', 'pragmas');
const EXCLUDE_PATTERN_FIXTURE = path.join(__dirname, 'fixtures', 'exclude-pattern');
const EXTENDS_PLUGIN_FIXTURE = path.join(__dirname, 'fixtures', 'extends-plugin');
const EXTENDS_EXCLUDE_FIXTURE = path.join(__dirname, 'fixtures', 'extends-exclude');
const TSCONFIG_EXCLUDE_FIXTURE = path.join(__dirname, 'fixtures', 'tsconfig-exclude');
const BASE_URL_FIXTURE = path.join(__dirname, 'fixtures', 'base-url');
const BASE_URL_INHERITED_FIXTURE = path.join(__dirname, 'fixtures', 'base-url-inherited');
const SOLUTION_STYLE_FIXTURE = path.join(__dirname, 'fixtures', 'solution-style');
const BASE_URL_PATHS_FIXTURE = path.join(__dirname, 'fixtures', 'base-url-paths');
const FILES_ARRAY_FIXTURE = path.join(__dirname, 'fixtures', 'files-array');
const ORPHAN_FILE_FIXTURE = path.join(__dirname, 'fixtures', 'orphan-file');
const TYPES_SUBPATH_FIXTURE = path.join(__dirname, 'fixtures', 'types-subpath');
const REPO_ROOT = path.resolve(__dirname, '..', '..', '..');

// Skip the full suite when the platform addon hasn't been staged. This keeps
// the test runnable on CI jobs where only the CLI binary is built.
const addonReady = (() => {
  try {
    const p = resolveNativeAddon();
    return existsSync(p);
  } catch {
    return false;
  }
})();

test('pickPackage returns a supported triple for the current host', () => {
  const id = pickPackage();
  assert.ok(id, 'current platform should be supported');
  assert.match(id, /^@tsgo-strict\//);
});

test('full project run reports strict errors only from in-scope paths', { skip: !addonReady }, async () => {
  const result = await run({ project: path.join(FIXTURE, 'tsconfig.json'), cwd: FIXTURE });

  assert.equal(typeof result.errorCount, 'number');
  assert.ok(result.errorCount > 0, 'expected strict errors from src/in-scope');
  assert.equal(result.exitCode, 1);

  for (const d of result.diagnostics) {
    assert.equal(typeof d.code, 'number');
    assert.equal(d.category, 'error');
    assert.equal(typeof d.message, 'string');
    assert.ok(d.file, 'strict diagnostics should have a file');
    assert.ok(d.file.includes('in-scope'), `unexpected file ${d.file}`);
    assert.ok(!d.file.includes('out-of-scope'), `out-of-scope file leaked: ${d.file}`);
  }
});

test('subset limited to out-of-scope returns no strict errors', { skip: !addonReady }, async () => {
  const result = await run({
    project: path.join(FIXTURE, 'tsconfig.json'),
    cwd: FIXTURE,
    subset: ['src/out-of-scope'],
  });

  assert.equal(result.errorCount, 0);
  assert.equal(result.exitCode, 0);
  assert.deepEqual(result.diagnostics, []);
});

test('run populates per-phase timings', { skip: !addonReady }, async () => {
  const result = await run({
    project: path.join(FIXTURE, 'tsconfig.json'),
    cwd: FIXTURE,
  });

  assert.ok(result.timings.length > 0, 'expected per-phase timings');
  for (const t of result.timings) {
    assert.equal(typeof t.label, 'string');
    assert.equal(typeof t.durationMs, 'number');
    assert.ok(t.durationMs >= 0);
  }
});

test('pragmas override plugin path membership', { skip: !addonReady }, async () => {
  const result = await run({ project: path.join(PRAGMA_FIXTURE, 'tsconfig.json'), cwd: PRAGMA_FIXTURE });

  const byFile = new Map();
  for (const d of result.diagnostics) {
    byFile.set(d.file, (byFile.get(d.file) ?? 0) + 1);
  }

  // strict.ts: under included/ and no pragma → strict-checked → reports errors
  assert.ok(
    [...byFile.keys()].some((f) => f.endsWith(path.join('included', 'strict.ts'))),
    `expected strict.ts to report diagnostics, got: ${[...byFile.keys()].join(', ')}`,
  );
  // opt-out.ts: under included/ but @ts-strict-ignore → skipped
  for (const f of byFile.keys()) {
    assert.ok(!f.endsWith(path.join('included', 'opt-out.ts')), `opt-out.ts should be skipped: ${f}`);
  }
  // opt-in.ts: outside included/ but @ts-strict → included
  assert.ok(
    [...byFile.keys()].some((f) => f.endsWith(path.join('excluded', 'opt-in.ts'))),
    `expected opt-in.ts to report diagnostics, got: ${[...byFile.keys()].join(', ')}`,
  );
  // loose.ts: outside included/ with no pragma → skipped
  for (const f of byFile.keys()) {
    assert.ok(!f.endsWith(path.join('excluded', 'loose.ts')), `loose.ts should be skipped: ${f}`);
  }
});

test('consumer-style scratch project resolves tsgo from a peer dep', { skip: !addonReady }, async () => {
  // Simulate a consumer project that has its own standalone tsconfig + src
  // tree and depends on `tsgo-strict` + `@typescript/native-preview` as peer
  // dep. The N-API resolver must walk up from `cwd` and find a usable tsgo.
  //
  // We cannot symlink the repo's pnpm-wrapped tsgo into the scratch dir (the
  // wrapper hard-codes pnpm-store paths), so instead we point `TSGO_BINARY`
  // at the repo's tsgo. That also exercises the env override branch of the
  // binary resolver.
  const scratch = mkdtempSync(path.join(os.tmpdir(), 'tsgo-strict-peer-'));
  const prevBinary = process.env.TSGO_BINARY;
  try {
    process.env.TSGO_BINARY = path.join(REPO_ROOT, 'node_modules', '.bin', 'tsgo');

    const srcDir = path.join(scratch, 'src');
    mkdirSync(srcDir, { recursive: true });
    writeFileSync(path.join(srcDir, 'bad.ts'), 'export function f(x) { return x + 1; }\n');
    writeFileSync(
      path.join(scratch, 'tsconfig.json'),
      JSON.stringify({
        compilerOptions: {
          target: 'ES2022',
          module: 'node16',
          moduleResolution: 'node16',
          strict: false,
          noEmit: true,
          skipLibCheck: true,
          plugins: [{ name: 'typescript-strict-plugin', paths: ['src'] }],
        },
        include: ['src/**/*'],
      }),
    );

    const result = await run({ project: 'tsconfig.json', cwd: scratch });
    assert.ok(result.errorCount > 0, 'consumer tsgo should drive a real strict run');
    assert.equal(result.exitCode, 1);
    assert.ok(
      result.diagnostics.some((d) => d.file && d.file.endsWith(path.join('src', 'bad.ts'))),
      `expected a diagnostic from src/bad.ts, got: ${result.diagnostics.map((d) => d.file).join(', ')}`,
    );
  } finally {
    if (prevBinary === undefined) delete process.env.TSGO_BINARY;
    else process.env.TSGO_BINARY = prevBinary;
    rmSync(scratch, { recursive: true, force: true });
  }
});

test('regex excludePattern excludes .spec/.test/.stories files', { skip: !addonReady }, async () => {
  const result = await run({
    project: path.join(EXCLUDE_PATTERN_FIXTURE, 'tsconfig.json'),
    cwd: EXCLUDE_PATTERN_FIXTURE,
  });

  // Only app.ts should report errors; .spec.ts, .test.ts, .stories.ts are excluded
  assert.ok(result.errorCount > 0, 'expected strict errors from app.ts');
  for (const d of result.diagnostics) {
    assert.ok(d.file, 'diagnostic should have a file');
    assert.ok(d.file.endsWith('app.ts'), `unexpected file in diagnostics: ${d.file}`);
    assert.ok(!d.file.includes('.spec.'), `spec file should be excluded: ${d.file}`);
    assert.ok(!d.file.includes('.test.'), `test file should be excluded: ${d.file}`);
    assert.ok(!d.file.includes('.stories.'), `stories file should be excluded: ${d.file}`);
  }
});

test('plugin config inherited via extends chain', { skip: !addonReady }, async () => {
  const result = await run({
    project: path.join(EXTENDS_PLUGIN_FIXTURE, 'tsconfig.json'),
    cwd: EXTENDS_PLUGIN_FIXTURE,
  });

  assert.ok(result.errorCount > 0, 'expected strict errors from src/bad.ts via inherited plugin');
  assert.equal(result.exitCode, 1);
  assert.ok(
    result.diagnostics.some((d) => d.file && d.file.endsWith(path.join('src', 'bad.ts'))),
    `expected diagnostic from src/bad.ts, got: ${result.diagnostics.map((d) => d.file).join(', ')}`,
  );
});

test('exclude inherited via extends chain filters files', { skip: !addonReady }, async () => {
  const result = await run({
    project: path.join(EXTENDS_EXCLUDE_FIXTURE, 'tsconfig.json'),
    cwd: EXTENDS_EXCLUDE_FIXTURE,
  });

  assert.ok(result.errorCount > 0, 'expected strict errors from src/app.ts');
  for (const d of result.diagnostics) {
    assert.ok(d.file, 'diagnostic should have a file');
    assert.ok(
      !d.file.includes('generated'),
      `generated/ should be excluded via inherited exclude: ${d.file}`,
    );
  }
});

test('tsconfig exclude skips file-specific and glob exclude entries', { skip: !addonReady }, async () => {
  const result = await run({
    project: path.join(TSCONFIG_EXCLUDE_FIXTURE, 'tsconfig.json'),
    cwd: TSCONFIG_EXCLUDE_FIXTURE,
  });

  // app.ts and good.ts are strict-clean, test-setup.ts and app.spec.ts are
  // excluded via tsconfig exclude — so we expect zero errors.
  assert.equal(result.exitCode, 0, `expected exit 0, got diagnostics: ${JSON.stringify(result.diagnostics)}`);
  assert.equal(result.errorCount, 0);
  for (const d of result.diagnostics) {
    assert.ok(!d.file?.includes('test-setup'), `test-setup.ts should be excluded: ${d.file}`);
    assert.ok(!d.file?.includes('.spec.'), `spec file should be excluded: ${d.file}`);
  }
});

test('baseUrl config does not cause silent TS5102 failure', { skip: !addonReady }, async () => {
  // baseUrl: "." triggers TS5102 in tsgo — normalization should strip it
  const result = await run({
    project: path.join(BASE_URL_FIXTURE, 'tsconfig.json'),
    cwd: BASE_URL_FIXTURE,
  });
  assert.ok(result.errorCount > 0, 'expected strict errors, not silent pass');
  assert.equal(result.exitCode, 1);
  assert.ok(
    result.diagnostics.some((d) => d.file?.endsWith('bad.ts')),
    `expected diagnostic from bad.ts, got: ${result.diagnostics.map((d) => d.file).join(', ')}`,
  );
});

test('baseUrl inherited via extends is normalized', { skip: !addonReady }, async () => {
  const result = await run({
    project: path.join(BASE_URL_INHERITED_FIXTURE, 'tsconfig.json'),
    cwd: BASE_URL_INHERITED_FIXTURE,
  });
  assert.ok(result.errorCount > 0, 'expected strict errors despite inherited baseUrl');
  assert.equal(result.exitCode, 1);
  assert.ok(
    result.diagnostics.some((d) => d.file?.endsWith('bad.ts')),
    `expected diagnostic from bad.ts, got: ${result.diagnostics.map((d) => d.file).join(', ')}`,
  );
});

test('nested cwd walks up to the repo node_modules/.bin/tsgo', { skip: !addonReady }, async () => {
  // Prove the walk-up branch of `resolve_tsgo_binary` works when run from a
  // deep subdirectory — the fixture lives at `npm/tsgo-strict/test/fixtures/
  // basic/`, four levels below the only `node_modules/.bin/tsgo` in the tree.
  const result = await run({ project: path.join(FIXTURE, 'tsconfig.json'), cwd: FIXTURE });
  assert.ok(result.errorCount > 0);
});

test('solution-style files:[] parent does not block child include', { skip: !addonReady }, async () => {
  // In Nx/Angular monorepos a "solution config" has files: [] + references.
  // Libs extend it and add their own include. The parent's empty files must
  // not short-circuit file enumeration in the child.
  const libDir = path.join(SOLUTION_STYLE_FIXTURE, 'libs', 'foo');
  const result = await run({
    project: path.join(libDir, 'tsconfig.lib.json'),
    cwd: libDir,
  });

  assert.ok(result.errorCount > 0, `expected strict errors from bad.ts, got errorCount=${result.errorCount}`);
  assert.equal(result.exitCode, 1);
  assert.ok(
    result.diagnostics.some((d) => d.file && d.file.endsWith(path.join('src', 'bad.ts'))),
    `expected diagnostic from src/bad.ts, got: ${result.diagnostics.map((d) => d.file).join(', ')}`,
  );
});

test('baseUrl + paths with aliased imports resolves correctly from temp dir', { skip: !addonReady }, async () => {
  // When baseUrl + paths are present, the transient tsconfig is written to a
  // temp dir. Path entries must be absolute so tsgo resolves them correctly
  // regardless of where the temp config lives.
  const consumerDir = path.join(BASE_URL_PATHS_FIXTURE, 'consumer');
  const result = await run({
    project: path.join(consumerDir, 'tsconfig.lib.json'),
    cwd: consumerDir,
  });

  // Should NOT have TS2307 (module not found) errors — paths must resolve
  const ts2307 = result.diagnostics.filter((d) => d.code === 2307);
  assert.equal(
    ts2307.length,
    0,
    `unexpected TS2307 errors (path alias not resolved): ${ts2307.map((d) => d.message).join('; ')}`,
  );

  // Should report strict violations from use-greeter.ts (type mismatch)
  assert.ok(result.errorCount > 0, `expected strict errors from use-greeter.ts, got errorCount=${result.errorCount}`);
  assert.equal(result.exitCode, 1);
  assert.ok(
    result.diagnostics.some((d) => d.file && d.file.endsWith(path.join('src', 'use-greeter.ts'))),
    `expected diagnostic from use-greeter.ts, got: ${result.diagnostics.map((d) => d.file).join(', ')}`,
  );
});

test('types subpath (e.g. "mock-lib/globals") resolves without TS2688', { skip: !addonReady }, async () => {
  const result = await run({
    project: path.join(TYPES_SUBPATH_FIXTURE, 'tsconfig.json'),
    cwd: TYPES_SUBPATH_FIXTURE,
  });

  // TS2688 = "Cannot find type definition file for '...'"
  const ts2688 = result.diagnostics.filter((d) => d.code === 2688);
  assert.equal(
    ts2688.length,
    0,
    `unexpected TS2688 errors (types subpath not resolved): ${ts2688.map((d) => d.message).join('; ')}`,
  );

  // Should report strict errors from src/index.ts (implicit any)
  assert.ok(result.errorCount > 0, `expected strict errors from src/index.ts, got errorCount=${result.errorCount}`);
  assert.equal(result.exitCode, 1);
  assert.ok(
    result.diagnostics.some((d) => d.file && d.file.endsWith(path.join('src', 'index.ts'))),
    `expected diagnostic from src/index.ts, got: ${result.diagnostics.map((d) => d.file).join(', ')}`,
  );
});

test('files-array tsconfig with plugin paths reports errors from transitive imports', { skip: !addonReady }, async () => {
  const result = await run({
    project: path.join(FILES_ARRAY_FIXTURE, 'tsconfig.json'),
    cwd: FILES_ARRAY_FIXTURE,
  });

  // broken.ts has an implicit-any parameter — should be detected even though
  // it is not in the tsconfig "files" array, because plugin paths walk ./src.
  assert.ok(result.errorCount > 0, `expected strict errors from broken.ts, got errorCount=${result.errorCount}`);
  assert.equal(result.exitCode, 1);
  assert.ok(
    result.diagnostics.some((d) => d.file && d.file.includes(path.join('lib', 'broken.ts'))),
    `expected diagnostic from broken.ts, got files: ${result.diagnostics.map((d) => d.file).join(', ')}`,
  );
});

test('orphan files not reachable from entry points are excluded', { skip: !addonReady }, async () => {
  const result = await run({
    project: path.join(ORPHAN_FILE_FIXTURE, 'tsconfig.json'),
    cwd: ORPHAN_FILE_FIXTURE,
  });

  // orphan.ts has an implicit-any but is NOT imported by main.ts, so it
  // should be excluded from strict checking via the reachable-set filter.
  assert.equal(result.exitCode, 0, `expected exit 0 (orphan excluded), got diagnostics: ${JSON.stringify(result.diagnostics)}`);
  assert.equal(result.errorCount, 0);
  for (const d of result.diagnostics) {
    assert.ok(!d.file?.includes('orphan'), `orphan file should be excluded: ${d.file}`);
  }
});
