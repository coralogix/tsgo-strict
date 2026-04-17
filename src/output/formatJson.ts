import type { Diagnostic, JsonOutput, Mode } from '../types/index.js';

export function formatJsonOutput(
  diagnostics: Diagnostic[],
  mode: Mode,
  maxDiagnostics?: number
): { payload: JsonOutput; text: string } {
  const { sliced, truncated } = limitDiagnostics(diagnostics, maxDiagnostics);
  const payload: JsonOutput = {
    mode,
    errorCount: diagnostics.length,
    diagnostics: sliced,
    truncated
  };

  return {
    payload,
    text: JSON.stringify(payload, null, 2)
  };
}

function limitDiagnostics(
  diagnostics: Diagnostic[],
  maxDiagnostics?: number
): { sliced: Diagnostic[]; truncated: boolean } {
  if (!maxDiagnostics || maxDiagnostics <= 0 || diagnostics.length <= maxDiagnostics) {
    return { sliced: diagnostics, truncated: false };
  }

  return {
    sliced: diagnostics.slice(0, maxDiagnostics),
    truncated: true
  };
}
