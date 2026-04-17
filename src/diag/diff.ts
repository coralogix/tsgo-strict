import type { Diagnostic } from '../types/index.js';
import { normalizeDiagnosticKey } from './normalize.js';

export function diffDiagnostics(
  strictDiagnostics: Diagnostic[],
  baselineDiagnostics: Diagnostic[]
): Diagnostic[] {
  const baselineKeys = new Set<string>(baselineDiagnostics.map(normalizeDiagnosticKey));
  return strictDiagnostics.filter(
    (diagnostic) => !baselineKeys.has(normalizeDiagnosticKey(diagnostic))
  );
}
