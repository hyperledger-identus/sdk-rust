# Show available commands
default:
    @just --list --list-submodules

# Build the workspace
[group('sdk-rust')]
build:
    cargo build --all-features

# Run all tests
[group('sdk-rust')]
test:
    cargo test --all-features

# Run tests with code coverage (requires cargo-llvm-cov)
[group('sdk-rust')]
coverage:
    cargo llvm-cov test --all-features --lcov --output-path lcov.info
    cargo llvm-cov report
    echo "Coverage report: lcov.info (use 'cargo llvm-cov report --html' for HTML)"

# Generate HTML coverage report
[group('sdk-rust')]
coverage-html: coverage
    cargo llvm-cov report --html
    echo "HTML report saved to target/llvm-cov/html/index.html"

# Build WASM module with wasm-pack (target: web)
[group('sdk-rust')]
build-wasm:
    cd lib/identus-crypto-wasm && wasm-pack build --target web

# Build the UniFFI shared library and generate Kotlin bindings
[group('sdk-rust')]
build-uniffi:
    #!/usr/bin/env bash
    set -euo pipefail
    cargo build -p identus-crypto-uniffi
    mkdir -p lib/identus-crypto-uniffi/bindings/kotlin
    # Locate the built library (.so on Linux, .dylib on macOS, .dll on Windows)
    LIB_FILE=$(ls target/debug/libidentus_crypto_uniffi.{so,dylib,dll} 2>/dev/null | head -1) || true
    if [ -z "$LIB_FILE" ]; then
        echo "Error: libidentus_crypto_uniffi library not found in target/debug/"
        exit 1
    fi
    uniffi-bindgen generate --library "$LIB_FILE" \
      --language kotlin \
      --out-dir lib/identus-crypto-uniffi/bindings/kotlin/
    echo "✓ UniFFI library built and Kotlin bindings generated"

# Generate Kotlin UniFFI bindings from the existing build artifact
[group('sdk-rust')]
generate-kotlin-bindings:
    #!/usr/bin/env bash
    set -euo pipefail
    mkdir -p lib/identus-crypto-uniffi/bindings/kotlin
    LIB_FILE=$(ls target/debug/libidentus_crypto_uniffi.{so,dylib,dll} 2>/dev/null | head -1) || true
    if [ -z "$LIB_FILE" ]; then
        echo "Error: libidentus_crypto_uniffi library not found in target/debug/"
        echo "Run 'cargo build -p identus-crypto-uniffi' first"
        exit 1
    fi
    uniffi-bindgen generate --library "$LIB_FILE" \
      --language kotlin \
      --out-dir lib/identus-crypto-uniffi/bindings/kotlin/
    echo "✓ Kotlin bindings generated"

# Generate Swift UniFFI bindings from the existing build artifact
[group('sdk-rust')]
generate-swift-bindings:
    #!/usr/bin/env bash
    set -euo pipefail
    mkdir -p lib/identus-crypto-uniffi/bindings/swift
    LIB_FILE=$(ls target/debug/libidentus_crypto_uniffi.{so,dylib,dll} 2>/dev/null | head -1) || true
    if [ -z "$LIB_FILE" ]; then
        echo "Error: libidentus_crypto_uniffi library not found in target/debug/"
        echo "Run 'cargo build -p identus-crypto-uniffi' first"
        exit 1
    fi
    uniffi-bindgen generate --library "$LIB_FILE" \
      --language swift \
      --out-dir lib/identus-crypto-uniffi/bindings/swift/
    echo "✓ Swift bindings generated"

# Build the identus-crypto-uniffi .so for all 4 Android ABIs and copy Kotlin
# bindings into the Android example project.
# Depends on build-uniffi to generate Kotlin bindings first.
[group('sdk-rust')]
build-android-example: build-uniffi
    #!/usr/bin/env bash
    set -euo pipefail

    ANDROID_JNI="examples/android-app/app/src/main/jniLibs"
    BINDINGS_SRC="lib/identus-crypto-uniffi/bindings/kotlin/uniffi/identus_crypto_uniffi/identus_crypto_uniffi.kt"
    BINDINGS_DST="examples/android-app/app/src/main/java/uniffi/identus_crypto_uniffi/"

    echo "=== Building identus-crypto-uniffi for all 4 Android ABIs ==="

    cargo ndk \
      -t aarch64-linux-android \
      -t armv7-linux-androideabi \
      -t x86_64-linux-android \
      -t i686-linux-android \
      -o "$ANDROID_JNI" \
      build --release -p identus-crypto-uniffi

    echo ""
    echo "=== Verifying .so files ==="
    for abi in arm64-v8a armeabi-v7a x86_64 x86; do
      SO_FILE="$ANDROID_JNI/$abi/libidentus_crypto_uniffi.so"
      if [ -f "$SO_FILE" ]; then
        echo "  ✅ $SO_FILE ($(du -h "$SO_FILE" | cut -f1))"
      else
        echo "  ❌ MISSING: $SO_FILE"
        exit 1
      fi
    done

    echo ""
    echo "=== Copying Kotlin bindings ==="
    mkdir -p "$BINDINGS_DST"
    cp "$BINDINGS_SRC" "$BINDINGS_DST/"
    echo "  ✅ Copied $(basename "$BINDINGS_SRC") -> $BINDINGS_DST"

    echo ""
    echo "✓ Android example build complete."
    echo "  Next: cd examples/android-app && gradle assembleDebug"

# Clean all build artifacts
[group('sdk-rust')]
clean:
    cargo clean

# Format all source files (Nix, TOML, Rust)
[group('sdk-rust')]
format:
    echo "Formatting Nix files..."
    find . -name '*.nix' -type f -exec sh -c 'echo "  → {}" && nixfmt {}' \;

    echo "Formatting TOML files..."
    find . -name '*.toml' -type f -exec sh -c 'echo "  → {}" && taplo format {}' \;

    echo "Formatting Rust files..."
    cargo fmt

# Lint text files (markdown, YAML, editorconfig, shell scripts)
[group('checks')]
lint-text:
    #!/usr/bin/env bash
    set -euo pipefail
    EXIT_CODE=0

    echo "=== Markdown Lint ==="
    markdownlint-cli2 "**/*.md" || EXIT_CODE=$?

    echo ""
    echo "=== YAML Lint ==="
    yamllint -c .yamllint.yml . || EXIT_CODE=$?

    echo ""
    echo "=== EditorConfig Check ==="
    editorconfig-checker || EXIT_CODE=$?

    echo ""
    echo "=== ShellCheck ==="
    sh_files=$(find . -name "*.sh" \
      -not -path "*/node_modules/*" \
      -not -path "*/.git/*" \
      -not -path "*/target/*")
    if [[ -n "$sh_files" ]]; then
      echo "$sh_files" | xargs shellcheck --severity=warning || EXIT_CODE=$?
    else
      echo "No .sh files found, skipping."
    fi

    exit $EXIT_CODE

# Run fast checks (tests, formatting, lints)
[group('checks')]
check: format build test
    echo "✓ All checks completed successfully."
