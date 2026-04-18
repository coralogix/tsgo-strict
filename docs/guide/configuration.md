# Configuration

`tsgo-strict` reads its configuration from the **`typescript-strict-plugin`**
entry inside `compilerOptions.plugins` in your `tsconfig.json`. This reuses
the same shape that the original TS-based
[`typescript-strict-plugin`](https://github.com/allegro/typescript-strict-plugin)
project uses, so existing config Just Works.

## Plugin block

```jsonc
{
  "compilerOptions": {
    "plugins": [
      {
        "name": "typescript-strict-plugin",
        "paths": ["./src/strict", "./src/shared/**/*.ts"],
        "excludePattern": "\\.test\\.ts$"
      }
    ]
  }
}
```

### `paths` (optional)

An array of minimatch glob patterns. Files whose path (relative to the
tsconfig) matches **any** of these patterns are included in the strict
subset.

- Omit `paths` (or leave it empty) to mean **"include everything"** — every
  source file in the project is eligible, and you rely on `excludePattern` or
  pragmas to scope down.
- Trailing directory paths like `./src/strict` are treated as recursive
  directory matches.

### `excludePattern` (optional)

A regular expression. Files whose path matches this regex are **excluded**
from the strict subset, even if `paths` would otherwise include them.

Common patterns:

```jsonc
{
  "excludePattern": "\\.test\\.ts$"        // skip tests
}
```

```jsonc
{
  "excludePattern": "(\\.spec|__mocks__)"  // skip specs + mocks
}
```

## Precedence

The final set of "strict files" is determined by the following rules, in
order:

1. A `// @ts-strict-ignore` pragma in a file **always** excludes it.
2. A `// @ts-strict` pragma in a file **always** includes it.
3. If neither pragma is present, the plugin config (`paths` + `excludePattern`)
   decides.

See [Pragmas](/guide/pragmas) for more.

## Extends chains

`tsgo-strict` understands `tsconfig.json` `extends`, whether relative or
using npm-style package references like `@tsconfig/node20`. The plugin block
is resolved from the effective, merged config — so you can put the strict
plugin in a base config and inherit it across projects.

## What "strict mode" means here

When `tsgo-strict` runs the underlying `tsgo` compiler, it writes a
temporary tsconfig that `extends` yours with **`"strict": true`** and pins
the include list to the selected files. This matches the behavior of the
original `typescript-strict-plugin`: flip `strict` and let the compiler
unfurl it into the standard strict bundle (`strictNullChecks`,
`noImplicitAny`, `strictFunctionTypes`, `strictBindCallApply`,
`strictPropertyInitialization`, `noImplicitThis`,
`useUnknownInCatchVariables`, `alwaysStrict`).

Additional opt-ins like `noUncheckedIndexedAccess`,
`exactOptionalPropertyTypes`, `noImplicitReturns`, `noUnusedLocals`, or
`noUnusedParameters` are **not** forced on — if you want them, enable them
in your own tsconfig.

Everything else in your tsconfig (paths, lib, jsx, target, moduleResolution,
etc.) is preserved.
