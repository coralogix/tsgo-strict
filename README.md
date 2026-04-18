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

## Why you'd use it

Flipping `"strict": true` on a large, legacy codebase typically surfaces
thousands of errors at once. `tsgo-strict` lets you enable strict mode **only
for the files or paths that are ready**, so you can migrate incrementally
without drowning the build.

You opt files in via one of:

- A **plugin config** in `tsconfig.json` listing the paths (and optional
  exclude regex) that should be checked strictly.
- A `// @ts-strict` comment at the top of a file to force it into scope.
- A `// @ts-strict-ignore` comment to force a file out of scope, even if the
  plugin paths would match it.

Everything else is checked under your normal, non-strict `tsconfig` settings
and its errors are filtered out of the output.

## How it works

In the default `--mode exact`, `tsgo-strict`:

1. **Loads your `tsconfig.json`** (including `extends` chains, relative or
   npm-style like `@tsconfig/node20`), pulls the plugin block out of
   `compilerOptions.plugins`, and computes the project's source file list.
2. **Selects the strict subset.** It reads the first 4 KB of each candidate
   file in parallel, checking for pragmas, then applies the plugin
   `paths` / `excludePattern` filter. Pragmas win over config.
3. **Writes two temporary tsconfigs** that `extend` yours — a *baseline*
   (strict flags off) and a *strict* one (14 strict-family flags on) — each
   pinned to the selected files.
4. **Spawns `tsgo` twice in parallel** (one per config), collecting
   diagnostics from each.
5. **Diffs the two diagnostic sets.** Any diagnostic that also appears in the
   baseline run is subtracted. What remains is the *net* errors that strict
   mode introduces — the only thing you need to fix.
6. **Formats and prints** the diff in `tsc`-style text or JSON, sorted for
   stable output, with an exit code reflecting whether anything remained.

The `--mode fast` variant skips the baseline pass and reports all strict
diagnostics on the selected subset — useful when you know the subset compiles
cleanly in non-strict mode (e.g. in pre-commit hooks scoped to changed files).

## Configure strict scope

Add the plugin block to your `tsconfig.json`:

```jsonc
{
  "compilerOptions": {
    "plugins": [
      {
        "name": "typescript-strict-plugin",
        "paths": ["./src/strict", "./src/shared/**/*.ts"],
        "excludePattern": "\\.test\\.ts$"
      }
    ]
  }
}
```

- `paths` — glob patterns (minimatch syntax) included in the strict subset.
  Omit for "include everything" and rely on pragmas/excludes.
- `excludePattern` — a regex applied to each file's path; matches are
  excluded.

Then drop pragmas into individual files to override:

```ts
// @ts-strict
export function alreadyReady() { /* forced in */ }

// @ts-strict-ignore
export function notYet() { /* forced out */ }
```

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
