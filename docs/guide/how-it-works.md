# How it works

Under the hood, `tsgo-strict` is a thin, fast coordinator around `tsgo`. It
does five things on every run.

<div class="tss-pipeline">
  <div class="tss-step"><strong>Load tsconfig</strong>Resolve the project config, walk the <code>extends</code> chain, pull the plugin block out of <code>compilerOptions.plugins</code>, and compute the project's source file list.</div>
  <div class="tss-step"><strong>Select strict subset</strong>Read the first 4 KB of each candidate file in parallel via <code>rayon</code>, check for pragmas, then apply <code>paths</code> / <code>excludePattern</code>. Pragmas win over config.</div>
  <div class="tss-step"><strong>Write temp tsconfig</strong>Emit a temporary tsconfig that <code>extends</code> yours with <code>"strict": true</code> enabled and <code>files</code> pinned to the selected subset.</div>
  <div class="tss-step"><strong>Run tsgo once</strong>Spawn <code>tsgo</code> against the temp config, stream its diagnostics, and collect them in memory.</div>
  <div class="tss-step"><strong>Format &amp; exit</strong>Sort diagnostics for deterministic output, print them in <code>tsc</code>-style text, and exit with <code>0</code> / <code>1</code> / <code>2</code>.</div>
</div>

## Why this architecture

- **One `tsgo` invocation**, not one per file. Strict flags are applied at
  the project level; we just constrain which files get included in that
  project.
- **Filter at the input, not the output.** We don't run the full project
  and grep for strict errors — that would still pay the full compile cost
  for all the loose files. Instead, we pin the `files` list so `tsgo` only
  loads the strict subset (plus its transitive type dependencies).
- **Pragma scanning is I/O-bound, so it goes parallel.** Reading the first
  4 KB of every source file in serial would dominate the runtime. `rayon`
  parallelizes it across cores and drops ~1.5s to ~475ms on a 4000-file
  corpus (see [Benchmarks](/benchmarks)).

## Where "strict" comes from

The temporary tsconfig we emit forces `"strict": true` regardless of what
your base tsconfig says. This matches the original
`typescript-strict-plugin`, which overrides the same single setting on the
language service host.

`strict` is the umbrella flag that TypeScript unfurls into the standard
strict bundle: `strictNullChecks`, `noImplicitAny`, `strictFunctionTypes`,
`strictBindCallApply`, `strictPropertyInitialization`, `noImplicitThis`,
`useUnknownInCatchVariables`, and `alwaysStrict`. Optional-but-related
knobs like `noUncheckedIndexedAccess` or `exactOptionalPropertyTypes` are
**not** forced on — opt in via your own tsconfig if you want them.

Alongside `strict`, the temp config pins `noEmit: true` and applies a
small set of TypeScript 6 compatibility shims so v6 default changes don't
surface as new errors on code that was clean under v5. See
[Configuration › TypeScript 6 compatibility](/guide/configuration#typescript-6-compatibility)
for the list. Everything else (module, target, jsx, paths, lib, …) is
inherited from your base config.

## Binary resolution

`tsgo-strict` drives the native TypeScript compiler (TypeScript 7 or later).
When it needs one, it resolves a binary in this order and uses the first hit:

1. The `TSGO_BINARY` environment variable.
2. A platform-specific native binary from an installed distribution, found by
   walking up from the working directory:
   `@typescript/typescript-<platform>-<arch>/lib/tsc` (TypeScript 7+), or
   `@typescript/native-preview-<platform>-<arch>/lib/tsgo` (the legacy preview).
3. The `bin` entry of an installed `typescript`, `@typescript/native-preview`,
   or `tsgo` package.
4. `tsgo`, then `tsc`, on `PATH`.

It deliberately skips the `node_modules/.bin` wrapper scripts: they add Node
startup overhead and can fail to resolve the native binary on some Node
versions.

## Rust distribution

The package ships as:

- An npm **launcher** (`@coralogix/tsgo-strict`) containing the JS entry points and
  type definitions.
- Per-platform subpackages (`@coralogix/tsgo-strict-<target>`) containing the
  prebuilt binary + N-API addon for that platform, declared as
  `optionalDependencies`.

npm / pnpm / yarn install exactly the subpackage matching your platform at
install time. No compile step, no postinstall script.
