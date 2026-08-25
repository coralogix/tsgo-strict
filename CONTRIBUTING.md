# Contributing

Thanks for contributing to `tsgo-strict`.

## Contributor License Agreement (CLA)

We require that contributors sign our [Contributor License Agreement](CLA.md)
(CLA) before their first contribution can be merged.

When you open your first pull request, the CLA Assistant bot will comment with a
link to sign the [Coralogix CLA](https://cla-assistant.io/coralogix/tsgo-strict).
Signing is a one-time step and takes a moment; once signed, the check turns green
and your pull request can be reviewed.

## Development setup

1. Install a recent Rust toolchain (1.80+, the `rust-version` in `Cargo.toml`) — the workspace pins `stable` in `rust-toolchain.toml`.
2. Install Node.js 20+ (only needed to exercise the npm launcher and N-API integration tests; CI uses 24).
3. Install pnpm (`corepack enable` or `npm i -g pnpm`).

## Local checks

- `cargo fmt --all -- --check` — formatting
- `cargo clippy --workspace --all-targets -- -D warnings` — lint
- `cargo test --workspace` — unit tests
- `pnpm test:node` — builds the N-API addon for the current host, stages it
  into the matching platform package, and runs the Node integration suite.
- `cargo build --release` — optimized CLI for local benchmarking

### License headers

Every first-party source file must carry the Apache 2.0 header. CI enforces
this with a self-contained Node script, `scripts/license-header.mjs` — no extra
tool to install. The header text lives in `license-header.txt`.

- `pnpm license:check` — verify all headers are present (what CI runs)
- `pnpm license:fix` — insert the header into any file missing it (keeps a
  leading `#!` shebang on line 1)

Test fixtures and the `perf-demo` corpus are user/third-party sample code and
are excluded.

## Pull requests

1. Keep PRs focused and small.
2. Add or update tests for behavior changes — both `cargo test` and
   `pnpm test:node` should stay green.
3. Run `cargo fmt --all` and `cargo clippy` before opening a PR.
4. Update `README.md` when user-facing behavior changes.

## Releases

Releases are cut manually via the **Release** workflow (`workflow_dispatch`
with a `bump` input of `patch`/`minor`/`major`). The workflow cross-builds
the CLI + N-API addons, publishes to the public npm registry under
`@coralogix/tsgo-strict*`, and pushes a `vX.Y.Z` tag.

The in-repo `package.json` and `Cargo.toml` `version` fields are **not**
the source of truth — they're frozen at their initial value and never
updated by the release workflow. The latest `v*` git tag is
authoritative. If you want to know what version is currently published,
run `git tag --list 'v*' --sort=-v:refname | head -n1`.

`scripts/stamp-versions.mjs <version>` writes the resolved version into
both, ephemerally on the CI runner. If you add a new place that reports a
version to users, stamp it there too — the script exits non-zero when a
target it expects has gone missing, so a rename fails the release rather
than silently shipping a stale number.

## Reporting bugs and requesting features

Use GitHub Issues and include:

- Expected behavior
- Actual behavior
- Minimal reproduction (if possible)
- Environment details (Node version, OS, command used)
