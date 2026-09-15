# Design: explicit Android SDK command discovery

## Boundary

The hosted runner's `ANDROID_SDK_ROOT`, falling back to `ANDROID_HOME`, is the
only SDK authority. The workflow does not search arbitrary filesystem paths or
install an alternate SDK.

## Discovery and execution

The macOS-only setup step copies the selected environment value into a quoted
local variable, requires it to be non-empty, derives
`cmdline-tools/latest/bin/sdkmanager`, and requires an executable file. It then
uses that absolute command for the existing exact package installation.

This keeps the resolution local to one step and avoids changing `PATH` for Nix,
Cargo, Gradle, or the native verification scripts.

## Verification

The support-policy checker requires the SDK-root fallback, the executable
guard, the absolute invocation, and all existing exact Android package names.
Mutation tests remove or weaken each marker. The hosted canary remains the only
proof that the image can install the packages and complete emulator-backed
runtime verification.

## Failure and rollback

Missing environment or command-line tools fail before any SDK mutation. A
failed installation retains the current shell's nonzero result. Rollback is a
repository revert and does not change consumer state.
