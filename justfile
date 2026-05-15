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

# Build the WASM web demo: compile the WASM crate and stage all static files
# into target/web-demo/ for serving.
# Depends on build-wasm (WASM bindings in lib/identus-crypto-wasm/pkg/).
[group('sdk-rust')]
build-web-example: build-wasm
    #!/usr/bin/env bash
    set -euo pipefail

    WASM_PKG="lib/identus-crypto-wasm/pkg"
    WEB_DEMO="examples/wasm-app"
    OUT_DIR="target/web-demo"

    echo "=== Staging WASM web demo into $OUT_DIR ==="

    # Create output directory
    mkdir -p "$OUT_DIR"

    # Copy WASM/JS bindings from the wasm-pack build output
    echo "Copying WASM bindings from $WASM_PKG/..."
    cp "$WASM_PKG"/*.wasm "$OUT_DIR/"
    cp "$WASM_PKG"/*.js "$OUT_DIR/"
    cp "$WASM_PKG"/*.ts "$OUT_DIR/" 2>/dev/null || true
    cp "$WASM_PKG"/package.json "$OUT_DIR/" 2>/dev/null || true

    # Copy static demo files
    echo "Copying static files from $WEB_DEMO/..."
    cp "$WEB_DEMO/index.html" "$OUT_DIR/"
    cp "$WEB_DEMO/index.js" "$OUT_DIR/"
    cp "$WEB_DEMO/style.css" "$OUT_DIR/"

    echo ""
    echo "✓ WASM web demo staged to $OUT_DIR"
    echo "  $(du -sh "$OUT_DIR" | cut -f1) total"
    echo "  Next: just run-web-example"

# Serve the WASM web demo locally via Python's HTTP server.
# Depends on build-web-example (staged output in target/web-demo/).
[group('sdk-rust')]
run-web-example: build-web-example
    #!/usr/bin/env bash
    set -euo pipefail

    PORT="${PORT:-8080}"
    DIR="target/web-demo"

    echo "=== Starting HTTP server ==="
    echo "Serving $DIR/ at http://localhost:$PORT"
    echo "Press Ctrl+C to stop."
    echo ""

    # Trap SIGINT for graceful shutdown
    trap 'echo ""; echo "Server stopped."; exit 0' INT TERM

    exec python3 -m http.server "$PORT" --directory "$DIR"

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

# Build the debug APK for the Android example app.
# Depends on build-android-example (native .so + Kotlin bindings).
[group('sdk-rust')]
build-android-apk: build-android-example
    #!/usr/bin/env bash
    set -euo pipefail

    echo "=== Checking Android SDK and Gradle setup ==="

    if [ -z "${ANDROID_HOME:-}" ]; then
        echo "Error: ANDROID_HOME is not set."
        echo "This command must be run inside the nix devshell."
        echo "  nix develop -c just build-android-apk"
        exit 1
    fi

    if ! command -v gradle &>/dev/null; then
        echo "Error: gradle not found on PATH."
        echo "This command must be run inside the nix devshell."
        echo "  nix develop -c just build-android-apk"
        exit 1
    fi

    echo "=== Building debug APK ==="
    cd examples/android-app
    AAPT2="$ANDROID_HOME/build-tools/34.0.0/aapt2"
    gradle -Pandroid.aapt2FromMavenOverride="$AAPT2" assembleDebug

    APK="app/build/outputs/apk/debug/app-debug.apk"
    if [ -f "$APK" ]; then
        echo ""
        echo "✓ APK built successfully:"
        echo "  $APK ($(du -h "$APK" | cut -f1))"
    else
        echo ""
        echo "❌ APK not found at expected path: $APK"
        exit 1
    fi

# Launch the Android emulator (headed mode), install the APK, and start the demo activity.
[group('sdk-rust')]
run-android-example: build-android-apk
    #!/usr/bin/env bash
    set -euo pipefail

    ANDROID_EXAMPLE="examples/android-app"
    APK="$ANDROID_EXAMPLE/app/build/outputs/apk/debug/app-debug.apk"
    AVD_NAME="identus-crypto-demo"
    SYSTEM_IMAGE="system-images;android-24;default;x86_64"
    BOOT_TIMEOUT=120

    # ---- Platform check ----
    case "$(uname -s)" in
        Darwin)
            if [ "$(uname -m)" = "arm64" ]; then
                echo "Error: Android emulator is not available on Apple Silicon (aarch64-darwin)."
                echo "The 'just run-android-example' command requires x86_64 Linux or x86_64 macOS."
                echo "'just build-android-apk' works on Darwin (only the emulator step is blocked)."
                exit 1
            fi
            ;;
        Linux)
            if [ ! -e /dev/kvm ]; then
                echo "Warning: /dev/kvm not found — emulator performance will be degraded."
                echo "On Linux, install KVM: sudo apt install qemu-kvm && sudo adduser $USER kvm"
                echo "Continuing with emulator launch (may be slow)..."
            fi
            ;;
        *)
            echo "Error: Unsupported platform '$(uname -s)'."
            echo "The Android emulator is only supported on Linux and macOS (x86_64)."
            exit 1
            ;;
    esac

    # ---- Tool check ----
    for tool in adb emulator avdmanager sdkmanager; do
        if ! command -v "$tool" &>/dev/null; then
            echo "Error: '$tool' not found on PATH."
            echo "This command must be run inside the nix devshell."
            echo "  nix develop -c just run-android-example"
            exit 1
        fi
    done

    # ---- AVD creation (if needed) ----
    echo "=== Checking AVD '$AVD_NAME' ==="
    if avdmanager list avd -c 2>/dev/null | grep -q "^${AVD_NAME}$"; then
        echo "AVD '$AVD_NAME' already exists, skipping creation."
    else
        echo "AVD '$AVD_NAME' not found. Creating..."
        echo "no" | avdmanager create avd \
            -n "$AVD_NAME" \
            -k "$SYSTEM_IMAGE" \
            -d "pixel_6"
        echo "AVD '$AVD_NAME' created."
    fi

    # ---- Launch emulator ----
    echo "=== Starting emulator (headed mode) ==="
    # Check if an emulator is already running
    if adb get-state 2>/dev/null | grep -q "device"; then
        echo "Emulator already running, reusing it."
    else
        # Launch emulator in background, headed mode (visible window)
        emulator \
            -avd "$AVD_NAME" \
            -no-boot-anim \
            -no-snapshot \
            -gpu auto &
        EMULATOR_PID=$!
        trap 'echo "SIGINT received: killing emulator (PID $EMULATOR_PID)..."; kill "$EMULATOR_PID" 2>/dev/null; echo "AVD preserved at ~/.android/avd/${AVD_NAME}.avd"; exit 0' INT TERM
    fi

    # ---- Wait for boot ----
    echo "=== Waiting for emulator to boot (${BOOT_TIMEOUT}s timeout) ==="
    boot_start=$(date +%s)
    while true; do
        elapsed=$(( $(date +%s) - boot_start ))
        if [ "$elapsed" -ge "$BOOT_TIMEOUT" ]; then
            echo "Error: Emulator did not boot within ${BOOT_TIMEOUT}s."
            echo "Check logs or try running 'emulator -avd $AVD_NAME' manually."
            exit 1
        fi
        boot_complete=$(adb shell getprop sys.boot_completed 2>/dev/null || echo "")
        if [ "$boot_complete" = "1" ]; then
            echo "Emulator booted in ${elapsed}s."
            break
        fi
        sleep 2
    done

    # ---- Install APK ----
    echo "=== Installing APK ==="
    adb install -r "$APK"

    # ---- Start activity ----
    echo "=== Starting demo activity ==="
    adb shell am start -n io.identus.example/.MainActivity

    echo ""
    echo "✓ Demo app is running in the emulator."
    echo "Activity: io.identus.example/.MainActivity"
    echo "Press Ctrl+C to stop the emulator (AVD preserved)."

    # Keep process alive so SIGINT is catchable
    wait

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

# Lint text files (markdown, YAML, Nix, editorconfig, shell scripts)
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
    echo "=== DeadNix (lint .nix files) ==="
    deadnix . || EXIT_CODE=$?

    echo ""
    echo "=== Statix (check .nix files) ==="
    statix check . || EXIT_CODE=$?

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
