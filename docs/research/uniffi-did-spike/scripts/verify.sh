#!/usr/bin/env bash

set -euo pipefail

spike_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
library="$spike_root/target/release/libidentus_uniffi_did_spike.dylib"
generated_a="$spike_root/target/uniffi-generated-a"
generated_b="$spike_root/target/uniffi-generated-b"

cargo test --locked --manifest-path "$spike_root/Cargo.toml"
cargo clippy --locked --manifest-path "$spike_root/Cargo.toml" --all-targets -- -D warnings
cargo build --locked --manifest-path "$spike_root/Cargo.toml" --release

for output in "$generated_a" "$generated_b"; do
    cargo run --locked --manifest-path "$spike_root/Cargo.toml" \
        --features bindgen-cli --bin uniffi-bindgen -- \
        generate --library "$library" --config "$spike_root/uniffi-global.toml" \
        --language swift --language kotlin --out-dir "$output" --no-format
done

diff -ru "$generated_a" "$generated_b"
diff -u "$spike_root/api/swift-api.txt" \
    <(awk -f "$spike_root/scripts/snapshot-swift-api.awk" "$generated_a/IdentusDidSpike.swift")
diff -u "$spike_root/api/kotlin-api.txt" \
    <(awk -f "$spike_root/scripts/snapshot-kotlin-api.awk" \
        "$generated_a/org/hyperledger/identus/spike/identus_uniffi_did_spike.kt")

swiftc "$generated_a/IdentusDidSpike.swift" "$spike_root/swift/Smoke.swift" \
    -Xcc "-fmodule-map-file=$generated_a/IdentusDidSpikeFFI.modulemap" \
    -I "$generated_a" -L "$spike_root/target/release" \
    -lidentus_uniffi_did_spike -o "$spike_root/target/swift-smoke"
DYLD_LIBRARY_PATH="$spike_root/target/release" "$spike_root/target/swift-smoke"

java_home=$(/usr/libexec/java_home -v 17 -a arm64)
JAVA_HOME="$java_home" gradle --no-daemon --console=plain -p "$spike_root/kotlin" run

printf 'uniffi-did-spike: verification passed\n'
