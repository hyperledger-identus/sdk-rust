# Research readiness

Research class: protocol
Research status: ready
Decision date: 2026-10-26
Source retrieval date: 2026-10-26
Research blockers: none

## Problem and existing implementation

The current implementation of A2 groups peer DID and portable PRISM behavior without fixing the
profiles, crate boundaries, or downstream reduction path. SDK-TS supplies the
most current Identus consumer behavior, but its DTOs and implementation are not
normative. At exact revision `4bf86ebf69d5e96616a148e4c973f831f95fa38e`,
SDK-TS creates and resolves peer DID numalgo 2 and supplies PRISM create,
update, deactivate, publish, long-form resolution, HTTP-proxy short-form
resolution, and key-path integration.

NeoPRISM revision `a2ddbc5a8a316841e2c8df3831e7904fef01c356`
contains mature Rust PRISM behavior, but mixes portable method/state rules with
Cardano, persistence, and node effects. Its relevant authored nonblank sizes
are approximately 2,618 lines for `lib/did-prism`, 4,153 for
`lib/did-prism-indexer`, 131 for `lib/did-prism-ledger`, and 721 for
`lib/did-prism-submitter`. The donor also depends on the transitional
`identus-apollo`; an SDK extraction must use responsibility-based crates and
`identus-crypto` instead.

## Normative sources

