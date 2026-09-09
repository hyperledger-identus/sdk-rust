# Exact-diff architecture and security review

Review status: completed
Review date: 2026-09-09
Evidence head: ff298fb5eb4dde38c64eec5b2d3e1719093243ac
Specification commit: 480ab60fa7a8cde8330e8810ddf7ca6e6d72d92b
Unresolved blockers: none

## Scope reviewed

The review inspected the complete `origin/develop`-to-evidence-head diff, the
existing DID parser/error bridge, fixture manifests and locks, wrapper source,
both language programs, generated API extractors/snapshots, complete generated
outputs, dependency trees, upstream release manifests and the ADR/report. It
also verified that root manifests/lock and all production crates are unchanged.

## Findings

1. **Repository boundary — accepted.** The fixture is an independent unpublished
   nested workspace. No UniFFI dependency, annotation or type enters
   `identus-did`, `identus-bindings`, the root manifest/lock or another
   production crate. No generated source/build artifact is tracked.
2. **Value and resource boundary — accepted.** Only owned public identifier
   strings cross the ABI. DID and DID URL parsing retains the SDK's
   2,048/4,096-byte ceilings, exact representations and component semantics.
   There are no secrets, handles, callbacks, futures, storage or network calls.
3. **Error boundary — accepted with production follow-up.** Swift and Kotlin
   receive closed invalid-DID/invalid-DID-URL cases and neither runtime reflects
   caller text. Production #222 must add stable code access because Kotlin's
   generated exception message is empty and Swift defaults to enum identity.
4. **Generation — accepted.** Two complete library-mode generations are
   byte-identical and the normalized snapshots cover every exported record,
   property, error case and function. The generator/tool feature is isolated,
   although production should use a separate build tool rather than compile CLI
   packages into a runtime feature graph.
5. **Supply chain — accepted for research only.** Versions and Cargo/Gradle
   locks are committed; UniFFI and React Native candidate licenses/revisions are
   pinned. The UniFFI source contains dependency-owned unsafe and native FFI
   behavior. The host `cargo-audit` cannot parse current CVSS 4.0 database
   syntax. These are explicit #222 production gates, not waived assurance.
6. **Platform separation — accepted.** Swift/Kotlin host runtime evidence
   supports a native implementation issue. Current React Native 0.31 coupling
   fails the 0.32 compatibility gate and browser React has a separate WASM
   contract. Issues #222, #223 and #224 preserve those boundaries.
7. **Reproducibility limitation — accepted.** The macOS fixture uses installed
   Swift, arm64 JDK 17 and Gradle 8.12.1. Dependency versions are locked but the
   host tools and downloaded artifact checksums are not production-pinned. This
   is sufficient to decide the research direction, not to claim mobile or
   release reproducibility.

## Validation review

The fixture's Rust, Swift and Kotlin runtime proof passes. Root all-feature and
no-default tests, docs, factory checks and the full compatible-host Nix flake
pass. A direct host Cargo Clippy command reports four new Rust 1.98 lint findings
in unchanged `develop` files; the pinned Nix Clippy gate passes and the diff
contains no production source changes. This is recorded as a host/base mismatch
rather than repaired inside the research issue.

## Decision

Accept ADR 0097's native direction and deliver the research-only evidence. Keep
FFI unsupported until #222 proves its exact production/mobile contract. No
correctness, architecture, security, privacy, compatibility, provenance or
scope blocker remains for this research PR.
