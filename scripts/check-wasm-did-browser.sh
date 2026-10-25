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
test "$(wasm-bindgen --version)" = "wasm-bindgen 0.2.121"
cargo deny --manifest-path tests/wasm-did-browser/Cargo.toml check
cargo audit --file tests/wasm-did-browser/Cargo.lock --deny warnings

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
  local browser_binary
  local capabilities="$evidence_root/webdriver-$browser.json"
  local driver_arg
  local driver_binary

  case "$browser" in
    chrome)
      browser_binary="$(command -v chromium || command -v chromium-browser || command -v google-chrome || true)"
      if [[ -z "$browser_binary" && -x "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" ]]; then
        browser_binary="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"
      fi
      test -n "$browser_binary"
      driver_binary="${CHROMEDRIVER:-$(command -v chromedriver)}"
      driver_arg="--chromedriver"
      test -x "$driver_binary"
      printf '{"goog:chromeOptions":{"binary":"%s"}}\n' "$browser_binary" > "$capabilities"
      "$browser_binary" --version
      "$driver_binary" --version
      ;;
    firefox)
      browser_binary="$(command -v firefox || true)"
      if [[ -z "$browser_binary" && -x "/Applications/Firefox.app/Contents/MacOS/firefox" ]]; then
        browser_binary="/Applications/Firefox.app/Contents/MacOS/firefox"
      fi
      test -n "$browser_binary"
      driver_binary="${GECKODRIVER:-$(command -v geckodriver)}"
      driver_arg="--geckodriver"
      test -x "$driver_binary"
      printf '{"moz:firefoxOptions":{"binary":"%s"}}\n' "$browser_binary" > "$capabilities"
      "$browser_binary" --version
      "$driver_binary" --version | sed -n '1p'
      ;;
  esac

  WASM_BINDGEN_TEST_WEBDRIVER_JSON="$capabilities" \
    wasm-pack test --headless "--$browser" "$driver_arg" "$driver_binary" \
      tests/wasm-did-browser --locked
}

case "$mode" in
  --chrome) run_browser chrome ;;
  --firefox) run_browser firefox ;;
  --all-browsers)
    run_browser chrome
    run_browser firefox
    ;;
esac
