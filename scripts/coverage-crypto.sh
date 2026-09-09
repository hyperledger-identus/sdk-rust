#!/usr/bin/env bash
set -euo pipefail

repository_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
output_directory="${1:-artifacts/crypto-coverage}"
if [[ "$output_directory" != /* ]]; then
  output_directory="$repository_root/$output_directory"
fi

revision="${GITHUB_SHA:-$(git -C "$repository_root" rev-parse HEAD)}"
rust_version="$(rustc --version)"
tool_version="$(cargo llvm-cov --version)"
temporary_directory="$(mktemp -d)"
trap 'rm -rf "$temporary_directory"' EXIT

cd "$repository_root"
cargo llvm-cov clean --workspace
cargo llvm-cov --package identus-crypto --locked --no-report
cargo llvm-cov --package identus-crypto --locked --all-features --no-report
cargo llvm-cov --package identus-crypto --locked --no-default-features --no-report
cargo llvm-cov --package identus-crypto --locked --features kmp-compat --no-report
cargo llvm-cov report --json --summary-only \
  --output-path "$temporary_directory/llvm-summary.json"
cargo llvm-cov report --lcov --output-path "$temporary_directory/lcov.info"

"$repository_root/scripts/report-crypto-coverage.py" \
  "$temporary_directory/llvm-summary.json" \
  "$temporary_directory/normalized" \
  --repository-root "$repository_root" \
  --revision "$revision" \
  --rust-version "$rust_version" \
  --tool-version "$tool_version"

mkdir -p "$output_directory"
for artifact in summary.json summary.md; do
  cp "$temporary_directory/normalized/$artifact" "$output_directory/.$artifact.tmp.$$"
  mv "$output_directory/.$artifact.tmp.$$" "$output_directory/$artifact"
done
cp "$temporary_directory/lcov.info" "$output_directory/.lcov.info.tmp.$$"
mv "$output_directory/.lcov.info.tmp.$$" "$output_directory/lcov.info"
printf 'crypto-coverage: wrote %s\n' "$output_directory"
