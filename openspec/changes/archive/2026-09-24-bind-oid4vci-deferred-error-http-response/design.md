# Design

## Request-bound error surface

Add `DeferredCredentialErrorResponse`, which privately owns an existing
`CredentialErrorResponseCore`, and `DeferredCredentialErrorKind` with exact
`InvalidTransactionId`, `CredentialRequestDenied`, and
`Inherited(CredentialEndpointErrorKind)` branches. The wrapper exposes a
borrowed core for exact bounded code and deliberately untrusted description
access.

`DeferredCredentialRequest::validate_error_response` borrows the request and
accepts caller-supplied status, Content-Type, body and existing
`CredentialErrorHttpResponseLimits`. It delegates envelope and body validation
to `CredentialErrorResponseCore::parse_http_response`, classifies the parsed
code in deferred context, and returns the wrapper without performing HTTP.

## Classification and guidance

1. Exact `invalid_transaction_id` becomes `InvalidTransactionId`.
2. Exact `credential_request_denied` becomes `CredentialRequestDenied`.
3. Every other accepted code becomes `Inherited(core.error_kind())`, including
   bounded extensions.
4. `should_stop_polling` returns true only for
   `CredentialRequestDenied`, reflecting section 9.3's explicit recommendation
   without scheduling, mutation or persistence.

The request's retained transaction identifier is not compared because a Final
error body contains no identifier. Calling through the request associates the
transition structurally but does not prove provenance or correlation.

## Reuse and diagnostics

The generic parser remains the single owner of status precedence, JSON media
syntax, body/resource limits, forbidden generic `invalid_request`, exact codes,
untrusted descriptions and static errors. The wrapper adds no new error
contracts and Debug reports only deferred classification plus the existing
redacted core shape.

## Risks and mitigations

- Semantic drift: delegate the full inherited envelope/body contract rather
  than copying checks.
- Enum compatibility: keep deferred classification separate from the existing
  public closed generic enum.
- Accidental side effects: expose guidance as a pure query and keep request
  state immutable/reusable.
- False correlation claim: explicitly document that an error body cannot be
  transaction-matched and that origin remains caller-owned.
- Remote-content leakage: retain only the existing zeroizing bounded core and
  keep Debug free of exact code/description text.

## Rollback

Remove the additive wrapper, enum, request method, tests, ADR and capability.
No generic parser, successful response, wire, storage or migration behavior is
affected.
