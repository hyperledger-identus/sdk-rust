#!/usr/bin/env bash

set -euo pipefail

repo_root=$(git rev-parse --show-toplevel 2>/dev/null || pwd)
mode=${1:-smoke}

usage() {
  printf 'Usage: %s {replay|smoke|soak}\n' "$0" >&2
}

case "$mode" in
  replay) campaign=(-runs=0 -seed=424242) ;;
  smoke) campaign=(-runs=4096 -seed=424242) ;;
  soak) campaign=(-max_total_time=300) ;;
  *) usage; exit 2 ;;
esac

if ! cargo fuzz --version >/dev/null 2>&1; then
  nix_cmd=$(command -v nix || true)
  if [[ -z "$nix_cmd" && -x /nix/var/nix/profiles/default/bin/nix ]]; then
    nix_cmd=/nix/var/nix/profiles/default/bin/nix
  fi
  if [[ -z "$nix_cmd" ]]; then
    printf 'fuzz-jws: cargo-fuzz is unavailable; enter the Nix shell\n' >&2
    exit 1
  fi
  exec "$nix_cmd" develop "$repo_root" --command "$repo_root/scripts/fuzz-jws.sh" "$mode"
fi

actual_version=$(cargo fuzz --version)
if [[ "$actual_version" != "cargo-fuzz 0.13.2" ]]; then
  printf 'fuzz-jws: expected cargo-fuzz 0.13.2, found %s\n' "$actual_version" >&2
  exit 1
fi

campaign_dir=$(mktemp -d "${TMPDIR:-/tmp}/identus-jws-fuzz.XXXXXX")
trap 'rm -rf "$campaign_dir"' EXIT
corpus_dir="$campaign_dir/jws_compact"
cp -R "$repo_root/fuzz/corpus/jws_compact" "$corpus_dir"
mkdir -p "$repo_root/fuzz/artifacts/jws_compact"

started=$SECONDS
cargo fuzz run jws_compact "$corpus_dir" --sanitizer address -- \
  -max_len=131072 \
  -timeout=5 \
  -rss_limit_mb=1024 \
  -reload=0 \
  -workers=1 \
  -dict="$repo_root/fuzz/dictionaries/jws_compact.dict" \
  "${campaign[@]}"
elapsed=$((SECONDS - started))
printf 'fuzz-jws: mode=%s target=jws_compact elapsed_seconds=%d\n' "$mode" "$elapsed"
