# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-29
Source retrieval date: 2026-09-29
Research blockers: none

## Problem and existing implementation

At protected `develop@6e4183a8e02ef0f2ed9984279d91e0660a36125f`, the
public consuming method obtains the selected Authorization Server endpoint,
parses and strictly validates any existing form query, constructs bounded
Authorization Details, projects optional issuer state, materializes the fixed
managed parameter list, computes the complete encoded length with checked
arithmetic, enforces the final URI ceiling, allocates exactly once, and renders
the endpoint plus managed parameters in fixed order.

The private helpers already own strict existing-query validation, parameter
enumeration, checked form length, bounded JSON writing, and form encoding. The
public method is therefore orchestration rather than the owner of those
grammars. Its current error order is endpoint requirement and endpoint-query
parsing/validation before Authorization Details, then checked complete length,
then final ceiling and rendering. Existing tests bind each isolated branch and
positive wire form but do not bind one combined-fault matrix across those
phases.

The canonical signal is
`try_into_authorization_request`: 60 / 7 / 18. The 343-line module is below the
attention threshold and has no module signal.

## Normative sources

Issue #467, Discussion #399, ADRs 0083, 0102 and 0141, the archived issue #354
research and verification, and the canonical Authorization Request,
resource-boundary, dependency-boundary, code-health, and spec-driven-delivery
contracts are authoritative. The accepted OpenID4VCI 1.0 Final, RFC 6749,
RFC 7636, RFC 9396 and RFC 9700 decisions remain unchanged. No new external
protocol or crate research is required because this slice changes no grammar,
field, dependency, wire form, or interoperability decision.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| One private borrowed request-assembly owner | `adopt` | It aligns endpoint/query preparation, bounded intent construction, checked sizing and rendering behind the consuming transition without public exposure. | Characterization requires a public type, duplicated grammar, extra allocation, or reordered failure. |
| Keep the method intact and document an exception | `not-adopt` | Correctness remains the fallback if an owner obscures the exact sequence. | Stop/go review rejects the ownership boundary. |
| Separate planner and renderer public types | `not-adopt` | It exposes lifecycle mechanics and creates an unnecessary intermediate API. | A separately approved adapter capability needs a public prepared request. |
| One helper per parameter or conditional | `not-adopt` | Mechanical forwarding would displace metrics without semantic ownership. | Never as a metric-only technique. |
| Replace local form/JSON mechanics with general crates | `not-adopt` | ADRs 0083 and 0102 already reject lossy/permissive or over-broad production dependencies. | A separate ADR proves exact bounded parity and multi-capability payoff. |
| Add PAR/JAR, scope/resource, extensions, or transport | `not-adopt` | Product and protocol growth is outside this refactor. | Separately approved capability evidence requires it. |

## Compatibility and dependency evidence

The public method, types, paths, constants, limits, request bytes, parameter
order, errors and precedence, sensitive accessor, predecessor lineage,
zeroizing ownership, and redacted Debug form remain unchanged. No manifest,
lockfile, feature, dependency, MSRV, unsafe, native, FFI, serialization, wire,
or target change is needed.

## Security, privacy and maintenance evidence

Existing endpoint-query parameter/name/value limits, strict decoded duplicate
and reserved-name rejection, Authorization Details ceiling, checked encoded
length, final URI ceiling, and single exact allocation remain authoritative.
The owner borrows the validated input during preparation and rendering, then
the public result consumes the same input exactly once. No caller value enters
diagnostics. No callback, trait object, executor, synchronization, hashing,
ambient I/O, or unbounded work path is added.

## Rejected or deferred candidates

PAR, JAR, request objects, caller extensions, scope/resource, browser dispatch,
HTTP execution, callback correlation, token exchange, new fields, external
OAuth/form dependencies, and performance thresholds are rejected or deferred.

## Open questions and blockers

There are no planning blockers. Characterization is a stop/go gate. If one
private owner cannot preserve exact query-before-details-before-size precedence,
wire ordering, and allocation behavior without duplicating policy, production
remains unchanged and the signal receives a measured exception.

## Evidence commands

Planning inspected the exact protected base, issue #467, issue #354 evidence,
the complete implementation and focused tests, canonical report, manifests,
and governing specs. Before production edits, run the focused Authorization
Request suite and add the combined-fault matrix. Afterward run focused/workspace
tests, strict Clippy/format/docs, public/source/code-health/factory checks,
portable targets, relevant Nix gates, distinct exact-diff review, and protected
exact-head CI.
