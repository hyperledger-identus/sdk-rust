# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/505
Constraint blockers: none

## Existing entries affected

`SDK-ARCH-001` keeps product policy downstream; `SDK-ARCH-002` preserves
acyclic ownership; `SDK-SEC-003` preserves bounds and redaction;
`SDK-DELIVERY-001` requires specification and evidence; `SDK-LIM-005` and
`SDK-LIM-006` separate core SDK delivery from consumer implementation. ADRs
0169, 0170, and 0173 define donor precedence, canonical Rust contracts, and A1
sequencing. None is weakened.

## Introduced or changed constraints

Rust crate/module/symbol or public error code is always canonical. Each mapping
has a stable ID, capability, owner, state, canonical revision/version origin,
exact pinned language source, conversion direction, compatibility class,
lossless/lossy decision, version window, deprecation phase, vectors and exact
selectors, bounds/redaction, async/cancellation ownership, migration,
observability, fallback, rollback, and removal gate.

Field mappings, error mappings, derived constants, ignored fields, and
unsupported cases are explicit. A lossy mapping must name the lost information
and deterministic failure or migration behavior. Language error strings are
never stable identifiers. An adapter cannot weaken Rust resource bounds or
redaction. The mapping's byte limit resolves to a named public Rust constant;
its declared value may be stricter but not larger. Supported language windows
use a machine-checked ordered semantic-version interval containing the pinned
language version. Unknown registry fields fail closed.

## Introduced or changed limitations

The first records cover only the pinned SDK-TS 8.1.4 DID, DID URL, and legacy
invalid-string error surface. They describe compatibility but do not implement
or test SDK-TS. Swift/Kotlin DTOs, FFI, WASM, React Native, browser/Node
networking, and protocol/runtime mappings remain future records. The four
records are mandatory seeds, not a closed allowlist; conforming future records
do not require validator code changes.

The DID URL legacy mapping is intentionally lossy. It cannot promise raw query
round trips, duplicate/order preservation, or absent-versus-empty query and
fragment preservation. Unsupported values fail at the adapter boundary; the
Rust core is not normalized down to the donor shape.

## Consumer and product impact

No current consumer behavior changes. The registry gives future adapters a
bounded migration contract and gives reviewers an explicit incompatibility
list. Product, deployment, storage, custody, consent, and chain behavior remain
downstream.

## Activation and rollback

Activation requires a signed/DCO planning commit, issue-bound preflight,
versioned registry, strict validator and mutations, deterministic rendered
documentation, factory integration, protected review, and green fast CI.
Before merge, rollback removes the registry. After stable IDs merge, semantic
changes require a new version or explicit replacement/deprecation path.

## Evidence

Issue #505, parent #504, ADRs 0169/0170/0173, the A1 blueprint, current
`identus-did` contracts, and the pinned SDK-TS model/parser/error sources
support these constraints.
