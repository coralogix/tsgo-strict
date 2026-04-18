# tsgo-strict

`tsgo-strict` is a fast, strict-only TypeScript checker. It wraps Microsoft's
`tsgo` compiler (`@typescript/native-preview`) and emits only the diagnostics
you would see if strict mode were turned on for a specific subset of your
project — enabling a file-by-file or path-by-path migration to strict.

Written in Rust and distributed through per-platform npm packages (the
`tsgo-strict` launcher plus one prebuilt binary + N-API addon per target,
resolved via `optionalDependencies`).

## What it does

- Reads `typescript-strict-plugin` config from `compilerOptions.plugins`.
- Honors `@ts-strict` / `@ts-strict-ignore` pragmas.
- Supports checking an explicit subset of files or globs.
- In `--mode exact` (default), runs a baseline pass and a strict pass in
  parallel and reports only the **net** strict diagnostics.
- Emits `tsc`-style text output or JSON.

## Install

```bash
npm install --save-dev tsgo-strict @typescript/native-preview
# or
pnpm add -D tsgo-strict @typescript/native-preview
```

`@typescript/native-preview` is declared as an optional peer dependency — any
tsgo available on `PATH`, in `node_modules/.bin`, or via the `TSGO_BINARY`
env var works too.

## CLI usage

```bash
tsgo-strict [fileOrGlob ...]
```

Options:

- `-p, --project <path>` — tsconfig path (default `tsconfig.json`)
- `--json` — JSON diagnostics
- `--pretty` / `--no-pretty` — forward pretty output to tsgo
- `--trace-performance` — per-phase timings on stderr
- `--strict-plugin <name>` — plugin name (default `typescript-strict-plugin`)
- `--mode <exact|fast>` — default `exact`
- `--max-diagnostics <n>` — cap the diagnostic output
- `--cwd <path>` — override working directory

Exit codes:

- `0` — no strict diagnostics
- `1` — strict diagnostics found
- `2` — tool/config/runtime error

Environment:

- `TSGO_BINARY` — explicit path to a `tsgo` binary (highest-priority resolver)
- `TSGO_STRICT_PARALLEL=0` — force sequential baseline+strict passes in
  `--mode exact` (default runs them concurrently)

## Programmatic API

```js
import { run } from 'tsgo-strict';

const result = await run({
  project: 'tsconfig.json',
  subset: ['src/in-scope'],
  mode: 'exact',
});

console.log(result.errorCount, result.diagnostics);
```

Returns `{ mode, errorCount, exitCode, truncated, diagnostics[], timings[] }`.
Full type definitions ship with the package.

## Development

See [CONTRIBUTING.md](./CONTRIBUTING.md). The short version:

```bash
cargo build --release
cargo test --workspace
pnpm test:node          # builds the N-API addon + runs Node integration tests
```

## Open source project files

- [CONTRIBUTING.md](./CONTRIBUTING.md)
- [CODE_OF_CONDUCT.md](./CODE_OF_CONDUCT.md)
- [SECURITY.md](./SECURITY.md)
- [SUPPORT.md](./SUPPORT.md)
- [BENCHMARKS.md](./BENCHMARKS.md)
