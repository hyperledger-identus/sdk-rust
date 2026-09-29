# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-09-30
Source retrieval date: 2026-09-30
Research blockers: none

## Problem and existing implementation

ADR 0169 correctly orders standards, accepted profiles, maintained Rust
implementations, Identus contracts, cross-SDK interoperability, and donor
precedent. Its inventory nevertheless contains target conclusions that follow
SDK-TS packaging too closely. The most material example is the statement that
Rust-backed behavior must preserve idiomatic TypeScript DTOs and errors. That
reverses the dependency direction: a Rust core would become constrained by a
single donor facade instead of exposing coherent Rust contracts that language
adapters translate deliberately.

The SDK-TS private workspace contains domain, protobuf, AnonCreds WASM,
DIDComm WASM, JWE WASM, and build-configuration packages. Protobuf exists to
support Prism DID and the SDK-Rust Nix shell already supplies protobuf codegen.
Private npm packaging is not an ownership argument. Portable domain/protocol
behavior must be implemented in Rust or delegated to a qualified Rust crate;
only package loading, host APIs, and compatibility translation remain in the
language layer.

The current implementation in SDK-Rust already supplies generic DID values,
documents, resolution/dereferencing/registration contracts, experimental WASM
DID values, bounded protocol crates, ports, and conformance machinery. Its
messaging and agent packages remain quarantined placeholders, so this change
corrects ownership without pretending those capabilities already exist.

## Normative sources

The controlling sources are RFC 9901, the exact current SD-JWT VC draft, the
DIF Peer DID draft, ratified DIDComm Messaging v2.1, independently versioned
DIDComm application protocols, accepted SDK-Rust ADRs, and reviewed Identus
consumer contracts. Donor implementations remain lower-authority evidence.

## Standards and implementation evidence

### SD-JWT

RFC 9901, published in November 2025, is the stable Standards Track base for
SD-JWT and SD-JWT+KB. It intentionally does not define credential claims. The
current credential profile is `draft-ietf-oauth-sd-jwt-vc-19`, dated
2026-08-31. It uses `application/dc+sd-jwt` and `typ=dc+sd-jwt`. The draft
history records that revisions before the November 2024 transition used
`vc+sd-jwt`; the current draft removed the suggestion that both values be
accepted indefinitely.

Therefore the target is not "two equal SD-JWT specifications." It is:

1. one canonical RFC 9901 base engine;
2. one separately versioned SD-JWT VC profile module pinned to the selected
   draft or future RFC; and
3. an optional, time-bounded legacy `vc+sd-jwt` compatibility adapter only
   when a named consumer and fixture require it.

Issue #489 owns the Rust engine and dependency decision.

### Peer DID

The DIF Peer DID Method Specification is still a draft and defines numalgo 0,
1, and 2. The current crates.io search found two direct Rust candidates:

| Candidate | Exact release | Evidence | Preliminary use |
| --- | --- | --- | --- |
| `did-peer` | 0.7.5, Apache-2.0, Rust 1.88 | maintained in Affinidi TDK; supports numalgo 0 and 2; includes Affinidi DID/secret types and unconditional WASM dependencies | strong oracle; conditional dependency only after facade/cone/target review |
| `equs-vcx-did-peer` | 0.1.0, Apache-2.0 | VCX-derived implementation; larger VCX-specific type cone and no declared MSRV | secondary differential oracle, not a preferred dependency |

The draft status and partial numalgo coverage prevent immediate dependency
activation. They do not justify leaving peer DID outside SDK-Rust. Issue #493
must choose exact supported numalgos and decide whether to adapt a crate behind
`identus-did` types or implement the small method-specific codec against
qualified oracles.

### DIDComm and application protocols

