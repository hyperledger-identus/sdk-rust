# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/497
Constraint blockers: none

## Existing entries affected

ADRs 0162 through 0164 establish SDK-Rust as the reusable core, capability
migration, and test authority. ADR 0169 establishes SDK-TS as evidence rather
than authority. Issue #497 records the sponsor's exact corrections to DTO/error
ownership, private packages, peer DID, SD-JWT, DIDComm, the agent runtime,
networking, DID bindings, service interoperability, and quality evidence.
SDK-SEC-003 still requires bounded new inputs; SDK-LIM-002 and SDK-LIM-003
still prevent this roadmap from claiming released FFI or runtime support.

## Introduced or changed constraints

SDK-Rust SHALL define canonical reusable Rust DTOs and errors. A language SDK
MAY expose a versioned adapter that translates them into an existing idiomatic
or legacy surface for a bounded migration window. Such an adapter SHALL NOT
constrain the Rust core or silently become its source of truth.

Portable private-workspace behavior SHALL move to SDK-Rust or a qualified Rust
dependency. The protobuf toolchain and generated models used by Prism DID are
not evidence that unrelated private npm packages should remain duplicated.

Generic DID domain, document, service, resolution, dereferencing, registration,
and method-registry semantics SHALL live in SDK-Rust. Peer DID becomes a
planned method capability. Method-specific codec/state may live in a focused
Rust crate; concrete ledger, network, custody, and product policy remain ports
or downstream adapters.

DIDComm Messaging v2.1 SHALL be treated separately from application protocols.
Every supported application protocol SHALL pin its own version/status/PIURI
and specify roles, messages, state, effects, vectors, compatibility, and
deprecation independently.

The platform SHALL target a portable Rust agent runtime that is executor- and
transport-neutral. Browser/Node, native, mobile, and server integrations SHALL
implement explicit ports; protocol validation and state SHALL NOT be copied
into TypeScript adapters.

Property, fuzz, benchmark, and differential evidence SHALL be required when
the capability risk calls for them. The delivery plan MAY route expensive
campaigns to slow/release lanes, but SHALL record omissions and SHALL NOT
misclassify these methods as non-required merely because they are not
normative specifications.

## Introduced or changed limitations

No peer DID numalgo, SD-JWT VC draft, DIDComm application protocol, agent
runtime API, networking implementation, service replacement, or target support
is implemented by this change. The Peer DID method remains a draft. The
SD-JWT VC profile remains an Internet-Draft. Existing Cloud Agent and Mediator
deployments are not deprecated until replacement gates are independently met.

## Consumer and product impact

There is no runtime impact. The roadmap stops promising that SDK-TS DTOs and
errors shape Rust. Future SDK-TS adoption may preserve its released API through
a separately versioned mapping layer. Existing services become explicit E2E
oracles before any replacement attempt.

## Activation and rollback

Activation requires the planning commit and exact preflight receipt, accepted
successor ADRs, updated human and machine inventories plus mutation tests,
updated child issues, local review, signed/DCO commits, and green protected CI.
Rollback reverts documentation and planning metadata only; no dependency,
wire, persisted state, target claim, service, or consumer is changed.

## Evidence

Issue #497, ADRs 0162 through 0169, RFC 9901, SD-JWT VC draft-19, DIDComm
Messaging v2.1, the Peer DID draft, crates.io candidate metadata, and the
linked SSI framework repositories establish the direction and remaining
research boundaries.
