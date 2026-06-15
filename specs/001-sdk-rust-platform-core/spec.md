# Feature Specification: Identus Rust SDK Platform Core

**Feature Branch**: `agent/manager/sdk-rust-seed-spec`

**Spec Kit Feature**: `001-sdk-rust-platform-core`

**Created**: 2026-06-13

**Status**: Draft

**Input**: User description: "Create the seed specification for `sdk-rust` as the Rust monorepo core that reaches and exceeds feature parity with `sdk-ts`, `sdk-swift`, and `sdk-kmp`, enables TypeScript/Kotlin/Swift/web/mobile bindings, and later becomes the shared base for Mediator, Cloud-Service, and neoprism module convergence."

## User Scenarios & Testing

### User Story 1 - Plan The Unified SDK Surface (Priority: P1)

As an Identus platform architect, I need one specification that maps existing SDK capabilities and backlog gaps to Rust crates so the team can execute without re-litigating module boundaries.

**Why this priority**: Without an agreed crate map and capability inventory, later implementation work will recreate the fragmented SDK split in a new language.

**Independent Test**: Reviewers can compare this spec and research document against the current SDK module families and backlog themes, then confirm every existing SDK domain has a Rust crate home.

**Acceptance Scenarios**:

1. **Given** the existing legacy SDK capability model, **When** a maintainer reads the Rust crate map, **Then** each cryptography, DID, credential, presentation, DIDComm, mediation, wallet, and agent capability has a target domain crate and at least one adapter path.
2. **Given** backlog items for OID4VCI, OID4VP, OpenBadges, multi-mediator, DID rotation, revocation, and deterministic PRISM DID creation, **When** the spec is reviewed, **Then** each backlog family is represented as a first-class requirement or explicit future milestone.

---

### User Story 2 - Build One Core For Many Languages (Priority: P1)

As a TypeScript, Kotlin, Swift, React, or React Native SDK maintainer, I need the Rust core to expose stable binding surfaces so platform packages become thin wrappers instead of separate implementations.

**Why this priority**: The long-term goal is lower maintenance cost and consistent behavior across web, mobile, server, and native applications.

**Independent Test**: A binding design review can verify that core DTOs, errors, async boundaries, and storage/transport ports are language-neutral and fixture-tested.

**Acceptance Scenarios**:

1. **Given** a credential verification fixture, **When** Rust, TypeScript, Kotlin, Swift, and WASM bindings invoke the verifier, **Then** they return equivalent success or error results.
2. **Given** a platform-specific secure store, **When** a mobile wrapper supplies the storage adapter, **Then** the Rust domain logic remains unchanged.

---

### User Story 3 - Preserve And Improve Edge Wallet Capability (Priority: P1)

As an edge wallet developer, I need Rust crates for DIDs, keys, wallet storage, credentials, presentations, DIDComm, mediator coordination, and OpenID4VC flows so I can build wallets without depending on legacy SDK internals.

**Why this priority**: Wallet functionality is the highest-value cross-platform surface and the largest area of current duplication.

**Independent Test**: A reference wallet harness can create keys and DIDs, receive credentials, store credentials, generate presentations, communicate through DIDComm mediation, and complete OID4VCI/OID4VP flows with shared fixtures.

**Acceptance Scenarios**:

1. **Given** an issuer offers JWT, SD-JWT VC, AnonCreds, W3C VC, or OpenBadge credentials, **When** the wallet accepts the offer through DIDComm or OID4VCI, **Then** the SDK stores and verifies the credential according to its format.
2. **Given** a verifier requests a presentation through DIDComm or OID4VP, **When** the wallet evaluates the request, **Then** it selects matching credentials, applies disclosure rules, validates challenge/domain/audience, and returns a standards-compliant presentation.
3. **Given** one or more mediators are configured, **When** the wallet sends, receives, removes, or rotates mediator connections, **Then** pickup and routing state remain consistent.

---

### User Story 4 - Prepare Rust Services Reuse (Priority: P2)

As a Mediator, Cloud-Service, or neoprism maintainer, I need reusable Rust crates for protocol, crypto, DID, VC, storage, observability, and HTTP boundaries so future service ports reuse the SDK instead of duplicating logic.

**Why this priority**: The Rust SDK should become the shared product kernel for future services, not only a wallet library.

**Independent Test**: Service spike plans can import crates without pulling mobile or binding dependencies, and adapters can be swapped for server-grade storage, HTTP, queues, and observability.

**Acceptance Scenarios**:

1. **Given** a server process needs DID resolution and PRISM VDR operations, **When** it imports the DID crates, **Then** it can reuse neoprism-compatible primitives without wallet-only dependencies.
2. **Given** a Cloud-Service issuance flow, **When** it needs credential formatting and status-list logic, **Then** it can call SDK crates with service adapters for persistence and transport.

---

### User Story 5 - Compete With Current SSI Leaders (Priority: P2)

As a product owner, I need sdk-rust to track the capabilities offered by leading SSI stacks so Identus remains competitive in EUDI, OpenID4VC, enterprise, education, and mobile-wallet markets.

