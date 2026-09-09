#!/usr/bin/env bash
set -euo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
fixture="$repo_root/docs/research/iota-did-syntax-oracle"

if rg -q 'identity_did|identity-did|identity_core|identity-core' \
  "$repo_root/Cargo.toml" "$repo_root/Cargo.lock"; then
  echo "IOTA Identity entered the root dependency graph" >&2
  exit 1
fi

cargo test --manifest-path "$fixture/Cargo.toml" --locked
cargo clippy --manifest-path "$fixture/Cargo.toml" --locked --all-targets -- -D warnings

tree=$(cargo tree --manifest-path "$fixture/Cargo.toml" --locked --edges normal,build)
grep -q 'identity_did v1.5.1' <<<"$tree"
grep -q 'identity_core v1.5.1' <<<"$tree"
grep -q 'identity_jose v1.5.1' <<<"$tree"

echo "iota-did-syntax-oracle: exact locked corpus passed"
