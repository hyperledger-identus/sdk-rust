# Design

## Private ownership boundary

`dereference_services` remains the private entry point. A private borrowed
context retains the DID URL, resolved document, optional resolver content type,
and prepared request, and owns the same content metadata currently consumed by
the function.

The owner exposes only two semantic phases:

- select matching services by expanding the optional service ID and applying
  ID and service-type constraints conjunctively in document order; and
- route the non-empty selection to endpoint or filtered-document output from
  the existing force-URI and accepted-media rules.

Existing `expand_service_id`, `filtered_document_result`, and
`endpoint_result` helpers retain their complete invariants. There is no generic
predicate/callback graph and no helper per match arm.

## Exact policy order

The design preserves this order:

1. optional service-ID expansion and URI validation;
2. one source-order scan with conjunctive ID and service-type filtering;
3. cloning retained services and rejecting an empty selection as `notFound`;
4. deriving endpoint-forcing from `relativeRef` or a DID URL fragment;
5. routing absent `accept` to endpoints when forced, otherwise a filtered
   document with the resolver content type;
6. routing `text/uri-list` to endpoints;
7. routing an accepted DID document media type to endpoints when forced,
   otherwise a filtered document with the requested content type; and
8. rejecting every other explicit representation.

The owner cannot mutate inputs, reorder services, add a second URI or media
grammar, inspect endpoint contents during selection, invoke the resolver,
broaden endpoint-map support, or expose state publicly.

## Characterization boundary

Before production movement, a compact matrix binds ID spelling, conjunctive
selection, empty-selection priority, implicit/explicit media routing,
fragment/relative-reference forcing, and endpoint-map behavior. Existing tests
continue to bind URI resolution, traversal defense, endpoint ordering,
metadata, resolver behavior, and public result conversion.

## Compatibility and code-health ratchet

The normalized public API and manifests remain identical. The touched function
signal must disappear without producing an equivalent context method,
forwarding-only helper chain, generated code, moved test, waiver, new
allocation, or weaker threshold. Canonical evidence is rebound to the
protected implementation squash in a second issue-linked PR.

## Risks and rollback

Primary risks are selection-order drift, changing `notFound` versus
representation errors, changing implicit content type, mishandling forced
endpoint output, and extra clones. The matrix, existing corpus, allocation
review, and exact-diff review make those visible. Rollback inlines the private
context without consumer or stored-data migration.
