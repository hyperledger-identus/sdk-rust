# Identus Crypto — Android App Demo

An example Android application that demonstrates calling Identus SDK crypto
functions (key generation, Schnorr signing, and signature verification) from
an Android app using the [UniFFI](https://mozilla.github.io/uniffi-rs/)-generated
Kotlin bindings.

## Architecture

```text
lib/identus-crypto-uniffi/         ← Rust crate exposing crypto via UniFFI
  └── src/lib.rs                    ←   generate_key(), sign(), verify()
  └── bindings/kotlin/              ←   Pre-generated Kotlin bindings (TASK-9)
nix/devShells/default.nix           ← Devshell with cargo-ndk, NDK toolchain
examples/android-app/               ← Android Gradle project (THIS DIR)
  ├── settings.gradle.kts           ←   Root settings
  ├── build.gradle.kts              ←   Root build config
  ├── gradle.properties             ←   AndroidX, JVM args
  ├── app/
  │   ├── build.gradle.kts          ←   Module config (jniLibs, JNA dep)
  │   └── src/main/
  │       ├── AndroidManifest.xml   ←   App manifest
  │       ├── java/
  │       │   ├── io/identus/example/
  │       │   │   └── MainActivity.kt   ←   Activity calling generated bindings
  │       │   └── uniffi/identus_crypto_uniffi/
  │       │       └── identus_crypto_uniffi.kt  ←   Copied from TASK-9 output
  │       ├── jniLibs/
  │       │   ├── arm64-v8a/
  │       │   │   └── libidentus_crypto_uniffi.so    ←  Built by cargo ndk
  │       │   ├── armeabi-v7a/
  │       │   │   └── libidentus_crypto_uniffi.so
  │       │   ├── x86_64/
  │       │   │   └── libidentus_crypto_uniffi.so
  │       │   └── x86/
  │       │       └── libidentus_crypto_uniffi.so
  │       └── res/layout/
  │           └── activity_main.xml  ←   UI layout
  └── README.md                      ←   This file
```

### How it works

1. The **UniFFI binding crate** (`lib/identus-crypto-uniffi/`) is a Rust `cdylib`
   that exposes three functions via UniFFI:
   - `generateKey()` → `KeyInfo(secretKey: String, publicKey: String)`
   - `sign(secretKey: String, message: ByteArray)` → `String` (signature hex)
   - `verify(publicKey: String, message: ByteArray, signature: String)` → `Boolean`

2. `just build-android-example` cross-compiles the Rust crate for all 4 Android
   ABIs using `cargo ndk`, placing the `.so` files directly into the Android
   project's `jniLibs/` directory. It also copies the pre-generated Kotlin
   bindings from TASK-9's output into the project source tree.

3. The **Android app** (`MainActivity.kt`) imports the generated Kotlin types
   from `uniffi.identus_crypto_uniffi.*` and calls them directly — no manual
   JSON parsing or JNI code. The UI is a simple three-section layout:
   - **Generate Key**: button + secret/public key display
   - **Sign**: secret key + message inputs, signature output
   - **Verify**: public key + message + signature inputs, ✅/❌ result

## Prerequisites

- [Nix](https://nixos.org/download.html) with flakes enabled
- Android NDK (provided by the sdk-rust devshell, configured in TASK-4)
- **Android SDK (required for building the APK):** Install via
  [Android Studio](https://developer.android.com/studio) or `sdkmanager`:

  ```bash
  sdkmanager "platforms;android-24" "build-tools;34.0.0"
  ```

  Set `ANDROID_HOME` or `ANDROID_SDK_ROOT` to point to the SDK installation.

- **Gradle** (for building the APK; available via nixpkgs or system install):

  ```bash
  nix shell nixpkgs#gradle -c gradle --version
  ```

## Quick Start

All commands should be run from the `sdk-rust/` directory inside `nix develop`.

### 1. Build native libraries and prepare the Android project

```bash
nix develop -c just build-android-example
```

This will:

- Cross-compile `identus-crypto-uniffi` for all 4 Android ABIs
- Place `.so` files in `examples/android-app/app/src/main/jniLibs/<abi>/`
- Copy the generated Kotlin bindings into the Android source tree

Expected output:

```text
=== Building identus-crypto-uniffi for all 4 Android ABIs ===
   Compiling identus-crypto-uniffi ...
   ...

=== Verifying .so files ===
  ✅ app/src/main/jniLibs/arm64-v8a/libidentus_crypto_uniffi.so (X.XM)
  ✅ app/src/main/jniLibs/armeabi-v7a/libidentus_crypto_uniffi.so (X.XM)
  ✅ app/src/main/jniLibs/x86_64/libidentus_crypto_uniffi.so (X.XM)
  ✅ app/src/main/jniLibs/x86/libidentus_crypto_uniffi.so (X.XM)

=== Copying Kotlin bindings ===
  ✅ Copied identus_crypto_uniffi.kt -> examples/android-app/app/src/main/java/uniffi/identus_crypto_uniffi/

✓ Android example build complete.
  Next: cd examples/android-app && gradle assembleDebug
```

### 2. Build the debug APK

```bash
cd examples/android-app
gradle assembleDebug
```

On success, the APK is at:

```text
app/build/outputs/apk/debug/app-debug.apk
```

### 3. Install and run

Install on a connected device or emulator:

```bash
adb install app/build/outputs/apk/debug/app-debug.apk
```

Or open in Android Studio for easier debugging:

1. Open `sdk-rust/examples/android-app/` as a project
2. Set up a device/emulator
3. Run the app

## Usage

The demo has three independent sections:

### Generate Key

Tap **Generate** to create a new random Schnorr key pair. The secret key
and public key are shown in selectable text fields.

### Sign

Paste the secret key (from Generate Key) into the "Secret Key (hex)" field,
type a message, and tap **Sign**. The signature hex appears below.

### Verify

Paste the public key, the original message, and the signature hex into the
three fields, then tap **Verify**. The result shows ✅ **Valid** or ❌ **Invalid**.

## Expected Outputs

- **Generate Key**: Two 64-character hex strings (secret key, public key)
- **Sign**: A 128-character hex string (64-byte Schnorr signature)
- **Verify**: "✅ Valid" for a correct signature, "❌ Invalid" otherwise

## Troubleshooting

### JNA dependency

The UniFFI-generated Kotlin bindings use Java Native Access (JNA) via
`com.sun.jna.*` imports. On Android, JNA is not bundled with the SDK.
The `app/build.gradle.kts` includes:

```kotlin
implementation("net.java.dev.jna:jna:5.14.0@aar")
```

The `@aar` classifier is essential — it provides Android-specific native stubs.
Without it, the build will fail with missing `com.sun.jna.*` classes.

### NDK discovery

`cargo ndk` finds the NDK via `$ANDROID_NDK_HOME`. This environment variable
is set automatically by the sdk-rust devshell (configured in TASK-4). If you
get NDK-related errors, verify the variable is set:

```bash
echo $ANDROID_NDK_HOME
```

### Kotlin bindings not found

If the Android build fails with missing `uniffi.identus_crypto_uniffi.*`
imports, run `just build-android-example` to copy the generated bindings
into the project:

```text
examples/android-app/app/src/main/java/uniffi/identus_crypto_uniffi/identus_crypto_uniffi.kt
```

### Gradle version compatibility

This project requires a Gradle version compatible with Android Gradle Plugin
(AGP) 8.x. If using a system-installed Gradle, verify compatibility:

```bash
gradle --version
```

The project does **not** include a Gradle wrapper — use `gradle` from
your system or nixpkgs (`nix shell nixpkgs#gradle`).

## Related Tasks

- **TASK-4** — Android cross-compilation targets (`aarch64-linux-android`,
  `armv7-linux-androideabi`, `x86_64-linux-android`, `i686-linux-android`)
- **TASK-9** — UniFFI binding crate (`lib/identus-crypto-uniffi/`); generates
  the Kotlin bindings consumed by this app
- **TASK-7** — WASM binding crate (analogous for web target)
