#!/usr/bin/env bash

set -euo pipefail

repo_root=$(git rev-parse --show-toplevel 2>/dev/null || pwd)
mode=${1:-smoke}
target=${2:-direct_post_response}

case "$mode" in
  replay | smoke | soak) ;;
  *)
    printf 'Usage: %s {replay|smoke|soak} [direct_post_response|encrypted_direct_post_response|request_object_security]\n' "$0" >&2
    exit 2
    ;;
esac
if [[ "$target" != "direct_post_response" && "$target" != "encrypted_direct_post_response" && "$target" != "request_object_security" ]]; then
  printf 'fuzz-oid4vp: unsupported target: %s\n' "$target" >&2
  exit 2
fi

if ! cargo fuzz --version >/dev/null 2>&1; then
  nix_cmd=$(command -v nix || true)
  if [[ -z "$nix_cmd" && -x /nix/var/nix/profiles/default/bin/nix ]]; then
    nix_cmd=/nix/var/nix/profiles/default/bin/nix
  fi
  if [[ -z "$nix_cmd" ]]; then
    printf 'fuzz-oid4vp: cargo-fuzz is unavailable; enter the Nix fuzz shell\n' >&2
    exit 1
  fi
  exec "$nix_cmd" develop "$repo_root#fuzz" --command "$repo_root/scripts/fuzz-oid4vp.sh" "$mode" "$target"
fi

runner_version=$(cargo fuzz --version)
if [[ "$runner_version" != "cargo-fuzz 0.13.2" ]]; then
  printf 'fuzz-oid4vp: expected cargo-fuzz 0.13.2, found %s\n' "$runner_version" >&2
  exit 1
fi

tmp_root=$(mktemp -d "${TMPDIR:-/tmp}/identus-oid4vp-fuzz.XXXXXX")
trap 'rm -rf "$tmp_root"' EXIT
cp -R "$repo_root/fuzz/corpus/$target" "$tmp_root/$target"
mkdir -p "$repo_root/fuzz/artifacts/$target"

max_len=65536
if [[ "$target" == "request_object_security" || "$target" == "direct_post_response" ]]; then
  max_len=131072
fi
common=(
  -max_len="$max_len"
  -timeout=5
  -rss_limit_mb=1024
  -reload=0
  -print_final_stats=1
)
case "$mode" in
  replay) limits=(-runs=0 -seed=487487) ;;
  smoke) limits=(-runs=4096 -seed=487487) ;;
  soak) limits=(-max_total_time=300) ;;
esac

started=$SECONDS
cargo fuzz run "$target" "$tmp_root/$target" --sanitizer address -- "${common[@]}" "${limits[@]}"
elapsed=$((SECONDS - started))
printf 'fuzz-oid4vp: mode=%s target=%s elapsed_seconds=%d\n' "$mode" "$target" "$elapsed"
