# Benchmarks: TS vs Rust `tsgo-strict`

Hardware: Linux dev container. `perf-demo/` has 4001 TS files across 20 batches.
5 warm runs each (1 warmup discarded), medians reported. All TS runs are
`node dist/cli.js ...`; Rust runs are `target/release/tsgo-strict ...`. Both
drive the same `tsgo` binary (`@typescript/native-preview`), so the `strict-run`
phase is a floor neither implementation can beat.

## Wall-clock (median of 5 runs)

| Scenario | TS (ms) | Rust (ms) | Speedup |
|---|---:|---:|---:|
| Full project                    | 3749 | 1514 | 2.48× |
| Subset `src/batch-00`           | 1436 | 1235 | 1.16× |

## Phase breakdown (single run, timings collected via the programmatic API)

### Full project

| Phase | TS (ms) | Rust (ms) | Delta |
|---|---:|---:|---:|
| config-load     | 103  | 0    | −103 |
| file-resolution | 1474 | 462  | −1012 |
| strict-run      | 997  | 979  | −18 |
| formatting      | 1    | 0    | −1 |

### Subset `src/batch-00`

| Phase | TS (ms) | Rust (ms) | Delta |
|---|---:|---:|---:|
| config-load     | 105  | 0    | −105 |
| file-resolution | 115  | 37   | −78 |
| strict-run      | 1216 | 1198 | −18 |
| formatting      | 1    | 0    | −1 |

## Takeaways

- The Rust port gives a consistent **~2.5× end-to-end speedup** on full-project
  runs. The remaining wall-clock is dominated by the `tsgo` child-process
  compile (`strict-run`), which is identical between implementations.
- The two concrete wins are:
  1. **`config-load`**: Node startup + TS module graph cost is ~100 ms; Rust
     parses tsconfig in <1 ms.
  2. **`file-resolution`**: `rayon` parallel head-read for pragma detection
     across 4000 files drops from ~1.5 s to ~475 ms (≈3×). On the subset path
     (where only the explicit input dir is walked) it drops from 115 ms to
     37 ms.
- `strict-run` is bounded by `tsgo` itself. Rust still runs a touch faster
  because the Node parent doesn't stall the child's stdio and the temp-config
  write uses `tempfile` directly instead of `fs.promises`.
