# Contributing

Thanks for contributing to `tsgo-strict`.

## Development setup

1. Install a recent Rust toolchain (1.75+) — the workspace pins `stable` in `rust-toolchain.toml`.
2. Install Node.js 20+ (only needed to exercise the npm launcher and N-API integration tests).
3. Install pnpm (`corepack enable` or `npm i -g pnpm`).

## Local checks

- `cargo fmt --all -- --check` — formatting
- `cargo clippy --workspace --all-targets -- -D warnings` — lint
- `cargo test --workspace` — unit tests
- `pnpm test:node` — builds the N-API addon for the current host, stages it
  into the matching platform package, and runs the Node integration suite.
- `cargo build --release` — optimized CLI for local benchmarking

## Pull requests

1. Keep PRs focused and small.
2. Add or update tests for behavior changes — both `cargo test` and
   `pnpm test:node` should stay green.
3. Run `cargo fmt --all` and `cargo clippy` before opening a PR.
4. Update `README.md` when user-facing behavior changes.

## Releases

Releases are cut manually via the **Release** workflow (`workflow_dispatch`
with a `bump` input of `patch`/`minor`/`major`). The workflow cross-builds
the CLI + N-API addons, publishes to JFrog under `@cx/tsgo-strict*`, and
pushes a `vX.Y.Z` tag.

The in-repo `package.json` `version` fields are **not** the source of
truth — they're frozen at their initial value and never updated by the
release workflow. The latest `v*` git tag is authoritative. If you want
to know what version is currently on JFrog, run
`git tag --list 'v*' --sort=-v:refname | head -n1`.

## Reporting bugs and requesting features

Use GitHub Issues and include:

- Expected behavior
- Actual behavior
- Minimal reproduction (if possible)
- Environment details (Node version, OS, command used)
