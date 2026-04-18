#!/usr/bin/env bash
# Compare tsgo-strict (Rust) against allegro/typescript-strict-plugin's
# `tsc-strict` CLI on the same perf-demo project. Reports wall-clock in ms,
# median of N runs after 1 warmup. No external deps (no hyperfine).
set -euo pipefail

cd "$(dirname "$0")/.."

RUST_BIN="target/release/tsgo-strict"
PROJECT="perf-demo/tsconfig.json"
SUBSET_DIR="perf-demo/src/batch-00"
RUNS=${RUNS:-5}
WARMUP=${WARMUP:-1}

if [[ ! -x "$RUST_BIN" ]]; then
  echo "error: $RUST_BIN not built. run: cargo build --release" >&2
  exit 1
fi

if [[ ! -x perf-demo/node_modules/.bin/tsc-strict ]]; then
  echo "error: perf-demo/node_modules/.bin/tsc-strict missing. run: (cd perf-demo && npm install --no-save typescript typescript-strict-plugin @types/node)" >&2
  exit 1
fi

time_ms() {
  local start end
  start=$(date +%s%N)
  "$@" >/dev/null 2>&1 || true
  end=$(date +%s%N)
  echo $(( (end - start) / 1000000 ))
}

median() {
  local sorted
  sorted=$(printf '%s\n' "$@" | sort -n)
  local count=$#
  local mid=$((count / 2))
  if (( count % 2 == 1 )); then
    echo "$sorted" | sed -n "$((mid + 1))p"
  else
    local a b
    a=$(echo "$sorted" | sed -n "${mid}p")
    b=$(echo "$sorted" | sed -n "$((mid + 1))p")
    echo $(( (a + b) / 2 ))
  fi
}

bench() {
  local label=$1; shift
  for _ in $(seq 1 "$WARMUP"); do "$@" >/dev/null 2>&1 || true; done
  local times=()
  for _ in $(seq 1 "$RUNS"); do
    times+=($(time_ms "$@"))
  done
  printf '| %-38s | %6d ms | samples: %s |\n' "$label" "$(median "${times[@]}")" "${times[*]}"
}

run_tsc_strict_full() {
  ( cd perf-demo && PATH="$PWD/node_modules/.bin:$PATH" ./node_modules/.bin/tsc-strict -p tsconfig.json )
}

echo
echo "Runs: $RUNS  Warmup: $WARMUP"
echo '| Scenario                               | Median    | Samples (ms) |'
echo '|----------------------------------------|-----------|--------------|'
bench "tsgo-strict full project"    "$RUST_BIN" --project "$PROJECT"
bench "tsc-strict  full project"    bash -c 'cd perf-demo && PATH="$PWD/node_modules/.bin:$PATH" ./node_modules/.bin/tsc-strict -p tsconfig.json'
bench "tsgo-strict subset batch-00" "$RUST_BIN" --project "$PROJECT" "$SUBSET_DIR"
