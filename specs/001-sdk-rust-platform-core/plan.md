# Implementation Plan: Identus Rust SDK Platform Core

**Branch**: `agent/manager/sdk-rust-seed-spec` | **Date**: 2026-06-13 | **Spec**: `specs/001-sdk-rust-platform-core/spec.md`

**Input**: Feature specification from `specs/001-sdk-rust-platform-core/spec.md`

## Summary

Seed the repository as a Spec Kit-managed Rust SDK monorepo. The first implementation increment creates the workspace skeleton, crate ownership boundaries, architecture diagrams, and docs-derived baseline needed to develop the platform safely. Production protocol implementation is deliberately deferred until crate boundaries and conformance fixtures are reviewed.

## Technical Context

**Language/Version**: Rust stable 1.95.0 in the current workspace, edition 2024 for the initial skeleton.

**Primary Dependencies**: Cargo workspace, optional `serde`, `thiserror` or `derive_more`, `tracing`, `tokio` for async adapters, JOSE/COSE/CBOR/JSON-LD dependencies behind crate features, `wasm-bindgen`/`napi`/`uniffi` in binding crates only.

**Storage**: Domain storage ports first; adapters for in-memory, SQLite, mobile secure storage, browser storage, and future server stores.

**Testing**: `cargo test`, shared fixture tests, protocol negative tests, markdown/yaml/editorconfig linting, dependency review, scorecard, coverage, and future conformance suites.

**Target Platform**: Server, CLI, WASM/web, Node.js, iOS/macOS, Android/JVM, React Native through bindings.

**Project Type**: Rust library monorepo with optional adapter/binding packages.

**Performance Goals**: Domain crates avoid unnecessary allocation; verification and DID resolution expose async APIs; mobile and WASM bindings keep payload copies explicit and measurable.

**Constraints**: Secret redaction by default, no platform dependencies in domain crates, protocol behavior fixture-tested across bindings, DCO/GPG signed commits, signer abstractions for hardware keys/KMS/mobile secure enclaves.

**Scale/Scope**: Multi-year SDK and service-core replacement path covering three legacy SDKs, future Mediator/Cloud-Service Rust ports, and neoprism convergence.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- Rust core owns product semantics: PASS for this seed; bindings are planned as wrappers.
- Hexagonal architecture: PASS; crate families separate domain, ports, adapters, and bindings.
- Standards-first interoperability: PASS; standards matrix is included in `research.md`.
- Cross-platform bindings without forked logic: PASS; DTO and fixture requirements are explicit.
- Security, privacy, and conformance gates: PASS; tests and threat-model notes are required before implementation.
- Neoprism-grade quality: PASS; CI and workspace quality targets are named.
- SSI domain naming: PASS; crate families and new public surfaces use domain
  names, while historical SDK codenames are restricted to source evidence in
  migration material.
- Type-safe SSI primitives: PASS; the first DID slice adds validated `Did` and
  `DidUrl` types before method-specific resolver adapters.
- Every increment has acceptance criteria: PASS; the DIDComm v2 seed is tied to
  tasks T063-T067 with concrete acceptance criteria and executable validation
  hooks before commit.

## Project Structure

### Documentation

```text
specs/001-sdk-rust-platform-core/
|-- spec.md
|-- research.md
|-- plan.md
`-- tasks.md          # Created in the next Spec Kit phase.
```

### Source Code

```text
crates/
|-- foundation/
|-- crypto/
|-- did/
|-- credentials/
|-- presentations/
|-- messaging/
|-- openid4vc/
|-- trust/
|-- wallet/
|-- adapters/
`-- bindings/

fixtures/
|-- did/
|-- credentials/
|-- presentations/
|-- didcomm/
`-- openid4vc/

tests/
|-- conformance/
|-- integration/
`-- bindings/
```

## Phase 0 Output

Completed in `research.md`.

## Components And Relations

The component model is recorded in `docs/architecture/components.md`.

