# Design

## Public request surface

Add `DeferredCredentialRequestLimits` with one positive
`max_json_body_bytes`; the default is 16 KiB. Add a non-Clone
`DeferredCredentialRequest` that owns a duplicated validated
`DeferredCredentialEndpoint` and a zeroizing JSON byte vector. It exposes the
endpoint, POST method, `application/json` media type, body byte length,
`access_token_required() == true`, and `expose_sensitive_json_body()`.

`DeferredCredentialResponseCore::try_deferred_credential_request` borrows the
response and matched issuer metadata, requires the optional endpoint, and
returns the request. It does not consume the response because the same
transaction may legitimately be polled again after a later deferred response.

## Bounded serialization

Serialize the fixed object prefix, the transaction string through
`serde_json::to_writer`, and the object suffix into a private `Write`
implementation. The writer checks `current + incoming` with checked arithmetic
against the complete-body limit before extending its zeroizing vector. Thus
escaping remains standards-owned and no unbounded encoded-string intermediate
is allocated. Any writer failure maps to one static request-too-large error.

The existing response parser remains the only constructor for
`DeferredTransactionId`; its decoded input bounds and non-empty requirement
therefore compose with the independent complete request-body limit.

## Errors and privacy

Add fieldless `InvalidDeferredCredentialRequestLimits`,
`DeferredCredentialEndpointRequired`, and `DeferredCredentialRequestTooLarge`
variants with stable `oid4vci.*` codes and static messages. Request Debug
reports body bytes and the access-token requirement but omits endpoint and body
content. No Display or Serde implementation is added.

## Risks and mitigations

- JSON escaping errors: delegate scalar escaping to existing `serde_json`.
- Allocation amplification: the bounded writer checks every write before
  retention; configurable response limits cannot bypass request limits.
- Secret leakage: body storage zeroizes and only an explicitly sensitive
  accessor reveals bytes.
- Endpoint confusion: only the metadata-owned typed Deferred Credential
  Endpoint can enter the request.
- Retry/replay inference: docs explicitly keep timing, reuse, invalidation and
  response correlation outside this structural value.

## Rollback

Remove the additive module, limit, errors, exports, tests, ADR and capability.
Existing metadata and deferred response APIs remain source and wire compatible.
