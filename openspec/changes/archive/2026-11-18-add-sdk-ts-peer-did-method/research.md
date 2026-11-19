# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-11-18
Source retrieval date: 2026-11-18
Research blockers: none

## Problem and existing implementation

The current implementation in `identus-did-peer` already owns the portable
method contract. `numalgo4::InputDocument` validates duplicate names, JSON
depth/nodes/members/live-key bytes, the 16 KiB decoded-document ceiling,
private JWK members, and generic DID Document projection before retention.
`LongForm` deterministically creates and verifies the JSON multicodec payload,
SHA2-256 multihash, long/short relationship, and local document projection.
`ShortForm` requires caller-supplied matching long-form material and never
performs I/O.

`numalgo2_compat` accepts caller-owned public multicodec keys and compact
service inputs. It bounds keys, services, decoded service bytes, aggregate
elements, DID bytes, and projected documents; validates purpose/codec
compatibility; creates/parses the pinned Identus form; and resolves locally to
the generic DID Document model. The two modules have separate public values
and reject cross-profile or unsupported numalgos. Stable crate errors redact
input material. The existing core declaration `did.peer.profiles.quality.v1`
already owns property, fuzz, benchmark, and differential obligations for the
Rust engine.

The private `identus-wasm-did` API version 3 currently depends only on
`identus-did` and exposes DID/DID URL/document plus resolution/dereferencing
views. `packages/identus-sdk` copies generated values into strict frozen
capability-oriented TypeScript values, frees generated ownership exactly once,
and assembles separate browser and Node artifacts reused by React Web. It has
no peer-method surface. This is an adapter gap, not a request for a new peer
DID engine.

## Normative sources

The primary source URLs are
<https://github.com/decentralized-identity/peer-did-method-spec> and the DID
Methods Working Group material pinned by ADR 0174. The selected protocol/draft
currency is explicit: current numalgo 4 follows proposal revision
`90850c3e700c4714afd7e91816c4530e4631ebb3`; numalgo 2 is an opt-in Identus
compatibility profile bound to DIF draft revision
`c6a1f4c079cb4816ce88a1ea33991fcf17b3f4c8`, SDK-TS compatibility evidence,
and the already accepted canonical `peer-did-method` specification. W3C DID
Core 1.0 governs projected documents.

The canonical implementation source is
`hyperledger-identus/sdk-rust@bd1981372fcb08772d1196131a0cf26291fad9cb`,
Apache-2.0. License and provenance are retained in the repository, source
records, and immutable donor inventory. The exact MSRV and source/compiler
contract is Rust 1.99.1. The
compatibility donor is
`hyperledger-identus/sdk-ts@4bf86ebf69d5e96616a148e4c973f831f95fa38e`,
tree `bed8bfecfdf79c48ac02f95d97bc489fe0b258a1`, package
`@hyperledger/identus-sdk@8.1.4`, Apache-2.0. SDK-TS implements numalgo 2 only;
it is consumer-regression evidence, not normative authority.

The maintained `did-peer` 0.7.5 and `equs-vcx-did-peer` 0.1.0 crates remain
Apache-2.0 differential oracles selected by ADR 0174, not public dependencies.
Their version/features, target, dependency, and maintenance limitations are
already recorded in the delivered Rust core research. This binding train does
not reopen that engine decision.

## Donor inventory and provenance

The pinned donor production inventory is:

| Path | Git blob / SHA-256 | Disposition |
| --- | --- | --- |
| `packages/lib/sdk/src/castor/methods/peer/PeerDID.ts` | blob `5cd27224ce0659b014e9208dc902e96610f587d2`; SHA-256 `a0e393809a4d3feb8020562def0156c2b41328a90cfd76cf8ce73bfc76687e77` | Adapt compact service DTO compatibility only; do not copy the class/name. |
| `packages/lib/sdk/src/castor/methods/peer/PeerDIDCreate.ts` | blob `6e472192982290f4a2dfc2c1fd607a62e36c4c0b`; SHA-256 `b2b658b67eb8685e2fa4dd24e418bc1889415be31f5284c20a5c53111f10e195` | Adapt deterministic numalgo-2 identifier behavior from fixed public keys/services; reject donor key/custody implementation. |
| `packages/lib/sdk/src/castor/resolver/PeerDIDResolver.ts` | blob `940b1160451bbfb76e9bdcaf8561a15d04781a12`; SHA-256 `60dec73c3d010d40c92d7a21537b91b0dd02c5097fbc75c1a438569fe3b04731` | Compatibility oracle for numalgo-2 document/key/service projection; generic throws and JWK conversion are not canonical. |
| `packages/shared/domain/src/peer-did/PeerDID.ts` | blob `a6f3d166c557b05524cbbb5757748e2ae2e21ed9`; SHA-256 `948d286adb6216d3f7db261bc90716039053b636818457e6908feb365e4d66ae` | Legacy regex/value-shape inventory only; it accepts forms the donor resolver cannot handle. |
| `packages/lib/sdk/src/edge-agent/didFunctions/CreatePeerDID.ts` | blob `6eada6f40d60126be85081f790a41d21b11acc84`; SHA-256 `ffcdc8898f90dbc1cbc6a9a3c8132566297c1d503a72f516451b93708d7a6233` | Platform-owned/excluded: secret generation, storage, mediator update, and agent lifecycle. |

