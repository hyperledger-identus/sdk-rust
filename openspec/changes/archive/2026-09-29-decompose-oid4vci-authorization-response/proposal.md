# Decompose OID4VCI Authorization Response ownership

## Why

The module-level decomposition milestone removed every production module above
1,000 authored nonblank lines. The Authorization Response capability still has
two unowned product-code signals: the consuming correlation method at 78 SLOC,
cognitive complexity 21, cyclomatic complexity 25, and bounded query decoding
at 75 / 16 / 25. They combine distinct parsing and authority-transition
responsibilities. Issue #464 requires characterization before movement because
state, RFC 9207 issuer, branch, and field-grammar precedence are security
observable.

## What changes

- Bind query, state/issuer, branch-shape, and value-grammar precedence before
  production edits.
- Keep `AuthorizationRequest::try_into_authorization_response` as the unchanged
  consuming public entry point.
- Give bounded strict-form query decoding and request-bound outcome correlation
  distinct private owners with semantic phases.
- Preserve percent decoding, decoded duplicate detection, parameter and role
  limits, unknown-field handling, zeroizing ownership, exact errors, request
  lineage, issuer evidence, and success/error values.
- Remove both touched signals only if the resulting owners remain cohesive and
  easier to review than the current functions.

## Capability

### Modified capability

- `oid4vci-authorization-response`: decoding and correlation have explicit
  private ownership and unchanged outward protocol behavior.

## Non-goals

No callback transport/routing, browser behavior, form-post, JARM, PAR,
confidential-client authentication, new OAuth/OID4VCI fields, trust policy,
public helper, dependency, feature, wire form, error, or resource-budget change.

## Delivery

Issue #464 owns planning, characterization, implementation, protected evidence,
OpenSpec archive, and metrics. Protected squash delivery requires an
implementation PR followed by a canonical-evidence closeout.
