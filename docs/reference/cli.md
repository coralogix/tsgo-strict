# CLI reference

## Synopsis

```bash
tsgo-strict [fileOrGlob ...] [-p/--project <path>] [--backend <auto|tsc|tsrs>] [--fast]
```

## Arguments

### `fileOrGlob ...` (optional)

Zero or more file paths or minimatch globs, relative to the current
working directory. When provided, only these paths are candidates for the
strict check — overriding the plugin config's `paths`.

Pragmas still apply: `// @ts-strict-ignore` can exclude a given file even
if it was passed on the command line.

```bash
# Check only one directory
tsgo-strict src/strict

# Check a glob
tsgo-strict "src/**/*.ts"

# Check the whole project (use the plugin config)
tsgo-strict
```

## Options

### `-p, --project <path>`

Path to the tsconfig to use. Default: `tsconfig.json` (in the current
working directory).

```bash
tsgo-strict --project tsconfig.app.json
```

### `--backend <auto|tsc|tsrs>`

The type checker to drive. Default: `$TSGO_STRICT_BACKEND`, then `auto`.

- `auto` uses [tsrs](https://github.com/oxc-project/tsrs) when the project
  installs it, otherwise the native TypeScript compiler.
- `tsc` always uses the native TypeScript compiler.
- `tsrs` always uses tsrs, the Rust port of the TypeScript 7 compiler.

See [Backends](/guide/how-it-works#backends).

### `--fast`

Enable tsrs fast mode. It is faster on large machines but may change
diagnostics in rare cases, so it is off unless asked for. Only valid when tsrs
is the resolved backend. An ambient `TSRS_FAST` variable is ignored.

### `--timing`

Print the backend that ran and a per-phase timing breakdown to stderr.

## Exit codes

| Code | Meaning |
| ---- | --- |
| `0`  | No strict diagnostics. |
| `1`  | Strict diagnostics found. |
| `2`  | Tool, config, or runtime error (invalid tsconfig, missing binary, etc.). |

See [Exit codes](/reference/exit-codes) for how to interpret each in CI.

## Environment variables

### `TSGO_BINARY`

Explicit path to a compiler binary. When set, this is the highest-priority
resolver for every backend. Under `auto` it is treated as the native
TypeScript compiler; pass `--backend tsrs` if it points at tsrs.

```bash
TSGO_BINARY=/usr/local/bin/tsgo tsgo-strict
```

### `TSGO_STRICT_BACKEND`

Default for `--backend` (`auto`, `tsc` or `tsrs`).

### `TSRS_BINARY`

Explicit path to a tsrs binary, used by the `auto` and `tsrs` backends after
`TSGO_BINARY`. The same variable the `tsrs` launcher honors.

## Output format

Diagnostics are emitted in `tsc`-style text, one per line, followed by a
summary:

```text
src/strict/foo.ts(12,5): error TS2345: Argument of type '...' is not assignable to parameter of type '...'.
src/strict/bar.ts(3,10): error TS2322: Type '...' is not assignable to type '...'.

Found 2 errors in 2 files.
```

Diagnostics are sorted by file → line → column for deterministic output
across runs. This keeps diffs in CI logs stable.

## See also

- [Programmatic API](/reference/api) — call from Node.
- [Exit codes](/reference/exit-codes) — detailed exit code reference.
- [Configuration](/guide/configuration) — plugin block.
