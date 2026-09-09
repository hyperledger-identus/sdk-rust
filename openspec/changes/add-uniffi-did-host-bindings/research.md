# Native DID binding foundation research

Research class: storage-ffi
Research status: ready
Decision date: 2026-09-09
Source retrieval date: 2026-09-09
Research blockers: none

## Problem and existing implementation

The current implementation has no supported FFI. `identus-bindings` is a quarantined marker, while
the #215 fixture proves a value-only UniFFI call shape outside the workspace.
`identus-did` already owns the normative parser, exact round trip, component
views, 2,048-byte DID and 4,096-byte DID URL limits, and redaction-safe public
error codes. Production should reuse that domain component rather than fork its
grammar or annotate it with a binding framework.

The accepted foundation must make the ABI contract explicit while remaining
unpublished and removable. It must not imply that a host test is a mobile
package, device, store or certification proof.

## Normative sources

- UniFFI 0.32 guide and library-mode contract:
  https://mozilla.github.io/uniffi-rs/latest/
- UniFFI release `v0.32.0`, commit
  `5c7b73906358e1a7acdc1bdc7bf5cd86fb27e44c`, MPL-2.0:
  https://github.com/mozilla/uniffi-rs/tree/v0.32.0
- W3C DID Core 1.0 DID syntax:
  https://www.w3.org/TR/did-core/#did-syntax
- SDK decision ADR 0097 and executable fixture at
  `docs/research/uniffi-did-spike`, merge commit
  `808119d7f3b6d52c8fd89cfae68f0d61cb68d402`, Apache-2.0 repository code.
- Current `identus-did` parser at the same base commit, Apache-2.0.

No donor repository code is copied and no donor or consumer repository is
mutated.

## Candidate decisions

| Candidate | Version/revision | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- | --- |
| New `identus-uniffi-did` outer-boundary crate | repository-local | `retain-local` | Highest cohesion: the binding ABI owns DTO/error/version policy and `identus-did` remains reusable | Replace only through a versioned migration ADR |
| Reuse `identus-did` parser | `develop@808119d` | `adopt` | Already bounded, tested and W3C DID Core-aligned; avoids duplicate grammar | Parser contract or limits change incompatibly |
| UniFFI proc macros and compiled-library generation | 0.32.0 / `5c7b7390` | `conditional-adopt` | #215 proved deterministic Swift/Kotlin output and runtime calls | Host/mobile ABI, audit or maintenance gates fail |
| Exact separate bindgen tool | UniFFI 0.32.0 | `adopt` | Keeps CLI/template dependencies outside the runtime workspace cone while binding generator and runtime versions | Upstream offers a narrower pinned generator artifact |
| Closed errors carrying a stable `code` field | repository-local | `retain-local` | Swift and Kotlin can branch on stable machine data without caller text or Rust error details | Consumer evidence requires a versioned structured detail schema |
| Manual `Display`/`Error` implementation | repository-local | `retain-local` | Avoids adding `thiserror` to a two-error facade and preserves exact redacted text | Error surface grows enough to justify the dependency |
| Replace `identus-bindings` placeholder now | repository-local | `not-adopt` | It would broaden a generic namespace and the machine support policy before mobile/package evidence | #222 completes supported native package/runtime evidence |
| Commit complete generated language sources | UniFFI 0.32.0 | `not-adopt` | Reproducible generation plus normalized snapshots is smaller and avoids stale derived source | Packaging requires generated-source artifacts with provenance |

## Compatibility and dependency evidence

The runtime crate adds only `identus-did` and `uniffi = =0.32.0` with default
features disabled. The separate tool enables only UniFFI's CLI feature under its
own nested workspace and lock. This records the direct and resolved dependency cone: #215 measured 54 unique package/version
renderings in the combined default runtime/build graph and 79 with CLI tooling;
the new layout prevents the CLI-only graph from entering the root runtime lock.

UniFFI is MPL-2.0, pre-1.0, edition 2021 and declares no package MSRV. The exact
release passed the SDK Rust 1.98.1 etalon in #215. Target evidence currently
covers the macOS host plus the ordinary Rust target gates; it does not cover
mobile runtime. Its implementation and
generated scaffolding contain dependency-owned unsafe/native behavior; authored
SDK source remains under `unsafe_code = "forbid"`. The production locks must
pass the repository-pinned supply-chain license/advisory gates. License and
provenance are pinned above. No external crate type crosses the ABI.

Public and wire compatibility is additive and experimental: no existing Rust
wire value changes, and ABI version `1` is new rather than a replacement.

## Security, privacy and maintenance evidence

Only owned, public identifier strings cross the ABI. The wrapper rejects input
through `identus-did` before returning a record, maps all domain failures to
closed error variants, carries constant SDK-owned codes, and discards domain
details. It catches unexpected unwind before returning from authored wrapper
logic and maps it to a constant internal code; panic payloads are never exposed
as return data. Generated UniFFI scaffolding remains a dependency-owned native
boundary and is covered by version pinning and executable smoke tests.

The wrapper is stateless and exports no object handles, borrowed memory,
callbacks, futures or thread-affine values. Every return is an owned value.
`binding_api_version()` returns `1`; breaking DTO, function or error changes
require a versioned ADR and snapshot update. Pre-1.0 Rust crate version `0.0.0`
is not the cross-language ABI version.

Maintenance, release and security posture remains deliberately pre-release:
the component is experimental repository code, not a published support claim.
`SDK-LIM-002` remains effective because there is no XCFramework/SwiftPM or
AAR/NDK package and no iOS/Android runtime proof. Host Swift and Kotlin/JVM are
verification environments only.

Protocol/draft currency is stable for DID syntax through W3C DID Core 1.0; UniFFI
is an implementation mechanism rather than an identity protocol or draft.

## Rejected or deferred candidates

UDL-first and hybrid definitions remain rejected by ADR 0097 because this slice
has no independently governed external ABI schema or macro expressiveness gap.
Hand-written C/JNI/Swift bindings duplicate memory and error lowering. Diplomat
and CXX do not provide the accepted direct Swift/Kotlin contract. React Native
and browser WASM remain #223/#224. Secrets, objects, callbacks and async work are
deferred until separate threat and lifecycle contracts exist.

## Open questions and blockers

No implementation blocker remains. Exact generated shapes for error fields and
the API-version function will be captured by the first generation and reviewed
before archive. If UniFFI cannot express a closed error with a stable code field,
implementation must stop and revise the contract rather than substituting
caller-readable messages.

## Evidence commands

Exact commands will run locked root build/test/Clippy/doc/audit/deny gates,
including `cargo test --workspace --all-features`, `cargo clippy --workspace
--all-targets --all-features -- -D warnings`, `cargo deny check` and
`nix flake check`; plus locked tool generation,
two complete-tree diffs, normalized API snapshot diffs, Swift compile/link/run,
Kotlin 2.2.20/JVM 17/JNA 5.18.1 compile/link/run, first-party unsafe inventory,
root/tool dependency graphs, factory checks and full compatible-host Nix flake.
Mobile packaging/device, React Native, browser/Node, publication and release are
intentionally unrun checks and remain outside #226. Rollback removes the new
crate/tool/tests and inventory entry without changing the domain crate or data.
