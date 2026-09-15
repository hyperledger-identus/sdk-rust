# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-09-16
Source retrieval date: 2026-09-16
Research blockers: none

## Problem and existing implementation

The current implementation is the single macOS-only verifier described below.
Manual slow run `34992633358` executed exact protected `develop`
`f2221d444e85e4b5e4bffdc0c0fd2d7e43ec83da`. Revision binding, candidate,
coverage, performance, browser DID, Ubuntu, macOS code health/Clippy/Nix,
exact AOSP/NDK installation, two deterministic ARM64 libraries/AARs, exact NDK
metadata and ELF identity, and Android library/consumer assembly passed. The
ARM64 AVD process then exited before boot. The script kept `emulator.log` only
under the failed job's ephemeral `target/` tree, so the process diagnostic did
not survive.

The current design couples two different questions: whether the intended ARM64
artifact is correctly built, and whether shared Rust/Kotlin behavior executes
inside any Android runtime. Hosted arm64 macOS can answer the first but cannot
reliably host the nested ARM64 VM required by the second.

## Normative sources

- Failed exact-head evidence:
  https://github.com/hyperledger-identus/sdk-rust/actions/runs/34992633358
- GitHub's hosted-runner reference states that nested virtualization is not
  supported on arm64 macOS runners and that hosted Linux runners support
  hardware acceleration for Android SDK tools:
  https://docs.github.com/en/actions/reference/runners/github-hosted-runners
- Android's official acceleration guide requires matching host/guest
  architecture for VM acceleration and identifies KVM as the Linux mechanism:
  https://developer.android.com/studio/run/emulator-acceleration
- Android's official command-line reference documents emulator startup and
  acceleration controls:
  https://developer.android.com/studio/run/emulator-commandline
- The official AOSP system-image catalog retrieved on 2026-09-16, SHA-256
  `ebf2d810d9e0c0b511ae49ee6e8c671a8fa67d210d06f5e3e068f83244ece435`,
  lists both exact revision-2 packages
  `system-images;android-35;default;arm64-v8a` and
  `system-images;android-35;default;x86_64` under `android-sdk-license`:
  https://dl.google.com/android/repository/sys-img/android/sys-img2-1.xml
- ADRs 0100, 0120, 0122 and 0123 govern the accepted package, scheduler, AOSP,
  and exact-NDK boundaries.

## Candidate decisions

| Candidate | Version/revision | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- | --- |
| ARM64 package proof on macOS plus test-only x86_64 runtime proof on accelerated Linux | Rust/NDK/API/JNA versions retained; AOSP API-35 revision 2 | `adopt` | Preserves exact distributable ABI evidence and executes the same source/API contract on infrastructure GitHub supports. | A hosted ARM64 runner with proven virtualization becomes available, or physical-device CI is accepted. |
| Keep ARM64 emulator on hosted arm64 macOS | GitHub hosted arm64 macOS as retrieved 2026-09-16 | `not-adopt` | Conflicts with GitHub's documented nested-virtualization limitation and has now failed in the active canary. | GitHub changes the documented capability and a probe passes. |
| Skip Android runtime behavior | Current ADR 0100 behavior families | `not-adopt` | Would weaken the accepted binding behavior evidence. | A stronger physical-device or external-lab proof replaces it. |
| Add x86_64 to the SDK AAR | `x86_64-linux-android`, API 21 | `not-adopt` | Expands the consumer ABI and support surface merely to satisfy CI. | A named consumer and separate compatibility decision require distribution. |
| Third-party emulator action | Not selected | `not-adopt` | Adds an unnecessary action/supply-chain abstraction around commands the repository can own directly. | Direct runner integration becomes unmaintainable after evidence. |
| Self-hosted ARM64 macOS/device runner | Future exact revision | `spike` | Could execute the exact ABI but requires provisioned hardware, credentials, ownership, and availability policy outside this issue. | Maintainers establish a protected runner/device service. |

