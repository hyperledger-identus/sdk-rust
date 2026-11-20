#!/usr/bin/env bash
# SPDX-License-Identifier: Apache-2.0
set -euo pipefail

output="target/sdk-ts-crypto-artifacts"
if [[ ${1:-} == "--output" ]]; then
  output=${2:?missing output directory}
elif [[ $# -ne 0 ]]; then
  echo "usage: $0 [--output DIR]" >&2
  exit 2
fi

root=$(git rev-parse --show-toplevel)
case "$output" in
  /*) ;;
  *) output="$root/$output" ;;
esac
scratch=$(mktemp -d "${TMPDIR:-/tmp}/identus-sdk-ts-crypto.XXXXXX")
trap 'rm -rf "$scratch"' EXIT

for target in web nodejs; do
  wasm-pack build --release --target "$target" --out-dir "$scratch/$target" \
    "$root/crates/wasm-crypto"
done
rm -f "$scratch/web/.gitignore" "$scratch/nodejs/.gitignore"

rm -rf "$output"
mkdir -p "$output"
cp -R "$scratch/web" "$output/web"
cp -R "$scratch/nodejs" "$output/nodejs"
printf '%s\n' "sdk-ts crypto artifacts: $output"
