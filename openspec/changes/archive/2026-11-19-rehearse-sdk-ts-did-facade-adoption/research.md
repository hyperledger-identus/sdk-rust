# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-11-19
Source retrieval date: 2026-11-19
Research blockers: none

## Problem and existing implementation

Issues #605 through #608 established a private capability-named package with
strict DID, DID URL, DID Document, injected resolution/dereferencing, peer DID,
target, measurement, and provider-selection evidence. Each assertion suite is
currently invoked independently. No single consumer-shaped journey composes
them through both provider routes or retains a rollback receipt.

## Normative sources

- Issue #609 defines the bounded rehearsal and non-goals.
- SDK-TS revision `4bf86ebf69d5e96616a148e4c973f831f95fa38e`
  (8.1.4) is the immutable donor inventory.
- `sdk-ts-peer-did-donor.toml` classifies the pairwise relationship and
  handshake values while excluding DIDComm packing, transport, key custody,
  storage, mediation, and agent lifecycle.
- The canonical DID facade, document, resolution, peer-method, parity, mapping,
  vector, and quality contracts remain authoritative.

## Candidate decisions

| Candidate | Decision | Reason |
| --- | --- | --- |
| Mutate SDK-TS for an external rehearsal | `not-adopt` | Issue #609 does not authorize downstream mutation. |
| Duplicate each existing target suite | `not-adopt` | It does not prove one composed consumer journey or provider neutrality. |
| Build one repository-local consumer fixture from the pinned donor dispositions | `adopt` | It proves composition while keeping platform effects and downstream ownership explicit. |
| Model a complete DIDComm or agent flow | `not-adopt` | Messaging, custody, persistence, mediation, and lifecycle are separately owned. |
| Share one assertion body between a compatibility provider and Rust provider | `adopt` | It directly exercises opt-in adoption, applicable parity, and rollback. |

## Compatibility and dependency evidence

The fixture reuses the current private package, provider interfaces, canonical
vectors, and target runners. It adds no runtime dependency, package export,
wire behavior, default selection, or SDK-TS source change. The compatibility
provider exists only inside the test fixture and implements the reviewed public
capability contract.

## Security, privacy and maintenance evidence

Inputs are fixed public test vectors. Receipts contain only exact revisions,
target, provider route, assertion counts, stable categories, and digests. The
fixture includes no private keys, network access, persistence, credentials,
DIDComm payloads, automatic fallback, or caller data logging.

## Rejected or deferred candidates

External SDK-TS activation, default switching, support budgets, publication,
React Native, DIDComm processing, credential workflows, and mdoc remain
deferred to separately authorized capability trains.

## Open questions and blockers

None. Hosted CI and protected integration are later gates, not research
blockers.

## Evidence commands

Planning uses factory doctor, strict OpenSpec validation, research readiness,
constraint readiness, factory check, and issue-bound preflight. Implementation
will run the provider-neutral unit suite and package, Node.js, browser, React
SSR/hydration, parity, vector, change-ledger, factory, and Nix gates.

## Selected approach

Define one frozen journey descriptor and one async assertion function. Each
target supplies a compatibility provider and a Rust-backed provider plus
injected resolution effects. The assertion body verifies equivalent applicable
outcomes, explicit deviations, pairwise `from`/`to` stability, terminal
failures, and rollback selection. A deterministic receipt summarizes every
target at the exact repository and donor revisions.

## Provenance

Repository-owned files and the pinned SDK-TS donor are Apache-2.0. No donor
production source, secret, credential, wallet database, or generated artifact
is copied.
