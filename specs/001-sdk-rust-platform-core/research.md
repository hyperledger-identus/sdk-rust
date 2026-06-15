# Research: Identus Rust SDK Platform Core

**Created**: 2026-06-13

## Source Inventory

### Existing Identus SDKs

- `sdk-ts` is a Yarn/Nx monorepo with `@hyperledger/identus-sdk`, `@hyperledger/identus-domain`, protobufs, and WASM packages for AnonCreds, DIDComm, and JWE.
- `sdk-swift` exposes Swift Package products with historical Identus building-block names plus authentication and aggregate SDK targets.
- `sdk-kmp` is a Kotlin Multiplatform repository with `:sdk`, `:protosLib`, and `:sampleapp`, currently focused on Android/JVM.
- `neoprism` is a Rust Cargo workspace with `identus-apollo`, `identus-did-core`, `identus-did-prism`, PRISM indexer/ledger/submitter/resolver crates, storage, and `neoprism-node`.

### Identus Docs Repository

- `documentation/reference/specifications.md` is the current public standards matrix for Cloud Agent and SDKs. It covers DIDComm Messaging v2.x, Peer DID 1.0, DID Core, PRISM DID, VC JSON Schema, JOSE/COSE, SD-JWT, SD-JWT VC, AnonCreds, DID:PRISM and HTTP AnonCreds methods, Bitstring Status List, OOB, mediation, connection, issue credential, present proof, revocation notification, report problem, OID4VCI, and OID4VP.
- `documentation/develop/cloud-agent/building-blocks.md` maps historical Identus building-block names to cryptography, DID, credential, and messaging capability areas. `sdk-rust` should use SSI domain names for the crate family split.
- `documentation/learn/basic-concepts.md` defines Cloud Agent, Wallet SDKs, and Mediator relations: Cloud Agent owns server-side issuing/verifying/DID/connection/multi-tenancy, SDKs own client-side credential storage/presentations/key management/DIDComm, and Mediator owns offline routing and forwarding.
- `documentation/develop/cloud-agent/deterministic-did-creation.md` defines deterministic PRISM DID derivation with BIP-39 PBKDF2, path `m/29'/29'/didIndex'/keyUsage'/keyIndex'`, hardened derivation, compressed secp256k1 public keys, protobuf `AtalaOperation`, `SHA-256(serialized AtalaOperation)`, and a test vector.
- `documentation/develop/cloud-agent/vdr.md` defines VDR CRUD/verify and HTTP binding, with drivers for memory, database, NeoPRISM, PRISM Node, and Blockfrost PRISM. NeoPRISM is recommended for new production blockchain-backed VDR use.
- `cloud-agent/mercury` and `cloud-agent/pollux` docs define DIDComm protocol state machines for connection, issue credential, present proof, report problem, routing, mediation, and revocation notification. These are fixture candidates for sdk-rust conformance tests.

### Capability Baseline

| Capability | Existing Evidence | Rust SDK Implication |
|---|---|---|
| Crypto/key management | Legacy SDK cryptography modules; neoprism `identus-apollo` | Start with feature-gated `identus-crypto` or converge with `identus-apollo` |
| DID Core/PRISM/Peer DID | Legacy SDK DID modules, neoprism DID crates | Separate DID Core, PRISM DID, peer DID, and resolver crates |
| Wallet storage | Legacy wallet stores via RIDB, CoreData/Keychain, SQLDelight/SQLite | Define storage ports plus encrypted adapters |
| Credentials | Legacy credential modules support JWT, SD-JWT, W3C, AnonCreds | Credential model and verifier crates must be format-pluggable |
| Presentations | Presentation Exchange and proof protocols | Presentation request/evaluation crate plus DIDComm/OID4VP adapters |
| DIDComm/mediator | Legacy SDK messaging and agent orchestration modules | DIDComm core, protocol-state, and mediator coordination crates |
| OID4VCI/OID4VP | Strongest in sdk-ts, partial Swift, backlog in KMP/cloud-agent | OpenID4VC must be a first-class crate family, not a wrapper plugin |
| OpenBadges | Backlog requires flexible VC `type` and `@context`; no first-party SDK code found | Treat OpenBadges 3.0 as a required credential profile |
| Revocation/status | DIF revocation, StatusList models, revocation notification | Status and revocation crate with format-specific verifiers |
| Bindings | Current SDKs duplicate logic per language | Rust core plus WASM, N-API, UniFFI, Swift/Kotlin/TS wrappers |

