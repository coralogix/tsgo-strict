#!/usr/bin/env bash
# Build the N-API addon for the current host and stage it into the matching
# npm/platforms/<triple>/native/ directory so `require('@coralogix/tsgo-strict')` can
# load it in local dev / test runs.
set -euo pipefail

cd "$(dirname "$0")/.."

cargo build --release -p tsgo-strict-napi

case "$(uname -s)" in
  Linux)
    ext=so
    if ldd --version 2>&1 | grep -qi musl; then
      libc=musl
    else
      libc=gnu
    fi
    case "$(uname -m)" in
      x86_64)  triple="linux-x64-${libc}" ;;
      aarch64) triple="linux-arm64-${libc}" ;;
      *) echo "unsupported arch $(uname -m)" >&2; exit 1 ;;
    esac
    ;;
  Darwin)
    ext=dylib
    case "$(uname -m)" in
      arm64) triple="darwin-arm64" ;;
      x86_64) triple="darwin-x64" ;;
      *) echo "unsupported arch $(uname -m)" >&2; exit 1 ;;
    esac
    ;;
  *) echo "unsupported OS $(uname -s)" >&2; exit 1 ;;
esac

src="target/release/libtsgo_strict_napi.${ext}"
dst="npm/platforms/${triple}/native/tsgo-strict.node"
mkdir -p "$(dirname "$dst")"
cp "$src" "$dst"
echo "Staged $src -> $dst"