DIF lists DIDComm Messaging v2.1 as ratified. Application protocols are
separately versioned and have independent status. For example, the DIDComm
registry publishes Present Proof 3.0 as draft and Coordinate Mediation 3.0 as
a distinct protocol. SDK-TS behavior is useful compatibility evidence, but it
cannot collapse message packing, routing, mediation, pickup, issue-credential,
present-proof, revocation notification, out-of-band, discovery, and basic
message into one unversioned feature.

The engine and each application protocol therefore need separate capability
IDs, pinned PIURIs/spec revisions, roles, message families, state transitions,
effects, fixtures, and compatibility status. Issue #491 owns the engine and
protocol-boundary decision; #419 owns mediator decomposition.

### Agent runtime and service composition

Existing SSI implementations converge on a useful separation even though
their frameworks differ:

- OpenWallet Foundation VCX keeps protocol libraries, messages, credential,
  ledger, wallet, and agent crates distinct; its agents compose the lower
  layers.
- Procivis One Core separates providers from core orchestration services.
- archived Aries Framework Go demonstrates protocol services, message
  dispatch, events, storage, and transport, but its archived status makes it an
  architecture oracle rather than a dependency.
- SDK-TS exposes jobs, events, lifecycle, fetch, plugins, and user interaction,
  but those JavaScript abstractions are compatibility evidence, not the Rust
  runtime design.

The reusable Rust runtime should be a library, not an ambient Tokio service or
product policy engine. It owns deterministic dispatch and protocol state,
typed commands/effects, correlation, cancellation, retry/timer intent,
checkpoint/event contracts, and explicit resource budgets. Executors,
transports, stores, clocks, entropy, user consent, custody, and telemetry are
injected ports. Native, WASM, mobile, server, and language SDKs supply thin
executors/adapters.

The same split lets SDK-Rust eventually compose Cloud Agent- or Mediator-like
services without importing either deployment. Phase one remains black-box E2E
interoperability with immutable versions of those services. Retirement needs
equivalent protocol, persistence/migration, tenancy, operations, performance,
and rollback evidence and is not authorized here.

## Candidate decisions

| Area | Candidate | Disposition | Reason |
| --- | --- | --- | --- |
| Contract shape | TypeScript DTOs/errors constrain Rust | `not-adopt` | reverses dependency direction and imports donor coupling |
| Contract shape | canonical Rust model plus versioned language adapters | `adopt` | preserves idiomatic Rust and gives consumers an explicit migration path |
| Private workspaces | retain portable behavior in npm-private packages | `not-adopt` | packaging privacy does not establish architecture ownership |
| Private workspaces | port or adopt qualified Rust crates | `adopt` | one reusable implementation with explicit target/dependency evidence |
| Peer DID | indefinite defer | `not-adopt` | DIDComm and pairwise identity require it as a planned Rust capability |
| Peer DID | Rust-owned method with candidate crates as oracle/dependency | `conditional-adopt` | exact numalgo/profile and cone remain issue #493 decisions |
| SD-JWT | treat legacy and current VC media types as equal cores | `not-adopt` | current draft has one canonical media type and documents the transition |
| SD-JWT | RFC base + pinned VC profile + bounded legacy adapter | `adopt` | separates stable mechanics, draft profile, and consumer compatibility |
| Agent | TypeScript-only runtime | `not-adopt` | portable state/effect orchestration is a reusable SDK capability |
| Agent | small Rust runtime with injected executors and effects | `adopt` | portable, testable, and bindable without product policy |
| Networking | all browser/Node behavior in TypeScript | `not-adopt` | mixes protocol semantics with host I/O |
| Networking | Rust protocol/port contracts plus target adapters | `adopt` | allows either Rust WASM or thin TS transport without duplicating semantics |
| Quality | property/fuzz/benchmark/differential are optional exploration | `not-adopt` | each closes a distinct defect/performance/interoperability risk |
| Quality | risk-routed required evidence | `adopt` | obligations can be proportional without forcing every test on every PR |
| Roadmap | one SDK-wide parity milestone | `not-adopt` | hides dependency order and delays usable adoption evidence |
| Roadmap | proof-based vertical milestones | `adopt` | each capability can become independently usable and reversible |

