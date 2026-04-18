# Pragmas

Pragmas are single-line comments at the top of a source file that override
the plugin config on a per-file basis. They're the recommended way to
migrate individual files into — or park them out of — the strict subset.

## `// @ts-strict`

Forces the file **into** the strict subset, even if the plugin config's
`paths` / `excludePattern` wouldn't select it.

```ts
// @ts-strict
export function alreadyReady(input: string): number {
  return input.length;
}
```

Use this when:

- You're migrating one file at a time and don't want to touch the tsconfig
  between commits.
- You have a file outside your strict path roots that's ready to go.

## `// @ts-strict-ignore`

Forces the file **out of** the strict subset, even if the plugin config or a
parent `// @ts-strict` would include it.

```ts
// @ts-strict-ignore
export function notYet() {
  /* not strict-clean yet — tracked in JIRA-1234 */
}
```

Use this when:

- A file in your strict path is temporarily breaking strict checks and
  you need a quick out without reshaping `excludePattern`.
- A legacy file in an otherwise-strict directory is deliberately loose.

## Precedence rules

`tsgo-strict` reads the **first 4 KB** of each candidate source file in
parallel (via `rayon`) looking for these pragmas. The decision for each file
is:

1. If the file contains `// @ts-strict-ignore` → **excluded**.
2. Else if the file contains `// @ts-strict` → **included**.
3. Else the plugin `paths` / `excludePattern` decides.

In short: **pragmas beat config, and `ignore` beats `strict`.**

## Placement

Put the pragma at the **top of the file**, before any imports or code. Only
the first 4 KB of the file is scanned, so placing it at the bottom of a
large file won't work.

```ts
// @ts-strict   ← put it here
import { foo } from './foo';
```

A pragma can be in a line comment (`//`) or a block comment (`/* ... */`).
Both forms are recognized.

## See also

- [Configuration](/guide/configuration) — plugin block reference.
- [Incremental migration](/guide/incremental-migration) — how to combine
  pragmas and config for a smooth rollout.
