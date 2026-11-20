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
cd "$root"
case "$output" in
  /*) ;;
  *) output="$root/$output" ;;
esac

command -v wasm-pack >/dev/null
command -v wasm-bindgen >/dev/null
[[ "$(wasm-pack --version)" == "wasm-pack 0.15.0" ]]
[[ "$(wasm-bindgen --version)" == "wasm-bindgen 0.2.121" ]]

output_parent=$(dirname "$output")
mkdir -p "$output_parent"
if [[ -e "$output" ]]; then
  echo "refusing to replace existing crypto artifact directory: $output" >&2
  exit 1
fi

scratch=$(mktemp -d "${TMPDIR:-/tmp}/identus-sdk-ts-crypto.XXXXXX")
staging=$(mktemp -d "$output_parent/.identus-sdk-ts-crypto.XXXXXX")
rmdir "$staging"
cleanup() {
  rm -rf "$scratch" "$staging"
}
trap cleanup EXIT

for build in first second; do
  for target in web nodejs; do
    wasm-pack build --release --target "$target" \
      --out-dir "$scratch/$build/$target" \
      "$root/crates/wasm-crypto" --locked
    rm -f "$scratch/$build/$target/.gitignore"
  done
done

diff -ruN "$scratch/first/web" "$scratch/second/web"
diff -ruN "$scratch/first/nodejs" "$scratch/second/nodejs"
mv "$scratch/first" "$staging"
mv "$staging" "$output"
printf '%s\n' "sdk-ts crypto artifacts: $output"
