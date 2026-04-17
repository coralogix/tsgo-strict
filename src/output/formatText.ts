import path from 'node:path';
import type { Diagnostic } from '../types/index.js';

export function formatTextOutput(
  diagnostics: Diagnostic[],
  cwd: string,
  maxDiagnostics?: number
): { text: string; displayedCount: number; totalCount: number; truncated: boolean } {
  const sorted = sortDiagnostics(diagnostics);
  const totalCount = sorted.length;
  const truncated = Boolean(maxDiagnostics && maxDiagnostics > 0 && totalCount > maxDiagnostics);
  const displayed = truncated && maxDiagnostics ? sorted.slice(0, maxDiagnostics) : sorted;

  const lines = displayed.map((diag) => formatDiagnosticLine(diag, cwd));

  if (truncated && maxDiagnostics) {
    lines.push(
      `... ${totalCount - maxDiagnostics} additional diagnostics omitted by --max-diagnostics=${maxDiagnostics}`
    );
  }

  lines.push(`Found ${totalCount} strict error${totalCount === 1 ? '' : 's'}.`);

  return {
    text: lines.join('\n'),
    displayedCount: displayed.length,
    totalCount,
    truncated
  };
}

function formatDiagnosticLine(diag: Diagnostic, cwd: string): string {
  const code = `TS${diag.code}`;
  const category = diag.category;
  if (!diag.file || diag.line === undefined || diag.column === undefined) {
    return `${category} ${code}: ${diag.message}`;
  }

  const file = makeRelative(diag.file, cwd);
  return `${file}(${diag.line},${diag.column}): ${category} ${code}: ${diag.message}`;
}

function sortDiagnostics(diagnostics: Diagnostic[]): Diagnostic[] {
  return [...diagnostics].sort((a, b) => {
    const af = a.file ?? '';
    const bf = b.file ?? '';
    if (af !== bf) {
      return af.localeCompare(bf);
    }
    const al = a.line ?? 0;
    const bl = b.line ?? 0;
    if (al !== bl) {
      return al - bl;
    }
    const ac = a.column ?? 0;
    const bc = b.column ?? 0;
    if (ac !== bc) {
      return ac - bc;
    }
    if (a.code !== b.code) {
      return a.code - b.code;
    }
    return a.message.localeCompare(b.message);
  });
}

function makeRelative(file: string, cwd: string): string {
  const relative = path.relative(cwd, file);
  if (!relative || relative.startsWith('..')) {
    return file;
  }
  return relative.split(path.sep).join('/');
}
