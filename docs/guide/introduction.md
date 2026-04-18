# What is tsgo-strict?

`tsgo-strict` is a **fast, strict-only TypeScript checker**. It wraps
Microsoft's `tsgo` compiler (`@typescript/native-preview`) and emits only the
diagnostics you would see if strict mode were turned on for a specific subset
of your project — enabling a file-by-file or path-by-path migration to
`strict: true`.

It is written in Rust and distributed through per-platform npm packages (a
`tsgo-strict` launcher plus one prebuilt binary + N-API addon per target,
resolved via `optionalDependencies`).

## What it does

- Reads `typescript-strict-plugin` config from `compilerOptions.plugins`.
- Honors `// @ts-strict` / `// @ts-strict-ignore` pragmas.
- Supports checking an explicit subset of files or globs from the CLI.
- Runs `tsgo` once with `"strict": true` enabled, scoped to the selected
  files, and reports the diagnostics it produces.

## Why you'd use it

Flipping `"strict": true` on a large, legacy codebase typically surfaces
thousands of errors at once. `tsgo-strict` lets you enable strict mode **only
for the files or paths that are ready**, so you can migrate incrementally
without drowning the build.

You opt files in via one of:

- A **plugin config** in `tsconfig.json` listing the paths (and optional
  exclude regex) that should be checked strictly. See [Configuration](/guide/configuration).
- A `// @ts-strict` comment at the top of a file to force it into scope.
- A `// @ts-strict-ignore` comment to force a file out of scope, even if the
  plugin paths would match it.

See [Pragmas](/guide/pragmas) for precedence rules.

Everything else is checked under your normal, non-strict `tsconfig` settings
and its errors are filtered out of the output.

## What it's not

- **Not a replacement for `tsc`.** Use `tsc` (or `tsgo`) for your normal
  build. `tsgo-strict` runs alongside them and reports *only* the strict
  diagnostics you care about during migration.
- **Not a code transformer.** It doesn't edit files or auto-fix errors.
- **Not a watch-mode checker.** It runs once and exits.

## Next steps

- [Install and run your first check](/guide/getting-started)
- [Configure the strict subset](/guide/configuration)
- [See the pipeline](/guide/how-it-works)
- [Benchmarks](/benchmarks)
