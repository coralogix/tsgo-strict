import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { afterEach, describe, expect, it } from 'vitest';
import { resolveSubsetInputs } from '../../src/files/resolveSubset.js';

const tempDirs: string[] = [];

afterEach(() => {
  for (const dir of tempDirs.splice(0, tempDirs.length)) {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

describe('resolveSubsetInputs', () => {
  it('resolves explicit files and globs', () => {
    const root = fs.mkdtempSync(path.join(os.tmpdir(), 'tsgo-strict-test-'));
    tempDirs.push(root);

    fs.mkdirSync(path.join(root, 'src'), { recursive: true });
    fs.writeFileSync(path.join(root, 'src', 'a.ts'), 'export {};\n', 'utf8');
    fs.writeFileSync(path.join(root, 'src', 'b.tsx'), 'export {};\n', 'utf8');
    fs.writeFileSync(path.join(root, 'src', 'c.js'), 'export {};\n', 'utf8');

    const files = resolveSubsetInputs(['src/a.ts', 'src/*.tsx'], root);
    const normalized = files
      .map((file) => path.relative(root, file).split(path.sep).join('/'))
      .sort();

    expect(normalized).toEqual(['src/a.ts', 'src/b.tsx']);
  });
});
