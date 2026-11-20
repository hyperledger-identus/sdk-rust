#!/usr/bin/env bash
# SPDX-License-Identifier: Apache-2.0
set -euo pipefail

mode="${1:-all}"
case "$mode" in
  package-types | browser | nodejs | react-web | react-native | all | differential | adapter-fuzz | measurements) ;;
  *)
    echo "usage: $0 [package-types|browser|nodejs|react-web|react-native|all|differential|adapter-fuzz|measurements]" >&2
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

target_contract_digest() {
  local label="$1"
  shift
  {
    printf '%s\0%s\0' "$label" "$assembly_sha"
    for file in "$@"; do
      printf '%s\0' "$file"
      cat "$root/$file"
      printf '\0'
    done
  } | hash_stdin
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

write_adoption_receipt() {
  local target="$1"
  local route_instances="$2"
  local runtime_sources="$3"
  local sources="$root/packages/identus-sdk/tests/adoption-journey.mjs"
  local source
  IFS=',' read -r -a adoption_sources <<< "$runtime_sources"
  for source in "${adoption_sources[@]}"; do
    sources+=",$root/$source"
  done
  node scripts/ci/write-adoption-receipt.mjs \
    --head "$head_sha" \
    --target "$target" \
    --route-instances "$route_instances" \
    --sources "$sources" \
    --output "$receipt_dir/adoption-$target.json"
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
crypto_artifact_dir="$scratch/crypto-artifacts"
./scripts/build-sdk-ts-did-artifacts.sh --output "$artifact_dir"
node scripts/assemble-sdk-ts-did-artifacts.mjs --verify "$artifact_dir" >/dev/null
./scripts/build-sdk-ts-crypto-artifacts.sh --output "$crypto_artifact_dir"

(
  cd "$package_root"
  npm ci --ignore-scripts
  SDK_TS_DID_ARTIFACTS="$artifact_dir" npm run sync-artifacts
  SDK_TS_CRYPTO_ARTIFACTS="$crypto_artifact_dir" npm run sync-crypto-artifacts
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
  write_adoption_receipt browser 2 packages/identus-sdk/tests/browser-runtime.mjs
  local digest
  digest="$(target_contract_digest browser \
    docs/conformance/fixtures/did/v1/vectors.json \
    docs/conformance/fixtures/did-document/v1/vectors.json \
    docs/conformance/fixtures/did-resolution/v1/vectors.json \
    docs/conformance/fixtures/did-peer/v1/vectors.json \
    packages/identus-sdk/tests/vector-assertions.mjs \
    packages/identus-sdk/tests/document-vector-assertions.mjs \
    packages/identus-sdk/tests/resolution-vector-assertions.mjs \
    packages/identus-sdk/tests/peer-vector-assertions.mjs \
    packages/identus-sdk/tests/prism-vector-assertions.mjs \
    packages/identus-sdk/tests/adoption-journey.mjs \
    packages/identus-sdk/tests/browser-runtime.mjs)"
  write_receipt browser selected success "$digest"
}

run_nodejs() {
  (cd "$package_root" && npm run test:node)
  write_adoption_receipt nodejs 4 packages/identus-sdk/tests/node-runtime.test.mjs
  local digest
  digest="$(target_contract_digest nodejs \
    docs/conformance/fixtures/did/v1/vectors.json \
    docs/conformance/fixtures/did-document/v1/vectors.json \
    docs/conformance/fixtures/did-resolution/v1/vectors.json \
    docs/conformance/fixtures/did-peer/v1/vectors.json \
    packages/identus-sdk/tests/vector-assertions.mjs \
    packages/identus-sdk/tests/document-vector-assertions.mjs \
    packages/identus-sdk/tests/resolution-vector-assertions.mjs \
    packages/identus-sdk/tests/peer-vector-assertions.mjs \
    packages/identus-sdk/tests/prism-vector-assertions.mjs \
    packages/identus-sdk/tests/adoption-journey.mjs \
    packages/identus-sdk/tests/node-runtime.test.mjs)"
  write_receipt nodejs selected success "$digest"
}

run_react_web() {
  (cd "$package_root" && npm run test:react)
  write_adoption_receipt react-web 4 packages/identus-sdk/tests/react-runtime.test.mjs,packages/identus-sdk/tests/react-browser-fixture.tsx,packages/identus-sdk/tests/react-browser-runtime.mjs
  local digest
  digest="$(target_contract_digest react-web \
    docs/conformance/fixtures/did/v1/vectors.json \
    docs/conformance/fixtures/did-document/v1/vectors.json \
    docs/conformance/fixtures/did-resolution/v1/vectors.json \
    docs/conformance/fixtures/did-peer/v1/vectors.json \
    packages/identus-sdk/tests/vector-assertions.mjs \
    packages/identus-sdk/tests/document-vector-assertions.mjs \
    packages/identus-sdk/tests/resolution-vector-assertions.mjs \
    packages/identus-sdk/tests/peer-vector-assertions.mjs \
    packages/identus-sdk/tests/prism-vector-assertions.mjs \
    packages/identus-sdk/tests/adoption-journey.mjs \
    packages/identus-sdk/tests/react-runtime.test.mjs \
    packages/identus-sdk/tests/react-browser-fixture.tsx \
    packages/identus-sdk/tests/react-browser-runtime.mjs)"
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
  all)
    run_package_types
    run_browser
    run_nodejs
    run_react_web
    write_receipt react-native inapplicable inapplicable none
    ;;
  differential)
    cargo test --locked -p identus-did-peer --test conformance \
      portable_peer_did_packet_matches_both_profiles
    run_package_types
    run_browser
    run_nodejs
    run_react_web
    write_receipt react-native inapplicable inapplicable none
    ;;
  adapter-fuzz)
    (cd "$package_root" && node --test tests/peer-adapter-fuzz.test.mjs)
    ;;
esac
