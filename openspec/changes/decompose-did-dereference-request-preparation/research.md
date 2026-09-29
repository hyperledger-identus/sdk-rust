# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-29
Source retrieval date: 2026-09-29
Research blockers: none

## Problem and existing implementation

At protected `develop@9288221dfe5288a4fbe78b57ba4d06057d9f181d`,
`PreparedRequest::new` performs this exact ordered transition:

1. clone the caller's `accept` and extension values and initialize empty typed
   resolution/query-selector state;
2. insert `verificationRelationship` into the cloned extensions, failing with
   `InvalidOptions` on a collision before inspecting the DID URL query;
3. parse the complete query by splitting on `&`, splitting each item once on
   `=`, percent-decoding name and value once, validating text, and rejecting
   duplicate decoded names into a `BTreeMap`;
4. apply that map in decoded-name sort order, validating known resolution
   parameters and generic service selectors while retaining `hl` and unknown
   parameters as extensions;
5. construct and validate `ResolutionOptions`, mapping any structural failure
   to `InvalidOptions`;
6. reject `relativeRef` without `service` or `serviceType`; then
7. require any verification relationship to have a fragment and no service,
   service-type, or relative-reference selector; otherwise retain the prepared
   values unchanged.

`parse_parameters`, `MediaType`, `VersionId`, `DidResolutionDateTime`,
`validate_relative_ref`, and `ResolutionOptions::new` remain the authoritative
grammar and resource boundaries. Existing DID URL dereferencing tests cover
individual malformed, duplicate, collision, typed projection, cross-field,
redaction, successful resolver-input, and resolver non-invocation cases. They
do not bind a compact combined-fault phase-priority matrix before movement.

The canonical signal is 127 SLOC / cognitive 18 / cyclomatic 43. The containing
module is below the 1,000-line threshold, and no production module currently
exceeds that threshold.

## Normative sources

Issue #452, Discussion #399, ADR 0014, ADR 0115, the archived issue #46
implementation/review evidence, and the canonical DID core, code-health,
input-resource, dependency-boundary, and spec-driven-delivery contracts are
authoritative. No new external protocol or crate research is required because
this slice changes no algorithm, wire grammar, dependency, or interoperability
decision.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Private owned preparation state with semantic phases | `adopt` | One owner can retain exact mutable state and ordering while separating query application from final validation. | Characterization requires public exposure, reordered map traversal, extra allocation, or callback indirection. |
| Keep constructor intact and document an exception | `not-adopt` | The current implementation is correct and remains preferable if extraction obscures the pre-resolution sequence. | Stop/go review rejects the private owner. |
| One helper per parameter arm | `not-adopt` | Mechanical forwarding fragments displace metrics without creating ownership. | Never as a metric-only technique. |
| Replace `BTreeMap` with source-order parsing or a hash map | `not-adopt` | Decoded-name sort order and duplicate semantics are existing observable security behavior. | Separately approved protocol evidence requires source order. |
| Adopt a URL/form/query crate | `not-adopt` | This refactor must preserve literal-plus, single-decoding, duplicate, and DID URL-specific policy exactly; it is not a parser replacement decision. | A separate ADR demonstrates exact semantic equivalence and a justified dependency cone. |

## Compatibility and dependency evidence

The public dereferencer signature, crate-root paths, type identity, constants,
errors, resolver call count/input, parameter order, successful values, and all
W3C-facing behavior remain unchanged. No manifest, lockfile, MSRV, unsafe,
native, FFI, serialization, wire, feature, or target change is needed.

## Security, privacy and maintenance evidence

Preparation remains bounded by the validated DID URL, dereferencing options,
extension JSON budgets, and existing typed constructors. The private owner
keeps the same cloned state that the constructor already owns and never formats
caller values. Failures remain a private two-variant enum projected to static
standard errors before any resolver call. No new callback, trait object,
executor, synchronization, hashing, ambient I/O, or allocation class is added.

## Rejected or deferred candidates

Generic query frameworks, new dependencies, URL normalization, service
retrieval, method resources, resolver orchestration, fragment projection,
relative-reference semantics, cache behavior, and performance thresholds are
rejected or deferred.

## Open questions and blockers

There are no planning blockers. Characterization is a stop/go gate. If one
private owner cannot preserve exact collision-before-query, full-parse-before-
application, sorted application, typed-construction, and cross-field priority
without extra indirection, production remains unchanged and the signal receives
a measured exception instead.

## Evidence commands

Planning inspected the exact protected base, issue #452, preparation function,
complete dereferencing corpus, archived issue #46 evidence, canonical report,
crate manifest, and governing specs. Before production edits, run focused DID
tests and add the compact precedence/non-invocation characterization. Afterward
run focused/workspace tests, strict Clippy/format/docs, public/source/code-
health/factory checks, portable targets, relevant Nix gates, distinct exact-
diff review, and protected exact-head CI.
