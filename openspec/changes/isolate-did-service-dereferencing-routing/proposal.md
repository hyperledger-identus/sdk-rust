# Isolate DID service dereferencing selection and routing

## Why

The module-level decomposition milestone removed every production module above
1,000 authored nonblank lines. After issue #459, `dereference_services` is the
next unowned DID function signal at 66 SLOC, cognitive complexity 8, and
cyclomatic complexity 20. It combines service-ID expansion, conjunctive
selection, empty-result handling, fragment/relative-reference mode selection,
and media negotiation. Issue #461 requires characterization before movement so
a metric improvement cannot change service filtering or result precedence.

## What changes

- Bind the service-selector and output-routing matrix before production edits.
- Keep the existing generic dereferencer entry point and all public result
  types unchanged.
- Give borrowed dereferencing context one private owner with cohesive service
  selection and result-routing phases.
- Preserve service order, cloning, ID expansion, conjunctive type filtering,
  not-found behavior, endpoint forcing, accepted media handling, content
  metadata, and exact failures.
- Remove the touched function signal only if the private owner makes the
  current policy easier to review without fragmenting it into predicates.

## Capability

### Modified capability

- `did-core`: generic service dereferencing has explicit private ownership and
  unchanged selection, negotiation, and result behavior.

## Non-goals

No new DID method, service type, endpoint representation, media type, network
retrieval, URL normalization, cache behavior, public helper, dependency,
feature, wire form, error, resource budget, or performance threshold.

## Delivery

Issue #461 owns planning, characterization, implementation, protected evidence,
OpenSpec archive, and metrics. Protected squash delivery requires an
implementation PR followed by a canonical-evidence closeout.
