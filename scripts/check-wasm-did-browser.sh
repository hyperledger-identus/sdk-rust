#!/usr/bin/env bash
set -euo pipefail

mode="${1:---package-only}"
case "$mode" in
  --package-only | --chrome | --firefox | --all-browsers) ;;
  *)
    echo "usage: $0 [--package-only|--chrome|--firefox|--all-browsers]" >&2
    exit 2
    ;;
esac

command -v wasm-pack >/dev/null
test "$(wasm-pack --version)" = "wasm-pack 0.15.0"

evidence_root="$(mktemp -d "${TMPDIR:-/tmp}/identus-wasm-did.XXXXXX")"
trap 'rm -rf "$evidence_root"' EXIT
first="$evidence_root/first"
second="$evidence_root/second"

wasm-pack build --release --target web --out-dir "$first" \
  --out-name identus_wasm_did crates/wasm-did --locked
wasm-pack build --release --target web --out-dir "$second" \
  --out-name identus_wasm_did crates/wasm-did --locked

diff -ruN "$first" "$second"
cmp crates/wasm-did/api/typescript-api.d.ts "$first/identus_wasm_did.d.ts"

for artifact in \
  identus_wasm_did.js \
  identus_wasm_did.d.ts \
  identus_wasm_did_bg.wasm \
  identus_wasm_did_bg.wasm.d.ts \
  package.json; do
  test -f "$first/$artifact"
  bytes="$(wc -c < "$first/$artifact" | tr -d ' ')"
  if command -v sha256sum >/dev/null; then
    digest="$(sha256sum "$first/$artifact" | awk '{print $1}')"
  else
    digest="$(shasum -a 256 "$first/$artifact" | awk '{print $1}')"
  fi
  echo "$artifact bytes=$bytes sha256=$digest"
done

run_browser() {
  local browser="$1"
  wasm-pack test --headless "--$browser" crates/wasm-did --locked
}

case "$mode" in
  --chrome) run_browser chrome ;;
  --firefox) run_browser firefox ;;
  --all-browsers)
    run_browser chrome
    run_browser firefox
    ;;
esac
