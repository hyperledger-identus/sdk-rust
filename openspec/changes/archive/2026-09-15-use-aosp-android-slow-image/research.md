# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-09-15
Source retrieval date: 2026-09-15
Research blockers: none

## Problem and existing implementation

Manual slow run `34972066673` at protected `develop`
`59a8db6b7c34439f4f84d762bc1fd56c6ecba9d0` passed the complete macOS Clippy
and 31-check Nix matrix. Job `104390541721` then invoked the corrected absolute
`sdkmanager` path and failed installing
`system-images;android-35;google_apis_playstore;arm64-v8a`: the headless prompt
named `Google Play ARM 64 v8a System Image`, reported the license unaccepted and
exited 1. This current implementation couples a DID/JNA smoke test to Google
Play even though its app manifest and behavior use no Google service.

## Normative sources

- ADR 0100 and current `native-did-bindings` require a fresh arm64 API-35 AVD,
  not Google Play/Google APIs.
- Google's official AVD documentation states AOSP images omit Google apps and
  services, while Google APIs/Play images add those capabilities:
  https://developer.android.com/studio/run/managing-avds
- Google's official `sdkmanager` documentation says headless license acceptance
  uses an interactive `sdkmanager --licenses` flow:
  https://developer.android.com/tools/sdkmanager
- Stable AOSP catalog retrieved from
  https://dl.google.com/android/repository/sys-img/android/sys-img2-1.xml at
  SHA-256 `ebf2d810d9e0c0b511ae49ee6e8c671a8fa67d210d06f5e3e068f83244ece435`
  contains API-35 default arm64-v8a revision 2 under `android-sdk-license`.
- Stable Google Play catalog retrieved from
  https://dl.google.com/android/repository/sys-img/google_apis_playstore/sys-img2-1.xml
  at SHA-256
  `61bbbf975058b60a80e7fe6760ea4eab9b72519b1f3dd458e7b9e1d56d0bc6f0`
  contains the old API-35 arm64-v8a revision 9 under distinct
  `android-sdk-arm-dbt-license`.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| API-35 default AOSP arm64-v8a | `adopt` | Exact required API/ABI, no unused Google services, ordinary SDK license, current stable catalog. | Smoke test gains a declared Google service dependency or package disappears. |
| Blanket `yes \| sdkmanager --licenses` | `not-adopt` | Accepts every outstanding license in the SDK, including unrelated future packages. | Organization explicitly authorizes a reviewed isolated license set and runner provisioning cannot supply it. |
| Google Play image plus targeted license file | `not-adopt` | Preserves unnecessary coupling and relies on opaque license-hash state. | Product behavior genuinely requires Play Store runtime. |
| Google APIs image | `not-adopt` | Still adds unused Google services and vendor coupling. | Runtime test requires a Google API but not Play Store. |
| Delete emulator execution | `not-adopt` | Removes accepted end-to-end AAR/JNA proof. | A stronger physical/device-farm oracle replaces it. |

## Compatibility and dependency evidence

The current implementation changes only test-infrastructure package identity.
Exact version and feature evidence remains Rust/Cargo 1.98.1, NDK
27.0.12077973, Android API 35, application minimum API 21, ABI arm64-v8a, JNA
5.18.1 and every existing feature profile.
The direct and resolved Cargo dependency cone is unchanged. The external image
cone becomes smaller by removing Google Play apps/services.

Consumer target evidence and public/wire compatibility are unchanged: no SDK or
AAR bytes, API, ABI, serialization or behavior case changes. The native binding
facade remains `identus-uniffi-did`; no image type crosses it. MSRV and portable
target claims are unaffected.

## Security, privacy and maintenance evidence

No unsafe/native production code, secret, PII, credential or write authority is
added. The AOSP image reduces external software and legal coupling. The test
continues to isolate AVD state below ignored `target/`, uses fixed package/path
values and rejects drift before execution. Supply-chain evidence includes exact
catalog URLs/hashes, package path, current revision/license and hosted receipts.

License and provenance remain Google-distributed Android SDK test tooling plus
Apache-2.0 first-party scripts; no upstream code/fixture is copied. Maintenance,
release and security posture remains experimental/unpublished. Protocol or
draft currency is not applicable because SSI and wire behavior do not change.

## Rejected or deferred candidates

Broad license acceptance, targeted opaque license hashes, Google service images
and removal of runtime proof are rejected above. Hermetic archive checksum
pinning and migration to the new Android CLI are deferred to focused research;
the hosted runner currently supplies `sdkmanager` and existing policy owns it.
Rollback restores the prior package identity and known-red prompt without
consumer or artifact migration.

## Open questions and blockers

No planning blocker remains. This host has no Android SDK, so local validation
cannot install or boot the image. Protected macOS CI is authoritative. If the
ordinary SDK license is also missing there, the lane fails closed and requires
explicit organization-level license/provisioning direction rather than an
automatic acceptance fallback.

## Evidence commands

- `gh api .../actions/jobs/104390541721/logs` isolated the exact prompt/failure.
- `curl -fsSL <official-catalog> | sha256sum` produced the hashes above; bounded
  XML inspection verified path, API, ABI, revision, license and dependency.
- Planned local commands: support-policy mutation suite, shell syntax, factory
  check/readiness, OpenSpec strict validation, `git diff --check`, and
  proportional Nix checks.
- Unrun checks: local SDK installation/emulator boot (SDK absent), physical
  devices, other APIs/ABIs and natural weekly cadence. Hosted exact-head PR CI
  and post-merge slow canary remain required.

## Reconsideration triggers

- The DID/AAR smoke test acquires a Google services dependency.
- The exact AOSP package disappears, changes license, or cannot boot reliably.
- Android CLI replaces sdkmanager on the hosted runner.
- Organization adopts hermetic emulator image provenance or explicit license
  provisioning.
