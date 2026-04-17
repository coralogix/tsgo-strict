# Performance Demo Project

This demo project is designed to benchmark `tsgo-strict` on a large TS codebase.

## Current shape

- Generated files: `4000` + `src/index.ts`
- Total TS files: `4001`
- Layout: `src/batch-XX/file-YYYY.ts`

## Regenerate

```bash
node perf-demo/scripts/generate.mjs [fileCount] [batchSize]
```

Examples:

```bash
node perf-demo/scripts/generate.mjs 4000 200
node perf-demo/scripts/generate.mjs 10000 250
```

## Run checks

Full project:

```bash
pnpm perf:check
```

Subset (single batch directory):

```bash
pnpm perf:subset
```

## Notes

- `tsconfig` includes `typescript-strict-plugin` under `compilerOptions.plugins`.
- No intentional type errors are included in generated files.
- For reliable benchmarking, run each command multiple times and compare medians.