- The [peer DID method draft](https://identity.foundation/peer-did-method-spec/) is
  pinned at `decentralized-identity/peer-did-method-spec@c6a1f4c079cb4816ce88a1ea33991fcf17b3f4c8`.
- The current DID Methods WG proposal is pinned at
  `w3c-cg/did-methods@90850c3e700c4714afd7e91816c4530e4631ebb3`;
  it proposes numalgo 4 and treats earlier algorithms as compatibility history.
- The PRISM DID method specification is pinned at
  `input-output-hk/PRISM-did-method-spec@fdd9007e23dc08d94d1a2b66b2dc26d1db499422`.
- W3C DID Core and the generic SDK contracts remain authoritative for DID and
  DID-document semantics outside method-specific rules.

These sources outrank donor behavior. Exact specification versions and vectors
must be refreshed again in each implementation child before a support claim.

## Candidate decisions

| Candidate | Version/revision | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- | --- |
| Affinidi `did-peer` | 0.7.5 / `3bca5b6358d05b07773ee6445834427b0411d083` | `oracle` | Apache-2.0, current and MSRV 1.88, with numalgos 0 and 2 but no numalgo 4, Affinidi DID/key/secrets ownership, unconditional WASM bindings, and production `unwrap` sites. | A narrow numalgo-4 release with owned public boundaries and qualified targets. |
| `equs-vcx-did-peer` | 0.1.0 / `7c4b1f849027f18408dd42bcc044503db061b7ef` | `oracle` | Covers numalgos 0–4, but has no declared MSRV and pulls a broad VCX DID/document/parser/resolver/key family. | A maintained narrow release with declared compiler/target policy. |
| SDK-owned `identus-did-peer` | child #523 | `adopt` | Keeps generic DID neutral while making numalgo 4 current and numalgo 2 an explicit compatibility profile. | Normative peer-DID standardization changes. |
| NeoPRISM portable method/state behavior | `a2ddbc5` | `adapt` | Cohesive deterministic PRISM syntax, protobuf, CRUD, validation, reduction and document projection can be effect-isolated. | Hard reuse gate, provenance, or conformance failure. |
| NeoPRISM VDR/indexer behavior | `a2ddbc5` | `adapt` | PRISM-specific storage-entry semantics and deterministic ordered indexing can use bounded ports without Cardano or SQL implementations. | No independent consumer, unbounded ancestry/work, or inseparable concrete effects. |
| NeoPRISM Cardano/SQL/node adapters | `a2ddbc5` | `not-adopt` | Concrete chain clients, persistence, process, deployment, and finality policy remain consumer responsibilities initially. | A separately justified reusable adapter with two consumers and its own ADR. |

## Compatibility and dependency evidence

Peer numalgo 4 and numalgo 2 are separate profiles; neither silently accepts
or emits the other. The Rust candidates are private test oracles only, so their
public types and dependency cones do not become SDK contracts. Cross-oracle
fixtures must record the exact crate release and profile.

`identus-did` stays method-neutral. `identus-did-peer` and
`identus-prism-did` depend inward on generic DID/crypto contracts.
`identus-prism-vdr` depends on the method crate and owns bounded root
traversal. `identus-prism-indexer` depends on the VDR crate and owns ordered
ingestion, batching, replay, and cursor semantics. No generic layer points
upward. The distinct SDK names avoid collisions with NeoPRISM's existing
`identus-did-prism*` packages. Concrete Blockfrost, db-sync, Oura, wallet, SQL,
HTTP, CLI, executor, configuration, and deployment code remains in NeoPRISM
for the first adoption.

The SDK lands first. NeoPRISM then pins an immutable released revision and
runs old/new paths over the same preprod snapshot. Ordered operations,
terminal DID/VDR state, errors, duration, CPU, and peak RAM are compared before
covered downstream code becomes removable. SDK-TS compatibility is delivered
through a thin facade adapter and does not dictate Rust DTOs or errors. Public
and wire compatibility is explicit in each implementation child rather than
inferred from donor DTO similarity.

## Security, privacy and maintenance evidence

All untrusted encoded/decoded bytes, entries, nesting, ancestry depth, pages,
batches, channels, retries, and expensive cryptographic work require explicit
bounds at the narrowest safe boundary. Discovery found that the current
`MAX_DID_BYTES = 2_048` cannot represent the pinned PRISM specification's
long-form state at its bounded key/service cardinalities; #524 must decide a
measured generic and method-specific budget before #521. It must not widen the
unrelated crypto text codec to make the payload fit. Parsed, validated, signed,
applied, and resolved states remain distinct. Public errors are stable and
redact caller-controlled keys, services, operations, and ledger payloads.
Secret generation and custody remain caller-owned.

Both peer candidates are Apache-2.0. `did-peer` declares MSRV 1.88 but carries
the coupling and panic concerns above. `equs-vcx-did-peer` has broader profile
coverage but no declared MSRV and a larger ownership cone. Oracle-only use
avoids public supply-chain commitment while retaining differential evidence.
NeoPRISM is an Apache-2.0 Identus donor; every adapted file and fixture still
requires exact path, history, license, and transformation provenance.
The inspected candidates contain no selected SDK production path in this
planning slice, so reachable unsafe and native-code evidence is deferred to
each exact implementation cone and must fail closed before adoption.

## Rejected or deferred candidates

Direct adoption of either peer crate, support for peer numalgos 0/1/3, copying
NeoPRISM crate boundaries, and leaving all PRISM logic downstream are rejected
for the reasons in the candidate table. Concrete Cardano clients, databases,
network execution, node orchestration, services, publication, and downstream
deletion are deferred to separate adoption evidence.

## Open questions and blockers

There is no blocker for the planning decision. Each child must stop before
implementation if its exact normative profile, fixture provenance, public
ownership, bounds, dependency cone, target plan, or consumer proof is missing.
Issue #522 depends on #521, and #521 depends on the bounded long-form decision
in #524. #523 can proceed independently after its A1 evidence prerequisites
are satisfied.

Rollback before public delivery is a focused revert of ADR 0174 and roadmap
records. After crate publication, a superseding ADR and change-ledger migration
are required; source provenance and explicit peer profiles are never erased.

## Evidence commands

Read-only `git fetch`, `git rev-parse`, `rg`, `find`, and line-count inspection
were run against the pinned SDK-TS and NeoPRISM checkouts and the pinned peer
and PRISM sources. Crate manifests and source were inspected for license, MSRV,
features, public ownership, unsafe/panic sites, and direct dependency shape.
Exact commands are: `scripts/factory research-ready`,
`scripts/factory constraints-ready`, `scripts/factory validate`, the platform
roadmap and SSI backlog validators, the full factory contract, OpenSpec
validation, text/TOML checks, and `git diff --check`. No production build,
target claim, consumer mutation, or downstream deletion is part of this slice.
Unrun checks are production Rust builds, target matrices, donor test suites,
network synchronization, benchmarks, and consumer E2E because this is a
behavior-neutral planning change.