## Compatibility and dependency evidence

No production dependency is added. `did-peer` 0.7.5 and
`equs-vcx-did-peer` 0.1.0 were inspected through crates.io metadata and local
published sources. Neither is selected. Their exact default/minimal cones,
unsafe/native reachability, source integrity, WASM/mobile behavior, API
conversion cost, fuzz posture, and conformance vectors remain issue #493 work.

Both published candidates report Apache-2.0 license metadata; their complete
license and provenance chain still requires source-integrity verification.
`did-peer` declares MSRV 1.88, while `equs-vcx-did-peer` declares no MSRV.
Neither candidate has an accepted direct and resolved dependency cone for this
SDK. Supply-chain advisories, yanked status, transitive source/license data,
minimal features, and release provenance remain unverified. Public and wire
compatibility must be proven through SDK-owned DTO/error conversion and exact
numalgo vectors before adoption.

No agent framework is selected as a dependency. Runtime research uses VCX,
One Core, Aries Framework Go, Credo/SDK-TS, Cloud Agent, and Mediator as design
or interoperability oracles only. The planned runtime must compile without a
mandatory async executor and must keep WASM, iOS, Android, server, and language
binding effects explicit.

## Security, privacy and maintenance evidence

The runtime and protocols must never serialize secret handles, user consent,
raw credentials, tokens, or correlation data into generic diagnostics or
telemetry. Message and event queues need byte/count/depth/work limits;
correlation, retries, timers, and cancellation need deterministic semantics;
and persistence checkpoints need versioning and atomicity. Network adapters
must own TLS, redirects, decompression, DNS/private-address policy, timeouts,
and response byte limits while protocol crates own request/response semantics.

Legacy DTO/error and wire compatibility stays observable, opt-in or version
scoped, tested, and removable. Unsafe legacy behavior is rejected rather than
emulated.

## Rejected or deferred candidates

No peer DID, SD-JWT, DIDComm, runtime, networking, or service framework is
adopted by this architecture correction. `did-peer` is a strong oracle and a
conditional dependency candidate; the VCX-derived crate is a secondary oracle.
The SDK-TS embedded engines and agent abstractions are compatibility evidence,
not production dependencies. The reconsideration trigger for a rejected or
deferred engine is a maintained release with an exact supported profile,
acceptable MSRV/features and direct/resolved cone, source and license
provenance, clean supply-chain evidence, bounded owned facade, target proof,
conformance, security review, migration, and rollback.

## Open questions and blockers

There are no blockers to correcting the roadmap. Implementation questions are
deliberately delegated to bounded child issues: peer DID numalgos and engine,
SD-JWT crate/profile, DIDComm engine/protocols, runtime API, target networking
adapters, and service-composition canaries.

## Sources

- https://www.rfc-editor.org/rfc/rfc9901.html
- https://datatracker.ietf.org/doc/draft-ietf-oauth-sd-jwt-vc/19/
- https://identity.foundation/peer-did-method-spec/
- https://identity.foundation/specs/
- https://identity.foundation/didcomm-messaging/spec/v2.1/
- https://didcomm.org/present-proof/3.0/
- https://didcomm.org/coordinate-mediation/3.0/
- https://crates.io/crates/did-peer/0.7.5
- https://crates.io/crates/equs-vcx-did-peer/0.1.0
- https://github.com/openwallet-foundation/vcx
- https://github.com/procivis/one-open-core
- https://github.com/hyperledger-aries/aries-framework-go

## Evidence commands

Evidence used `cargo search`, `cargo info --verbose`, published crate source
inspection, official specification pages, accepted repository ADRs, exact
SDK-TS inventory evidence, and current issue bodies. No consumer or donor
checkout was mutated. Runtime code, target builds, dependency activation,
protocol conformance, E2E execution, and performance measurements remain
unrun and are assigned to implementation issues.
