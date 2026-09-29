# Design

## Private ownership boundary

`PreparedRequest::new` remains the private construction entry point. A private
owned preparation state clones the same caller values and owns the typed
resolution fields, selector fields, extension map, and custom-resource flag
that are currently local variables.

The owner exposes semantic operations for:

- inserting the verification relationship during initialization;
- parsing and applying the complete sorted query parameter map;
- applying one parameter without changing existing typed validators; and
- constructing `ResolutionOptions`, validating selector combinations, and
  returning `PreparedRequest`.

Small helpers are acceptable only when they own one complete invariant. There
is no generic predicate/callback graph and no helper per conditional.

## Exact validation order

The design preserves this order:

1. caller `accept`/extensions clone;
2. verification-relationship extension collision;
3. complete query decode/text/duplicate validation;
4. decoded-name sorted parameter application;
5. `ResolutionOptions` construction and JSON extension validation;
6. relative-reference service-selector requirement;
7. verification-relationship fragment and selector restrictions; and
8. successful retention of identical prepared values.

The owner cannot mutate the DID URL or caller options, reorder the `BTreeMap`,
introduce a second decoding path, invoke the resolver, or expose state publicly.

## Characterization boundary

Before production movement, a compact multi-fault matrix binds collision-
before-query, query/parameter-before-cross-field, and all-preparation-before-
resolver behavior. Existing tests continue to bind every accepted parameter,
typed projection, individual rejection, service/fragment behavior, resource
boundary, redaction contract, and successful resolver call/input.

## Compatibility and code-health ratchet

The normalized public API and manifests remain identical. The touched
constructor signal must disappear without producing an equivalent large
builder method, forwarding-only helper chain, generated code, moved test,
waiver, new allocation, or weaker threshold. Canonical evidence is rebound to
the protected implementation squash in a second issue-linked PR.

## Risks and rollback

Primary risks are failure-priority drift, changing sorted parameter order,
changing extension collision behavior, invoking the resolver after a former
preflight rejection, and silently changing resolver inputs. The combined-fault
matrix, existing recording resolver, and exact-diff review make those visible.
Rollback inlines the private owner without consumer or stored-data migration.
