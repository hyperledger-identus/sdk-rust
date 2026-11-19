# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-11-19
Source retrieval date: 2026-11-19
Research blockers: none

## Problem and existing implementation

Issue #605 owns confirmed findings F-007, F-008, and F-009 from the review of
PR #604. The peer DID engine, TypeScript facade, generated WASM boundary,
portable packet, target harnesses, and quality registry already exist. The
defects are inaccurate donor transformation metadata, incorrect fuzz-boundary
attribution, and incomplete assertions in the native packet runner. No new
protocol behavior, dependency, public type, profile, or target is required.

The native runner currently skips every error vector and compares only the DID
for the two numalgo-2 positive cases. The shared packet already contains the
expected relationship/service fields and stable errors needed for the repair.
The TypeScript assertion body covers these fields and applies additional host-
language structural preconditions, but those TypeScript-only checks must not
be credited to Rust.

## Normative sources

- Issue #605 and review findings F-007/F-008/F-009 define the repair.
- The canonical `sdk-ts-peer-did-method` contract requires immutable donor
  dispositions and forbids relabeling core evidence as adapter evidence.
- The canonical quality-evidence contract requires class-specific selectors,
  commands, targets, bounds, receipts, and source revisions.
- W3C DID Core 1.0 and the peer profile decisions already selected by ADR 0174
  continue to govern the resolved document. This change does not reinterpret
  them.
- The compatibility donor remains `hyperledger-identus/sdk-ts` at
  `4bf86ebf69d5e96616a148e4c973f831f95fa38e`, Apache-2.0.

At that exact donor revision, selectors
`PEERDID CreateTest > Should create the peerDID correctly` (lines 54-94),
`Should resolver peerdid correctly` (lines 96-107), and
`Should resolver peerdid's kid ... correctly` (lines 109-130) use bare service
IDs (`didcomm` and `new-id`). The selected Rust profile requires fragment IDs,
so the adapted positive packet omits the donor ID while
`did.peer.numalgo2.invalid-service-id` records the intentional negative
disposition.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Relabel the existing Rust ASAN soak as adapter fuzz | `not-adopt` | It never enters TypeScript ingress or projection. | Never without an adapter entry selector. |
| Mark adapter fuzz not applicable | `not-adopt` | The adapter parses hostile structured values and generated projection shapes, so generative evidence is useful and feasible. | The adapter boundary is removed. |
| Add deterministic adapter-entry/projection fuzz tests | `adopt` | Exercises the owned host-language boundary without a new dependency and remains reproducible in the isolated TypeScript lane. | A stronger reviewed JS fuzz runner replaces it. |
| Duplicate the portable packet for Rust | `not-adopt` | Risks semantic drift and weakens one-packet parity. | Packet schema becomes target-specific by design. |
| Extend the native runner over the shared packet | `adopt` | Uses existing source-bound expectations and states applicability explicitly. | The Rust API or packet schema changes. |
| Claim TypeScript JSON/prototype preconditions as native failures | `not-adopt` | Those checks occur before Rust and have no native equivalent. | The Rust API begins accepting the same structured host values. |

## Compatibility and dependency evidence

The repair adds assertions and evidence metadata only. It does not change
identifier construction, document projection, error mapping, generated WASM,
TypeScript declarations, runtime packaging, Cargo/npm dependencies, external
SDK-TS, or any support promise. Existing successful vectors must continue to
produce byte-identical identifiers.

## Security, privacy and maintenance evidence

The adapter fuzz route uses deterministic local generation, bounded case
counts, bounded strings/collections, no secrets, no network, and no retained
artifacts containing caller values. It asserts only stable outcome categories,
immutability, redaction, and ownership cleanup. The native runner compares
public fixture values and stable error categories without logging rejected
input.

The existing Rust libFuzzer/ASAN campaign remains authoritative only for the
Rust core. The new adapter route is independently named, owned, and executable
through the TypeScript target environment.

## Rejected or deferred candidates

New peer profiles, public APIs, identifier or service semantics, DIDComm,
custody, networking, persistence, React Native, external adoption,
publication, release, support budgets, and mdoc remain excluded.

## Open questions and blockers

None. All three findings have exact existing owners and reversible local
repairs. Hosted CI and protected integration are later gates, not research
blockers.

## Evidence commands

Planning evidence uses factory doctor, strict validation, research readiness,
constraint readiness, and issue-bound preflight. Implementation evidence will
run the focused Rust conformance test, TypeScript adapter fuzz selector,
cross-language vector and quality registry validators, the isolated
TypeScript differential/target gates, factory check, and full Nix validation.

Implementation tests, final receipts, local review, hosted CI, and protected
merge evidence are intentionally unrun at planning time.

## Selected approach

Keep one portable packet. Add small Rust assertion helpers that compare the
shared positive document fields and map only applicable packet errors to
stable `identus-did-peer` categories. Record skipped TypeScript-only
preconditions explicitly. Add deterministic bounded mutation generation at
the TypeScript adapter entry/projection seam and bind only that selector to the
TypeScript fuzz obligation.

## Provenance

Repository-owned files are Apache-2.0 at base
`ccd470f56515fdf54f8977482c670559f8f64aea`. The only external source inspected
is the already pinned Apache-2.0 SDK-TS donor revision above; no donor source or
fixture bytes are copied by this change.