**Why this priority**: Feature parity with legacy Identus SDKs is not enough; market parity requires modern OpenID4VC, SD-JWT VC, mdoc, HAIP, and OpenBadges capabilities.

**Independent Test**: A capability matrix can compare sdk-rust against Veramo, Sphereon, MATTR, walt.id, Trinsic, Credo/Aries, and EUDI reference requirements.

**Acceptance Scenarios**:

1. **Given** OID4VCI, OID4VP, SIOPv2, OpenID Federation, SD-JWT VC, ISO mdoc, and W3C VC profiles, **When** the sdk-rust roadmap is reviewed, **Then** each standard has an owning crate and conformance plan.
2. **Given** OpenBadges 3.0 requirements for extra VC types and contexts, **When** a badge credential is issued or verified, **Then** the SDK preserves required metadata and validates the badge as a VC-based credential.

## Edge Cases

- Unsupported DID methods must return typed errors and expose extension points rather than panicking or silently treating all methods as PRISM.
- Credential verification must reject expired, not-yet-valid, revoked, status-list-mismatched, wrong-audience, wrong-domain, wrong-challenge, or malformed disclosure credentials.
- DIDComm messages with optional `body` or protocol-specific optional fields must parse according to the DIDComm v2 specification and preserve forward compatibility.
- OID4VCI/OID4VP QR and request URI flows must handle large authorization requests, deferred issuance, issuer metadata drift, malformed wallet metadata, and clock skew.
- Multi-mediator operation must handle partial mediator outage, duplicate message pickup, mediator removal, and routing key rotation.
- Mobile and WASM targets must handle offline operation, secure storage unavailability, browser storage restrictions, and no network during credential verification.

## Requirements

### Functional Requirements

- **FR-001**: The repository MUST be a Cargo workspace organized as a monorepo of independently useful crates.
- **FR-002**: The workspace MUST define domain crates for crypto/key material, DID Core, PRISM DID, DID resolution, credential models, credential verification, presentation exchange, DIDComm, mediator coordination, wallet storage, OpenID4VC, status/revocation, and agent orchestration.
- **FR-003**: The workspace MUST define adapter crates for HTTP, storage, DIDComm transport, PRISM VDR/neoprism integration, signer/KMS integrations, WASM, UniFFI, N-API/Node, Swift, Kotlin, and TypeScript packaging as separate layers from domain crates.
- **FR-004**: The SDK MUST support existing Identus SDK capability families using SSI domain names: cryptographic key management, DID creation/resolution, credential issuance and verification, presentation exchange, DIDComm messaging, mediation, wallet storage, agent orchestration, authentication challenges, protobuf-compatible protocol models, and plugin-style credential/protocol extensions.
- **FR-005**: The SDK MUST support key algorithms and representations used by Identus today: Ed25519, X25519, Secp256k1, BIP-39 seed/mnemonic restoration, JWK, multibase/multicodec, and PRISM derivation paths.
- **FR-006**: The SDK MUST support DID parsing, DID URL parsing, did:prism long-form and published resolution, did:peer creation/resolution, HTTP DID resolver delegation, and extension points for emerging DID methods.
- **FR-007**: The SDK MUST support deterministic PRISM DID creation from mnemonic/master key material with fixtures that match sdk-ts, sdk-kmp, sdk-swift, and neoprism behavior.
- **FR-008**: The SDK MUST support DID rotation and master-key rotation as roadmap capabilities with protocol and storage implications captured before implementation.
- **FR-009**: The SDK MUST support JWT VC/VP, SD-JWT VC/VP, W3C VC/VP, AnonCreds credentials/proofs, DIF Presentation Exchange, OpenBadgeCredential, and future credential plugins.
- **FR-010**: The SDK MUST support OID4VCI pre-authorized code flow, authorization code flow 1a and 1b, issuer metadata, authorization-server metadata, credential offers, credential requests, deferred issuance, and wallet-initiated issuance.
- **FR-011**: The SDK MUST support OID4VP authorization request parsing, request_uri retrieval, direct_post response mode, Presentation Definition retrieval/evaluation, VP token construction, verifier response handling, and cross-device QR flows.
- **FR-012**: The SDK MUST support SIOPv2 where required for OpenID4VC interoperability and wallet authentication.
- **FR-013**: The SDK MUST support OpenID Federation, trust anchors, trusted issuer lists, wallet/client attestations, and X.509/IACA trust needed by mdoc and EUDI-style deployments.
- **FR-014**: The SDK MUST support credential status and revocation verification, including W3C Bitstring Status List, IETF Token Status List, mdoc status mechanisms, AnonCreds revocation registries, and format-specific revocation checks.
- **FR-015**: The SDK MUST support DCQL and Digital Credentials API compatibility where required by OpenID4VC and browser/mobile wallet interoperability.
- **FR-016**: The SDK MUST support signer abstractions for in-process keys, hardware-backed keys, Secure Enclave, Android Keystore, cloud KMS, external signing, and wallet-held key attestations.
- **FR-017**: The SDK MUST keep BLE, NFC, proximity presentation, QR rendering, and platform UI adapters outside the protocol core while preserving ports for ISO mdoc proximity and remote flows.
- **FR-018**: The SDK MUST support DIDComm v2 pack/unpack, OOB, issue-credential, present-proof, mediation, pickup, forwarding, basic message, problem report, revocation notification, and DID rotation protocols.
- **FR-019**: The SDK MUST support multiple simultaneous mediators and runtime add/remove/stop behavior.
- **FR-020**: The SDK MUST provide encrypted wallet storage ports and adapters for in-memory tests, SQLite/server, mobile secure storage, browser/WASM storage, and backup/restore.
- **FR-021**: The SDK MUST provide a plugin/extension model for credential formats, proof formats, DID methods, trust methods, transports, signer backends, and storage adapters without requiring agent-core recompilation.
- **FR-022**: The SDK MUST provide shared fixture suites and conformance tests so bindings and adapters prove equivalence against the Rust core.
- **FR-023**: The SDK MUST expose observability hooks for tracing, metrics, and protocol diagnostics while redacting secrets and private claims by default.
- **FR-024**: The SDK MUST include a migration map from each existing SDK public module to the planned Rust crate or binding surface.
- **FR-025**: The SDK MUST model VDR operations through ports that can target memory, database, NeoPRISM REST, PRISM Node gRPC, and Blockfrost-style PRISM adapters.
- **FR-026**: The SDK MUST model DIDComm protocol state machines for connection, issue credential, present proof, mediation, routing, report problem, and revocation notification as testable core state transitions.
- **FR-027**: The SDK MUST include a Docker-free acceptance-test agent model that can behave as issuer, holder, verifier, and peer for portable BDD scenarios.
- **FR-028**: The SDK MUST support an embedded mediator mode where agent communication can be driven through an in-process message queue before external mediator infrastructure is introduced.

