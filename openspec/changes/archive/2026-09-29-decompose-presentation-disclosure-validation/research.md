# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-29
Source retrieval date: 2026-09-29
Research blockers: none

## Problem and existing implementation

At protected `develop@e15314797408b5d7904b5075081dadb144254276`,
`PresentationDisclosurePlan::new` performs this exact ordered transition:

1. reject an empty or oversized selection collection;
2. revalidate the candidate set against the exact request;
3. reject a duplicate `(query_id, credential_handle)` selection;
4. for each selection in caller order, require its query and candidate;
5. for each selected claim in caller order, require a requested path, identical
   intent, and candidate availability;
6. reject a missing required claim for that selection;
7. for each request query in request order, reject absence and then reject
   multiplicity when the query is not marked multiple; and
8. clone the request and retain the caller's selection vector unchanged.

The focused suite already names every individual public error, successful
cross-query handle reuse, collection limits, exact-request rebinding, candidate
revalidation, debug redaction, and static error contracts. It does not bind a
compact multi-fault precedence matrix before production movement.

The canonical signal is 73 SLOC / cognitive 23 / cyclomatic 27. The containing
module is 350 authored nonblank lines and is below the 1,000-line threshold.

## Normative sources

Issue #444, Discussion #399, ADR 0115, archived issue #407 evidence, and the
canonical presentation-module-ownership, code-health, input-resource,
dependency-boundary, and spec-driven-delivery contracts are authoritative. No
external protocol or crate research is required because this slice changes no
algorithm, wire grammar, dependency, or interoperability decision.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Private borrowed validation context with semantic phases | `adopt` | One owner can retain request/candidate/selection authority while separating deterministic validation from successful construction. | Characterization requires extra public bounds, callback indirection, or reordered errors. |
| Keep constructor intact and document an exception | `not-adopt` | The current control flow is correct and cohesive enough to retain if the proposed boundary obscures order. | The stop/go review rejects the private context. |
| One helper per condition | `not-adopt` | Forwarding fragments displace metrics without creating ownership. | Never as a metric-only technique. |
| Shared presentation/OID4VP validation framework | `not-adopt` | Different authorities, error vocabularies, and change cadence would increase coupling. | A separately approved invariant appears unchanged in multiple consumers. |
| Adopt a validation dependency | `not-adopt` | No crate owns Identus request/candidate/claim correlation or its exact error priority. | A maintained crate proves identical semantics and portable targets. |

## Compatibility and dependency evidence

The constructor signature, crate-root paths, type identity, constants, derives,
error variants, request clone, selection ordering, `PartialEq` bounds, debug
redaction, and successful values remain unchanged. No manifest, lockfile, MSRV,
unsafe, native, FFI, serialization, wire, feature, or target change is needed.

## Security, privacy and maintenance evidence

Validation remains bounded by existing request, candidate, selection, and
claim constructors. The private context borrows validated inputs and does not
clone or format identifiers, handles, paths, or claims. Errors remain closed
enum values. No callback table, trait object, executor, synchronization,
hashing, ordering, or new allocation is introduced.

## Rejected or deferred candidates

Mechanical condition helpers, shared protocol validation, new dependencies,
selection ranking, consent, proof generation, policy inference, caching, and
performance thresholds are rejected or deferred.

## Open questions and blockers

There are no planning blockers. Characterization is a stop/go gate. If one
private owner cannot preserve exact phase and iteration priority without extra
indirection, production remains unchanged and the signal receives a measured
exception instead.

## Evidence commands

Planning inspected the exact protected base, issue #444, current selection
owner, complete presentation tests, canonical report, crate manifest, and
governing specs. Before production edits, run focused tests and add the compact
precedence characterization. Afterward run focused/workspace tests, strict
Clippy/format/docs, public/source/code-health/factory checks, portable targets,
relevant Nix gates, distinct exact-diff review, and protected exact-head CI.
