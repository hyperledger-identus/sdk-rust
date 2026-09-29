# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-09-30
Source retrieval date: 2026-09-30
Research blockers: none

## Problem and existing implementation

The current implementation has strong but separate evidence systems:
`apollo-crypto-parity.toml` owns crypto capability/vector provenance;
crate-specific error-contract CSV files own stable Rust error mappings;
`identus-platform-ts-capabilities.toml` owns normalized SDK-TS discovery; and
the A0 roadmap owns milestone ordering. There is no shared record that lets a
consumer adapter reference the exact same vector, Rust contract, mapped
language result, change classification, and quality obligation.

The current platform change ledger is intentionally empty and comment-driven.
The roadmap identifies A1 but names #420 as its owner and mixes downstream
canary #492 with infrastructure. This is insufficient for autonomous workers
because dependencies and closure evidence are implicit.

## Normative sources

- W3C DID Core 1.0 defines DID and DID URL syntax and the DID data model, and
  points to a dedicated conformance test suite:
  https://www.w3.org/TR/did-core/
- JSON Schema Draft 2020-12 provides the selected schema vocabulary for
  portable JSON fixture payloads and receipts:
  https://json-schema.org/draft/2020-12
- ADR 0164 defines test-authority precedence; ADR 0169 makes SDK-TS evidence,
  not authority; ADR 0170 makes Rust DTO/error contracts canonical.
- The pinned language baselines are SDK-TS package 8.1.4 revision
  `4bf86ebf69d5e96616a148e4c973f831f95fa38e`, SDK-Swift revision
  `ebbfdb666a3b0c9905dd443589e03ccf2ae8087b`, and SDK-KMP revision
  `5a8fda770bbbca84192979b2c8e6090ee94790e3`.
- Existing provenance baselines pin Apollo revision
  `ccee22bcd693e618b9b8ff3e15ed6f9c9156c27c` and NeoPRISM generic DID
  evidence revision `8becb225132efb1d9302b2c5f6ed4d87b84e8685`.

All five donor repositories declare Apache-2.0 at the assessed revisions.
License compatibility permits reuse, but provenance and test authority still
decide whether an example may constrain SDK behavior.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| One issue and one combined catalog | `not-adopt` | Couples independently reviewable authority, mapping, change, and quality concerns and creates noisy agent contention. | The four records become inseparable in practice after two delivery cycles. |
| Four independent records with stable cross-references | `adopt` | Maximizes cohesion, allows parallel source and mapping work, and lets each validator fail locally. | Schema evolution proves cross-record integrity cannot remain atomic. |
| Treat repeated TS/Swift/KMP examples as normative | `not-adopt` | The same DID parser cases were copied across SDKs and do not outrank the standard. | A reviewed Identus profile explicitly promotes an exact case. |
| Preserve donor cases as pinned consumer-regression evidence | `adopt` | They reveal migration requirements without dictating canonical Rust shapes. | The last supported consumer removes the behavior and its deprecation gate closes. |
| Put SDK-TS adapter code inside A1 | `not-adopt` | It would mix infrastructure with the adoption proof and mutate a consumer before shared evidence exists. | Never; the consumer remains a downstream issue. |
| Seed A1 with bounded DID/DID URL cases | `adopt` | SDK-Rust already owns bounded parsing and redacted errors, while all three SDKs contain comparable cases. | A smaller non-secret cross-language capability becomes a better first proof. |

## Compatibility and dependency evidence

The pinned SDK-TS, SDK-Swift, and SDK-KMP DID parser tests share positive and
negative examples, demonstrating historical compatibility but also copied-test
risk. SDK-TS additionally carries peer DID and Prism fixtures; those remain
method-specific evidence for A2, not A1 normative data. Apollo already has a
closed machine vector inventory, so A1 references its stable vector IDs rather
than duplicating bytes. NeoPRISM's synthetic generic DID projection remains a
consumer-shaped oracle and does not import chain or Prism operation semantics.

No runtime dependency, Cargo feature, public type, wire format, native library,
or unsafe code is selected by this planning change. The direct and resolved
dependency cone is therefore unchanged. Implementation is expected to use
standard-library/Python validation already present in the factory unless a
child issue separately qualifies another dependency. The Rust 1.89.0 MSRV,
Rust 1.98.1 etalon, WASM/iOS/Android support tiers, and fast/weekly lanes remain
unchanged; A1 records target evidence but does not manufacture it.

Stable cross-record identifiers prevent duplicate payloads: a vector record
owns immutable inputs/outputs and authority; a mapping record references the
Rust contract and vector IDs; a change record references affected capability,
mapping, and evidence IDs; a quality declaration references exact selectors
and artifacts. The facade boundary remains Identus-owned canonical Rust types
inside SDK-Rust and explicit, versioned, deprecatable language adapters outside.

## Security, privacy and maintenance evidence

Fixture payloads must be public, synthetic, standards-owned, or explicitly
redistributable; secrets, production identifiers, credentials, and personal
data are prohibited. Hashes bind immutable payloads. Validators reject unknown
authority classes, unpinned donor revisions, dangling identifiers, duplicate
ownership, missing redistribution decisions, and undeclared quality outcomes.

This is supply-chain metadata rather than executable dependency adoption. It
adds no native code or unsafe Rust. Maintenance is divided by cohesive issue
ownership, schema versions are explicit, and unknown future fields fail closed
until a migration is accepted. Release and security posture do not change.
Public and wire compatibility remain unchanged because no API is implemented.
Rollback removes the planning records before implementation; after stable IDs
ship, schema migration must preserve or explicitly map them.

Protocol or draft currency is recorded per future packet rather than inferred
from donor recency. The initial DID seed pins W3C DID Core 1.0; peer DID,
SD-JWT, AnonCreds, DIDComm, and OID4VC profiles stay in later milestones with
their own exact version decisions.

## Rejected or deferred candidates

A central database/service, generated bindings, automatic fixture harvesting,
remote schemas, and a workspace-wide test-vector migration are deferred. They
would increase coupling or network dependence before the four local contracts
prove useful. YAML is rejected for the first machine registry because the
repository already validates TOML and JSON deterministically. Storing full
donor source snapshots is rejected; immutable repository/revision/path plus
approved redistributable payloads are sufficient.

SDK-Swift and SDK-KMP adapter implementation, peer DID vectors, SD-JWT dual-
profile vectors, AnonCreds 1.0, DIDComm v2.1, and agent-runtime fixtures remain
research inputs for later milestones, not A1 implementation scope.

## Open questions and blockers

None for specification and planning. Child issue #420 must still decide the
final filesystem layout for payload blobs, and #505 must decide exact generated
adapter documentation. Those are bounded implementation choices and do not
change the four-contract architecture.

## Evidence commands

Research used GitHub tree/content API reads at the five pinned revisions,
existing SDK-Rust inventories, `rg` over current SDK-Rust tests, and official
W3C/JSON Schema sources. Planning validation commands are
`scripts/factory research-ready specify-a1-compatibility-foundation`,
`scripts/factory constraints-ready specify-a1-compatibility-foundation`, and
`scripts/factory validate specify-a1-compatibility-foundation`.

Implementation commands intentionally unrun at planning time include the new
catalog/mapping/ledger/quality validators, Rust/SDK-TS shared-packet tests,
property/fuzz/benchmark/differential jobs, consumer builds, and target runtime
evidence. Those belong to #420, #505, #422, #501, and downstream #492.