### Key Entities

- **Identifier**: DID, DID URL, DID document, verification method, key agreement method, service endpoint, and DID method metadata.
- **Key Material**: Public key, private key handle, seed, mnemonic, derivation path, JWK, key use, curve, and secure-store reference.
- **Credential**: Format-specific credential payload, issuer, subject, schema/context/type metadata, credential status, proof/signature, and disclosure metadata.
- **Presentation**: Presentation request, presentation definition, selected credentials, disclosures, VP token, challenge, domain, audience, and verifier response.
- **DIDComm Message**: Plaintext/encrypted message, attachments, headers, thread IDs, sender/recipient DIDs, routing, and protocol state.
- **Mediator Connection**: Mediator DID, routing keys, pickup cursor, availability status, and message queue state.
- **Wallet Record**: Stored DID, key, credential, message, link secret, mediator, settings, backup snapshot, and migration version.
- **Binding DTO**: Stable cross-language request, response, error, event, and callback model.
- **Trust Material**: DID trust anchor, X.509/IACA root, OpenID Federation entity statement, trusted issuer list entry, wallet attestation, status-list reference, and verifier policy.
- **VDR Entry**: Data payload, driver identifier, driver family, driver version, owner DID, VDR key reference, entry hash, operation status, and backend URL.
- **Protocol State**: Role-specific DIDComm or OpenID4VC state transition, transition trigger, message payload, persisted record, and terminal outcome.
- **Acceptance Agent**: Test entity with issuer, holder, verifier, or peer roles; local wallet records; connection state; credential inventory; and message-queue transport.
- **Embedded Mediator Queue**: In-process envelope queue keyed by recipient id that allows offline and restored agents to pick up pending messages.

## Success Criteria

- **SC-001**: The first implementation plan can create a minimal Cargo workspace with all top-level crate families named in FR-002 and FR-003.
- **SC-002**: The migration map accounts for all current sdk-ts, sdk-swift, and sdk-kmp module families and marks unsupported backlog gaps explicitly.
- **SC-003**: At least one shared fixture format is defined for cross-language DID/key/credential/protocol behavior before any binding implementation begins.
- **SC-004**: The standards matrix identifies source specifications and conformance strategy for OID4VCI, OID4VP, SIOPv2, SD-JWT VC, mdoc, DIDComm, DID Core, VC Data Model, AnonCreds, Presentation Exchange, and OpenBadges.
- **SC-005**: Security review can trace each cryptographic or credential-verification requirement to tests, negative cases, and secret-handling constraints.

## Out Of Scope For This Seed

- Implementing production Rust crates.
- Replacing existing SDK package releases.
- Porting Mediator, Cloud-Service, or neoprism code.
- Publishing crates or language packages.
- Choosing final crate names where a later plan needs maintainer approval.

## Assumptions

- `sdk-rust` will be the canonical implementation repository for new SDK behavior once the initial workspace is accepted.
- Existing SDKs remain supported until bindings and migration guides are production-ready.
- neoprism modules should be reused or migrated into sdk-rust only after API ownership and release implications are explicitly planned.
- OpenBadges support means Open Badges 3.0-compatible VC modeling, issuing, verification, and exchange where applicable.
