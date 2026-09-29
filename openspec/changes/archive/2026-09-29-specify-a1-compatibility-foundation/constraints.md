# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/504
Constraint blockers: none

## Existing entries affected

`SDK-ARCH-001` keeps product and chain behavior downstream; `SDK-ARCH-002`
keeps dependency direction acyclic; `SDK-COMPAT-001` preserves stable Rust;
`SDK-DELIVERY-001` requires issue, OpenSpec, review, and evidence before
implementation; `SDK-PRODUCT-002` separates SDK delivery from consumer
adoption. ADRs 0164, 0169, and 0170 already govern test authority, donor
precedence, and canonical Rust contracts. None is weakened.

## Introduced or changed constraints

A1 has one parent and exactly four cohesive implementation contracts. Every
record has a schema version and stable ID. Cross-record relationships use IDs,
not copied payloads. Vector authority is explicit; donor repetition cannot
promote a case to normative. Rust DTOs and error codes remain canonical;
language shapes exist only as versioned compatibility mappings.

Issue #422 cannot close before #420 and #505 provide stable reference IDs.
Issue #492 cannot begin implementation until all four A1 contracts are
validated. Every quality class is either linked to exact evidence or carries a
reviewed not-applicable rationale; a generic test or CI run cannot satisfy it.

## Introduced or changed limitations

The initial catalog is deliberately incomplete and seeded only with bounded
DID/DID URL evidence. A catalog row is not implementation, target support,
conformance, certification, release, or consumer adoption. Donor paths may
disappear; immutable revisions preserve evidence but do not create an ongoing
upstream compatibility promise.

The planning packet does not select peer DID numalgos, SD-JWT profiles,
AnonCreds engines, DIDComm engines, agent executors, browser/Node networking,
or binding generators. It also does not validate remote sources during normal
CI; reproducible local metadata and hashes are the gate.

## Consumer and product impact

No current consumer behavior changes. Future TypeScript, Swift, Kotlin, WASM,
UniFFI, or React Native adapters gain a common evidence vocabulary and can
retain legacy DTO/error shapes without constraining Rust. Product, consent,
custody, persistence, chain, and deployment policy remain downstream.

## Activation and rollback

Activation requires a planning-only signed/DCO commit, issue-bound preflight,
an offline-valid implementation blueprint, protected PR review, and green
factory gates. The four child issues implement separately in dependency order.
The downstream canary remains blocked until their completion receipts exist.

Before any child implementation, rollback removes the new planning artifacts.
After records exist, a superseding ADR and schema migration must preserve IDs
or provide explicit replacement mappings; silently rewriting provenance is
prohibited.

## Evidence

Issue #504 and milestone 5 provide sponsor authority. GitHub sub-issue and
blocked-by relationships encode the delivery graph. Pinned donor revisions,
official DID Core/JSON Schema sources, the existing cross-SDK inventory, and
ADRs 0164/0169/0170 support the selected constraints.
