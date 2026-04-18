# tsgo-strict

Strict-only TypeScript checking powered by the [`tsgo`](https://www.npmjs.com/package/@typescript/native-preview) native compiler, reading `typescript-strict-plugin`-style config so you can adopt `strict: true` gradually.

Ships a native Rust CLI plus an N-API addon via platform-specific subpackages; no Node runtime work on the hot path.

## Install

```sh
npm install --save-dev tsgo-strict @typescript/native-preview
```

The correct native binary and addon for your platform are installed automatically through `optionalDependencies`.

## CLI usage

```sh
tsgo-strict --project tsconfig.json
tsgo-strict src/feature   # run strict check only against this subtree
tsgo-strict --json        # JSON output
```

See `tsgo-strict --help` for all flags.

## Programmatic API

```js
import { run } from 'tsgo-strict';

const result = await run({
  project: 'tsconfig.json',
  subset: ['src/feature'],
});

console.log(result.errorCount, result.diagnostics);
```

Full TypeScript types are shipped with the package.

## Supported platforms

- linux-x64 (gnu, musl)
- linux-arm64 (gnu)
- darwin-x64, darwin-arm64
- win32-x64 (msvc)