### BDD Scenario Baseline

- The source `.feature` corpus is concentrated in `cloud-agent/tests/integration-tests/src/test/resources/features`, `sdk-ts/integration-tests/e2e-tests/features`, and `sdk-kmp/tests/end-to-end/src/test/resources/features`. The docs repository mirrors these suites and should be treated as documentation copy rather than independent behavior.
- The portable SDK BDD layer covers connection, JWT/SD-JWT/AnonCreds issuance, present proof, peer verification, JWT revocation, backup/restore, connectionless JWT flows, and mediator pickup after wallet restore.
- The Cloud Agent BDD layer covers the same credential protocols plus PRISM DID lifecycle, VDR memory/database/ledger CRUD, OID4VCI issuer/config/issuance, credential schemas, verification API, verification policies, wallet multitenancy, health, and metrics.
- The first Rust acceptance target is a Docker-free `identus-agent` model that can execute portable role and mediator-queue behavior before protocol crates attach real DIDComm, cryptography, storage, HTTP, ledger, and OID provider adapters.

### Integration Repository Baseline

- `integration/README.md` defines the current release compatibility matrix across `cloud-agent`, `mediator`, `sdk-ts`, `sdk-kmp`, and `sdk-swift`, including component-under-test, weekly all-main, manual version matrix, and draft/final release flows.
- The integration E2E matrix confirms portable SDK flows that `sdk-rust` must eventually cover as a runner: backup/restore, connection, JWT/SD-JWT/AnonCreds issuance, JWT/SD-JWT/AnonCreds proof, JWT revocation notification, peer verification, and out-of-band JWT issuance/proof.
- `integration/src/test-runner` defines the runner contract: clone selected SDK version, optionally build from source, run SDK-specific tests, pass Cloud Agent and Mediator URLs, and copy Allure results to `tmp/<runner>`.
- `sdk-ts` runner uses `AGENT_URL` and `MEDIATOR_OOB_URL`; `sdk-swift` uses `TEST_RUNNER_PRISM_AGENT_URL`, `TEST_RUNNER_MEDIATOR_OOB_URL`, and `TEST_RUNNER_DEBUG`. `sdk-rust` should support a normalized environment contract plus compatibility aliases for wrappers.
- `integration/src/runner/report.ts` and repository tests define report expectations: aggregate Allure statuses, generate release metadata, clean draft release reports when final releases publish, and notify Slack for failed/broken results or report exceptions.
- `sdk-kmp` is currently skipped in the integration runner as broken/non-functional. `sdk-rust` replacement claims should include wrapper parity gates that make skipped rows explicit until Rust-backed KMP/Swift/TS wrappers pass the matrix.

### DID Method Inventory

- Product paths in SDKs, Mediator, Cloud Agent, neoprism, and PRISM VDR center
  on `did:prism` and `did:peer`, with Mediator currently requiring
  `did:peer:2` for routing and accepting operator-supplied `did:prism`.
- DIDComm test vectors and examples use `did:example`; these are fixture-only
  identifiers and must not imply product resolver support.
- AnonCreds external fixtures include `did:web` resource identifiers.
- Broader ecosystem/test material references `did:key`, `did:jwk`, and
  `did:pkh`; these should be accepted by safe core parsing and evaluated as
  resolver adapters before product claims.
