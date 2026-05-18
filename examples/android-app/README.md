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
nix/devShells/default.nix           ← Devshell with cargo-ndk, NDK toolchain, Android SDK, JDK 17, Gradle
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

2. `just build-demo-android` cross-compiles the Rust crate for all 4 Android
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
- **All other dependencies are provided by the sdk-rust devshell:**
  Android NDK, Android SDK (platform-tools, emulator, system images),
  JDK 17, Gradle, `ANDROID_HOME`, and `ANDROID_NDK_HOME`.

  Enter the devshell:

  ```bash
  cd sdk-rust
  nix develop
  ```

  Or run individual commands without entering the shell:

  ```bash
  cd sdk-rust
  nix develop -c just package-demo-android
  nix develop -c just run-demo-android
  ```

## Quick Start

All commands should be run from the `sdk-rust/` directory inside the devshell.

### One-command APK build

Build native `.so` files and the debug APK in one step:

```bash
nix develop -c just package-demo-android
```

This runs `build-demo-android` (cross-compile `.so` files, copy Kotlin
bindings) then `gradle assembleDebug` to produce the debug APK.

On success, the APK is at:

```text
examples/android-app/app/build/outputs/apk/debug/app-debug.apk
```

If you only need the native `.so` files (without the APK), use:

```bash
nix develop -c just build-demo-android
```

### Build + run on emulator

Build the APK, launch the Android emulator (headed mode with visible window),
install the APK, and start the demo activity:

```bash
nix develop -c just run-demo-android
```

This depends on `package-demo-android`, so the APK is always up to date.

**Note:** `run-demo-android` is only supported on **Linux** and **x86_64 macOS**.
On Apple Silicon (aarch64-darwin), the emulator is not available — use
`just package-demo-android` to produce the APK, then open it in Android Studio
(which includes its own emulator via Rosetta 2).

### Manual steps (alternative)

If you prefer to run steps individually:

1. Build native libraries:

   ```bash
   nix develop -c just build-demo-android
   ```

2. Build the APK:

   ```bash
   cd examples/android-app
   gradle assembleDebug
   ```

3. Install and run (on a connected device or running emulator):

   ```bash
   adb install examples/android-app/app/build/outputs/apk/debug/app-debug.apk
   adb shell am start io.identus.example/.MainActivity
   ```

Or open `sdk-rust/examples/android-app/` in Android Studio for easier
debugging.

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
imports, run `just build-demo-android` to copy the generated bindings
into the project:

```text
examples/android-app/app/src/main/java/uniffi/identus_crypto_uniffi/identus_crypto_uniffi.kt
```

### Gradle version compatibility

This project requires a Gradle version compatible with Android Gradle Plugin
(AGP) 8.x. The sdk-rust devshell provides a compatible `gradle`.

If running outside the devshell (`nix develop`), ensure your Gradle version
is compatible. The project does **not** include a Gradle wrapper.

### Running outside the devshell

`just package-demo-android` and `just run-demo-android` check for `ANDROID_HOME`,
`gradle`, and other required tools. If any are missing, they print a helpful
error message suggesting you run inside the devshell:

```bash
nix develop -c just package-demo-android
```

### KVM not available (Linux)

On Linux, `just run-demo-android` checks for `/dev/kvm`. If KVM is not
available, it prints a warning about degraded performance but continues.
Install KVM for better emulator performance:

```bash
sudo apt install qemu-kvm
sudo adduser $USER kvm
# log out and back in, or reboot
```

### Emulator boot timeout

The emulator boot timeout is 120 seconds. If your machine is slow or the
system image needs to be downloaded, the first boot may take longer.
On subsequent runs, the AVD is cached and boot is faster.

### Apple Silicon (aarch64-darwin)

`just package-demo-android` works on Darwin (both x86_64 and ARM).
`just run-demo-android` prints an error on Apple Silicon because the
Android emulator does not run natively on ARM macOS.

## Related Tasks

- **TASK-4** — Android cross-compilation targets (`aarch64-linux-android`,
  `armv7-linux-androideabi`, `x86_64-linux-android`, `i686-linux-android`)
- **TASK-8** — Android example app project (this directory)
- **TASK-9** — UniFFI binding crate (`lib/identus-crypto-uniffi/`); generates
  the Kotlin bindings consumed by this app
- **TASK-13** — Dev workflow (`just package-demo-android`, `just run-demo-android`,
  devshell tooling)
