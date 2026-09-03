#!/usr/bin/env bash

set -euo pipefail

repo_root=$(git rev-parse --show-toplevel 2>/dev/null || pwd)
mode=${1:-smoke}
selection=${2:-all}

usage() {
  printf 'Usage: %s {replay|smoke|soak} {public_jwk|public_cose|all}\n' "$0" >&2
}

case "$mode" in
  replay) runs=0 ;;
  smoke) runs=4096 ;;
  soak) runs=-1 ;;
  *) usage; exit 2 ;;
esac

case "$selection" in
  public_jwk|public_cose) targets=("$selection") ;;
  all) targets=(public_jwk public_cose) ;;
  *) usage; exit 2 ;;
esac

if ! cargo fuzz --version >/dev/null 2>&1; then
  nix_cmd=$(command -v nix || true)
  if [[ -z "$nix_cmd" && -x /nix/var/nix/profiles/default/bin/nix ]]; then
    nix_cmd=/nix/var/nix/profiles/default/bin/nix
  fi
  if [[ -z "$nix_cmd" ]]; then
    printf 'fuzz-crypto: cargo-fuzz is unavailable; enter the Nix shell\n' >&2
    exit 1
  fi
  exec "$nix_cmd" develop "$repo_root" --command "$repo_root/scripts/fuzz-crypto.sh" "$mode" "$selection"
fi

actual_version=$(cargo fuzz --version)
if [[ "$actual_version" != "cargo-fuzz 0.13.2" ]]; then
  printf 'fuzz-crypto: expected cargo-fuzz 0.13.2, found %s\n' "$actual_version" >&2
  exit 1
fi

campaign_dir=$(mktemp -d "${TMPDIR:-/tmp}/identus-crypto-fuzz.XXXXXX")
trap 'rm -rf "$campaign_dir"' EXIT

for fuzz_target in "${targets[@]}"; do
  started=$SECONDS
  corpus_dir="$campaign_dir/$fuzz_target"
  cp -R "$repo_root/fuzz/corpus/$fuzz_target" "$corpus_dir"
  common=(
    -max_len=8192
    -timeout=5
    -rss_limit_mb=1024
    -reload=0
    -workers=1
    -dict="$repo_root/fuzz/dictionaries/$fuzz_target.dict"
  )
  mkdir -p "$repo_root/fuzz/artifacts/$fuzz_target"

  case "$mode" in
    replay) campaign=(-runs=0 -seed=424242) ;;
    smoke) campaign=(-runs="$runs" -seed=424242) ;;
    soak) campaign=(-max_total_time=300) ;;
  esac

  cargo fuzz run "$fuzz_target" "$corpus_dir" --sanitizer address -- "${common[@]}" "${campaign[@]}"
  elapsed=$((SECONDS - started))
  printf 'fuzz-crypto: mode=%s target=%s elapsed_seconds=%d\n' "$mode" "$fuzz_target" "$elapsed"
done
