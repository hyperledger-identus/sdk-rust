# Stabilize macOS Android tool discovery

## Why

The first post-activation slow canary reached the macOS Android setup step but
failed because `sdkmanager` is installed in the hosted image without being on
`PATH`. The workflow must resolve the command-line tool from the Android SDK it
is about to mutate instead of relying on ambient shell configuration.

## What changes

- Resolve `sdkmanager` from the exact `ANDROID_SDK_ROOT`/`ANDROID_HOME` SDK
  selected by the hosted runner.
- Fail before installation when the SDK root or executable is unavailable.
- Retain the existing exact NDK, platform, and arm64 emulator-image package
  identifiers.
- Add an offline policy regression for this discovery boundary.

## Capabilities

### Modified capabilities

- `weekly-slow-evidence`: add an explicit fail-closed Android command-line tool
  discovery contract for the macOS native-bindings evidence.

## Non-goals

- No Android package, API, ABI, NDK, emulator, Rust target, runner label,
  compiler, dependency, or release-policy change.
- No download bootstrap outside the Android SDK's own pinned package manager.
- No claim that the failed manual canary is successful or naturally scheduled
  evidence.

## Delivery

Issue #276 owns this second activation defect found by manual slow run
`34951058164`. It follows the independently reviewed candidate determinism
repair in one focused stabilization PR to `develop`, then the exact-head manual
canary is rerun.
