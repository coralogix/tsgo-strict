# Programmatic API

The `@cx/tsgo-strict` npm package exports a single `run()` function that runs
the strict checker and resolves with structured diagnostics and per-phase
timings. It's the same code path the CLI uses — the CLI is a thin wrapper
around `run()`.

## Quick example

```ts
import { run } from '@cx/tsgo-strict';

const result = await run({
  project: 'tsconfig.json',
  subset: ['src/in-scope'],
});

console.log(result.errorCount, result.diagnostics);
process.exit(result.exitCode);
```

## `run(options?)`

```ts
function run(options?: RunOptions): Promise<RunResult>;
```

Runs the strict checker once and resolves with the full result. Never
throws for user-facing errors (e.g. missing tsconfig) — those are surfaced
via `exitCode: 2` and a diagnostic in `diagnostics`.

## Types

### `RunOptions`

```ts
interface RunOptions {
  /** Path to the project tsconfig, absolute or relative to `cwd`. Defaults to `tsconfig.json`. */
  project?: string;
  /** Working directory for binary and tsconfig resolution. Defaults to `process.cwd()`. */
  cwd?: string;
  /** Restrict the check to these files or directories. Empty / omitted means the full project. */
  subset?: string[];
}
```

| Field | Type | Default | Description |
| ----- | ---- | ------- | ----------- |
| `project` | `string` | `"tsconfig.json"` | tsconfig path, absolute or relative to `cwd`. |
| `cwd` | `string` | `process.cwd()` | Working directory for binary + config resolution. |
| `subset` | `string[]` | `[]` | Files/directories to scope the check to. Empty = full project. |

### `RunResult`

```ts
interface RunResult {
  /** Total diagnostic count. */
  errorCount: number;
  /** `0` clean, `1` strict errors, `2` internal failure. */
  exitCode: number;
  diagnostics: RunDiagnostic[];
  timings: RunTiming[];
}
```

### `RunDiagnostic`

```ts
interface RunDiagnostic {
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

type Category = 'error' | 'warning' | 'message';
```

### `RunTiming`

```ts
interface RunTiming {
  label: string;       // "config-load" | "file-resolution" | "strict-run" | "formatting"
  durationMs: number;
}
```

## Use cases

### Gate CI with a custom summary

```ts
import { run } from '@cx/tsgo-strict';

const result = await run();

if (result.exitCode === 1) {
  console.log(`❌ ${result.errorCount} strict error(s) across ${new Set(result.diagnostics.map(d => d.file)).size} file(s)`);
  process.exit(1);
}
```

### Group diagnostics by file for a custom report

```ts
import { run } from '@cx/tsgo-strict';

const { diagnostics } = await run();
const byFile = new Map<string, typeof diagnostics>();

for (const d of diagnostics) {
  const key = d.file ?? '(global)';
  const existing = byFile.get(key) ?? [];
  existing.push(d);
  byFile.set(key, existing);
}

for (const [file, items] of byFile) {
  console.log(`${file}: ${items.length}`);
}
```

### Record per-phase timings

```ts
import { run } from '@cx/tsgo-strict';

const { timings } = await run();
for (const t of timings) {
  console.log(`${t.label}\t${t.durationMs.toFixed(1)}ms`);
}
```

The four phases are `config-load`, `file-resolution`, `strict-run`, and
`formatting`. See [Benchmarks](/benchmarks) for what to expect.

## See also

- [CLI reference](/reference/cli)
- [Exit codes](/reference/exit-codes)
- [Configuration](/guide/configuration)
