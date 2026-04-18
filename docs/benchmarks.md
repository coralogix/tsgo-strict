# Benchmarks

`tsgo-strict` is a Rust rewrite of an earlier TypeScript implementation of
the same tool, both driving the same underlying `tsgo` binary
(`@typescript/native-preview`). The numbers below measure how much time the
coordinator layer itself costs — the `tsgo` compile step is a floor neither
implementation can beat.

::: info Methodology
Hardware: Linux dev container. `perf-demo/` has **4,001 TS files** across
20 batches. 5 warm runs each (1 warmup discarded), medians reported. All TS
runs are `node dist/cli.js …`; Rust runs are `target/release/tsgo-strict …`.
:::

## Wall-clock (median of 5 runs)

| Scenario | TS (ms) | Rust (ms) | Speedup |
| --- | ---: | ---: | ---: |
| Full project | 3,749 | **1,514** | **2.48×** |
| Subset `src/batch-00` | 1,436 | **1,235** | **1.16×** |

<div class="tss-stats">
  <div class="tss-stat">
    <div class="tss-stat-value">2.48×</div>
    <div class="tss-stat-label">end-to-end speedup on full-project runs</div>
  </div>
  <div class="tss-stat">
    <div class="tss-stat-value">&lt;1 ms</div>
    <div class="tss-stat-label">config-load (vs ~100 ms in TS)</div>
  </div>
  <div class="tss-stat">
    <div class="tss-stat-value">~3×</div>
    <div class="tss-stat-label">file-resolution speedup via rayon parallel I/O</div>
  </div>
</div>

## Phase breakdown (single run)

Timings collected via the [programmatic API](/reference/api) (`timings[]`
on the result).

### Full project

| Phase | TS (ms) | Rust (ms) | Delta |
| --- | ---: | ---: | ---: |
| `config-load` | 103 | **0** | −103 |
| `file-resolution` | 1,474 | **462** | −1,012 |
| `strict-run` | 997 | 979 | −18 |
| `formatting` | 1 | 0 | −1 |

### Subset `src/batch-00`

| Phase | TS (ms) | Rust (ms) | Delta |
| --- | ---: | ---: | ---: |
| `config-load` | 105 | **0** | −105 |
| `file-resolution` | 115 | **37** | −78 |
| `strict-run` | 1,216 | 1,198 | −18 |
| `formatting` | 1 | 0 | −1 |

## Takeaways

- The Rust port gives a consistent **~2.5× end-to-end speedup** on
  full-project runs. The remaining wall-clock is dominated by the `tsgo`
  child-process compile (`strict-run`), which is identical between
  implementations.
- The two concrete wins are:
  1. **`config-load`**: Node startup + TS module graph cost is ~100 ms; Rust
     parses tsconfig in **<1 ms**.
  2. **`file-resolution`**: parallel head-read for pragma detection across
     ~4,000 files drops from **~1.5 s to ~475 ms** (≈3×) via `rayon`. On
     the subset path (where only the explicit input dir is walked) it
     drops from 115 ms to 37 ms.
- **`strict-run`** is bounded by `tsgo` itself. Rust still runs a touch
  faster because the Node parent doesn't stall the child's stdio and the
  temp-config write uses `tempfile` directly instead of `fs.promises`.

## Reproduce these locally

The `perf-demo/` workspace generates a synthetic 4,001-file project:

```bash
pnpm perf:generate     # materializes perf-demo/src/
pnpm perf:check        # full-project timing
pnpm perf:subset       # subset timing (src/batch-00)
```

Numbers will vary with your hardware, but the shape of the phase breakdown
— `config-load` near zero, `file-resolution` at a third of TS, `strict-run`
identical — is stable.
