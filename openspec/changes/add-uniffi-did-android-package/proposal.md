## Why

Issue #230 is the Android packaging child of native-binding issue #222. The
host and Apple proofs establish ABI version 1 and generated Kotlin behavior on
a macOS JVM, but they do not prove an Android-loadable native library, an AAR,
or execution in an Android runtime.

The smallest honest next slice is one deterministic arm64-v8a local AAR and an
ephemeral emulator consumer. It closes that packaging gap without publishing
an artifact or activating Android support.

## What changes

- Build `identus-uniffi-did` for `aarch64-linux-android` with Rust 1.98.1,
  NDK 27.0.12077973 and API level 21.
- Generate Kotlin with exact UniFFI 0.32.0 and package it with the Rust shared
  object as a local arm64-v8a AAR.
- Keep exact JNA 5.18.1 `@aar` as an explicit consumer dependency rather than
  copying its multi-ABI dispatch libraries into the SDK AAR.
- Build/package twice, compare normalized complete trees, and inspect ELF,
  ABI, API floor, exported symbols, paths, dependency closure and sizes.
- Install an ephemeral consumer application on one arm64 API-35 emulator and
  exercise API version, valid, invalid, oversized and redacted-error behavior.
- Run the proof only in the weekly/manual slow macOS lane; keep fast CI
  unchanged.

## Capabilities

### Modified capabilities

- `native-did-bindings`: adds deterministic Android AAR construction and
  emulator execution evidence to the existing host/Apple foundation.

## Non-goals

- No Maven publication, signing, production support, physical-device proof or
  compatibility matrix.
- No x86/x86_64/armeabi-v7a SDK native library, JNI bridge, Android Keystore,
  storage, networking, secrets, callbacks, async or object handles.
- No downstream repository mutation or generic-domain dependency change.

## Delivery

Issue #230 owns this slice under #222/#163. It starts at
`develop@f157820aa312e56aa558361f8332609933c6b8e2` and requires this committed
specification before implementation, exact-diff architecture/security review,
local verification, a signed/DCO PR and green required hosted CI.
