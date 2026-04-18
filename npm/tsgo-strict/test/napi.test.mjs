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
          plugins: [{ name: 'typescript-strict-plugin', paths: ['src/**'] }],
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

test('nested cwd walks up to the repo node_modules/.bin/tsgo', { skip: !addonReady }, async () => {
  // Prove the walk-up branch of `resolve_tsgo_binary` works when run from a
  // deep subdirectory — the fixture lives at `npm/tsgo-strict/test/fixtures/
  // basic/`, four levels below the only `node_modules/.bin/tsgo` in the tree.
  const result = await run({ project: path.join(FIXTURE, 'tsconfig.json'), cwd: FIXTURE });
  assert.ok(result.errorCount > 0);
});
