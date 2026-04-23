# Exit codes

`tsgo-strict` uses three exit codes, following `tsc` conventions where
possible.

| Code | Meaning | What to do |
| ---- | ------- | ---------- |
| `0`  | Clean — no strict diagnostics in the selected subset. | Nothing. Let CI pass. |
| `1`  | Strict diagnostics were reported. | Inspect the output; fix or add `// @ts-strict-ignore`. |
| `2`  | Tool, config, or runtime error. | Check stderr — likely a missing/invalid tsconfig or an unresolvable `tsgo` binary. |

## Distinguishing `1` and `2` in CI

When you want your pipeline to report "type errors" vs "tool broken"
differently, branch on the exit code:

```bash
npx tsgo-strict
code=$?

case $code in
  0) echo "✔ strict clean" ;;
  1) echo "✘ strict type errors"; exit 1 ;;
  2) echo "!! tsgo-strict itself failed"; exit 2 ;;
esac
```

From Node, check `result.exitCode` from the [programmatic
API](/reference/api):

```ts
const { exitCode } = await run();
if (exitCode === 2) {
  throw new Error('tsgo-strict failed to run');
}
```

## Common causes of exit code 2

- The `project` path doesn't resolve to a readable `tsconfig.json`.
- No `tsgo` binary was found (neither `TSGO_BINARY`,
  `node_modules/.bin/tsgo`, nor `PATH` has one).
- The plugin block in `compilerOptions.plugins` is malformed JSON or has
  the wrong shape.
- An `excludePattern` glob is invalid.
