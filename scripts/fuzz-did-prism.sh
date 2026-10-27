#!/usr/bin/env bash

set -euo pipefail

repo_root=$(git rev-parse --show-toplevel 2>/dev/null || pwd)
mode=${1:-smoke}
selection=${2:-all}

usage() {
  printf 'Usage: %s {replay|smoke|soak} {prism_operation|prism_did|all}\n' "$0" >&2
}

case "$mode" in
  replay | smoke | soak) ;;
  *)
    usage
    exit 2
    ;;
esac

case "$selection" in
  prism_operation) targets=(prism_operation) ;;
  prism_did) targets=(prism_did) ;;
  all) targets=(prism_operation prism_did) ;;
  *)
    usage
    exit 2
    ;;
esac

if ! cargo fuzz --version >/dev/null 2>&1; then
  nix_cmd=$(command -v nix || true)
  if [[ -z "$nix_cmd" && -x /nix/var/nix/profiles/default/bin/nix ]]; then
    nix_cmd=/nix/var/nix/profiles/default/bin/nix
  fi
  if [[ -z "$nix_cmd" ]]; then
    printf 'fuzz-did-prism: cargo-fuzz is unavailable; enter the Nix fuzz shell\n' >&2
    exit 1
  fi
  exec "$nix_cmd" develop "$repo_root#fuzz" --command "$repo_root/scripts/fuzz-did-prism.sh" "$mode" "$selection"
fi

runner_version=$(cargo fuzz --version)
if [[ "$runner_version" != "cargo-fuzz 0.13.2" ]]; then
  printf 'fuzz-did-prism: expected cargo-fuzz 0.13.2, found %s\n' "$runner_version" >&2
  exit 1
fi

tmp_root=$(mktemp -d "${TMPDIR:-/tmp}/identus-prism-did-fuzz.XXXXXX")
trap 'rm -rf "$tmp_root"' EXIT

for target in "${targets[@]}"; do
  source_corpus="$repo_root/fuzz/corpus/$target"
  run_corpus="$tmp_root/$target"
  artifact_dir="$repo_root/fuzz/artifacts/$target"
  dictionary="$repo_root/fuzz/dictionaries/$target.dict"
  cp -R "$source_corpus" "$run_corpus"
  mkdir -p "$artifact_dir"

  common=(
    -max_len=139264
    -timeout=5
    -rss_limit_mb=1024
    -reload=0
    -print_final_stats=1
    "-dict=$dictionary"
  )

  started=$SECONDS
  case "$mode" in
    replay) limits=(-runs=0 -seed=424242) ;;
    smoke) limits=(-runs=4096 -seed=424242) ;;
    soak) limits=(-max_total_time=300) ;;
  esac

  cargo fuzz run "$target" "$run_corpus" --sanitizer address -- "${common[@]}" "${limits[@]}"
  elapsed=$((SECONDS - started))
  printf 'fuzz-did-prism: mode=%s target=%s elapsed_seconds=%d\n' "$mode" "$target" "$elapsed"
done