The authoritative donor test file
`packages/lib/sdk/tests/castor/PeerDID.test.ts` has blob
`0ee432237089d99fadbc40cf794664b1c378b9cb` and SHA-256
`66a3b70e73c5d707c4475f29f732f6be982753c6e4a43d191dbe70b1b2e4ae54`.
Its eight discovered tests receive exactly one disposition: codec-only behavior
is superseded by owned vectors; deterministic numalgo-2 create, resolve,
key-ID, service, and unsupported-purpose behavior is adapted; signature and
random-secret tests are excluded under crypto/custody owners. The donor
fixture `packages/lib/sdk/tests/fixtures/dids.ts` has blob
`8a1bbb014d576ca1c7aa049adc5518dbe2a5fb2b` and SHA-256
`d4f7e6ea49c5451ad0c59fda8e5d1b27fcf35c9722484afa233d9c8e16934fde`.

`packages/lib/sdk/tests/agent/protocols/connection/HandShakeRequest.test.ts`
has blob `bba2fc945936804bf31a6ba79f871d1b194cfec1` and SHA-256
`79b369c453b0e96149ebc34635701a78a8e7b59b7e9b60cf2cc980ac6a98993a`.
Its DIDComm protocol behavior is excluded. Only the consumer-shaped outcome
that frozen `from`/`to` peer DID values survive a pairwise request boundary is
adapted. No donor production source or private key is copied.

The donor's locked `@hyperledger/identus-apollo@1.8.0` and
`multiformats@13.4.2` are implementation details and are not selected as
dependencies.

## Build-versus-adopt decision

Build the thin binding and adapter over the existing Rust owner. The direct
portable dependency direction becomes:

```text
identus-did / identus-crypto
    -> identus-did-peer
        -> identus-wasm-did generated API v4
            -> private TypeScript peer-profile projection/provider
                -> browser / Node.js / React target loader
```

No platform or binding dependency points inward. The resolved third-party
Cargo cone is already present in the workspace; the direct and resolved
dependency cone changes only by adding the inward `identus-did-peer` workspace
edge. The direct binding edge adds
`identus-did-peer` but no new engine/package. Extending the existing DID WASM
artifact is `adopt`: it reuses the exact DID Document projection and target
loader. A separate WASM artifact is `conditional-adopt` only if measured size,
initialization, or dependency evidence shows the shared artifact is not
reviewable or operable.

The public facade accepts structured numalgo-4 input documents and explicit
numalgo-2 public-key/service inputs. TypeScript applies iterative
cycle/prototype/unsafe-number/depth/node/member checks and closed
serialization; Rust remains authoritative for method syntax, profile,
private-member rejection, resource limits, hash linkage, codecs, purposes, and
projection. Generated views return only identifiers and normalized DID
Document JSON needed for strict projection. No generated owner crosses the
public surface.

## Candidate decisions

| Candidate | Decision | Reason |
| --- | --- | --- |
| Extend the existing DID WASM/package artifact | `adopt` | Reuses the reviewed projection, lifecycle, target loaders, and rollback route with one inward workspace edge. |
| Add a second peer DID engine/dependency | `not-adopt` | Duplicates delivered Rust semantics and expands supply-chain/behavior authority. |
| Expose one ambiguous peer DID operation | `not-adopt` | Would silently mix current numalgo 4 and legacy numalgo 2 behavior. |
| Expose explicit numalgo-4 and numalgo-2-compatibility methods | `adopt` | Preserves version/profile authority and deterministic migration behavior. |
| Accept private keys or generate entropy in the facade | `not-adopt` | Violates the custody-neutral core and creates an unnecessary secret boundary. |
| Accept only caller-owned public multibase keys/services | `adopt` | Matches the Rust API and keeps crypto/custody independently owned. |
| Reuse strict DID Document projection | `adopt` | Prevents a second document DTO/validator and preserves current lossless semantics. |
| Adapt the donor pairwise fixture as DIDComm execution | `not-adopt` | DIDComm engine/application protocols have separate ownership and readiness gates. |
| Adapt only frozen pairwise `from`/`to` values | `adopt` | Proves the consumer boundary without importing messaging semantics. |
| Add React Native through browser WASM | `not-adopt` | Contradicts the accepted native-binding target contract. |

