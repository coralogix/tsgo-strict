import { describe, expect, it } from 'vitest';
import { formatTextOutput } from '../../src/output/formatText.js';

describe('formatTextOutput', () => {
  it('respects max diagnostics', () => {
    const result = formatTextOutput(
      [
        { file: '/repo/src/a.ts', line: 1, column: 1, code: 1111, category: 'error', message: 'A' },
        { file: '/repo/src/b.ts', line: 2, column: 2, code: 2222, category: 'error', message: 'B' }
      ],
      '/repo',
      1
    );

    expect(result.text).toContain('additional diagnostics omitted');
    expect(result.totalCount).toBe(2);
  });
});
