#!/usr/bin/env bash

set -euo pipefail

repo_root=$(git rev-parse --show-toplevel 2>/dev/null || pwd)
mode=${1:-smoke}

case "$mode" in
  replay | smoke | soak) ;;
  *)
    printf 'Usage: %s {replay|smoke|soak}\n' "$0" >&2
    exit 2
    ;;
esac

if ! cargo fuzz --version >/dev/null 2>&1; then
  nix_cmd=$(command -v nix || true)
  if [[ -z "$nix_cmd" && -x /nix/var/nix/profiles/default/bin/nix ]]; then
    nix_cmd=/nix/var/nix/profiles/default/bin/nix
  fi
  if [[ -z "$nix_cmd" ]]; then
    printf 'fuzz-did-peer: cargo-fuzz is unavailable; enter the Nix fuzz shell\n' >&2
    exit 1
  fi
  exec "$nix_cmd" develop "$repo_root#fuzz" --command "$repo_root/scripts/fuzz-did-peer.sh" "$mode"
fi

runner_version=$(cargo fuzz --version)
if [[ "$runner_version" != "cargo-fuzz 0.13.2" ]]; then
  printf 'fuzz-did-peer: expected cargo-fuzz 0.13.2, found %s\n' "$runner_version" >&2
  exit 1
fi

tmp_root=$(mktemp -d "${TMPDIR:-/tmp}/identus-did-peer-fuzz.XXXXXX")
trap 'rm -rf "$tmp_root"' EXIT
cp -R "$repo_root/fuzz/corpus/peer_did" "$tmp_root/peer_did"
mkdir -p "$repo_root/fuzz/artifacts/peer_did"

common=(
  -max_len=131072
  -timeout=5
  -rss_limit_mb=1024
  -reload=0
  -print_final_stats=1
)
case "$mode" in
  replay) limits=(-runs=0 -seed=424242) ;;
  smoke) limits=(-runs=4096 -seed=424242) ;;
  soak) limits=(-max_total_time=300) ;;
esac

started=$SECONDS
cargo fuzz run peer_did "$tmp_root/peer_did" --sanitizer address -- "${common[@]}" "${limits[@]}"
elapsed=$((SECONDS - started))
printf 'fuzz-did-peer: mode=%s target=peer_did elapsed_seconds=%d\n' "$mode" "$elapsed"
