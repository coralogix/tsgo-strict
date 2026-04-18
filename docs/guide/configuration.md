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
        "paths": ["./src/strict", "./src/shared"],
        "excludePattern": ["**/*.test.ts"]
      }
    ]
  }
}
```

### `paths` (optional)

An array of **directory prefixes**, resolved against the tsconfig's
directory. A file is included if its absolute path lives under any entry
(equivalent to `startsWith(entry + '/')`). This matches the original
`typescript-strict-plugin`.

- Omit `paths` (or leave it empty) to mean **"include everything"** — every
  source file in the project is eligible, and you rely on `excludePattern`
  or pragmas to scope down.
- Entries are directory-prefix matches, not globs: `"src/components"`
  includes every file under `src/components/`, recursively. `**` and `*`
  are **not** special here.
- Absolute paths are allowed and bypass the tsconfig directory.

### `excludePattern` (optional)

An array of **minimatch glob patterns** matched against each file's
absolute posix path. A file is excluded from the strict subset if any
pattern matches.

Accepts either a single string or an array; a bare string is treated as a
one-element array for convenience.

Common patterns:

```jsonc
{
  "excludePattern": ["**/*.test.ts"]                  // skip tests
}
```

```jsonc
{
  "excludePattern": ["**/*.spec.ts", "**/__mocks__/**"] // skip specs + mocks
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

### Opting out of a specific strict sub-flag

`tsgo-strict` doesn't expose a plugin option for disabling individual
strict sub-flags, and it doesn't need to: the temp tsconfig it emits
`extends` yours, so any sub-flag you set **explicitly** in your own
`compilerOptions` still wins. TypeScript evaluates each strictness flag
independently, and an explicit setting overrides the `strict: true`
implication.

For example, to keep `strict` on but relax `strictPropertyInitialization`:

```jsonc
{
  "compilerOptions": {
    "strictPropertyInitialization": false,
    "plugins": [
      { "name": "typescript-strict-plugin", "paths": ["./src/strict"] }
    ]
  }
}
```

Because the loose files are never strict-checked by `tsgo-strict`, this
setting only affects the strict subset in practice.
