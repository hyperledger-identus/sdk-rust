#!/usr/bin/env bash
# SPDX-License-Identifier: Apache-2.0
set -euo pipefail

mode="${1:-all}"
case "$mode" in
  package-types | browser | nodejs | react-web | all | differential | benchmark) ;;
  *)
    echo "usage: $0 [package-types|browser|nodejs|react-web|all|differential|benchmark]" >&2
    exit 2
    ;;
esac

root="$(git rev-parse --show-toplevel)"
cd "$root"
head_sha="${SDK_TS_CRYPTO_HEAD_SHA:-${SDK_TS_DID_HEAD_SHA:-$(git rev-parse HEAD)}}"
receipt_dir="${SDK_TS_CRYPTO_RECEIPT_DIR:-${SDK_TS_DID_RECEIPT_DIR:-$root/target/sdk-ts-crypto-receipts}}"
package_root="$root/packages/identus-sdk"
scratch="$(mktemp -d "$root/target/.sdk-ts-crypto-target.XXXXXX")"
started="$SECONDS"
trap 'rm -rf "$scratch"' EXIT

for command in node npm wasm-pack wasm-bindgen; do
  command -v "$command" >/dev/null
done
[[ "$head_sha" =~ ^[0-9a-f]{40}$ ]]
[[ "$(node --version)" == v24.* ]]
[[ "$(wasm-pack --version)" == "wasm-pack 0.15.0" ]]
[[ "$(wasm-bindgen --version)" == "wasm-bindgen 0.2.121" ]]

artifact_dir="$scratch/artifacts"
./scripts/build-sdk-ts-crypto-artifacts.sh --output "$artifact_dir"
(
  cd "$package_root"
  npm ci --ignore-scripts
  SDK_TS_CRYPTO_ARTIFACTS="$artifact_dir" npm run sync-crypto-artifacts
  npm run build
)

hash_artifacts() {
  (
    cd "$artifact_dir"
    find . -type f -print0 \
      | LC_ALL=C sort -z \
      | xargs -0 shasum -a 256 \
      | shasum -a 256 \
      | awk '{print $1}'
  )
}

write_receipt() {
  local target="$1"
  local receipt_target="crypto-$target"
  node scripts/ci/write-target-receipt.mjs \
    --head "$head_sha" \
    --target "$receipt_target" \
    --applicability selected \
    --status success \
    --artifact-sha256 "$(hash_artifacts)" \
    --duration-seconds "$((SECONDS - started))" \
    --output "$receipt_dir/$receipt_target.json"
}

run_package_types() {
  (
    cd "$package_root"
    npm run typecheck
    node --test tests/crypto-provider.test.mjs tests/crypto-runtime.test.mjs tests/public-api.test.mjs
    node --experimental-test-coverage \
      --test-coverage-include=dist/internal/crypto-provider.js \
      --test-coverage-lines=80 \
      --test-coverage-branches=70 \
      --test tests/crypto-provider.test.mjs
    npm pack --dry-run --json >/dev/null
  )
  write_receipt package-types
}

run_browser() {
  (cd "$package_root" && npm run test:browser)
  write_receipt browser
}

run_nodejs() {
  (cd "$package_root" && node --test tests/crypto-runtime.test.mjs tests/node-runtime.test.mjs)
  write_receipt nodejs
}

run_react_web() {
  (cd "$package_root" && npm run test:react)
  write_receipt react-web
}

case "$mode" in
  package-types) run_package_types ;;
  browser) run_browser ;;
  nodejs) run_nodejs ;;
  react-web) run_react_web ;;
  all)
    run_package_types
    run_browser
    run_nodejs
    run_react_web
    ;;
  differential)
    cargo test --locked -p identus-wasm-crypto
    run_browser
    run_nodejs
    run_react_web
    ;;
  benchmark)
    (cd "$package_root" && node tests/crypto-measurements.mjs)
    printf 'crypto artifact sha256: %s\n' "$(hash_artifacts)"
    ;;
esac
