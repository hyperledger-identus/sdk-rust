#!/usr/bin/env bash
# SPDX-License-Identifier: Apache-2.0
set -euo pipefail

mode="${1:-all}"
case "$mode" in
  package-types | browser | nodejs | react-web | react-native | all | differential | measurements) ;;
  *)
    echo "usage: $0 [package-types|browser|nodejs|react-web|react-native|all|differential|measurements]" >&2
    exit 2
    ;;
esac

root="$(git rev-parse --show-toplevel)"
cd "$root"
head_sha="${SDK_TS_DID_HEAD_SHA:-$(git rev-parse HEAD)}"
[[ "$head_sha" =~ ^[0-9a-f]{40}$ ]]
receipt_dir="${SDK_TS_DID_RECEIPT_DIR:-$root/target/sdk-ts-did-receipts}"
package_root="$root/packages/identus-sdk"
started="$SECONDS"
scratch=""

cleanup() {
  if [[ -n "$scratch" && -d "$scratch" ]]; then
    rm -rf "$scratch"
  fi
}
trap cleanup EXIT

hash_stdin() {
  if command -v sha256sum >/dev/null; then
    sha256sum | awk '{print $1}'
  else
    shasum -a 256 | awk '{print $1}'
  fi
}

write_receipt() {
  local target="$1"
  local applicability="$2"
  local status="$3"
  local artifact="$4"
  local elapsed="$((SECONDS - started))"
  node scripts/ci/write-target-receipt.mjs \
    --head "$head_sha" \
    --target "$target" \
    --applicability "$applicability" \
    --status "$status" \
    --artifact-sha256 "$artifact" \
    --duration-seconds "$elapsed" \
    --output "$receipt_dir/$target.json"
}

if [[ "$mode" == "react-native" ]]; then
  # ADR 0163 owns the later native New Architecture/TurboModule package. This
  # structural receipt deliberately installs and executes no native toolchain.
  write_receipt react-native inapplicable inapplicable none
  exit 0
fi

for command in node npm wasm-pack wasm-bindgen; do
  command -v "$command" >/dev/null
done
[[ "$(node --version)" == v24.* ]]
[[ "$(wasm-pack --version)" == "wasm-pack 0.15.0" ]]
[[ "$(wasm-bindgen --version)" == "wasm-bindgen 0.2.121" ]]

mkdir -p "$root/target"
scratch="$(mktemp -d "$root/target/.sdk-ts-did-target.XXXXXX")"
artifact_dir="$scratch/artifacts"
./scripts/build-sdk-ts-did-artifacts.sh --output "$artifact_dir"
node scripts/assemble-sdk-ts-did-artifacts.mjs --verify "$artifact_dir" >/dev/null

(
  cd "$package_root"
  npm ci --ignore-scripts
  SDK_TS_DID_ARTIFACTS="$artifact_dir" npm run sync-artifacts
  npm run build
)

assembly_sha="$(node -e 'const fs=require("node:fs"); const p=JSON.parse(fs.readFileSync(process.argv[1], "utf8")); process.stdout.write(p.assemblySha256)' "$artifact_dir/manifest.json")"

run_package_types() {
  node scripts/check-sdk-ts-did-boundaries.mjs
  (
    cd "$package_root"
    npm run typecheck
    node --test tests/provider.test.mjs tests/public-api.test.mjs
    npm pack --dry-run --json >/dev/null
  )
  local digest
  digest="$({
    printf 'package-types\0'
    cat "$package_root/package-lock.json"
    cat "$package_root/api/public-api.d.ts"
    cat "$package_root/api/public-exports.txt"
    cat "$package_root/api/browser-api.d.ts"
    cat "$package_root/api/node-api.d.ts"
    cat "$package_root/api/react-api.d.ts"
  } | hash_stdin)"
  write_receipt package-types selected success "$digest"
}

run_browser() {
  (cd "$package_root" && npm run test:browser)
  local digest
  digest="$(printf 'browser:%s' "$assembly_sha" | hash_stdin)"
  write_receipt browser selected success "$digest"
}

run_nodejs() {
  (cd "$package_root" && npm run test:node)
  local digest
  digest="$(printf 'nodejs:%s' "$assembly_sha" | hash_stdin)"
  write_receipt nodejs selected success "$digest"
}

run_react_web() {
  (cd "$package_root" && npm run test:react)
  local digest
  digest="$(printf 'react-web:%s' "$assembly_sha" | hash_stdin)"
  write_receipt react-web selected success "$digest"
}

case "$mode" in
  package-types) run_package_types ;;
  browser) run_browser ;;
  nodejs) run_nodejs ;;
  react-web) run_react_web ;;
  measurements)
    (cd "$package_root" && npm run test:measurements)
    ;;
  all | differential)
    run_package_types
    run_browser
    run_nodejs
    run_react_web
    write_receipt react-native inapplicable inapplicable none
    ;;
esac
