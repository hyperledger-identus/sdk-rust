# Design

## Private ownership boundary

`GeneratedPresentation::new` remains the public constructor and owns collection
cardinality, exact request/plan preflight, and successful plan/artifact
retention. A private borrowed `GeneratedPresentationValidator<'a>` owns
deterministic aggregate byte-budget, binding correlation, format, duplicate,
and coverage validation.

The owner exposes one validation operation implemented through semantic phases:

- validate the checked aggregate payload budget;
- validate artifacts and bindings in caller order; and
- validate plan coverage in plan-selection order.

Small helpers are acceptable only when they own one complete invariant. There
is no generic predicate/callback graph and no helper per branch.

## Exact validation order

The design preserves this order:

1. generated artifact collection cardinality;
2. disclosure-plan rebinding to the exact request;
3. checked aggregate payload budget;
4. artifact-order and binding-order selection lookup;
5. request query lookup, then artifact/query format equality;
6. duplicate binding search across earlier artifacts;
7. plan-order selection coverage; and
8. successful plan/artifact retention.

The validator borrows only. It cannot mutate, reorder, clone, index, or format
caller values.

## Characterization boundary

Before production movement, a compact multi-fault matrix binds phase priority
that individual existing tests do not demonstrate directly. Existing tests
continue to bind every error variant, collection/byte boundary, valid direct
and aggregate forms, debug redaction, and successful ordering.

## Compatibility and code-health ratchet

The normalized public API and manifests remain identical. The touched
constructor signal must disappear without producing an equivalent large
validator, forwarding-only helper chain, generated code, moved test, waiver,
index allocation, or weaker threshold. Canonical evidence is rebound to the
protected implementation squash in a second issue-linked PR.

## Risks and rollback

Primary risks are error-priority drift, changing nested caller iteration order,
accidental extra allocation, and hiding plan/request authority. The precedence
matrix and exact-diff review make those visible. Rollback inlines the private
validator without any consumer or stored-data migration.

