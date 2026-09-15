## Why

The exact-merged-head canary for issue #276 passed the AOSP image install and
every other job, then the Android verifier rejected the built SDK library's NDK
build identity. The workflow installs NDK 27.0.12077973, but the verifier gives
the hosted runner's ambient `ANDROID_NDK_ROOT` precedence over that exact
side-by-side path. The current runner points this variable at 27.3.13750724.

The script therefore asserts one NDK version while allowing another compiler
to produce the artifact. This hidden ambient input was masked by the earlier
Google Play license failure.

## What changes

- Accept ADR 0123 requiring exact side-by-side NDK selection from the already
  selected Android SDK root.
- Make the native verifier select and validate
  `$android_sdk/ndk/27.0.12077973`, overwrite inherited NDK aliases for child
  tools, and remove the ambient latest-NDK alias.
- Validate the selected NDK's `source.properties` and record its checksum in
  the evidence receipt before relying on the existing ELF identity check.
- Extend offline support-policy checks and mutations to reject ambient NDK
  precedence, missing metadata validation, and unbound child-tool aliases.

## Capabilities

### Modified capabilities

- `native-did-bindings`: binds Android compilation and evidence to one exact
  installed NDK rather than a mutable runner default.
- `weekly-slow-evidence`: makes exact NDK installation and native verification
  an end-to-end contract.

## Non-goals

- No NDK upgrade, Android API/ABI expansion, AOSP change, Gradle change,
  production Rust change, support activation, release, or consumer migration.
- No trust in ambient `ANDROID_NDK`, `ANDROID_NDK_HOME`,
  `ANDROID_NDK_LATEST_HOME`, or `ANDROID_NDK_ROOT` values.

## Delivery

Issue #276 owns this zero-stack-depth repair. After protected merge, a new full
canary must pass at the exact merged `develop` SHA. The issue stays open until
the first successful natural scheduled run is also reviewed.
