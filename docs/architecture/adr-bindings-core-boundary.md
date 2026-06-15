# ADR: Binding Core Boundary

Status: Accepted

Date: 2026-06-13

## Context

`sdk-rust` is intended to replace duplicated SDK semantics while preserving
usable TypeScript, Kotlin, Swift, React, React Native, browser, mobile, and
server packages. Existing language SDKs must not be recreated as independent
logic stacks. They must become wrappers over Rust-owned primitives, ports, and
conformance fixtures.

The binding boundary must also respect the constitution rules:

- Rust core owns product semantics and protocol state.
- Names exposed by new Rust crates, modules, public APIs, docs headings, and
  backlog items must use SSI domain language, not legacy SDK codenames except
  when citing historical source evidence.
- Each increment needs a backlog task with acceptance criteria.
- Secrets and raw private keys must not cross public Rust, UniFFI, WASM, Node,
  Swift, Kotlin, TypeScript, React, or React Native APIs.
- Binding behavior must be proven by the same checked-in conformance fixtures
  used by the Rust crates.

## Decision

`identus-bindings` owns the public binding contract. It exposes stable target
and facade metadata before implementation-specific bridge crates are added.

The first supported targets are:

| Target | Bridge | Package family | Purpose |
|---|---|---|---|
| Browser/WASM | `wasm-bindgen` | TypeScript and browser packages | Web wallets, browser verification, QR/deep-link flows. |
| Node/N-API | `napi-rs` | Node and server-side TypeScript packages | Server tools, test runners, backend integrations. |
| UniFFI | UniFFI | Shared Swift and Kotlin bindings | Common mobile binding ABI and DTO generation. |
| Swift | SwiftPM over UniFFI | iOS, macOS, and React Native native module support | Apple secure storage, deep links, and wallet UX. |
| Kotlin | Gradle/KMP over UniFFI | Android and JVM wrapper support | Android secure storage, JVM test runners, mobile UX. |
| TypeScript | Generated facade over WASM or N-API | Browser, Node, and React wrapper support | Existing TS package migration and web developer ergonomics. |
| React | TypeScript package over browser facade | React hooks and components | Web application integration. |
| React Native | TypeScript package over native modules | Mobile app integration | Mobile wrapper over Swift/Kotlin native bindings. |

The first facade surfaces are:

| Surface | Owner crate | Binding responsibility |
|---|---|---|
| Agent | `identus-agent` | Issuer, holder, verifier, peer, embedded mediator harness, and high-level workflow events. |
| Wallet | `identus-wallet` | Wallet records, backup/restore, secure-store handles, credential inventory, and DID inventory. |
| DID | `identus-did` | DID parsing, DID URL parsing, PRISM/peer/web/key/jwk/pkh method support, and resolver handles. |
| Credential | `identus-credentials` | Credential parsing, verification, status checks, and typed verification errors. |
| Presentation | `identus-presentations` | Presentation request parsing, selection, disclosure, and verifier result DTOs. |
| Messaging | `identus-messaging` | DIDComm message parsing, protocol state transitions, and transport-independent events. |
| OpenID4VC | `identus-openid4vc` | OID4VCI, OID4VP, SIOPv2, HAIP, and federation-backed request/response DTOs. |
| Trust | `identus-trust` | Trust anchors, trust-chain result DTOs, status-list result DTOs, and policy decisions. |
| Adapters | `identus-adapters` | Platform-specific storage, signer, resolver, transport, VDR, and proximity adapters. |

Bindings must exchange JSON-compatible DTOs, typed error codes, opaque handle
ids, and explicit byte buffers. Public APIs must not expose Rust lifetimes,
generic types, trait objects, raw pointers, raw private keys, or transport-
specific implementation details.

## Safety Rules

- Binding APIs return typed result envelopes with stable error codes and
  redaction-safe diagnostics.
- Long-running protocol operations report deterministic events instead of
  hiding state transitions inside wrapper-specific callbacks.
- Async APIs must expose cancellation and timeout behavior per target.
- Secret material is represented by opaque key handles and secure-store handles.
- Browser/WASM builds must not assume filesystem, threads, or unrestricted
  network access.
- Mobile bindings must route secure storage, biometric gating, notifications,
  and deep links through adapter ports.
- React and React Native libraries must stay thin; hooks and components can
  improve ergonomics but cannot own credential, DID, DIDComm, OpenID4VC, or
  trust semantics.
- Wrapper packages must replay the same fixtures as Rust core before claiming
  parity for a capability.

## Conformance Requirements

Every binding target must pass:

- Static facade inventory tests from `identus-bindings`.
- JSON DTO round-trip fixtures shared with Rust core.
- Typed error parity fixtures for credential, presentation, DID, messaging,
  OpenID4VC, trust/status, and storage failures.
- No-secret-leak tests for logs, errors, event payloads, and serialized DTOs.
- Target-build checks before any bridge crate is promoted from experimental to
  supported.

Wrapper API parity inventories for existing TS, Swift, and Kotlin packages
remain a separate backlog item because they should be generated from source
exports and checked in as machine-readable manifests.

## Consequences

- Rust crates can evolve internal APIs while binding DTOs remain stable.
- Existing SDKs can migrate capability-by-capability without forked semantics.
- Platform packages retain native ergonomics while conformance remains shared.
- Future Cloud-Service, Mediator, and NeoPRISM service ports can reuse the same
  DTOs and error envelopes for service APIs where useful.
