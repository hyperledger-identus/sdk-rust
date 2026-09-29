# Isolate DID Resolution HTTP query collection

## Why

The production module-decomposition program is complete, and the cohesive
`DidDocument::validate` residual remains intentionally unsplit under issue
#408 research. The next unowned adapter signal with a plausible responsibility
boundary is `decode_resolution_options`: 78 SLOC / cognitive 13 / cyclomatic
32. It combines representation-derived `accept`, raw-query preflight, ordered
component decoding, duplicate/control checks, known-option conversion,
extension collection, version correlation, and final option construction.

Issue #473 requires characterization before movement because the first
`invalidOptions` phase determines whether the resolver is invoked and is
observable through the W3C HTTP binding.

## What changes

- Bind query size/count/structure/component/duplicate/type/conflict/final
  construction priority before production edits.
- Keep the Axum router, handler, public API, response mapping, resolver port,
  `ResolutionOptions`, and percent-decoder behavior unchanged.
- Introduce at most one private query-fields owner so the coordinator retains
  representation and empty-query projection while query collection owns
  source-order field state.
- Preserve exact decoded values, extension ordering, limits, W3C errors,
  redaction, allocation classes, and resolver non-invocation on rejection.
- Remove the touched signal only if the new boundary creates no equivalent
  replacement signal or helper-per-parameter chain.

## Capability

### Modified capability

- `did-resolution-http`: strict bounded GET resolution options gain explicit
  private query-field ownership with unchanged outward behavior.

## Non-goals

No new option, DID Resolution feature, query or form framework, OAuth/OIDC
abstraction, public parser, dependency, feature, limit, error, routing,
runtime, resolver, representation, or target change.

## Delivery

Issue #473 owns planning, characterization, implementation, protected evidence,
OpenSpec archive, Discussion #399 update, and metrics. Protected squash delivery
requires an implementation PR followed by a canonical-evidence closeout.
