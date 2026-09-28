# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-29
Source retrieval date: 2026-09-29
Research blockers: none

## Problem and existing implementation

The existing source defines fourteen public resource constants, four scalar or
opaque value roles, request claims/filters/queries, presentation requests,
credential candidates and validated sets, selections and disclosure plans,
opaque artifacts, generated-presentation coverage, and value-free receipt
projection. It is 1,277 physical lines and 1,134 authored nonblank lines.

`identus-presentations` keeps the module private and explicitly re-exports the
public vocabulary from `lib.rs`. It contains no protocol decoding, discovery,
ranking, consent, cryptographic proof implementation, trust decision,
persistence, network operation, or runtime binding. The safe seam is a private
module move behind the unchanged crate root.

## Normative sources

Issue #407 and Discussion #399 direct the decomposition. Existing presentation
model/lifecycle tests, `presentation-core`, presentation-error, input-resource,
and code-health specifications, immutable error evidence, ADR 0115, and the
current crate root own behavior. This routine refactor does not reinterpret any
presentation protocol or credential format.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Private facade plus value, request, selection, and artifact modules | `adopt` | Separates syntax, request intent, candidate/disclosure validation, and generation/receipt change axes while preserving one public surface. | A public namespace or independently versioned package becomes a product requirement. |
| Split at arbitrary line counts | `not-adopt` | Hides the hotspot without semantic ownership and violates ADR 0115. | Never without a semantic boundary. |
| Export child modules | `not-adopt` | Creates new supported paths without consumer need. | Consumers demonstrate a stable public namespace requirement. |
| Generalize collections or validation | `not-adopt` | Similar bounds have distinct errors and validation precedence; abstraction would hide domain review. | Multiple owners prove the complete invariant and error contract. |
| Adopt a protocol/model dependency | `not-adopt` | This slice is an ownership move over SDK format-neutral semantics, not protocol implementation. | Dedicated adoption research proves semantic, dependency-cone, MSRV, and target fit. |

## Compatibility and dependency evidence

The crate root export list remains unchanged. Public names, derives, signatures,
constness, limits, validation predicates and order, exact format/request/candidate
bindings, byte aggregation, error projection, debug redaction, and receipt
semantics remain owned by the same crate. No manifest, dependency, feature,
lockfile, MSRV, unsafe-code, FFI, or target change is needed.

## Security, privacy and maintenance evidence

Untrusted text, bytes, filters, queries, candidates, selections, bindings, and
artifacts retain their existing positive limits, uniqueness checks, validation
precedence, and static errors. Claim intent remains value-free; credential
handles, challenges, artifact bytes, verifier/purpose state, and debug surfaces
retain their existing exposure boundaries. No new recursion, secret, logging,
I/O, synchronization, or panic path is introduced.

Private siblings use narrow `pub(super)` lookup and consistency methods rather
than exposing fields. No descendant may exceed the 1,000-line attention
threshold or become a forwarding-only public API.

## Rejected or deferred candidates

Arbitrary splitting, public child modules, generalized validators, and a new
dependency are rejected. Protocol adapters, proof execution, discovery,
ranking, consent, trust, lifecycle, persistence, and wire changes remain
separately owned.

## Open questions and blockers

There are no semantic planning blockers. The planning commit must be rebased
onto the current merged `develop` tip before preflight. Any public API, error,
budget, validation-order, value-retention, format, lifecycle, manifest, or
target drift blocks this slice.

## Evidence commands

Planning inspected `5c469f7cd5c4ae7b54c54ee5090aa3382a41fbd6`, issue #407,
Discussion #399, `model.rs`, `lib.rs`, presentation tests, code-health policy,
and canonical specifications. After rebasing and before implementation run
`cargo test -p identus-presentations --all-features`. Afterward run the same
suite, strict crate/workspace Clippy and format, public/error/source/factory
contracts, code-health verification, portable targets, relevant Nix gates,
and protected exact-head CI.