| Component | Crate | Depends on | Primary role |
|---|---|---|---|
| Core | `identus-core` | none | Shared DTOs, errors, fixture contracts, metadata |
| Crypto | `identus-crypto` | Core | Cryptographic key management and signer abstraction |
| DID | `identus-did` | Core, Crypto | DID Core, PRISM DID, peer DID, and resolver ports |
| Trust | `identus-trust` | Core, Crypto, DID | Policy, federation, X.509/IACA, attestations, status trust |
| Credentials | `identus-credentials` | Core, Crypto, DID, Trust | Credential formats, issuance models, and verification |
| Presentations | `identus-presentations` | Core, Credentials, Trust | Presentation Exchange, DCQL, disclosure and VP checks |
| Messaging | `identus-messaging` | Core, Crypto, DID | DIDComm state machines, mediation, pickup, and routing |
| OpenID4VC | `identus-openid4vc` | Core, Credentials, Presentations, Trust | OID4VCI, OID4VP, SIOPv2, HAIP |
| Wallet | `identus-wallet` | Domain/protocol crates | Wallet storage and agent orchestration |
| Agent | `identus-agent` | Core, then Wallet/Messaging/OpenID4VC ports | Lightweight issuer, holder, verifier, peer, and embedded mediator acceptance-test model |
| Adapters | `identus-adapters` | Ports and Wallet | HTTP, storage, VDR, signer, transport adapters |
| Bindings | `identus-bindings` | Core, Wallet | WASM, Node, UniFFI, Swift, Kotlin, TypeScript DTO surface |

## Implementation Phases

### Phase 1 - Workspace And Architecture Skeleton

- Create a Cargo workspace with component crate families and dependency direction.
- Add architecture documentation and Mermaid diagrams.
- Add docs-derived specification baseline from the Identus docs repository.
- Validate with `cargo check --workspace`, `cargo test --workspace`, Spec Kit checks, and JSON parsing.

### Phase 2 - Fixtures And Migration Matrix

- Add legacy SDK module migration matrix.
- Add BDD scenario inventory grouped by feature, capability, and use case.
- Add lightweight agent and embedded mediator acceptance-test harness so SDK
  behaviors can be ported without Docker-first infrastructure.
- Define fixture schemas for deterministic PRISM DID, keys, credential verification, DIDComm transcripts, and OpenID4VC transcripts.
- Add deterministic DID test vector from `repos/docs/documentation/develop/cloud-agent/deterministic-did-creation.md`.
- Add DID method inventory and dependency candidate notes for PRISM, peer, web,
  key, jwk, pkh, and example/test DIDs.

### Phase 3 - Foundation Crates

- Implement core error/DTO conventions, key handles, signer traits, DID parsing, resolver traits, trust/status policy traits, and negative tests.

### Phase 4 - Credential And Presentation Core

- Implement credential model registry, JWT VC, SD-JWT VC, AnonCreds adapter choice, OpenBadges profile, mdoc design, Presentation Exchange, and DCQL boundaries.

### Phase 5 - Protocol State Machines

- Implement DIDComm connection/issue/present/mediation/routing/problem/revocation state machines.
- Start with type-safe DIDComm v2 message-type, thread-id, protocol catalog,
  and plaintext-envelope primitives before pack/unpack or protocol state
  machines are introduced.
- Implement OID4VCI and OID4VP/SIOPv2 state machines with request URI and direct_post support.

### Phase 6 - Adapters And Bindings

- Add in-memory/SQLite/secure storage, HTTP, NeoPRISM VDR, KMS/mobile signer adapters.
- Add WASM, Node, UniFFI, Swift, Kotlin, and TypeScript proof-of-concept bindings.

### Phase 7 - Service Reuse And Deprecation Path

- Prove Cloud-Service, Mediator, and neoprism reuse with integration spikes.
- Publish migration guides for sdk-ts, sdk-swift, and sdk-kmp wrappers.

## Phase 1 Design Direction

- Define fixture schema before implementing behavior.
- Add a migration matrix from legacy SDK modules to crate families.
- Add ADRs for neoprism convergence, OpenID4VC conformance, binding strategy, and secure storage.
- Keep phase-1 crates compile-only and avoid production protocol behavior.
- Use SSI domain naming for all new public surfaces. Legacy SDK codenames may
  appear only as source evidence in migration and compatibility documents.

## Phase 2 Task Generation Notes

The first migration matrix is now checked in at
`docs/migration/legacy-sdk-map.md`. It maps sdk-ts, sdk-swift, sdk-kmp, and
neoprism source modules to Rust owner crates, wrapper targets, parity proofs,
and backlog follow-ups. Future `/speckit-tasks` output should split the
remaining work by crate family and wrapper gate, and avoid protocol behavior
outside fixture-backed state-machine boundaries.
