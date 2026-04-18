# tsgo-strict

📖 **Docs:** https://ashley-hunter.github.io/tsgo-strict/

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
- Runs `tsgo` once with `"strict": true` enabled, scoped to the selected
  files, and reports the diagnostics it produces.

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

`tsgo-strict`:

1. **Loads your `tsconfig.json`** (including `extends` chains, relative or
   npm-style like `@tsconfig/node20`), pulls the plugin block out of
   `compilerOptions.plugins`, and computes the project's source file list.
2. **Selects the strict subset.** It reads the first 4 KB of each candidate
   file in parallel, checking for pragmas, then applies the plugin
   `paths` / `excludePattern` filter. Pragmas win over config.
3. **Writes a temporary tsconfig** that `extends` yours with
   `"strict": true` enabled and pinned to the selected files.
4. **Spawns `tsgo`** once against that config and collects diagnostics.
5. **Prints** the result in `tsc`-style text, sorted for stable output, with
   an exit code reflecting whether anything remained.

## Configure strict scope

Add the plugin block to your `tsconfig.json`:

```jsonc
{
  "compilerOptions": {
    "plugins": [
      {
        "name": "typescript-strict-plugin",
        "paths": ["./src/strict", "./src/shared"],
        "excludePattern": ["**/*.test.ts"]
      }
    ]
  }
}
```

- `paths` — directory prefixes included in the strict subset. A file is
  included if its path lives under any entry. Omit for "include everything"
  and rely on pragmas / `excludePattern` to scope down.
- `excludePattern` — array of minimatch glob patterns (a single string is
  also accepted). Files matching any pattern are excluded.

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

Exit codes:

- `0` — no strict diagnostics
- `1` — strict diagnostics found
- `2` — tool/config/runtime error

Environment:

- `TSGO_BINARY` — explicit path to a `tsgo` binary (highest-priority resolver)

## Programmatic API

```js
import { run } from 'tsgo-strict';

const result = await run({
  project: 'tsconfig.json',
  subset: ['src/in-scope'],
});

console.log(result.errorCount, result.diagnostics);
```

Returns `{ errorCount, exitCode, diagnostics[], timings[] }`. Full type
definitions ship with the package.

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
