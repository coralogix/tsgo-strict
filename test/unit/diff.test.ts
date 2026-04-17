import { describe, expect, it } from 'vitest';
import { diffDiagnostics } from '../../src/diag/diff.js';
import { normalizeDiagnosticKey } from '../../src/diag/normalize.js';

describe('diagnostic normalization and diffing', () => {
  it('builds stable normalization keys', () => {
    const key = normalizeDiagnosticKey({
      file: '/Repo/src/a.ts',
      line: 10,
      column: 2,
      code: 2322,
      category: 'error',
      message: 'Type   X\n  is not assignable'
    });

    expect(key).toBe('/repo/src/a.ts|10|2|2322|error|Type X is not assignable');
  });

  it('subtracts baseline diagnostics from strict diagnostics', () => {
    const baseline = [
      {
        file: '/repo/src/a.ts',
        line: 1,
        column: 1,
        code: 7006,
        category: 'error' as const,
        message: 'Parameter implicitly has an any type.'
      }
    ];

    const strict = [
      baseline[0],
      {
        file: '/repo/src/b.ts',
        line: 2,
        column: 3,
        code: 2532,
        category: 'error' as const,
        message: 'Object is possibly undefined.'
      }
    ];

    const diff = diffDiagnostics(strict, baseline);
    expect(diff).toHaveLength(1);
    expect(diff[0]?.file).toBe('/repo/src/b.ts');
  });
});
