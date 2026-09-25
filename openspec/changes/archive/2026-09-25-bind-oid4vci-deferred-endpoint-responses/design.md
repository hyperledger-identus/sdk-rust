# Design

## Closed consuming transition

`RequestBoundDeferredCredentialRequest::try_into_deferred_credential_endpoint_response`
consumes the request and matches status before inspecting Content-Type or body.
It returns `DeferredCredentialEndpointResponseOutcome` with exactly three
branches: issued, pending, or deferred payload error.

## Branch ownership

- `200`: read proof count, drop the complete request, parse through the existing
  deferred-success media/body policy, then cap issued credentials to proof
  count and return `RequestBoundImmediateCredentialResponse`.
- `202`: destructure the request, erase its serialized body, retain its
  zeroizing request transaction only for equality, and move issuer, endpoint,
  Authorization and proof count into the existing continuation authority. Parse
  and correlate with the existing deferred-success rules. Only exact success
  returns `RequestBoundDeferredCredentialResponse`; every error drops authority.
- `400`: read proof count, drop the complete request, then parse and classify
  with the existing deferred payload-error implementation. Return a redacted
  request-bound wrapper containing only response evidence and proof count.
- other: drop the complete request and return the existing static unsupported
  Deferred Credential status error.

## Internal reuse

Extract private helpers or constructors only where necessary so the borrowed
and consuming paths share existing media, body, correlation, error and
cardinality rules. The existing public borrowed methods and their error
precedence do not change.

`DeferredCredentialEndpointResponseLimits` composes the existing successful
Deferred Credential HTTP policy with the existing Credential Error HTTP policy.
Its constructor is infallible because both child policies are already valid.

## Public result types

The issued and pending branches reuse existing request-bound public types. The
error branch adds `RequestBoundDeferredCredentialErrorResponse`, exposing the
originating proof count and the existing bounded deferred error response by
reference or consuming extraction. It owns no bearer, endpoint, issuer,
transaction or request body. Debug remains redacted through the nested response.

No public method accepts a replacement endpoint, bearer, issuer, transaction or
proof count. No bound state implements Clone or generic Serde.

## Error and compatibility model

Reuse existing status, media, body, transaction mismatch and proof-count
errors. Add no error variant or wire shape. Borrowed success/error validation
remains repeatable and behavior-compatible; the consuming API is the stronger
one-shot option.

## Rejected alternatives

- Returning a new wire model duplicates mature bounded parsers.
- Keeping the request beside every result extends bearer lifetime unnecessarily.
- Returning the same request on failure enables ambiguous retries and secret
  persistence.
- Automatic polling combines protocol data with transport and product policy.
