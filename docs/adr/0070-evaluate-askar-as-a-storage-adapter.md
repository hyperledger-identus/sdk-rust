# ADR 0070: evaluate Askar as a storage adapter

- **Status:** Accepted
- **Date:** 2026-09-08
- **Issue:** [#175](https://github.com/hyperledger-identus/sdk-rust/issues/175)
- **Implementation spike:** [#162](https://github.com/hyperledger-identus/sdk-rust/issues/162)
- **Decision authority:** [discussion #174](https://github.com/hyperledger-identus/sdk-rust/discussions/174) and ADR 0061
- **Assessed source:** [`openwallet-foundation/askar@48a49592`](https://github.com/openwallet-foundation/askar/tree/48a495920e2166771c6be1aca2fd057a8b0c8831), MIT OR Apache-2.0
- **Research:** [repository portfolio](../research/rust-library-reuse/repository-portfolio.md)

## Context

Askar is a maintained encrypted-record and software-key implementation with
useful native/mobile evidence. Its default graph brings database, migration,
logging, FFI, runtime, native, unsafe, and cryptographic policy that cannot
enter generic domain crates implicitly. It is not a replacement for platform
secure enclaves, Android Keystore, browser storage, HSMs, or remote signers.

## Decision

Classify Askar as `spike` for an optional native adapter implementing SDK-owned
secret-storage/key-store ports. Evaluate an exact source/release with
`default-features = false` and only minimum backend features. No Askar type may
cross a public, FFI, storage-record, or error boundary.

Issue #162 must prove transactions, concurrency/cancellation, migrations,
redaction, zeroization, key-handle behavior, native linking, reachable unsafe,
platform packaging, and a no-database core boundary before a later ADR may
approve production integration.

## Consequences

The SDK has a bounded path to evaluate mature software storage without making
Askar the custody policy. Production adoption remains undecided.

## Reconsideration and rollback

Promote to `conditional-adopt` only after #162 passes and records a measured
dependency/security receipt. Rollback is documentation-only.