- Candidate Rust crates discovered through `cargo search`/`cargo info`:
  `did-peer`, `did-key`, `did-method-key`, `did-jwk`, `ssi`, `ssi-dids`, and
  Affinidi DID crates. No dependency is added to the core parser until adapter
  ownership, transitive dependencies, WASM support, and conformance coverage are
  reviewed.

### DIDComm Protocol Inventory

- `repos/docs/documentation/reference/specifications.md` lists DIDComm
  Messaging v2.x, OOB 2.0, BasicMessage 2.0, Coordinate Mediation 2.0 and
  planned 3.0, Message Pickup 3.0, Trust Ping 2.0, Report Problem 2.0, Routing
  2.0, Issue Credential 3.0, Present Proof 3.0, and Identus Revocation
  Notification 1.0.
- `cloud-agent/mercury` source confirms current message type URIs for OOB,
  coordinate mediation 2.0, routing forward, trust ping, issue credential 3.0,
  present proof 3.0, report problem 1.0/2.0, DID exchange 1.0, and legacy
  connection compatibility.
- `mediator` source confirms Discover Features 2.0, routing 2.0, coordinate
  mediation 2.0, messagepickup 3.0, trust-ping 2.0, and BasicMessage 2.0 in
  mediator feature discovery and E2E tests.
- Docs and source use both `messagepickup/3.0` and `pickup/3.0`, plus both
  `coordinate-mediation/2.0` and historical `mediator-coordination/2.0`
  wording. The Rust parser should classify these as compatibility aliases while
  using canonical `messagepickup` and `coordinate-mediation` profiles.
- Base Rust crates discovered through `cargo search`/`cargo info`:
  `didcomm` 0.4.1 (`sicpa-dlab/didcomm-rust`, Apache-2.0, UniFFI/testvector
  features), `didcomm-rs` 0.7.2 (`decentralized-identity/didcomm-rs`,
  Apache-2.0, raw crypto/resolve/out-of-band features), and
  `affinidi-messaging-didcomm` 0.15.1 (Affinidi TDK, Apache-2.0, DIDComm v2.1,
  optional messaging-core feature). These are evaluation candidates for
  pack/unpack and transport adapters, not immediate dependencies of the typed
  domain boundary.

## Backlog Themes From Knowledge Base

- OID4VCI authorization code flow 1a and 1b for JWT credentials.
- OID4VP authorization endpoint logic in Swift/KMP and cloud-agent verifier endpoints for `direct_post`, request objects, and presentation submission status.
- OpenBadges 3.0 compatibility through flexible VC `type` and `@context`.
- DID rotation, master-key rotation, and deterministic PRISM DID creation from mnemonics.
- Multiple mediators and runtime mediator add/remove behavior.
- DIDComm v2 alignment, including optional `body` handling and valid protocol URIs.
- DIDComm v2 protocol parity for OOB, BasicMessage, Trust Ping, Discover
  Features, Routing/Forward, Coordinate Mediation, Message Pickup, Issue
  Credential, Present Proof, Report Problem, Revocation Notification, and
  compatibility aliases observed in source.
- Credential preview schema neutrality across AnonCreds, W3C VC, and future formats.
- SD-JWT verification gaps such as temporal claims and presentation frame/disclosure handling.
- Revocation/status-list behavior across edge agents and cloud flows.
- neoprism observability, storage, and PRISM DID quality patterns.
- Deterministic PRISM DID compatibility with the docs test vector and key usages: Master, Issuing, KeyAgreement, Authentication, Revocation, CapabilityInvocation, CapabilityDelegation, and VDR.
- VDR driver compatibility across memory, database, NeoPRISM, PRISM Node, and Blockfrost-style PRISM access.
- SD-JWT holder binding nuance: docs state that missing `cnf` prevents signing verifier challenge/domain, while disclosable `cnf` enables proof creation.

## Competitor And Ecosystem Scan

