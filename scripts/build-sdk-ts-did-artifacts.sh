#!/usr/bin/env bash
# SPDX-License-Identifier: Apache-2.0
set -euo pipefail

usage() {
  echo "usage: $0 [--output DIR]" >&2
}

output="target/sdk-ts-did-artifacts"
while [[ $# -gt 0 ]]; do
  case "$1" in
    --output)
      [[ $# -ge 2 ]] || { usage; exit 2; }
      output="$2"
      shift 2
      ;;
    *)
      usage
      exit 2
      ;;
  esac
done

root="$(git rev-parse --show-toplevel)"
cd "$root"
case "$output" in
  /*) ;;
  *) output="$root/$output" ;;
esac

command -v wasm-pack >/dev/null
command -v wasm-bindgen >/dev/null
command -v node >/dev/null
wasm_pack_version="$(wasm-pack --version)"
wasm_bindgen_version="$(wasm-bindgen --version)"
[[ "$wasm_pack_version" == "wasm-pack 0.15.0" ]]
[[ "$wasm_bindgen_version" == "wasm-bindgen 0.2.121" ]]

output_parent="$(dirname "$output")"
mkdir -p "$output_parent"
if [[ -e "$output" ]]; then
  echo "refusing to replace existing artifact directory: $output" >&2
  exit 1
fi

scratch="$(mktemp -d "${TMPDIR:-/tmp}/identus-sdk-ts-did-build.XXXXXX")"
staging_first="$(mktemp -d "$output_parent/.identus-sdk-ts-did-first.XXXXXX")"
staging_second="$(mktemp -d "$output_parent/.identus-sdk-ts-did-second.XXXXXX")"
rmdir "$staging_first" "$staging_second"
cleanup() {
  rm -rf "$scratch" "$staging_first" "$staging_second"
}
trap cleanup EXIT

for build in first second; do
  for target in web nodejs; do
    wasm-pack build --release --target "$target" \
      --out-dir "$scratch/$build/$target" \
      --out-name identus_wasm_did crates/wasm-did --locked
  done
done

diff -ruN "$scratch/first/web" "$scratch/second/web"
diff -ruN "$scratch/first/nodejs" "$scratch/second/nodejs"
cmp crates/wasm-did/api/typescript-api.d.ts "$scratch/first/web/identus_wasm_did.d.ts"

node scripts/assemble-sdk-ts-did-artifacts.mjs \
  --source "$scratch/first" \
  --output "$staging_first" \
  --wasm-pack "$wasm_pack_version" \
  --wasm-bindgen "$wasm_bindgen_version" > "$scratch/first-manifest.json"
node scripts/assemble-sdk-ts-did-artifacts.mjs \
  --source "$scratch/second" \
  --output "$staging_second" \
  --wasm-pack "$wasm_pack_version" \
  --wasm-bindgen "$wasm_bindgen_version" > "$scratch/second-manifest.json"

diff -ruN "$staging_first" "$staging_second"
cmp "$scratch/first-manifest.json" "$scratch/second-manifest.json"
mv "$staging_first" "$output"
node scripts/assemble-sdk-ts-did-artifacts.mjs --verify "$output" > "$scratch/final-manifest.json"
cmp "$scratch/first-manifest.json" "$scratch/final-manifest.json"
printf 'sdk-ts DID artifacts: %s\n' "$output"
cat "$output/manifest.json"
