# tsgo-strict

`tsgo-strict` is a fast strict-only checker that mirrors `tsc-strict` style diagnostics while executing checks through `tsgo`.

## What it does

- Reads `typescript-strict-plugin` config from `compilerOptions.plugins`.
- Supports checking an explicit subset of files or globs.
- In `--mode exact` (default), performs baseline-vs-strict diff and reports only net strict diagnostics.
- Emits `tsc`-like text output or JSON.

## Install

```bash
pnpm add -D tsgo-strict
```

## Usage

```bash
tsgo-strict [fileOrGlob ...]
```

### Options

- `-p, --project <path>`: tsconfig path (default `tsconfig.json`)
- `--json`: JSON diagnostics
- `--pretty` / `--no-pretty`: backend pretty printing toggle
- `--trace-performance`: timing breakdown to stderr
- `--strict-plugin <name>`: plugin name (default `typescript-strict-plugin`)
- `--mode <exact|fast>`: default `exact`
- `--max-diagnostics <n>`: output cap
- `--cwd <path>`: working directory

## Exit codes

- `0`: no strict diagnostics
- `1`: strict diagnostics found
- `2`: tool/config/runtime error

## Notes

- v1 supports single `tsconfig` runs.
- Set `TSGO_BINARY=/path/to/tsgo` to override backend binary discovery.
- Exact mode runs baseline/strict passes concurrently by default.
- Set `TSGO_STRICT_PARALLEL=0` to force sequential exact-mode passes.

## Development commands

- `pnpm typecheck`
- `pnpm lint`
- `pnpm lint:fix`
- `pnpm format`
- `pnpm format:write`

## Git hooks

- `pre-commit`: runs `lint-staged` (`prettier --write` + `eslint --fix` on staged files)
- `pre-push`: runs `pnpm prepush:check` (`typecheck`, `test`, `build`)

Husky is wired via the `prepare` script and activates when this directory is a Git repo.

## Open source project files

- [CONTRIBUTING.md](./CONTRIBUTING.md)
- [CODE_OF_CONDUCT.md](./CODE_OF_CONDUCT.md)
- [SECURITY.md](./SECURITY.md)
- [SUPPORT.md](./SUPPORT.md)
- [CHANGELOG.md](./CHANGELOG.md)
