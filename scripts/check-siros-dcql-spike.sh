#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
manifest="$repo_root/docs/research/siros-dcql-spike/Cargo.toml"
lockfile="$repo_root/docs/research/siros-dcql-spike/Cargo.lock"

if [[ "$(rustc --version)" != rustc\ 1.98.1\ * ]]; then
  echo "siros-dcql-spike: expected Rust 1.98.1" >&2
  exit 1
fi

cargo test --locked --manifest-path "$manifest"
cargo clippy --locked --manifest-path "$manifest" --all-targets -- -D warnings
cargo deny --manifest-path "$manifest" --config "$repo_root/deny.toml" check
cargo audit --file "$lockfile" --deny warnings

host_cone="$({
  cargo tree --locked --manifest-path "$manifest" -p siros-dcql \
    --edges normal,build --prefix none
} | sed 's/ (\*)$//' | sort -u | wc -l | tr -d ' ')"
if [[ "$host_cone" != "12" ]]; then
  echo "siros-dcql-spike: expected 12 normalized host cone lines, got $host_cone" >&2
  exit 1
fi

if ! rg -q '^siros-dcql[[:space:]]*=[[:space:]]*"=0\.3\.0"$' "$repo_root/Cargo.toml" || \
  [[ "$(rg -c '^name = "siros-dcql"$' "$repo_root/Cargo.lock")" != "1" ]]; then
  echo "siros-dcql-spike: exact root adoption evidence is missing" >&2
  exit 1
fi

if ! cargo tree --locked -p identus-oid4vp --edges normal,build --prefix none | \
  rg -q '^siros-dcql v0\.3\.0$'; then
  echo "siros-dcql-spike: identus-oid4vp does not privately consume exact 0.3.0" >&2
  exit 1
fi

if rg -n 'pub (use|fn|struct|enum|trait|type).*siros_dcql|pub use siros_dcql' \
  "$repo_root/crates/oid4vp/src"; then
  echo "siros-dcql-spike: candidate types escaped the OID4VP public facade" >&2
  exit 1
fi

echo "siros-dcql-spike: fixture, supply-chain and exact private-adoption checks passed"