| Project/Ecosystem | Relevant Signals | Required sdk-rust Response |
|---|---|---|
| Veramo | Modular JavaScript framework for DIDs and VCs with emphasis on vendor-neutral, composable APIs. Source: https://veramo.io/ | Keep crate APIs modular and protocol/vendor neutral. |
| Sphereon IDK/SSI SDK | OID4VCI, OID4VP, SIOPv2, Presentation Exchange, MS Entra, DID methods, BBS+, RSA. Sources: https://github.com/Sphereon-Opensource/SSI-SDK and https://docs.sphereon.com/idk/guides/oid4vci/overview | OpenID4VC and Presentation Exchange must be complete, not experimental edge plugins. |
| MATTR | OID4VCI issuance, OID4VP verification for remote mdocs, pre-authorized and authorization code flows. Sources: https://learn.mattr.global/docs/issuance/oid4vci-overview and https://learn.mattr.global/docs/verification/oid4vp | Support mdoc and high-assurance OpenID4VC profiles alongside W3C/SD-JWT formats. |
| walt.id | OID4VCI/OID4VP guides, SD-JWT VC, selective disclosure, W3C VC, mDL/mdoc ecosystem messaging. Sources: https://docs.walt.id/concepts/data-exchange-protocols/openid4vci, https://docs.walt.id/concepts/data-exchange-protocols/openid4vp, https://docs.walt.id/concepts/digital-credentials/sd-jwt-vc | Treat selective disclosure and OpenID4VC developer ergonomics as core SDK product quality. |
| Trinsic | Commercial VC acceptance network/API with hosted/direct provider sessions and verifier UX focus. Source: https://docs.trinsic.id/docs/trinsic-documentation | Track enterprise verification UX, provider session orchestration, tenancy, and credential template ergonomics. |
| Credo / Aries / ACA-Py | OpenWallet Foundation and Aries stacks for DIDComm, mediation, AnonCreds, W3C VC/Data Integrity, SD-JWT, Presentation Exchange, OpenID4VC, revocation, and agent patterns. Sources: https://github.com/openwallet-foundation/credo-ts, https://aca-py.org/1.1.0/, https://aca-py.org/latest/features/Mediation/ | Match modular agent architecture and interoperability tests for DIDComm plus OpenID4VC. |
| EUDI Wallet | ARF defines EUDI wallet architecture, interoperability, security, and privacy. Reference apps/libraries set expectations for PID/mDL obtain-store-present, proximity sharing, remote QES, OpenID4VCI/OID4VP, SD-JWT VC, mdoc, token status lists, wallet attestations, and trusted lists. Sources: https://eu-digital-identity-wallet.github.io/eudi-doc-architecture-and-reference-framework/2.4.0/architecture-and-reference-framework-main/, https://github.com/eu-digital-identity-wallet/eudi-app-android-wallet-ui, https://github.com/eu-digital-identity-wallet/eudi-lib-android-wallet-core | Align roadmap with EUDI ARF, HAIP, SD-JWT VC, mdoc, wallet attestation, trusted lists, and remote/proximity presentation. |

## Standards Baseline

- OpenID4VC consists of OpenID for Verifiable Credential Issuance, OpenID for Verifiable Presentations, and SIOPv2. Source: https://openid.net/sg/openid4vc/
- OID4VCI defines an OAuth-protected API for issuing credentials and supports formats including SD-JWT VC, ISO mdoc, and W3C VC. Source: https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0.html
- OID4VP 1.0 is final and should be tracked with draft compatibility where partner ecosystems still require it. Source: https://openid.net/specs/openid-4-verifiable-presentations-1_0-final.html
- OpenID Federation 1.0 is final and should be modeled as a trust backend for OpenID4VC ecosystems. Source: https://openid.net/specs/openid-federation-1_0.html
- The OpenID4VC High Assurance Interoperability Profile combines OpenID4VC with SD-JWT VC and ISO mdoc for higher-security interoperability. Source: https://openid.net/specs/openid4vc-high-assurance-interoperability-profile-1_0-04.html
- Open Badges 3.0 defines achievement credential metadata, verification, and exchange procedures. Source: https://www.imsglobal.org/spec/ob/v3p0/cert
- OpenBadges 3.0 and CLR 2.0 should be treated as VC-aligned education profiles. Source: https://www.imsglobal.org/spec/ob/v3p0/impl
- Status/revocation must cover W3C Bitstring Status List and IETF Token Status List in addition to mdoc and AnonCreds mechanisms. Sources: https://www.w3.org/TR/vc-bitstring-status-list/, https://datatracker.ietf.org/doc/draft-ietf-oauth-status-list/

