# ADR: DIDComm Pack/Unpack Dependency Strategy

**Status**: Proposed

**Date**: 2026-06-13

## Context

`sdk-rust` must support DIDComm v2-era pack/unpack, forwarding, mediation,
pickup, OOB, issue credential, present proof, problem reporting, and revocation
notification across server, web, mobile, Node, Swift, Kotlin, and TypeScript
wrappers. The Rust core also needs strict control over DID resolution,
cryptographic providers, key handles, observability, conformance fixtures, and
secret redaction.

The first DIDComm implementation increment added typed message type, protocol
catalog, message id, thread id, and plaintext envelope primitives in
`identus-messaging`. It intentionally did not add a pack/unpack dependency.

## Decision

Do not add a DIDComm pack/unpack crate to `identus-messaging` yet.

The next implementation increment must introduce an adapter spike behind
Identus-owned ports. The spike should keep the core domain model independent
from upstream message structs and should prove these interfaces before any
dependency becomes part of the public API:

- `DidCommPacker`: pack authenticated, anonymous encrypted, and signed messages.
- `DidCommUnpacker`: unpack messages and return typed plaintext envelopes plus
  protected metadata.
- `DidResolverPort`: resolve DID documents through `identus-did` owner traits.
- `KeyAgreementPort` and `SignerPort`: use `identus-crypto` key handles rather
  than exposing raw private keys to the DIDComm implementation.
- `SecretResolverPort`: retrieve secrets from wallet/server storage adapters
  with redaction-safe errors and tracing.

The default direction is adapter-first reuse: evaluate wrapping a crate for
pack/unpack internals, but keep Identus-owned domain types, resolver ports,
crypto ports, fixtures, and binding DTOs as the stable boundary.

## Candidate Comparison

| Candidate | Current Cargo signal | DIDComm v2.1 coverage | Crypto provider control | Resolver hooks | Dependency and target risk | Binding fit | Conformance signal | Assessment |
|---|---|---|---|---|---|---|---|---|
| `didcomm` 0.4.1 | Apache-2.0, `sicpa-dlab/didcomm-rust`, docs.rs, UniFFI and testvector features, unknown MSRV | Claims DIDComm for Rust; needs fixture validation against Identus protocol set | Must be verified; unknown from Cargo metadata alone | Must be verified | Potentially smaller surface; unknown MSRV and maintenance cadence require review | UniFFI feature is promising for Swift/Kotlin wrapper experiments | Testvector feature is useful, but must be mapped to checked-in conformance fixtures | Good first spike candidate if APIs allow external DID/secret resolvers and controlled crypto. |
| `didcomm-rs` 0.7.2 | Apache-2.0, decentralized-identity repo, default features include raw crypto, resolve, OOB | Targets DIDComm v2 specification and OOB | Default `raw-crypto` bundles algorithms directly; must avoid leaking provider ownership into core | `resolve` feature uses `ddoresolver-rs`; needs adapter isolation | Feature surface is broad and may pull crypto/resolver dependencies into non-server targets | No explicit binding feature in Cargo metadata | Useful as a reference implementation and vector source | Evaluate as reference or optional adapter, but do not expose its types across core boundaries. |
| `affinidi-messaging-didcomm` 0.15.1 | Apache-2.0, Affinidi TDK, DIDComm v2.1, Rust 1.95.0, optional `messaging-core` feature | Strong v2.1 signal and active messaging ecosystem | Must be verified; likely better for always-online messaging flows than minimal core primitives | Must be verified through TDK traits | Rust 1.95.0 matches current workspace; broader TDK ecosystem may be heavier for WASM/mobile | Needs binding and target review; no explicit UniFFI signal in Cargo metadata | Potentially strong v2.1 behavior source | Good second spike candidate, especially for mediator/client service adapters. |

Cargo search also surfaced related crates such as `co-didcomm`,
`affinidi-messaging-didcomm-service`, and `affinidi-messaging-sdk`. Those should
be tracked as ecosystem signals, not initial core candidates, unless the first
three candidates fail the port/control requirements.

## Required Spike Acceptance Criteria

Before a pack/unpack dependency can be added to a non-test crate:

- The spike must replay checked-in DIDComm transcript fixtures without changing
  fixture schema.
- Packed and unpacked messages must round-trip into `DidCommPlaintextMessage`
  and typed `DidCommMessageType` values.
- DID resolution must flow through `identus-did` ports.
- Signing, key agreement, and secret lookup must flow through `identus-crypto`
  and wallet/server storage ports; raw private key material must not cross the
  public adapter boundary.
- The dependency must be tested for Linux/server, macOS/iOS build viability,
  Android/JVM through UniFFI feasibility, and WASM/Node feasibility or explicit
  feature-gated exclusion.
- Negative tests must cover malformed envelopes, unsupported algorithms,
  missing secrets, wrong recipient, unknown DID method, unsupported key
  agreement, tampered ciphertext, and redaction-safe error reporting.
- The adapter must expose tracing hooks that never log plaintext bodies,
  claims, tokens, private keys, or decrypted attachments.

## Consequences

- `identus-messaging` remains dependency-light while protocol semantics and
  fixture coverage mature.
- Upstream crates can still be reused, but only behind Identus-owned ports.
- If no candidate can satisfy crypto/provider/resolver control, `sdk-rust`
  should implement pack/unpack natively against `identus-crypto` and use
  upstream crates only as test-vector/reference material.
- The decision keeps Swift/Kotlin/TypeScript wrappers free from accidental
  upstream API lock-in.

## Follow-Up Tasks

- T068: Add a pack/unpack adapter spike task for `didcomm`, `didcomm-rs`, and
  `affinidi-messaging-didcomm`.
- Add negative conformance fixtures for pack/unpack and mediator forwarding.
- T069: Add target-build checks for server, WASM, Node, iOS, and Android before
  promoting any dependency to production.
