#!/usr/bin/env bash
# Measure median wall-clock for the perf-demo scenarios we care about.
# Prints a small markdown table. No external deps (no hyperfine).
set -euo pipefail

cd "$(dirname "$0")/.."

BIN=target/release/tsgo-strict
RUNS=${RUNS:-5}
WARMUP=${WARMUP:-1}

run_once() {
  local out
  out=$("$@" 1>/dev/null 2>&1 && echo OK || echo FAIL)
  [[ "$out" == OK ]] || return 1
}

time_ms() {
  # Measure one execution in ms via `date +%N`.
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
  printf '| %-42s | %5d ms | samples: %s |\n' "$label" "$(median "${times[@]}")" "${times[*]}"
}

echo '| Scenario                                   | Median    | Samples (ms) |'
echo '|--------------------------------------------|-----------|--------------|'
bench "full / exact / parallel" "$BIN" --project perf-demo/tsconfig.json
bench "full / exact / sequential"   env TSGO_STRICT_PARALLEL=0 "$BIN" --project perf-demo/tsconfig.json
bench "full / fast"                 "$BIN" --mode fast --project perf-demo/tsconfig.json
bench "subset batch-00 / exact"     "$BIN" --project perf-demo/tsconfig.json perf-demo/src/batch-00
bench "subset batch-00 / fast"      "$BIN" --mode fast --project perf-demo/tsconfig.json perf-demo/src/batch-00
