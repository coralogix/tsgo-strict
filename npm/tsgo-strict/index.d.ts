// Type definitions for `@cx/tsgo-strict`.
//
// The runtime surface is implemented in `index.js` and backed by a native
// N-API addon shipped in `@cx/tsgo-strict-<triple>` platform subpackages.

export type Category = 'error' | 'warning' | 'message';

export interface RunOptions {
  /** Path to the project tsconfig, absolute or relative to `cwd`. Defaults to `tsconfig.json`. */
  project?: string;
  /** Working directory for binary and tsconfig resolution. Defaults to `process.cwd()`. */
  cwd?: string;
  /** Restrict the check to these files or directories. Empty / omitted means the full project. */
  subset?: string[];
}

export interface RunDiagnostic {
  /** Project-relative path, or `undefined` for global diagnostics. */
  file?: string;
  /** 1-based line number. */
  line?: number;
  /** 1-based column number. */
  column?: number;
  /** TypeScript diagnostic code (e.g. `2345`). */
  code: number;
  category: Category;
  message: string;
}

export interface RunTiming {
  label: string;
  durationMs: number;
}

export interface RunResult {
  /** Total diagnostic count. */
  errorCount: number;
  /** `0` clean, `1` strict errors, `2` internal failure. */
  exitCode: number;
  diagnostics: RunDiagnostic[];
  timings: RunTiming[];
}

/** Run the strict checker programmatically. Resolves with structured diagnostics and per-phase timings. */
export function run(options?: RunOptions): Promise<RunResult>;