The reconsideration trigger is a normative profile revision, a proved gap in
`identus-did-peer`, or measured evidence that adding the explicit exports to
the existing artifact violates an accepted target budget. Default activation,
publication, native bindings, secret custody, DIDComm, or another numalgo each
requires a new decision rather than scope expansion.

## Compatibility and dependency evidence

The private binding API increases from version 3 to 4 and adds exports; the
reviewed TypeScript surface is additive and unpublished. Existing DID syntax,
document, resolution, and dereferencing operations remain source/wire
compatible. Generated API identity, declarations, closed symbol/file
inventories, and target artifacts are refreshed together so stale loaders fail
closed. Rollback selects the provider without peer-method capabilities before
initialization or removes this unreleased additive route; an operation never
silently falls back to another profile/provider after failure.

Stable SDK-owned failures distinguish wrong profile, unsupported algorithm,
malformed encoding, unsupported codec/purpose, invalid input document, hash
mismatch, missing long-form material, resource limit, projection failure, and
binding-contract drift. Messages and observations contain no input DID,
public-key bytes, service values, document bytes, generated error, or stack.

Numalgo 2 deterministic identifier/document output is compatibility evidence.
Numalgo 4 is a Rust-owned addition/deviation with normative, negative,
resource, fuzz/property, and target-boundary evidence; no donor parity claim is
made. Browser, Node.js, and React Web each require exact-artifact runtime
proof. React Native is explicitly inapplicable because its separate native
binding is absent.

## Security, privacy and maintenance evidence

Structured documents, public multibase keys, service types/endpoints/routing
keys/accept values, DIDs, generated values, and observer callbacks are
untrusted. TypeScript performs bounded iterative structural validation before
closed serialization. Rust reapplies byte/work/profile/codec/purpose/private-
member/hash/document validation. Exact-once generated ownership closes on
success and every projection, observer, and mapped-error path. No secrets,
private key fields, raw logs, or caller values enter errors/observations.

The affected crates forbid unsafe code and add no native code or native
dependency. Supply-chain evidence consists of exact Cargo/npm locks, source-
distribution checks, Nix closure, closed artifact inventory, immutable donor
hashes, signed+DCO commits, and exact-head target receipts. Maintenance,
release, and security posture remain the existing SDK policies; this private
train creates no publication, support, stable-API, or production claim.

The core's public property/fuzz/benchmark/differential obligations remain
under `did.peer.profiles.quality.v1`. The binding adds a separate pending
`did.peer.typescript.quality.v1` declaration for adapter mapping, hostile
shapes, exact-free, target equivalence, artifact impact, pairwise DTO,
observability, and rollback. Artifact bytes/hash delta, cold/warm load,
create/parse/resolve/project p50/p95, normalized bytes, and reliable RSS are
measurement-only until separate budget authority exists.

The separate `tests/wasm-did-browser` fixture is currently stale: its manifest
still declares Rust 1.98.1 and its runtime assertion expects binding API v2
while the integrated private facade is API v3 on Rust 1.99.1. This train SHALL
repair that fixture to exact Rust 1.99.1/API v4 and add peer generated-value,
error, and cleanup coverage before treating it as browser evidence. The stale
fixture is a bounded test-harness defect, not a reason to weaken the current
workspace compiler contract.

## Rejected or deferred candidates

An ambiguous generic peer API, numalgos 0/1/3, another engine, raw private
keys, secret generation, signature verification, key custody, DIDComm
pack/unpack or protocols, relationship storage, networking, agent runtime,
PRISM, React Native, external SDK-TS mutation, default activation,
publication/support/release, and mdoc are `not-adopt` or deferred to their
independent owners.

## Evidence commands

The planned local sequence is:

```text
./scripts/factory research-ready add-sdk-ts-peer-did-method
./scripts/factory constraints-ready add-sdk-ts-peer-did-method
./scripts/factory validate add-sdk-ts-peer-did-method
./scripts/factory preflight add-sdk-ts-peer-did-method --issue 603 --write
cargo test -p identus-did-peer
cargo test -p identus-wasm-did
./scripts/check-sdk-ts-did-target.sh all
./scripts/factory quality-plan did.peer.typescript
./scripts/factory check
./scripts/factory ready add-sdk-ts-peer-did-method
nix flake check --print-build-logs
```

These are exact commands and unrun checks at planning time. Target, artifact,
property/fuzz/differential/mutation/resource/coverage/measurement, receipt,
archive, and fresh-review commands will be recorded exactly when the
implementation exposes them. They are unrun at planning time.

## Open questions and blockers

There are no blockers to the repository-local private train. The implementation
must measure the shared WASM artifact delta before retaining the single-artifact
choice, but no accepted threshold currently blocks it. External consumer
activation, accepted budgets, native binding, DIDComm, publication/support,
release, PRISM, and mdoc remain separate gates and are not inferred from local
success.
