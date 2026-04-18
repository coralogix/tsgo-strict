# Contributing

Contributions are welcome! The full contributor guide lives in the repo
alongside the code:

- **[CONTRIBUTING.md](https://github.com/ashley-hunter/tsgo-strict-plugin/blob/main/CONTRIBUTING.md)** — dev setup, Rust toolchain, running tests, coding style.
- **[CODE_OF_CONDUCT.md](https://github.com/ashley-hunter/tsgo-strict-plugin/blob/main/CODE_OF_CONDUCT.md)** — expectations for how we work together.
- **[SECURITY.md](https://github.com/ashley-hunter/tsgo-strict-plugin/blob/main/SECURITY.md)** — reporting security issues responsibly.
- **[SUPPORT.md](https://github.com/ashley-hunter/tsgo-strict-plugin/blob/main/SUPPORT.md)** — where to ask for help.

## Quick dev loop

```bash
# Rust build + unit tests
cargo build --release
cargo test --workspace

# Node integration tests against a freshly staged N-API addon
pnpm test:node

# Lint & format
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

## Docs contributions

The docs site is VitePress, sources in `docs/`:

```bash
pnpm install
pnpm docs:dev      # hot-reload on localhost:5173
pnpm docs:build    # production build
pnpm docs:preview  # preview built site
```

Any push to `main` that touches `docs/` deploys automatically via
`.github/workflows/docs.yml`.

## Reporting issues

Please file issues on
[GitHub Issues](https://github.com/ashley-hunter/tsgo-strict-plugin/issues)
with:

- tsgo-strict version
- Node version, OS, and architecture
- Minimal reproducing tsconfig + file(s), if applicable
- The full output of the failing run
