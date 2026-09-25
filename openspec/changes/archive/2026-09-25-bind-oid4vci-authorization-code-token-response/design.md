# Design

## Consuming response transition

Add `AuthorizationCodeTokenHttpResponseLimits`,
`AuthorizationCodeTokenResponseOutcome`,
`RequestBoundAuthorizationCodeTokenResponse`, and
`RequestBoundAuthorizationCodeTokenErrorResponse`. Expose
`AuthorizationCodeTokenRequest::try_bind_response`, taking already-extracted
status, Content-Type, Cache-Control, Pragma and body values.

The method first consumes the request into a private lineage object and its
zeroizing form body, then explicitly drops the form body before inspecting the
remote response. The private lineage contains only Credential Issuer Metadata,
Authorization Server Metadata, selected Credential Configuration and unchanged
issuer-identification evidence. Both public outcome branches delegate those
read-only queries through the same private owner.

## Ordered validation and classification

Validation applies in this order:

1. Classify exact status `200`, `400` or `401`; reject every other value.
2. Bound Content-Type, then require strict `application/json` with valid
   optional parameters.
3. Bound Cache-Control, then require a syntactically valid bare `no-store`.
4. Bound Pragma, then require a syntactically valid bare `no-cache`.
5. Parse `200` with `TokenResponseCore` and `400`/`401` with
   `TokenErrorResponseCore` under their independently supplied limits.
6. Require a parsed `401` error to be exact `invalid_client`.
7. Construct exactly one request-bound success or error outcome.

Status wins over body shape. The envelope does not infer outcome from JSON and
does not parse a body after an earlier status or header failure.

## Limits and HTTP field reuse

The composite limits type owns the existing success and error body limits plus
positive Content-Type, Cache-Control and Pragma field-value caps. Defaults use
1,024 bytes per effective header value and the existing body defaults.

Generalize the private cache directive scanner into an internal
`has_bare_directive` primitive while retaining the current
`has_bare_no_store` wrapper unchanged. Add only a crate-private
`has_bare_no_cache` wrapper for Pragma. Duplicate or combined HTTP field
handling stays outside the SDK; callers pass one effective bounded value.

## Error and compatibility surface

Append nine fieldless diagnostics: invalid limits; invalid status; a `401`
status/error mismatch; oversized
and invalid Content-Type; oversized and invalid Cache-Control; oversized and
invalid Pragma. Keep them in one focused authorization-code token-response
catalogue after every existing live row. The central wildcard-free router maps
each variant explicitly and all previous rows remain exact.

The feature is additive and unpublished. It changes no current parser,
constructor, serialization, dependency, feature, target or baseline error row.

## Risks and mitigations

- Outcome confusion: classify from the narrow RFC status set before parsing.
- Code/verifier retention: consume the request and drop its zeroizing body
  before remote validation.
- Token leakage: reuse zeroizing response cores and redact all new Debug and
  error surfaces.
- Cache leakage: require strict bounded `no-store` and `no-cache` evidence on
  both branches without claiming external cache compliance.
- Endpoint substitution: lineage comes only from the consumed request; the
  method does not accept a replacement endpoint or metadata object.
- Mix-up overclaim: preserve #356 evidence exactly.
- Resource exhaustion: independent positive header and success/error body caps
  precede copies and semantic parsing owned by this layer.

## Rollback

Remove the additive module/export/limits/errors/tests/ADR and private consuming
request helper. Existing request construction and response-core parsing remain
unchanged.
