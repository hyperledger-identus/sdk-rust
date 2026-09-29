# Design

## Private ownership boundary

`PresentationDisclosurePlan::new` remains the public constructor and owns
cardinality preflight plus successful request/selection retention. A private
borrowed `DisclosurePlanValidator<'a>` owns deterministic correlation of the
request, candidate set, and selection slice.

The owner exposes one validation operation implemented through semantic phases:

- reject duplicate selection identities;
- validate each selection and its selected claims in caller order; and
- validate query coverage and multiplicity in request order.

Small helpers are acceptable only when they own one complete invariant such as
one selected claim. There is no generic predicate/callback graph and no helper
per branch.

## Exact validation order

The design preserves this order:

1. selection collection cardinality;
2. candidate-set rebinding and revalidation;
3. duplicate selection identity;
4. selection query existence, then candidate existence;
5. selected-claim requested path, then intent, then availability;
6. per-selection required-claim coverage;
7. request-order query presence, then single-query multiplicity; and
8. successful cloning/retention.

The validator borrows only. It cannot mutate, reorder, clone, or format caller
values.

## Characterization boundary

Before production movement, a compact multi-fault matrix binds phase priority
that individual existing tests do not demonstrate directly. Existing tests
continue to bind every error variant, collection boundary, valid cross-query
handle reuse, request rebinding, debug redaction, and successful ordering.

## Compatibility and code-health ratchet

The normalized public API and manifests remain identical. The touched
constructor signal must disappear without producing an equivalent large
validator, forwarding-only helper chain, generated code, moved test, waiver,
or weaker threshold. Canonical evidence is rebound to the protected
implementation squash in a second issue-linked PR.

## Risks and rollback

Primary risks are error-priority drift, changing caller iteration order,
accidental extra clones, and hiding request/candidate authority. The precedence
matrix and exact-diff review make those visible. Rollback inlines the private
validator without any consumer or stored-data migration.
