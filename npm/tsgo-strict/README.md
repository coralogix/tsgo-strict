# tsgo-strict

Strict-only TypeScript checking powered by the [`tsgo`](https://www.npmjs.com/package/@typescript/native-preview) native compiler, emulating [`typescript-strict-plugin`](https://github.com/allegro/typescript-strict-plugin) so you can adopt `strict: true` gradually.

Ships a native Rust CLI via platform-specific subpackages; no Node runtime work on the hot path.

## Install

```sh
npm install --save-dev tsgo-strict @typescript/native-preview
```

The correct native binary for your platform is installed automatically through `optionalDependencies`.

## Usage

```sh
tsgo-strict --project tsconfig.json
tsgo-strict src/feature          # run strict check only against this subtree
tsgo-strict --mode fast --json   # single compile pass, JSON output
```

See `tsgo-strict --help` for all flags.

## Supported platforms

- linux-x64 (gnu, musl)
- linux-arm64 (gnu)
- darwin-x64, darwin-arm64
- win32-x64 (msvc)
