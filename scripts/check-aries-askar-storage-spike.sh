#!/usr/bin/env bash
set -euo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
fixture="$repo_root/docs/research/aries-askar-storage-spike"

if rg -q 'aries-askar|askar-storage|askar-crypto' "$repo_root/Cargo.toml" "$repo_root/Cargo.lock"; then
  echo "Aries Askar entered the root dependency graph" >&2
  exit 1
fi

cargo test --manifest-path "$fixture/Cargo.toml" --locked
cargo clippy --manifest-path "$fixture/Cargo.toml" --locked --all-targets -- -D warnings

tree=$(cargo tree --manifest-path "$fixture/Cargo.toml" --locked --edges normal,build)
grep -q 'aries-askar v0.4.6' <<<"$tree"
grep -q 'askar-storage v0.2.4' <<<"$tree"
grep -q 'askar-crypto v0.3.7' <<<"$tree"
grep -q 'libsqlite3-sys' <<<"$tree"
if grep -Eq 'ffi-support|postgres|env_logger' <<<"$tree"; then
  echo "disabled Askar feature entered the research graph" >&2
  exit 1
fi

echo "aries-askar-storage-spike: exact locked host evidence passed"
