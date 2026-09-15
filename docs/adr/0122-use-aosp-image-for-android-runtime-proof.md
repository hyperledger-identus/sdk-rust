# ADR 0122: use an AOSP image for the Android runtime proof

- **Status:** Accepted by project-sponsor direction
- **Date:** 2026-09-15
- **Issue:** [#276](https://github.com/hyperledger-identus/sdk-rust/issues/276)
- **Builds on:** ADR 0100 and ADR 0120
- **Review no later than:** 2026-12-08 and before any Android support claim

## Context

The first protected-`develop` slow canary reached Android provisioning after
the complete macOS Clippy and Nix gates passed. Installation of
`system-images;android-35;google_apis_playstore;arm64-v8a` then stopped at an
interactive license prompt for the Google Play ARM64 system image.

ADR 0100 requires a fresh arm64 API-35 AVD to execute the local DID AAR and JNA
consumer. The smoke test does not use Google APIs, Play Store or Google Play
services. Keeping the Google Play image would therefore add unrelated software,
license and availability coupling. Automatically running
`sdkmanager --licenses` would be broader still because it accepts every
outstanding license known to that SDK installation.

Google's stable AOSP catalog contained
`system-images;android-35;default;arm64-v8a` revision 2 under
`android-sdk-license` when retrieved on 2026-09-15. The catalog snapshot SHA-256
was `ebf2d810d9e0c0b511ae49ee6e8c671a8fa67d210d06f5e3e068f83244ece435`.

## Decision

1. Install and execute exact package path
   `system-images;android-35;default;arm64-v8a` in the weekly/manual macOS slow
   lane.
2. Keep the workflow package, verifier package, filesystem tag and
   `avdmanager --package` argument structurally bound to that AOSP identity.
3. Reject Google APIs/Play image drift and blanket `sdkmanager --licenses`
   acceptance in the repository's offline policy checks and mutation tests.
4. Fail closed when the selected package or its already-provisioned license is
   unavailable. License acceptance remains a runner/organization provisioning
   decision, not a repository script side effect.
5. Preserve every other ADR 0100 input and behavior: Rust/Cargo 1.98.1, NDK
   27.0.12077973, minimum application API 21, emulator API 35, arm64-v8a, JNA
   5.18.1, isolated AVD state and all existing behavior assertions.
6. Treat the hosted macOS canary as authoritative because the development host
   has no Android SDK. Keep Android and FFI support limitations unchanged.

## Consequences

The proof uses the least-capable emulator image that satisfies its declared
runtime needs. It removes an unnecessary Google Play dependency and avoids
broad automated legal acceptance while retaining end-to-end AAR/JNA execution.

The exact SDK package path does not pin the catalog's underlying archive or
future revision. The image remains Google-distributed external test tooling,
and its ordinary Android SDK license must already be accepted by runner
provisioning. A successful emulator run proves only the existing arm64 API-35
experimental slice; it does not activate publication, physical-device,
additional ABI/API or Android support claims.

No Rust crate, public API, ABI, AAR content, consumer dependency, MSRV, portable
target or fast-lane behavior changes.

## Alternatives considered

- Blanket `yes | sdkmanager --licenses`: rejected because it accepts unrelated
  current and future licenses without a bounded review decision.
- Targeted Google Play license-state injection: rejected because Play services
  are not a runtime requirement and opaque license hashes are brittle.
- Google APIs image: rejected because it still adds unused vendor services.
- Remove emulator execution: rejected because it weakens ADR 0100's runtime
  proof rather than repairing its provisioning dependency.

## Verification and rollback

Offline checks verify the ordered workflow install, the verifier package and
directory, AVD package binding, forbidden image tags and forbidden blanket
license acceptance. Protected PR CI validates the exact head; a complete
post-merge slow canary validates hosted installation and execution. The first
natural scheduled run remains the separate final acceptance for issue #276.

Rollback reverts this ADR and the package/checker changes, restoring the known
Google Play license failure without changing SDK artifacts or consumer state.
