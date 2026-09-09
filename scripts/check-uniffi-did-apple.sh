#!/usr/bin/env bash

set -euo pipefail

repository_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
crate_root="$repository_root/crates/uniffi-did"
tool_manifest="$repository_root/tools/uniffi-bindgen/Cargo.toml"
template_root="$repository_root/tests/uniffi-did-apple/package-template"
evidence_root="$repository_root/target/uniffi-did-apple"
tool_target="$repository_root/target/uniffi-bindgen-tool"
deployment_target=15.0

fail() {
    printf 'uniffi-did-apple: %s\n' "$1" >&2
    exit 1
}

if [[ $(uname -s) != "Darwin" ]]; then
    fail "macOS is required for the Apple package gate"
fi

for command in ar awk cargo cmp cp diff du grep mkdir python3 rustc stat swift tr xcodebuild xcrun; do
    command -v "$command" >/dev/null || fail "required command is unavailable: $command"
done

[[ $evidence_root == "$repository_root"/target/* ]] || fail "unsafe evidence path"
rm -rf "$evidence_root"
mkdir -p "$evidence_root"

rust_version=$(rustc --version)
llvm_nm=$(dirname "$(rustc --print target-libdir)")/bin/llvm-nm
[[ -x $llvm_nm ]] || fail "the pinned Rust llvm-nm is unavailable"
xcode_version=$(xcodebuild -version | tr '\n' ' ')
swift_version=$(swift --version | awk 'NR == 1 { first = $0 } END { print first }')
iphoneos_sdk=$(xcrun --sdk iphoneos --show-sdk-version)
simulator_sdk=$(xcrun --sdk iphonesimulator --show-sdk-version)
xcode_clang=$(xcrun --find clang)

build_package() {
    local label=$1
    local build_root="$evidence_root/build-$label"
    local generated_root="$evidence_root/generated-$label"
    local package_root="$evidence_root/package-$label"
    local device_library="$build_root/aarch64-apple-ios/release/libidentus_uniffi_did.a"
    local simulator_library="$build_root/aarch64-apple-ios-sim/release/libidentus_uniffi_did.a"
    local headers_root="$generated_root/headers"

    CARGO_INCREMENTAL=0 \
    CARGO_TARGET_DIR="$build_root" \
    CARGO_TARGET_AARCH64_APPLE_IOS_LINKER="$xcode_clang" \
    IPHONEOS_DEPLOYMENT_TARGET="$deployment_target" \
    cargo build --locked --release --package identus-uniffi-did \
        --target aarch64-apple-ios
    CARGO_INCREMENTAL=0 \
    CARGO_TARGET_DIR="$build_root" \
    CARGO_TARGET_AARCH64_APPLE_IOS_SIM_LINKER="$xcode_clang" \
    IPHONEOS_DEPLOYMENT_TARGET="$deployment_target" \
    cargo build --locked --release --package identus-uniffi-did \
        --target aarch64-apple-ios-sim

    mkdir -p "$headers_root" "$package_root/Artifacts" \
        "$package_root/Sources/IdentusDid"

    CARGO_TARGET_DIR="$tool_target" \
        cargo run --quiet --locked --manifest-path "$tool_manifest" \
        --bin uniffi-bindgen-swift -- --swift-sources \
        --config "$crate_root/uniffi-global.toml" \
        "$device_library" "$generated_root"
    CARGO_TARGET_DIR="$tool_target" \
        cargo run --quiet --locked --manifest-path "$tool_manifest" \
        --bin uniffi-bindgen-swift -- --headers \
        --config "$crate_root/uniffi-global.toml" \
        "$device_library" "$headers_root"
    CARGO_TARGET_DIR="$tool_target" \
        cargo run --quiet --locked --manifest-path "$tool_manifest" \
        --bin uniffi-bindgen-swift -- --modulemap --xcframework \
        --module-name IdentusDidFFI --modulemap-filename module.modulemap \
        --config "$crate_root/uniffi-global.toml" \
        "$device_library" "$headers_root"

    # UniFFI 0.32 emits a framework module and explicit builtin imports. That
    # shape does not import as a static-library SwiftPM binary target on Xcode
    # 26.4. Match the entire known template before applying the bounded adapter
    # so generator drift cannot be hidden.
    python3 - "$headers_root/module.modulemap" <<'PY'
import pathlib
import sys

path = pathlib.Path(sys.argv[1])
actual = path.read_text(encoding="utf-8").splitlines()
expected = [
    "framework module IdentusDidFFI {",
    '    header "IdentusDidFFI.h"',
    "    export *",
    '    use "Darwin"',
    '    use "_Builtin_stdbool"',
    '    use "_Builtin_stdint"',
    "}",
]
if actual != expected:
    raise SystemExit("unexpected UniFFI 0.32 module-map output")
normalized = [
    "module IdentusDidFFI {",
    '    header "IdentusDidFFI.h"',
    "    export *",
    "}",
]
path.write_text("\n".join(normalized) + "\n", encoding="utf-8")
PY

    cp -R "$template_root"/. "$package_root"
    cp "$generated_root/IdentusDid.swift" \
        "$package_root/Sources/IdentusDid/IdentusDid.swift"
    xcodebuild -create-xcframework \
        -library "$device_library" -headers "$headers_root" \
        -library "$simulator_library" -headers "$headers_root" \
        -output "$package_root/Artifacts/IdentusDidFFI.xcframework"

    # Xcode does not promise a stable AvailableLibraries order. Preserve its
    # values while normalizing that set and plist encoding before comparison.
    python3 - "$package_root/Artifacts/IdentusDidFFI.xcframework/Info.plist" <<'PY'
import plistlib
import sys

path = sys.argv[1]
with open(path, "rb") as source:
    document = plistlib.load(source)
document["AvailableLibraries"].sort(key=lambda item: item["LibraryIdentifier"])
with open(path, "wb") as destination:
    plistlib.dump(document, destination, fmt=plistlib.FMT_XML, sort_keys=True)
PY
}

build_package a
build_package b

device_a="$evidence_root/build-a/aarch64-apple-ios/release/libidentus_uniffi_did.a"
simulator_a="$evidence_root/build-a/aarch64-apple-ios-sim/release/libidentus_uniffi_did.a"
device_b="$evidence_root/build-b/aarch64-apple-ios/release/libidentus_uniffi_did.a"
simulator_b="$evidence_root/build-b/aarch64-apple-ios-sim/release/libidentus_uniffi_did.a"
package_a="$evidence_root/package-a"
package_b="$evidence_root/package-b"
xcframework="$package_a/Artifacts/IdentusDidFFI.xcframework"

cmp "$device_a" "$device_b"
cmp "$simulator_a" "$simulator_b"
diff -ru "$evidence_root/generated-a" "$evidence_root/generated-b"
diff -ru "$package_a" "$package_b"

[[ $(xcrun lipo -archs "$device_a") == "arm64" ]] || fail "device archive is not arm64"
[[ $(xcrun lipo -archs "$simulator_a") == "arm64" ]] || fail "Simulator archive is not arm64"

inspect_minimum_platform() {
    local archive=$1
    local expected_platform=$2
    local inspect_root=$3
    local member
    local build_record

    member=$(ar -t "$archive" | \
        awk '/^identus_uniffi_did.*\.o$/ && first == "" { first = $0 } END { print first }')
    [[ -n $member ]] || fail "SDK object is absent from $archive"
    mkdir -p "$inspect_root"
    (
        cd "$inspect_root"
        ar -x "$archive" "$member"
    )
    build_record=$(xcrun vtool -show-build "$inspect_root/$member")
    grep -q "platform $expected_platform" <<<"$build_record" || \
        fail "$archive has the wrong Apple platform"
    grep -q "minos $deployment_target" <<<"$build_record" || \
        fail "$archive has the wrong minimum iOS version"
}

inspect_minimum_platform "$device_a" IOS "$evidence_root/inspect-device"
inspect_minimum_platform "$simulator_a" IOSSIMULATOR "$evidence_root/inspect-simulator"

python3 - "$xcframework/Info.plist" <<'PY'
import plistlib
import sys

with open(sys.argv[1], "rb") as source:
    document = plistlib.load(source)
libraries = {item["LibraryIdentifier"]: item for item in document["AvailableLibraries"]}
if set(libraries) != {"ios-arm64", "ios-arm64-simulator"}:
    raise SystemExit("unexpected XCFramework library identifiers")
device = libraries["ios-arm64"]
simulator = libraries["ios-arm64-simulator"]
if device["SupportedArchitectures"] != ["arm64"] or device["SupportedPlatform"] != "ios":
    raise SystemExit("unexpected iOS device variant")
if "SupportedPlatformVariant" in device:
    raise SystemExit("device variant is incorrectly marked as a Simulator")
if simulator["SupportedArchitectures"] != ["arm64"]:
    raise SystemExit("unexpected Simulator architecture")
if simulator["SupportedPlatform"] != "ios" or simulator["SupportedPlatformVariant"] != "simulator":
    raise SystemExit("unexpected iOS Simulator variant")
for item in libraries.values():
    if item["LibraryPath"] != "libidentus_uniffi_did.a" or item["HeadersPath"] != "Headers":
        raise SystemExit("unexpected XCFramework library or header path")
PY

for symbol in binding_api_version parse_did parse_did_url; do
    "$llvm_nm" --defined-only --extern-only "$device_a" 2>/dev/null | \
        grep "_uniffi_identus_uniffi_did_fn_func_${symbol}$" >/dev/null || \
        fail "required device symbol is absent: $symbol"
    "$llvm_nm" --defined-only --extern-only "$simulator_a" 2>/dev/null | \
        grep "_uniffi_identus_uniffi_did_fn_func_${symbol}$" >/dev/null || \
        fail "required Simulator symbol is absent: $symbol"
done

for variant in ios-arm64 ios-arm64-simulator; do
    module_map="$xcframework/$variant/Headers/module.modulemap"
    grep -q '^module IdentusDidFFI {$' "$module_map" || \
        fail "$variant module map has the wrong module"
    grep -q 'header "IdentusDidFFI.h"' "$module_map" || \
        fail "$variant module map has the wrong header"
    if grep -q '^framework module\|^[[:space:]]*use "' "$module_map"; then
        fail "$variant module map was not normalized for a static library"
    fi
done
cmp "$xcframework/ios-arm64/Headers/IdentusDidFFI.h" \
    "$xcframework/ios-arm64-simulator/Headers/IdentusDidFFI.h"
cmp "$xcframework/ios-arm64/Headers/module.modulemap" \
    "$xcframework/ios-arm64-simulator/Headers/module.modulemap"

if grep -R -a -F -q "$repository_root" "$package_a"; then
    fail "Apple package contains an absolute repository path"
fi

simulator_id=${IDENTUS_IOS_SIMULATOR_ID:-}
if [[ -z $simulator_id ]]; then
    simulator_id=$(xcrun simctl list devices available | \
        awk '/iPhone/ && first == "" && match($0, /[0-9A-F]{8}-[0-9A-F-]{27}/) { first = substr($0, RSTART, RLENGTH) } END { print first }')
fi
[[ -n $simulator_id ]] || fail "no available iPhone Simulator was found"
xcrun simctl list devices available -j >"$evidence_root/simulators.json"
simulator_record=$(python3 - "$evidence_root/simulators.json" "$simulator_id" <<'PY'
import json
import sys

with open(sys.argv[1], encoding="utf-8") as source:
    devices = json.load(source)["devices"]
for runtime, entries in devices.items():
    for device in entries:
        if device["udid"] == sys.argv[2]:
            print("|".join((device["name"], runtime, device["state"])))
            raise SystemExit(0)
raise SystemExit("selected Simulator disappeared from the available inventory")
PY
)
IFS='|' read -r simulator_name simulator_runtime simulator_state <<<"$simulator_record"

(
    cd "$package_a"
    xcodebuild test -quiet \
        -scheme IdentusDid \
        -destination "platform=iOS Simulator,id=$simulator_id,arch=arm64" \
        -derivedDataPath "$evidence_root/derived-data" \
        CODE_SIGNING_ALLOWED=NO
)

device_bytes=$(stat -f '%z' "$device_a")
simulator_bytes=$(stat -f '%z' "$simulator_a")
package_bytes=$(du -sk "$package_a" | awk '{ print $1 * 1024 }')

cat >"$evidence_root/receipt.txt" <<EOF
rust=$rust_version
xcode=$xcode_version
swift=$swift_version
iphoneos_sdk=$iphoneos_sdk
iphonesimulator_sdk=$simulator_sdk
deployment_target=$deployment_target
device_arch=arm64
simulator_arch=arm64
simulator_id=$simulator_id
simulator_name=$simulator_name
simulator_runtime=$simulator_runtime
simulator_initial_state=$simulator_state
device_archive_bytes=$device_bytes
simulator_archive_bytes=$simulator_bytes
package_bytes=$package_bytes
deterministic=true
simulator_test=true
EOF

printf 'uniffi-did-apple: verification passed\n'
cat "$evidence_root/receipt.txt"
