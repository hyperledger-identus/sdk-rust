#!/usr/bin/env bash

set -euo pipefail

repository_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
crate_root="$repository_root/crates/uniffi-did"
tool_manifest="$repository_root/tools/uniffi-bindgen/Cargo.toml"
generated_root="$repository_root/target/uniffi-did-host"
generated_a="$generated_root/generated-a"
generated_b="$generated_root/generated-b"
library_dir="$repository_root/target/release"
library="$library_dir/libidentus_uniffi_did.dylib"

if [[ $(uname -s) != "Darwin" ]]; then
    printf 'uniffi-did-host: macOS is required for the Swift and Kotlin/JNA host gate\n' >&2
    exit 1
fi

for command in cargo swiftc gradle java awk diff; do
    command -v "$command" >/dev/null || {
        printf 'uniffi-did-host: required command is unavailable: %s\n' "$command" >&2
        exit 1
    }
done

cargo test --locked --package identus-uniffi-did
cargo clippy --locked --package identus-uniffi-did --all-targets -- -D warnings
cargo build --locked --package identus-uniffi-did --release

rm -rf "$generated_a" "$generated_b"
for output in "$generated_a" "$generated_b"; do
    CARGO_TARGET_DIR="$repository_root/target/uniffi-bindgen-tool" \
        cargo run --locked --manifest-path "$tool_manifest" -- \
        generate --library "$library" --config "$crate_root/uniffi-global.toml" \
        --language swift --language kotlin --out-dir "$output" --no-format
done

diff -ru "$generated_a" "$generated_b"
diff -u "$crate_root/api/swift-api.txt" \
    <(awk -f "$crate_root/scripts/snapshot-swift-api.awk" "$generated_a/IdentusDid.swift")
diff -u "$crate_root/api/kotlin-api.txt" \
    <(awk -f "$crate_root/scripts/snapshot-kotlin-api.awk" \
        "$generated_a/org/hyperledger/identus/did/identus_uniffi_did.kt")

swiftc "$generated_a/IdentusDid.swift" "$repository_root/tests/uniffi-did-host/swift/Smoke.swift" \
    -Xcc "-fmodule-map-file=$generated_a/IdentusDidFFI.modulemap" \
    -I "$generated_a" -L "$library_dir" -lidentus_uniffi_did \
    -o "$generated_root/swift-smoke"
DYLD_LIBRARY_PATH="$library_dir" "$generated_root/swift-smoke"

java_home=${IDENTUS_JAVA_HOME:-${JAVA_HOME:-}}
if [[ -z $java_home ]]; then
    java_home=$(/usr/libexec/java_home -v 17)
fi
IDENTUS_UNIFFI_GENERATED_DIR="$generated_a" \
IDENTUS_UNIFFI_LIBRARY_DIR="$library_dir" \
JAVA_HOME="$java_home" \
    gradle --no-daemon --console=plain -p "$repository_root/tests/uniffi-did-host/kotlin" run

printf 'uniffi-did-host: verification passed\n'
