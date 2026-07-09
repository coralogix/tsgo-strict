# @coralogix/tsgo-strict

Strict-only TypeScript checking powered by the native [`typescript`](https://www.npmjs.com/package/typescript) compiler (**TypeScript 7 or later**), reading `typescript-strict-plugin`-style config so you can adopt `strict: true` gradually.

Ships a native Rust CLI plus an N-API addon via platform-specific subpackages; no Node runtime work on the hot path.

## Install

```sh
npm install --save-dev @coralogix/tsgo-strict typescript@^7
```

Requires a **native TypeScript compiler**: either **TypeScript 7 or later** (the `typescript` package) or **`@typescript/native-preview`** if your app stays on TypeScript 5/6 — install that instead and tsgo-strict will use it for strict checking without changing your app's `typescript`. Both are declared as optional peer dependencies. The correct `tsgo-strict` native binary and addon for your platform are installed automatically through `optionalDependencies`.

## CLI usage

```sh
tsgo-strict --project tsconfig.json
tsgo-strict src/feature   # run strict check only against this subtree
```

## Programmatic API

```js
import { run } from '@coralogix/tsgo-strict';

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
