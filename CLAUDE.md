# CLAUDE.md

## Project overview

Monorepo for `tsgo-strict` — a high-performance strict-only TypeScript checker powered by `tsgo` (the native TS compiler). Contains a Rust CLI, a Rust core library, a N-API addon for Node, and npm packages.

## Workspace layout

- `crates/tsgo-strict` — Rust CLI binary
- `crates/tsgo-strict-core` — Core library (config, file resolution, runner)
- `crates/tsgo-strict-napi` — N-API addon for Node
- `npm/tsgo-strict` — npm package (JS wrapper + tests)
- `npm/platforms/*` — per-platform native addon packages

## Common commands

```bash
# Build
cargo build --release

# Run all Rust tests
cargo test --workspace

# Format check (CI enforces this)
cargo fmt --all -- --check

# Auto-format
cargo fmt --all

# Lint (CI enforces this — warnings are errors)
cargo clippy --workspace --all-targets -- -D warnings

# Build + stage N-API addon for local testing
bash scripts/stage-local-addon.sh

# Run Node integration tests (requires staged addon)
node --test npm/tsgo-strict/test/napi.test.mjs

# Or use the combined script:
pnpm test:node
```

## Before pushing

Always run these checks locally — CI will reject PRs that fail any of them:

1. `cargo fmt --all` — apply formatting
2. `cargo clippy --workspace --all-targets -- -D warnings` — fix any lint warnings
3. `cargo test --workspace` — all Rust tests must pass
4. If you changed Rust code that affects the N-API addon, rebuild and test:
   `bash scripts/stage-local-addon.sh && node --test npm/tsgo-strict/test/napi.test.mjs`
