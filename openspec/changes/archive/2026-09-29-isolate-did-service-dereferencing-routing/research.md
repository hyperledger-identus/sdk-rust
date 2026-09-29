# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-29
Source retrieval date: 2026-09-29
Research blockers: none

## Problem and existing implementation

At protected `develop@d27e455901c501d9611abbe0065e6a9b7270dd57`,
`dereference_services` performs this exact ordered transition:

1. expand an optional selector as an absolute URI, document-relative fragment,
   or document-relative bare name and reject an invalid expanded URI;
2. scan document services in source order, retaining only entries matching the
   optional expanded ID and optional service type conjunctively;
3. clone retained services and return `notFound` when none remain;
4. force endpoint output when `relativeRef` or a DID URL fragment exists;
5. with no explicit `accept`, return endpoints when forced and otherwise a DID
   document containing only retained services with the resolver content type;
6. for `text/uri-list`, always return URI endpoints;
7. for a DID document media type, return endpoints when forced and otherwise a
   filtered document with the requested content type; and
8. reject every other explicit representation as `representationNotSupported`.

`expand_service_id`, `filtered_document_result`, and `endpoint_result` retain
their existing policy. Endpoint projection still ignores map endpoints, keeps
URI order, applies bounded `relativeRef` resolution before fragment attachment,
requires exactly one endpoint for a fragment, and returns the existing static
errors. Existing tests cover individual selection, media, endpoint-map,
relative-reference, fragment, traversal, and method-resource cases. They do
not bind one compact cross-product that proves selector failure and empty
selection precede representation routing.

The canonical signal is 66 SLOC / cognitive 8 / cyclomatic 20. The containing
module is below the 1,000-line attention threshold, and no production module
exceeds that threshold. Issue #408 deliberately retained
`DidDocument::validate` as one cohesive cross-resource aggregate invariant, so
it is not a candidate for this slice.

## Normative sources

Issue #461, Discussion #399, ADR 0014, ADR 0115, issue #452 and its archived
request-preparation evidence, the existing dereferencing tests, and the
canonical DID core, code-health, input-resource, dependency-boundary, and
spec-driven-delivery contracts are authoritative. No new external protocol or
crate research is required because this slice changes no algorithm, wire
grammar, dependency, or interoperability decision.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Private borrowed service-dereferencing context with selection and routing phases | `adopt` | One owner can retain the inputs and exact policy order while naming the two existing responsibilities. | Characterization requires public exposure, extra allocation, or reordered policy. |
| Keep the function intact and document an exception | `not-adopt` | The current function is correct and remains preferable if ownership adds indirection or splits one invariant mechanically. | Stop/go review rejects the private owner. |
| One predicate/helper per match arm | `not-adopt` | Mechanical forwarding would move metrics without clarifying policy authority. | Never as a metric-only technique. |
| Replace selection with a generic query/filter framework | `not-adopt` | Existing source order, cloned result ownership, ID expansion, type matching, and errors are DID-specific. | A separate ADR proves exact semantics and justified reuse. |
| Change endpoint or media negotiation behavior | `not-adopt` | That is product/protocol work outside a refactor. | Separately approved conformance evidence requires it. |

## Compatibility and dependency evidence

The public dereferencer signature, crate-root paths, type identity, media and
error values, service order, selected document, endpoint output, metadata,
resolver inputs, and all W3C-facing behavior remain unchanged. No manifest,
lockfile, MSRV, unsafe, native, FFI, serialization, wire, feature, or target
change is needed.

## Security, privacy and maintenance evidence

The private owner borrows the same DID URL, document, prepared request, and
resolved content type while owning the same content metadata already consumed
by the function. Selection performs the same single bounded scan and same
clones. It adds no callback, trait object, executor, synchronization, hashing,
ambient I/O, diagnostics, or allocation class. Existing URI, extension,
relative-reference, endpoint, and document bounds remain authoritative.

## Rejected or deferred candidates

Network endpoint retrieval, method resources, new media types, URL
normalization, selection caching, borrowed public result views, endpoint map
interpretation, performance thresholds, and document-validation decomposition
are rejected or deferred.

## Open questions and blockers

There are no planning blockers. Characterization is a stop/go gate. If one
private context cannot preserve selector-expansion-before-selection,
empty-selection-before-media-routing, and endpoint-forcing precedence without
extra allocation or indirection, production remains unchanged and the signal
receives a measured exception instead.

## Evidence commands

Planning inspected the exact protected base, issue #461, issues #408 and #452,
the complete service dereferencing path and tests, the canonical report, crate
manifest, and governing specs. Before production edits, run the focused DID
dereferencing suite and add the compact selection/routing matrix. Afterward run
focused/workspace tests, strict Clippy/format/docs, public/source/code-health/
factory checks, portable targets, relevant Nix gates, distinct exact-diff
review, and protected exact-head CI.
