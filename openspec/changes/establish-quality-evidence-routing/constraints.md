# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/501
Constraint blockers: none

## Existing entries affected

`SDK-ARCH-001` and `SDK-ARCH-002` keep evidence declarations independent from
products, chains, and production dependencies. `SDK-SEC-003` requires explicit
resource and fuzz/property treatment for changed hostile-input boundaries.
`SDK-DELIVERY-001` requires issue-linked specification and review.
`SDK-LIM-009` keeps exhaustive/fuzz evidence on weekly/manual slow and release
lines rather than every pull request. `SDK-LIM-005` and `SDK-LIM-006` keep
product policy and consumer adoption outside this upstream infrastructure.
None is weakened.

## Introduced or changed constraints

Every delivered capability using this contract has one active, stable
declaration with exactly one property, fuzz, benchmark, and differential
obligation. Each obligation is `satisfied`, `required`, or `not-applicable` and
records risk, exact command/selectors, target, route, cadence, freshness,
budget, owner, receipt, debt state, rationale, and class-specific details as
applicable.

Satisfied evidence has an immutable source or hosted-run receipt. Time-bounded
receipts must be fresh at the declaration's recorded evaluation date. Required
but unsatisfied evidence is explicit debt with a named owner and open issue.
Not-applicable evidence has no executable/receipt claims and requires a bounded
risk rationale plus reviewing issue. Omission is never equivalent to not
applicable.

Lane/cadence pairs are closed and deterministic: focused/on-change,
fast/per-pull-request, slow/weekly-or-manual, and release/per-candidate.
`none/not-applicable` is allowed only for a reviewed not-applicable outcome.
The quality plan is non-executing data and cannot alter required branch checks.

## Introduced or changed limitations

The first declaration covers generic DID/DID URL lexical values only. It does
not cover DID documents, resolution, method behavior, Prism/peer DID,
cryptographic verification, credentials, DIDComm, networks, FFI, storage, or
product behavior.

Engineering evidence reduces risk but is not standards authority,
certification, formal proof, security audit, release authorization, or target
support. The historical freshness snapshot does not remain current forever;
promotion requires a new exact-candidate evaluation. Benchmarking remains not
applicable to the A1 DID seed until a named consumer defines a comparable
operation and budget.

## Consumer and product impact

No public API, wire behavior, crate dependency, consumer, product, or supported
target changes. Future SDK-TS adoption may consume the stable declaration ID,
but actual adapter behavior, observability, fallback, and rollback remain #492.

## Activation and rollback

Activation requires the signed/DCO planning commit and preflight receipt,
registry, validator, deterministic plan, mutation tests, template references,
factory integration, focused evidence, distinct review, and green protected
CI. Before merge, rollback removes these repository-local records. After merge,
semantic changes use a registry version increment and explicit replacement;
stable declaration IDs and historical receipts are not rewritten silently.

## Evidence

Issue #501, parent #504, ADRs 0127 and 0173, the A1 blueprint, #420 vector
catalog, current DID tests/fuzz workflow, and immutable scheduled run
`36517752658` support these decisions.