## Compatibility and dependency evidence

The distributable package remains `arm64-v8a` only, built from Rust 1.98.1 and
NDK 27.0.12077973 at minimum API 21. Its AAR shape, generated Kotlin API, JNA
5.18.1 dependency, public Rust API/ABI, wire behavior, locks, and consumer
contract do not change. The x86_64 target and AAR are test-only build inputs;
they are never copied into the ARM64 AAR or a publication directory.

Exact version and feature behavior is unchanged for every public crate. The
temporary MSRV and primary toolchain remain Rust 1.98.1 across existing target
and feature gates. The bindings Nix shell gains `x86_64-linux-android` solely for the dedicated
slow harness. No runtime Cargo dependency, default feature, generic crate edge,
or SDK support-policy tier changes. The same consumer source and fixed behavior
markers run in both the previous ARM64 local receipt and the new x86_64 hosted
receipt.

The direct and resolved dependency cone is unchanged: the test-only target adds
Rust standard-library and Android tool inputs, not a Cargo package or feature.
Public and wire compatibility is unchanged. The `identus-uniffi-did` facade
remains the only binding boundary; Android/Gradle/KVM types do not cross it.
Rollback removes only test harness/tool inputs and restores the prior topology.

## Security, privacy and maintenance evidence

License and provenance remain the exact existing project/tool sources: no code
or fixture is copied and the official Android catalog is cited by retrieval
date and hash. The change adds no authored unsafe Rust, secret/credential data,
signing, storage, network authority, or publication permission. Reachable
native code remains the NDK-built SDK plus JNA/emulator test tooling. KVM access is admitted only in
the ephemeral Linux job and is checked before emulator launch. AVD, Gradle,
generated code, libraries, APK, runtime logs, and receipts remain under ignored
artifact roots.

Both AOSP images use the existing Android SDK license; automation does not run
blanket license acceptance. Exact packages and the existing NDK/JNA inputs are
externally downloaded test tooling rather than vendored dependencies. Durable
diagnostics must be data-free emulator/build logs from synthetic fixtures.

The supply-chain posture retains exact package identities and pinned GitHub
actions. The maintenance, release and security posture remains experimental and
unpublished. Protocol or draft currency is not applicable because no SSI wire,
algorithm, or standards behavior changes.

The extra weekly job costs one Linux VM and x86_64 build/emulator duration. It
does not enlarge the pull-request fast lane. Job timeout, concurrency, exact
SHA binding, seven-day artifact retention, and immutable run receipt remain
enforced.

## Rejected or deferred candidates

ARM64-on-hosted-macOS, runtime removal, distributable x86_64, and a third-party
action are rejected above. A protected self-hosted ARM64 runtime is deferred;
it is the preferred future way to reunify exact ABI and runtime proof.

## Open questions and blockers

No planning blocker remains. This host has neither Android SDK nor KVM, so
hosted exact-head execution is required. The first Linux canary may reveal an
exact emulator/KVM or disk constraint; any such failure must retain diagnostics
and be repaired without changing the distributable ABI.

## Evidence commands

- `gh run view 34992633358 --log-failed` proves every package step completed
  before `Android emulator exited before boot`.
- Official runner/Android sources establish the virtualization boundary and
  matching-architecture acceleration requirement.
- Official catalog inspection proves the exact AOSP x86_64 package exists.
- Planned: factory readiness/preflight; policy mutations; shell syntax;
  actionlint; compatible local Nix; exact-head fast CI; post-merge full canary.
- Unrun locally: Android package/runtime execution because this host has no
  Android SDK or KVM. Hosted macOS/Linux are the respective authorities.

## Reconsideration triggers

- GitHub changes hosted macOS nested virtualization or Linux KVM availability.
- A protected self-hosted/device runner is accepted.
- Android removes either exact API-35 AOSP image or NDK 27.0.12077973.
- x86_64 becomes a requested distributable consumer ABI.
- Android support/publication activation begins.
