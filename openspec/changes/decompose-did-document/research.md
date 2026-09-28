# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-29
Source retrieval date: 2026-09-29
Research blockers: none

## Problem and existing implementation

The source defines eight public wire/resource constants, `OneOrMany<T>`,
validated JSON-LD contexts, verification relationships and methods, service
endpoints and services, the DID document and builder, raw duplicate-name and
resource preflight, cross-document validation, recursive-value budgets, and
iterative rejection cleanup. It is 1,197 physical lines and 1,083 authored
nonblank lines at the audited `develop` baseline.

`identus-did` keeps the module private and explicitly re-exports its public
vocabulary from `lib.rs`. The implementation owns representation-neutral DID
Core structure, not method-specific resolution, dereferencing, authorization,
cryptosuite execution, JSON-LD processing, persistence, networking, or runtime
policy. The safe seam is a private move behind the unchanged crate root.

## Normative sources

Issue #408 and Discussion #399 direct the decomposition. Existing DID document
and hardening tests, `did-core`, input-resource, public-error, unsafe-code, and
code-health specifications, ADR 0115, and the crate root own behavior. W3C DID
Core is not reinterpreted by this routine refactor; the implemented contract
and its tests remain authoritative for compatibility.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Private facade plus cardinality, extension, verification, service, and model modules | `adopt` | Separates independent syntax/security/change axes while keeping aggregate document invariants together. | A supported public namespace or separate package becomes a product requirement. |
| Split aggregate validation from `DidDocument` by line count | `not-adopt` | Cross-resource uniqueness, relationship binding, serde construction, and rejection cleanup form one aggregate invariant. | A reusable validation policy gains a second proven owner. |
| Export validation helpers or child modules | `not-adopt` | Creates unsupported public paths and bypass opportunities without consumer need. | Consumers demonstrate a stable supported boundary. |
| Generalize JSON limits with protocol modules | `not-adopt` | DID document extension limits have specific error precedence and cleanup ownership. | Dedicated research proves identical contracts and change cadence. |
| Adopt an external DID document model | `not-adopt` | This slice preserves hardened SDK semantics; external models do not prove identical limits, duplicate handling, cleanup, redaction, and target support. | A dependency assessment proves complete semantic and supply-chain fit. |

## Compatibility and dependency evidence

The crate root export list remains unchanged. Public names, derives,
signatures, serde representation, constants, validation predicates/order,
error projection, cleanup, and document equivalence remain owned by the same
crate. No manifest, dependency, feature, lockfile, MSRV, unsafe-code, FFI, or
target change is needed.

## Security, privacy and maintenance evidence

Raw bytes, decoded object names, container depth, node count, live key bytes,
collection sizes, properties, names, strings, verification material, service
maps, context maps, and aggregate extensions retain their exact positive
limits and fail-closed precedence. Rejected hostile JSON remains dismantled
iteratively. Private JWK material, ambiguous known material, noncanonical
multibase values, duplicate resources, and duplicate relationship members keep
the same static redacted errors.

Private siblings use narrow `pub(super)` validation/cleanup functions rather
than public helpers or shared fields. No descendant may exceed the 1,000-line
attention threshold or become a forwarding-only public API.

## Rejected or deferred candidates

Arbitrary splitting, public child modules, cross-protocol limit abstraction,
an external DID model, and changes to document semantics are rejected. DID
methods, resolution, dereferencing, cryptographic verification, authorization,
JSON-LD processing, persistence, and transport remain separately owned.

## Open questions and blockers

There are no semantic blockers. Planning must be rebased onto the merged issue
#407 `develop` tip before preflight. Any public API, wire, serde, error, budget,
validation-order, cleanup, manifest, or target drift blocks this slice.

## Evidence commands

Planning inspected issue #408, Discussion #399, `document.rs`, `lib.rs`, DID
document and hardening tests, code-health policy, and canonical specifications.
Before implementation run `cargo test -p identus-did --all-features`. Afterward
run the same suite, strict crate/workspace Clippy and formatting,
public/error/source/factory contracts, exact code-health verification,
portable targets, relevant Nix gates, and protected exact-head CI.
