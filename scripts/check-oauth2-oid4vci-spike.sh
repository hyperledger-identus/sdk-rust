#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
manifest="$repo_root/docs/research/oauth2-oid4vci-spike/Cargo.toml"
lockfile="$repo_root/docs/research/oauth2-oid4vci-spike/Cargo.lock"

if [[ "$(rustc --version)" != rustc\ 1.98.1\ * ]]; then
  echo "oauth2-spike: expected Rust 1.98.1" >&2
  exit 1
fi

cargo test --locked --manifest-path "$manifest"
cargo clippy --locked --manifest-path "$manifest" --all-targets -- -D warnings
cargo deny --manifest-path "$manifest" --config "$repo_root/deny.toml" check
cargo audit --file "$lockfile" --deny warnings

host_cone="$({
  cargo tree --locked --manifest-path "$manifest" -p oauth2 \
    --edges normal,build --prefix none
} | sed 's/ (\*)$//' | sort -u | wc -l | tr -d ' ')"
if [[ "$host_cone" != "68" ]]; then
  echo "oauth2-spike: expected 68 normalized host cone lines, got $host_cone" >&2
  exit 1
fi

if git -C "$repo_root" diff --quiet origin/develop -- Cargo.toml Cargo.lock; then
  echo "oauth2-spike: host, supply-chain and root-graph isolation checks passed"
else
  echo "oauth2-spike: root Cargo.toml or Cargo.lock differs from origin/develop" >&2
  exit 1
fi
