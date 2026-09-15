# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-09-15
Source retrieval date: 2026-09-15
Research blockers: none

## Problem and existing implementation

Manual slow run `34983050826` at exact merged `develop`
`6423a9e0d946c1f438f554a3b1fe6222fb28717a` passed the exact AOSP package
installation and all preceding macOS gates. The Android verifier then built
twice and packaged successfully but rejected the ELF with `SDK library has an
unexpected NDK build` before emulator creation.

The current implementation installs and claims NDK 27.0.12077973 while selecting
`${ANDROID_NDK_ROOT}` first. This makes exact selection dependent on mutable
host environment state.

## Normative sources

- ADR 0100 requires exact NDK 27.0.12077973 and an API-21 arm64 ELF; ADR 0120
  makes hosted macOS evidence active; ADR 0122 selects the AOSP runtime.
- GitHub's current official `macos-latest` runner image inventory at pinned
  source revision `f95c0c791f690fa64eaf9788bea06643a4176db5` records
  `ANDROID_NDK_ROOT`, `ANDROID_NDK`, and `ANDROID_NDK_HOME` as
  `/Users/runner/Library/Android/sdk/ndk/27.3.13750724`:
  https://github.com/actions/runner-images/blob/f95c0c791f690fa64eaf9788bea06643a4176db5/images/macos/macos-15-arm64-Readme.md
- Official Android guidance states that specific NDK versions are installed
  side by side under `<android-sdk>/ndk/<version>` and recommends explicit
  version selection for reproducibility:
  https://developer.android.com/studio/projects/install-ndk
- The failed job and immutable receipt are available at
  https://github.com/hyperledger-identus/sdk-rust/actions/runs/34983050826

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Exact SDK-relative 27.0.12077973 plus metadata/ELF validation | `adopt` | Matches ADR 0100 and the installed package; removes ambient selection. | Accepted NDK upgrade ADR. |
| Hosted default 27.3.13750724 | `not-adopt` | Mutable runner input and unreviewed version change. | Explicit upgrade research and compatibility evidence. |
| Ambient variable fallback | `not-adopt` | Makes an exact install bypassable and evidence untruthful. | Never for a required exact gate. |
| Metadata-only check | `not-adopt` | Does not prove the artifact used that linker. | A stronger signed tool invocation provenance replaces ELF evidence. |
| ELF-only check | `not-adopt` | Detects mismatch late and omits selected-tool metadata. | A hermetic builder makes selection implicit and auditable. |

## Compatibility and dependency evidence

No Cargo source, lock, dependency, feature or public surface changes. The same
Rust 1.98.1 target, NDK 27.0.12077973, API 21, arm64-v8a, Gradle/JNA inputs and
API-35 AOSP runtime remain the accepted set. The dependency cone is unchanged.
The direct and resolved dependency cone is unchanged, including every Cargo
feature profile and the repository's MSRV evidence.

The change affects test-tool provenance only. API, ABI, wire, serialization,
consumer packaging and support claims remain unchanged.
The native facade remains `identus-uniffi-did`; no NDK path or Android tool type
crosses that facade boundary.

## Security, privacy and maintenance evidence

Removing an ambient tool selector reduces supply-chain ambiguity. Exact
metadata plus binary identity fails closed before runtime. Exported aliases are
process-local and contain no secret or credential. No unsafe/production code,
PII, network authority, signing or publication surface is introduced.

Licensing remains the Android NDK package installed under existing runner
provisioning; no code is copied and no new license is accepted by automation.
The exact package path is maintained in one verifier constant and one workflow
install contract. Upgrade/rollback is explicit.

Maintenance, release and security posture remain experimental and unpublished.
Protocol or draft currency is not applicable because SSI and wire behavior do
not change.

## Rejected or deferred candidates

Adopting the ambient runner default, retaining fallback, or weakening binary
evidence are rejected above. Hermetic NDK archive pinning is deferred because
the current slow lane intentionally uses reviewed `sdkmanager` packages; it
would be a separate supply-chain milestone.

## Open questions and blockers

No planning blocker remains. Hosted execution is required because this host has
no Android SDK. The repair may expose a later emulator/runtime defect once the
provenance check passes; that would be separately evidenced, not hidden.

## Evidence commands

- `gh run view 34983050826 --job 104428062326 --log-failed` captured the exact
  error and proved the AOSP install passed first.
- Official runner inventory identifies ambient NDK 27.3.13750724; Android docs
  establish `<sdk>/ndk/<version>` as the side-by-side location.
- Planned: focused support-policy mutations, shell syntax, factory readiness,
  actionlint, proportional Nix, exact-head PR CI and a post-merge full canary.
- Unrun checks: local NDK compilation and emulator boot because this host has no
  Android SDK; hosted macOS remains authoritative.

## Reconsideration triggers

- ADR 0100's accepted NDK changes.
- Android removes the exact side-by-side package.
- The build gains another native tool that ignores the bound aliases/linker.
- The SDK adopts a hermetic NDK archive/toolchain derivation.
