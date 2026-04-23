# Incremental migration

`tsgo-strict` is built to make "turn strict on, eventually" a safe, staged
project. This page describes a migration pattern that works well across
codebase sizes.

## The goal

End state: `"strict": true` in the main `tsconfig.json`, and
`tsgo-strict` retired. Every intermediate state should pass CI.

## Phase 1 — Baseline (zero risk)

Add `tsgo-strict` to your project but **don't gate CI on it yet**.

1. Install the package — see [Getting Started](/guide/getting-started).
2. Add an empty plugin block to `tsconfig.json`:

   ```jsonc
   {
     "compilerOptions": {
       "plugins": [
         { "name": "typescript-strict-plugin", "paths": [] }
       ]
     }
   }
   ```

3. Run it locally: `npx tsgo-strict`. With `paths: []`, no files are in
   scope, so this is a clean green run.

At this point you can start opting in individual files via the
`// @ts-strict` pragma. Nothing is enforced yet.

## Phase 2 — Opt in files

Pick a leaf file — something that has few or no internal imports — and add
a pragma:

```ts
// @ts-strict
import { foo } from './foo';
// ...
```

Fix whatever strict diagnostics `tsgo-strict` reports for that file, commit,
repeat. The pragma travels with the file, so there's no centralized list to
maintain during this phase.

:::tip
Start from the bottom of your import graph. Strictening a utility module
first pays off for every caller that gets migrated later.
:::

## Phase 3 — Gate CI

Once you've got a handful of strict files, start gating on strict
diagnostics:

```yaml
- name: Typecheck (strict subset)
  run: npx tsgo-strict
```

Now regressions on already-migrated files will fail CI, but unmigrated code
is still free to be loose.

## Phase 4 — Switch to path-based config

When a whole directory is strict-clean, promote it from per-file pragmas to
config-level `paths`. This avoids pragma sprawl and makes new files in that
directory strict by default.

```jsonc
{
  "plugins": [
    {
      "name": "typescript-strict-plugin",
      "paths": ["./src/strict", "./src/shared"],
      "excludePattern": ["**/*.test.ts"]
    }
  ]
}
```

You can remove the per-file `// @ts-strict` pragmas from files now covered
by config — or leave them in place as documentation; both are fine.

If a single file in a strict directory regresses temporarily, tag it with
`// @ts-strict-ignore` and track the work in your tracker. This keeps the
merge unblocked without shrinking the strict surface.

## Phase 5 — Flip the main tsconfig

When strict coverage is close to 100%, flip `"strict": true` on the main
tsconfig, delete the plugin block, and retire `tsgo-strict` from your
scripts. Any remaining holdout files will need pragmas or fixes against
`tsc` directly — but by this point there should be very few.

## Tips

- **Don't chase fragility.** If a file is about to be deleted or rewritten,
  mark it `@ts-strict-ignore` and move on.
- **Run it in pre-commit.** Because it's fast, you can hook it into husky /
  lefthook / lint-staged without slowing people down.
- **Pair with a tracking metric.** Count `// @ts-strict` occurrences over
  time for a rough migration-progress graph.
