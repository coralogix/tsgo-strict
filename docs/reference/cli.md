# CLI reference

## Synopsis

```bash
tsgo-strict [fileOrGlob ...] [-p/--project <path>]
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

## Exit codes

| Code | Meaning |
| ---- | --- |
| `0`  | No strict diagnostics. |
| `1`  | Strict diagnostics found. |
| `2`  | Tool, config, or runtime error (invalid tsconfig, missing binary, etc.). |

See [Exit codes](/reference/exit-codes) for how to interpret each in CI.

## Environment variables

### `TSGO_BINARY`

Explicit path to a `tsgo` binary. When set, this is the highest-priority
resolver — used instead of `node_modules/.bin/tsgo` or `PATH`.

```bash
TSGO_BINARY=/usr/local/bin/tsgo tsgo-strict
```

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
