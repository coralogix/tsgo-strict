/*
 * Copyright 2026 Coralogix Ltd.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// Type definitions for `@coralogix/tsgo-strict`.
//
// The runtime surface is implemented in `index.js` and backed by a native
// N-API addon shipped in `@coralogix/tsgo-strict-<triple>` platform subpackages.

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
