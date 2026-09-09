# Android package research

Research class: storage-ffi
Research status: ready
Decision date: 2026-09-09
Source retrieval date: 2026-09-09
Research blockers: none

## Problem and existing implementation

`identus-uniffi-did` exposes only `binding_api_version`, `parse_did` and
`parse_did_url` through ABI version 1. The accepted host proof generates Kotlin
that calls JNA direct mapping and validates the same bounded/redacted behavior.
This change packages and executes that boundary; it adds no interface. The
current implementation therefore already supplies the consumer API and facade
boundary: dependency-owned native/JNA behavior is reachable only through
owned, bounded and redacted SDK values. Android changes remain outside generic
crates.

## Normative sources

- Android NDK SDK/API guidance:
  https://developer.android.com/ndk/guides/sdk-versions
- Android side-by-side NDK pinning:
  https://developer.android.com/studio/projects/configure-agp-ndk
- Android AAR structure:
  https://developer.android.com/studio/projects/android-library
- Android Gradle Plugin compatibility:
  https://developer.android.com/build/releases/about-agp
- UniFFI 0.32 Kotlin/Gradle integration:
  https://mozilla.github.io/uniffi-rs/latest/kotlin/gradle.html
- JNA 5.18.1 Maven Central artifacts:
  https://repo1.maven.org/maven2/net/java/dev/jna/jna/5.18.1/

The pinned source revision for UniFFI is tag `v0.32.0`, commit
`5c7b73906358e1a7acdc1bdc7bf5cd86fb27e44c`. Android NDK is side-by-side
revision `27.0.12077973`; Maven/Gradle inputs use exact released versions and
repository locks rather than mutable branches.

## Candidate decisions

| Candidate | Decision | Evidence and boundary |
| --- | --- | --- |
| Rust `aarch64-linux-android` cdylib | `conditional-adopt` | NDK r27 builds an arm64 ELF with API 21 metadata, RELRO/NOW, non-executable stack and only `libdl`/`libc` runtime needs. |
| AGP 8.13.2 / Gradle 8.14.4 / JDK 17 | `conditional-adopt` | A maintained AGP 8 line compatible with the repository's Nix Gradle and JDK; exact versions remain slow-lane build inputs. |
| Kotlin 2.2.20 | `adopt` | Reuses the exact version already proven by the host Kotlin fixture. |
| JNA 5.18.1 `@aar` as consumer dependency | `conditional-adopt` | UniFFI requires JNA 5.12+; the exact already accepted version supplies Android dispatch libraries. Keep it outside the SDK AAR to avoid hidden multi-ABI code, license merging and duplicate native resources. |
| Copy/shadow JNA into the SDK AAR | `not-adopt` | Obscures provenance and dependency metadata and introduces unrelated ABI payloads into an arm64-only SDK artifact. |
| Hand-authored JNI bridge | `not-adopt` | Creates a second FFI definition and duplicates the accepted UniFFI boundary. |
| Committed generated Kotlin/AAR | `not-adopt` | Generated material is reproducible evidence under ignored `target/`; source, locks and templates are the maintained inputs. |
| Gradle managed-device plugin | `defer` | Direct ephemeral AVD creation and `adb` execution prove this one slice with a smaller dependency cone. |

## Compatibility and dependency evidence

- Rust: exact repository etalon 1.98.1. The dedicated bindings toolchain needs
  the `aarch64-linux-android` standard library added; the default shell and fast
  toolchain remain unchanged.
- NDK: exact 27.0.12077973, API 21 linker. Local pre-code output is ELF64
  little-endian AArch64, carries Android API 21/r27 notes, has RELRO/NOW and a
  non-executable stack, and exports the required UniFFI checksum/version/DID
  symbols.
- Android build: AGP 8.13.2, Gradle 8.14.4, JDK 17, compile/target API 35 and
  minimum API 21. The emulator proof uses an exact arm64 API-35 system-image
  package selected by the gate.
- Kotlin/JNA: Kotlin 2.2.20 and `net.java.dev.jna:jna:5.18.1@aar`. The JNA AAR
  SHA-256 observed during research is
  `7f053e3ec99e14dd71259c82c1c8a02738d64a13c31226b2acc170f3060951e0`;
  it offers Apache-2.0 or LGPL-2.1-or-later licensing and contains dispatch
  libraries for several ABIs. It is not repackaged.

MSRV remains the repository's evidence-driven Rust floor while this packaging
gate uses exact Rust 1.98.1. Exact dependency versions and features remain
unchanged in the Rust workspace: UniFFI 0.32.0 uses the accepted runtime
features while the separately locked bindgen tool owns generator features. The
direct and resolved dependency cone of Cargo.lock does not change. The
Android/Gradle cone is build/test-only and independently locked.

License and provenance remain Apache-2.0 for SDK-authored content, MPL-2.0 for
the pinned UniFFI source and the documented Apache-2.0/LGPL-2.1-or-later choice
for exact JNA. Supply-chain evidence is the Cargo locks, Gradle dependency
lock, repository-pinned Nix/Rust inputs, exact Android versions, cargo-deny and
cargo-audit. Public and wire compatibility are unchanged because ABI version 1
and its function/type surface do not change. No new protocol/draft choice is
made; final DID Core behavior and existing bounds remain authoritative.

## Security, privacy and maintenance evidence

The SDK AAR contains only generated Kotlin bytecode and one arm64 SDK shared
object. Complete-tree comparison, ELF inspection, exported-symbol checks and
absolute-path rejection prevent silent packaging drift. The consumer resolves
JNA explicitly, demonstrating the local-file integration contract that future
Maven metadata would carry transitively.

The emulator and AVD live under ignored `target/` paths and are deleted or
recreated by the gate. No existing user AVD is mutated. No key, secret,
network, persistent storage or Android Keystore surface is reachable. One
emulator receipt is evidence, not a support or certification claim.

Unsafe and native-code evidence is explicit: no authored unsafe Rust is added;
the NDK-built Rust cdylib and JNA dispatch library are dependency/toolchain
native code quarantined behind the facade and inspected as artifacts. The
maintenance, release and security posture stays experimental, local and
slow-lane-only with no release artifact or compatibility promise.

## Rejected or deferred candidates

The table records `not-adopt` for copied/shadowed JNA, hand-authored JNI and
committed generated outputs. Managed devices and wider ABI/runtime coverage are
deferred until a named consumer or support milestone justifies their larger
cone. Reconsideration trigger: a supported Maven distribution, a UniFFI change
away from JNA, a JNA security/advisory change, or failed arm64 runtime evidence
requires this decision to be reopened.

Rollback removes the dedicated Android toolchain target, fixture, gate and
evidence without changing ABI version 1 or the accepted host/Apple proofs.

## Open questions and blockers

There are no research blockers. Emulator boot, exact Gradle locking and hosted
SDK availability remain implementation checks; a failed or unrun check must be
reported and cannot be converted into passing evidence.

## Evidence commands

Pre-code commands built `identus-uniffi-did` with the NDK API-21 linker, used
`llvm-readelf` and `llvm-nm` to inspect ELF metadata and symbols, generated
Kotlin with the exact locked bindgen tool, listed the JNA AAR with `unzip`, and
validated its checksum with `shasum -a 256`. Implementation will run
`./scripts/check-uniffi-did-android.sh`, focused Cargo/Clippy tests,
`./scripts/factory check`, dependency review and compatible `nix flake check`.
Hosted CI and publication are deliberately unrun at specification time.
