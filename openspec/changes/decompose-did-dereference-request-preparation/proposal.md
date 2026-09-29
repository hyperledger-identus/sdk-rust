# Decompose DID dereferencing request preparation

## Why

After the module-level decomposition milestone removed every production module
above 1,000 authored nonblank lines, `PreparedRequest::new` remains a
concentrated function-level signal at 127 SLOC, cognitive complexity 18, and
cyclomatic complexity 43. It combines option projection, query decoding,
parameter interpretation, resolution-option construction, and cross-field
validation. Issue #452 requires characterization before movement so a metric
improvement cannot change the security-sensitive pre-resolution boundary.

## What changes

- Bind the current preparation/error-priority and resolver non-invocation
  contract before production edits.
- Keep `PreparedRequest` as the private result consumed by the existing generic
  dereferencer.
- Give mutable preparation state one private owner with semantic phases for
  query parameter application, resolution construction, and cross-field
  validation.
- Preserve decoded-name ordering, parameter spellings, extensions, bounds,
  exact failure kinds, resolver inputs, and successful retained values.
- Remove the touched function signal only if the resulting owner remains more
  cohesive and reviewable than the current constructor.

## Capability

### Modified capability

- `did-core`: generic DID URL request preparation has explicit private
  ownership and unchanged outward dereferencing behavior.

## Non-goals

No new DID URL parameter, W3C behavior, method semantics, endpoint retrieval,
resolver/dereferencer API, media representation, public helper, dependency,
feature, wire form, cache, registry, transport, persistence, or runtime policy.

## Delivery

Issue #452 owns planning, characterization, implementation, protected evidence,
OpenSpec archive, and metrics. Protected squash delivery requires an
implementation PR followed by a canonical-evidence closeout.
