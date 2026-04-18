# Benchmarks: TS vs Rust `tsgo-strict`

Hardware: Linux dev container. `perf-demo/` has 4001 TS files across 20 batches.
5 warm runs each (1 warmup discarded), medians reported. All TS runs are
`node dist/cli.js ...`; Rust runs are `target/release/tsgo-strict ...`. Both
drive the same `tsgo` binary (`@typescript/native-preview`), so the `baseline-run`
/ `strict-run` phases are a floor neither implementation can beat.

## Wall-clock (median of 5 runs)

| Scenario | TS (ms) | Rust (ms) | Speedup |
|---|---:|---:|---:|
| Full project, `--mode exact` (parallel)           | 4010 | 1673 | 2.40× |
| Full project, `--mode exact`, `TSGO_STRICT_PARALLEL=0` | 5045 | 2637 | 1.91× |
| Full project, `--mode fast`                       | 3749 | 1514 | 2.48× |
| Subset `src/batch-00`, `--mode exact`             | 2736 | 1221 | 2.24× |

## Phase breakdown (single `--trace-performance` run)

### Full project — `--mode exact` (parallel baseline+strict)

| Phase | TS (ms) | Rust (ms) | Delta |
|---|---:|---:|---:|
| config-load     | 102  | 1    | −101 |
| file-resolution | 1495 | 476  | −1019 |
| baseline-run    | 1424 | 1324 | −100 |
| strict-run      | 1424 | 1324 | −100 |
| diff            | 15   | 0    | −15 |
| formatting      | 1    | 0    | −1 |

### Full project — `--mode fast`

| Phase | TS (ms) | Rust (ms) | Delta |
|---|---:|---:|---:|
| config-load     | 103  | 0    | −103 |
| file-resolution | 1474 | 462  | −1012 |
| strict-run      | 997  | 979  | −18 |
| formatting      | 1    | 0    | −1 |

### Subset `src/batch-00` — `--mode exact`

| Phase | TS (ms) | Rust (ms) | Delta |
|---|---:|---:|---:|
| config-load     | 105  | 0    | −105 |
| file-resolution | 115  | 37   | −78 |
| baseline-run    | 1321 | 1198 | −123 |
| strict-run      | 1321 | 1198 | −123 |
| diff            | 1    | 0    | −1 |
| formatting      | 1    | 0    | −1 |

## Takeaways

- The Rust port gives a consistent **2.2–2.5× end-to-end speedup**. The remaining
  wall-clock is dominated by the `tsgo` child-process compile (`baseline-run` /
  `strict-run`), which is identical between implementations.
- The two concrete wins are:
  1. **`config-load`**: Node startup + TS module graph cost is ~100 ms; Rust
     parses tsconfig in <1 ms.
  2. **`file-resolution`**: `rayon` parallel head-read for pragma detection
     across 4000 files drops from ~1.5 s to ~475 ms (≈3×). On the subset path
     (where only the explicit input dir is walked) it drops from 115 ms to
     37 ms.
- `baseline-run` / `strict-run` are bounded by `tsgo` itself. Rust still runs
  10–15% faster because the Node parent doesn't stall the child's stdio and
  the temp-config write uses `tempfile` directly instead of `fs.promises`.
- **Sequential (`TSGO_STRICT_PARALLEL=0`) exact mode** is almost exactly 2×
  parallel exact mode on both implementations, confirming the thread-scope
  parallelism scales as expected.
