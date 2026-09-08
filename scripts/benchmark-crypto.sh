#!/usr/bin/env bash
set -euo pipefail

repository_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
output_path=""
samples="20"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --output)
      [[ $# -ge 2 ]] || { printf 'crypto-baseline: --output requires a path\n' >&2; exit 2; }
      output_path="$2"
      shift 2
      ;;
    --samples)
      [[ $# -ge 2 ]] || { printf 'crypto-baseline: --samples requires a value\n' >&2; exit 2; }
      samples="$2"
      shift 2
      ;;
    *)
      printf 'crypto-baseline: unknown argument: %s\n' "$1" >&2
      exit 2
      ;;
  esac
done

[[ "$samples" =~ ^[0-9]+$ ]] && (( samples >= 20 )) || {
  printf 'crypto-baseline: --samples must be an integer of at least 20\n' >&2
  exit 2
}

revision="${GITHUB_SHA:-$(git -C "$repository_root" rev-parse HEAD)}"
compiler="$(rustc --version --verbose | tr '\n' ' ' | sed 's/[[:space:]]*$//')"
if command -v lscpu >/dev/null 2>&1; then
  cpu="$(lscpu | sed -n 's/^Model name:[[:space:]]*//p' | head -n 1)"
elif command -v sysctl >/dev/null 2>&1; then
  cpu="$(sysctl -n machdep.cpu.brand_string 2>/dev/null || uname -m)"
else
  cpu="$(uname -m)"
fi
environment="${RUNNER_ENVIRONMENT:-${RUNNER_OS:-local}}"

temporary_directory="$(mktemp -d)"
trap 'rm -rf "$temporary_directory"' EXIT
temporary_artifact="$temporary_directory/crypto-baseline.json"

(
  cd "$repository_root"
  IDENTUS_BENCH_REVISION="$revision" \
  IDENTUS_BENCH_COMPILER="$compiler" \
  IDENTUS_BENCH_CPU="${cpu:-unknown}" \
  IDENTUS_BENCH_ENVIRONMENT="$environment" \
    cargo run --quiet --release -p identus-crypto --example crypto_baseline -- --samples "$samples" \
      > "$temporary_artifact"
)

"$repository_root/scripts/check-crypto-benchmark.py" "$temporary_artifact"

if [[ -n "$output_path" ]]; then
  destination="$repository_root/$output_path"
  if [[ "$output_path" = /* ]]; then
    destination="$output_path"
  fi
  mkdir -p "$(dirname "$destination")"
  staging="$destination.tmp.$$"
  cp "$temporary_artifact" "$staging"
  mv "$staging" "$destination"
  printf 'crypto-baseline: wrote %s\n' "$destination"
else
  cat "$temporary_artifact"
fi
