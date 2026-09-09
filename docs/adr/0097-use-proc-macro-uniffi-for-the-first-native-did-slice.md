# ADR 0097: use proc-macro UniFFI for the first native DID slice

- **Status:** Accepted for a future isolated native implementation
- **Date:** 2026-09-09
- **Decision authority:** issue #215 under the SDK binding roadmap and the
  effective no-FFI limitation `SDK-LIM-002`
- **Related work:** #163; Discussion #178; ADRs 0061 and 0081

## Context

Apollo offers Kotlin, JavaScript and Swift consumable artifacts while SDK-Rust
has only Rust target compilation. Closing that distribution gap eventually
requires language bindings, but beginning with cryptographic secrets, storage,
async protocols or object handles would combine interface selection with the
highest-risk lifetime and custody questions.

`identus-did` already provides deterministic chain-neutral DID and DID URL
parsing with 2,048/4,096-byte ceilings, exact round trips and stable redacted
SDK errors. It is useful enough to test records, optional values, fallible
calls and generated language ergonomics without secrets or mutable state.

UniFFI 0.32.0 supports Swift and Kotlin directly through UDL or proc macros.
Library-mode generation extracts metadata from a compiled library and is the
upstream-recommended direction; single-UDL generation is deprecated. Current
`uniffi-bindgen-react-native` 0.31.0-5 is a separate MPL-2.0 project pinned to
UniFFI 0.31, so it cannot establish compatibility with the selected native
0.32 line. Browser JavaScript has a distinct WASM/runtime/package boundary.

## Evidence

The unpublished fixture in `docs/research/uniffi-did-spike` uses exact UniFFI
0.32.0 and Rust 1.98.1. It keeps `identus-did` unchanged and exposes two
functions, two SDK-owned records and a closed two-case error enum. Four Rust
tests plus compiled Swift 6.3 and Kotlin 2.2.20/JNA 5.18.1 host programs prove:

- DID and DID URL exact round trips and component mapping;
- invalid and oversized values return stable cases without caller-text
  reflection;
- two complete generated trees are byte-identical; and
- normalized Swift/Kotlin API snapshots match the reviewed surface.

The isolated lock contains 85 packages. The default normal/build graph contains
54 unique package/version renderings including the SDK/DID graph; enabling the
in-fixture bindgen CLI contains 79. No package declares Cargo `links`, but
runtime loading uses platform dynamic libraries and Kotlin uses JNA. A source
scan of the UniFFI 0.32 crates found 47 `unsafe {}` lines in runtime/macro code;
all are dependency-owned and none changes the SDK first-party unsafe policy.
All package license expressions are known; UniFFI is MPL-2.0. The installed
`cargo-audit` could not parse a CVSS 4.0 entry in the current advisory database,
so this local run is recorded as tool failure rather than a clean audit claim.
Production #163 must rerun supply-chain gates with the repository-pinned tool.

## Decision

1. The first production native binding candidate is bounded DID/DID URL parsing
   and validation.
2. Use UniFFI 0.32 proc macros and compiled-library generation. Do not use UDL
   as the primary definition and do not mix UDL and macros without a specific
   expressiveness or externally owned ABI requirement.
3. Keep UniFFI out of generic domain crates. A future `identus-ffi-did`-style
   isolated crate owns all annotations, generated metadata, DTOs and error
   lowering. No Rust, UniFFI or third-party domain type crosses the ABI.
4. Expose owned values and stable redacted error cases/codes only. Secrets,
   signing, storage, networking, callbacks, futures and object handles remain
   outside the first implementation.
5. Do not commit generated Swift/Kotlin source by default. Generate from an
   exact lock, compare the complete tree twice, and commit normalized public API
   snapshots for review. Packaging may later choose generated-source artifacts
   through a separate reproducibility decision.
6. Accept the native slice for implementation under a new child of #163, but
   retain `SDK-LIM-002` until device/runtime/package evidence and a versioned
   production contract pass.
7. Treat React Native and browser React as separate adapter programs. Do not
   adopt `uniffi-bindgen-react-native` until an exact UniFFI 0.32-compatible
   release passes iOS and Android TurboModule evidence. Research browser React
   through a separate `wasm-bindgen` adapter and bundler/runtime contract.

## Consequences

The SDK gains a tested direction without adding a production dependency or
support promise. Swift and Kotlin consumers can receive an idiomatic bounded
first surface once #163's child implementation completes. The domain model
stays cohesive and reusable by Rust, native and WASM adapters.

The future native crate accepts a material dependency cone and dependency-owned
unsafe/native behavior. Its production change therefore needs explicit panic,
ownership, memory, threading, versioning, artifact and platform gates. A host
smoke test is not iOS/Android packaging, device, store or certification proof.

## Reconsideration and rollback

Reconsider the generator if UniFFI fails a named consumer/API requirement, its
pre-1.0 contract changes incompatibly, its maintained release no longer passes
Rust 1.98.1/target/security gates, or another generator materially narrows the
same Swift/Kotlin proof. Reconsider UDL only when an independently governed ABI
schema must be authoritative.

This research decision rolls back by reverting its issue-linked PR. A future
production binding rolls back by removing its isolated crate and generated
artifacts; generic domain crates and downstream persisted data remain intact.
