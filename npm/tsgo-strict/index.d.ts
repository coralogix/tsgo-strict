// Type definitions for `tsgo-strict`.
//
// The runtime surface is implemented in `index.js` and backed by a native
// N-API addon shipped in `@tsgo-strict/<triple>` platform subpackages.

export type Category = 'error' | 'warning' | 'message';

export interface RunOptions {
  /** Path to the project tsconfig, absolute or relative to `cwd`. Defaults to `tsconfig.json`. */
  project?: string;
  /** Working directory for binary and tsconfig resolution. Defaults to `process.cwd()`. */
  cwd?: string;
  /** Plugin name to look up in `compilerOptions.plugins`. Defaults to `typescript-strict-plugin`. */
  strictPlugin?: string;
  /** Restrict the check to these files or directories. Empty / omitted means the full project. */
  subset?: string[];
  /** Cap on number of diagnostics returned. `0` or omitted disables the cap. */
  maxDiagnostics?: number;
  /** Forwarded to tsgo as `--pretty`. Defaults to `false` for stable parsing. */
  pretty?: boolean;
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
  /** Total diagnostic count before truncation. */
  errorCount: number;
  /** `0` clean, `1` strict errors, `2` internal failure. */
  exitCode: number;
  /** `true` when `maxDiagnostics` clipped the `diagnostics` array. */
  truncated: boolean;
  diagnostics: RunDiagnostic[];
  timings: RunTiming[];
}

/** Identifier of the platform subpackage that matches the current host, or `null` if unsupported. */
export function pickPackage(): string | null;

/** Absolute path to the per-platform `tsgo-strict` executable. Throws if no platform package is installed. */
export function resolveBinary(): string;

/** Absolute path to the per-platform N-API addon (`tsgo-strict.node`). Throws if no platform package is installed. */
export function resolveNativeAddon(): string;

/** Run the strict checker programmatically. Resolves with structured diagnostics and per-phase timings. */
export function run(options?: RunOptions): Promise<RunResult>;
