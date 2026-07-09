# Getting Started

This page gets you from zero to a first strict check in about two minutes.

## Prerequisites

- **Node.js ≥ 18** (for the npm launcher).
- A project with a `tsconfig.json`.
- **A native TypeScript compiler.** Either:
  - **TypeScript 7 or later** — the `typescript` package now ships the native
    compiler (recommended), or
  - **`@typescript/native-preview`** — for teams whose app stays on TypeScript
    5/6. It installs the native compiler for strict checking without changing
    the `typescript` your app builds with.

  Or set `TSGO_BINARY` to point at an existing `tsc`/`tsgo` binary.

## Install

::: code-group

```bash [npm]
npm install --save-dev @coralogix/tsgo-strict typescript@^7
```

```bash [pnpm]
pnpm add -D @coralogix/tsgo-strict typescript@^7
```

```bash [yarn]
yarn add -D @coralogix/tsgo-strict typescript@^7
```

:::

`typescript` is declared as an **optional peer dependency** — tsgo-strict only
needs a *native* compiler, which can come from `typescript@7+`,
`@typescript/native-preview`, a binary on `PATH`, or the `TSGO_BINARY` env var.

::: tip Not ready to move your app to TypeScript 7?
You don't have to. Keep your app on its current `typescript` and add the native
compiler just for strict checking:

```bash
npm install --save-dev @coralogix/tsgo-strict @typescript/native-preview
```

tsgo-strict prefers an installed `@typescript/native-preview` over your app's
`typescript`, so your build is unaffected.
:::

On install, npm will pick up exactly one of the
`@coralogix/tsgo-strict-<platform>` subpackages (via `optionalDependencies`) and use
its prebuilt native binary. No compile step runs on your machine.

## Configure a strict subset

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

See [Configuration](/guide/configuration) for the full reference.

## Run it

```bash
npx tsgo-strict
```

Output looks like `tsc`:

```text
src/strict/foo.ts(12,5): error TS2345: Argument of type '...' is not assignable to parameter of type '...'.

Found 1 error in 1 file.
```

Exit codes:

| Code | Meaning |
| ---- | --- |
| `0`  | No strict diagnostics. |
| `1`  | Strict diagnostics found. |
| `2`  | Tool, config, or runtime error. |

## Add it to CI

Run it as a separate step in your pipeline. Because it only reports strict
diagnostics, you can gate PRs on strict compliance for the opted-in subset
while letting the rest of the build run on your normal `tsc`:

```yaml
- name: Typecheck (normal)
  run: npx tsc --noEmit

- name: Typecheck (strict subset)
  run: npx tsgo-strict
```

See [Incremental migration](/guide/incremental-migration) for recommended
rollout patterns.

## What's next?

- [Pragmas](/guide/pragmas) — override the config on a per-file basis.
- [How it works](/guide/how-it-works) — the 5-step pipeline.
- [CLI reference](/reference/cli) — every flag and env var.
- [Programmatic API](/reference/api) — run it from Node.
