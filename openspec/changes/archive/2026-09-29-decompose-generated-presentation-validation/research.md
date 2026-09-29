# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-29
Source retrieval date: 2026-09-29
Research blockers: none

## Problem and existing implementation

At protected `develop@f1873f39ad4154a7139553e3d3c8221ac6abecf7`,
`GeneratedPresentation::new` performs this exact ordered transition:

1. reject an empty or oversized artifact collection;
2. revalidate the disclosure plan against the exact request;
3. sum artifact byte lengths with checked arithmetic and reject overflow or an
   aggregate above `MAX_GENERATED_PRESENTATION_BYTES`;
4. for every artifact in caller order and every binding in binding order,
   require an exact plan selection;
5. require the selection query and identical artifact/query format;
6. reject the same binding when it appeared in an earlier artifact; and
7. for every plan selection in plan order, require coverage by some artifact,
   then retain the plan and caller artifact vector unchanged.

`PresentationArtifact::new` separately enforces non-empty bounded unique
bindings and a non-empty bounded payload. Existing presentation tests cover
every individual generated-presentation error, successful per-credential and
aggregate artifacts, total byte bounds, same-format correlation, ordering,
debug redaction, and static error contracts. They do not bind a compact multi-
fault precedence matrix before production movement.

The canonical signal is 57 SLOC / cognitive 16 / cyclomatic 23. The containing
module is below the 1,000-line threshold, and no production module currently
exceeds that threshold.

## Normative sources

Issue #450, Discussion #399, ADR 0115, the archived #407 and #444 evidence,
and the canonical presentation-module-ownership, code-health, input-resource,
dependency-boundary, and spec-driven-delivery contracts are authoritative. No
external protocol or crate research is required because this slice changes no
algorithm, wire grammar, dependency, or interoperability decision.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Private borrowed generated-presentation validator with semantic phases | `adopt` | One owner can retain request/plan/artifact authority while separating deterministic correlation from successful construction. | Characterization requires extra public bounds, indexing allocation, callback indirection, or reordered errors. |
| Keep constructor intact and document an exception | `not-adopt` | The current control flow is correct and remains preferable if extraction obscures its aggregate order. | The stop/go review rejects the private owner. |
| One helper per conditional | `not-adopt` | Forwarding fragments displace metrics without creating ownership. | Never as a metric-only technique. |
| Pre-index bindings with a map or set | `not-adopt` | Existing public bounds make ordered scans sufficient; indexing changes allocation and order evidence. | A separately measured performance requirement proves bounded scans inadequate. |
| Shared OID4VP/artifact framework or dependency | `not-adopt` | Identus presentation-model correlation and its exact error priority are local responsibilities. | A separately approved unchanged invariant has multiple real consumers. |

## Compatibility and dependency evidence

The constructor signature, crate-root paths, type identity, constants, derives,
error variants, plan/artifact ordering, ownership transfer, `PartialEq` bounds,
debug redaction, and successful values remain unchanged. No manifest, lockfile,
MSRV, unsafe, native, FFI, serialization, wire, feature, or target change is
needed.

## Security, privacy and maintenance evidence

Validation remains bounded by existing request, plan, artifact, binding, and
payload constructors. The private context borrows validated inputs and does not
clone or format query ids, credential handles, formats, payloads, or claims.
Errors remain closed enum values. No callback table, trait object, executor,
synchronization, hashing, reordering, or new allocation is introduced.

## Rejected or deferred candidates

Mechanical condition helpers, shared protocol validation, new dependencies,
proof generation, response construction, consent, trust, policy inference,
caching, and performance thresholds are rejected or deferred.

## Open questions and blockers

There are no planning blockers. Characterization is a stop/go gate. If one
private owner cannot preserve exact phase and iteration priority without extra
indirection, production remains unchanged and the signal receives a measured
exception instead.

## Evidence commands

Planning inspected the exact protected base, issue #450, artifact owner,
complete presentation tests, canonical report, crate manifest, and governing
specs. Before production edits, run focused tests and add the compact precedence
characterization. Afterward run focused/workspace tests, strict Clippy/format/
docs, public/source/code-health/factory checks, portable targets, relevant Nix
gates, distinct exact-diff review, and protected exact-head CI.