## Proposed Crate Families

| Family | Candidate Crates | Notes |
|---|---|---|
| Foundation | `identus-core`, `identus-errors`, `identus-fixtures`, `identus-observability` | Shared DTOs, errors, test fixtures, tracing. |
| Crypto | `identus-crypto`, `identus-key`, `identus-jose` | Reuse or converge with neoprism `identus-apollo`. |
| DID | `identus-did-core`, `identus-did-prism`, `identus-did-peer`, `identus-did-resolver` | Align with neoprism DID crates and universal resolver needs. |
| Credentials | `identus-vc`, `identus-vc-jwt`, `identus-vc-sd-jwt`, `identus-vc-ld`, `identus-mdoc`, `identus-anoncreds`, `identus-openbadges` | Keep models separate from exchange protocols. |
| Presentations | `identus-presentation-exchange`, `identus-status`, `identus-revocation` | Shared by DIDComm and OpenID4VC flows. |
| Messaging | `identus-didcomm`, `identus-didcomm-protocols`, `identus-mediator-client` | OOB, issue credential, present proof, mediation, pickup, forward. |
| OpenID4VC | `identus-oid4vci`, `identus-oid4vp`, `identus-siopv2`, `identus-haip`, `identus-openid-federation` | Must support wallet, issuer, and verifier roles over time. |
| Trust | `identus-trust`, `identus-x509`, `identus-attestation`, `identus-policy` | DID trust, X.509/IACA roots, OpenID Federation, EUDI trusted lists, wallet attestations. |
| Wallet | `identus-wallet-core`, `identus-wallet-storage`, `identus-agent` | Domain orchestration plus storage ports. |
| Adapters | `identus-storage-sqlite`, `identus-storage-memory`, `identus-http`, `identus-vdr-neoprism`, `identus-signer-kms`, `identus-signer-mobile` | Server/mobile/browser adapters stay outside domain crates. |
| Bindings | `identus-wasm`, `identus-node`, `identus-uniffi`, `identus-swift`, `identus-kotlin`, `identus-typescript` | Thin wrappers generated or backed by stable DTOs. |

## Open Decisions

- Whether to move neoprism crates into sdk-rust immediately or depend on them until API ownership is agreed.
- Final crate names and publication namespace.
- Whether AnonCreds support is native Rust, external FFI, or feature-gated adapter around an upstream crate.
- Which OpenID4VC conformance suite becomes the initial CI gate.
- How much server-side issuer/verifier functionality belongs in SDK crates versus future Cloud-Service crates.
- Which Rust ecosystem crates to reuse or evaluate first: Impierce `openid4vc`, SpruceID `oidc4vci-rs`, SpruceID `openid4vp`, and SpruceID `isomdl`.
- How to represent DCQL and Digital Credentials API transport without coupling core protocol state machines to browser or mobile UI surfaces.
- Whether `sdk-rust` exposes Cloud Agent admin/client bindings for entities, wallets, API keys, UMA permissions, webhooks, health, and metrics, or stays focused on edge/core primitives.
- How to resolve docs drift: Presentation Exchange is marked unsupported in the specifications matrix, while Cloud Agent docs/OpenAPI include presentation definition and verification policy surfaces.
- Which SD-JWT and OID4VCI/OID4VP draft/final versions must remain interoperable with existing Identus releases.
