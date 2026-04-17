import fs from 'node:fs';
import path from 'node:path';
import os from 'node:os';
import { afterEach, describe, expect, it } from 'vitest';
import { findStrictCandidates } from '../../src/config/strictFileSelection.js';

const tempDirs: string[] = [];

afterEach(() => {
  for (const dir of tempDirs.splice(0, tempDirs.length)) {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

describe('findStrictCandidates', () => {
  it('applies paths and excludePattern with strict override comments', () => {
    const root = fs.mkdtempSync(path.join(os.tmpdir(), 'tsgo-strict-test-'));
    tempDirs.push(root);

    const a = path.join(root, 'src', 'a.ts');
    const b = path.join(root, 'src', 'legacy', 'b.ts');
    const c = path.join(root, 'ignored', 'c.ts');
    const d = path.join(root, 'src', 'd.ts');

    fs.mkdirSync(path.dirname(a), { recursive: true });
    fs.mkdirSync(path.dirname(b), { recursive: true });
    fs.mkdirSync(path.dirname(c), { recursive: true });

    fs.writeFileSync(a, 'export const a = 1;\n', 'utf8');
    fs.writeFileSync(b, 'export const b = 2;\n', 'utf8');
    fs.writeFileSync(c, '// @ts-strict\nexport const c = 3;\n', 'utf8');
    fs.writeFileSync(d, '// @ts-strict-ignore\nexport const d = 4;\n', 'utf8');

    const selected = findStrictCandidates(
      [a, b, c, d],
      {
        name: 'typescript-strict-plugin',
        paths: ['src/**/*.ts'],
        excludePattern: 'legacy/'
      },
      root
    );

    expect(selected.sort()).toEqual([a, c].sort());
  });

  it('defaults to all files when no plugin config exists', () => {
    const root = fs.mkdtempSync(path.join(os.tmpdir(), 'tsgo-strict-test-'));
    tempDirs.push(root);

    const a = path.join(root, 'src', 'a.ts');
    fs.mkdirSync(path.dirname(a), { recursive: true });
    fs.writeFileSync(a, 'export const a = 1;\n', 'utf8');

    const selected = findStrictCandidates([a], null, root);
    expect(selected).toEqual([a]);
  });
});
