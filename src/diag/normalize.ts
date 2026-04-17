import type { Diagnostic } from '../types/index.js';

export function normalizeDiagnosticKey(diagnostic: Diagnostic): string {
  const file = normalizePath(diagnostic.file ?? '');
  const line = diagnostic.line ?? 0;
  const column = diagnostic.column ?? 0;
  const code = diagnostic.code;
  const category = diagnostic.category;
  const message = flattenMessageText(diagnostic.message);
  return `${file}|${line}|${column}|${code}|${category}|${message}`;
}

export function flattenMessageText(message: string): string {
  return message.replace(/\s+/g, ' ').trim();
}

function normalizePath(file: string): string {
  return file.replace(/\\/g, '/').toLowerCase();
}
