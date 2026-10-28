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
    printf 'fuzz-did-key: cargo-fuzz is unavailable; enter the Nix fuzz shell\n' >&2
    exit 1
  fi
  exec "$nix_cmd" develop "$repo_root#fuzz" --command "$repo_root/scripts/fuzz-did-key.sh" "$mode"
fi

runner_version=$(cargo fuzz --version)
if [[ "$runner_version" != "cargo-fuzz 0.13.2" ]]; then
  printf 'fuzz-did-key: expected cargo-fuzz 0.13.2, found %s\n' "$runner_version" >&2
  exit 1
fi

tmp_root=$(mktemp -d "${TMPDIR:-/tmp}/identus-did-key-fuzz.XXXXXX")
trap 'rm -rf "$tmp_root"' EXIT
cp -R "$repo_root/fuzz/corpus/did_key" "$tmp_root/did_key"
mkdir -p "$repo_root/fuzz/artifacts/did_key"

common=(-max_len=144 -timeout=5 -rss_limit_mb=1024 -reload=0 -print_final_stats=1)
case "$mode" in
  replay) limits=(-runs=0 -seed=424242) ;;
  smoke) limits=(-runs=4096 -seed=424242) ;;
  soak) limits=(-max_total_time=300) ;;
esac

started=$SECONDS
cargo fuzz run did_key "$tmp_root/did_key" --sanitizer address -- "${common[@]}" "${limits[@]}"
elapsed=$((SECONDS - started))
printf 'fuzz-did-key: mode=%s target=did_key elapsed_seconds=%d\n' "$mode" "$elapsed"
