# ADR 0145: bind bounded OID4VCI Authorization Code Token Responses

- **Status:** Accepted
- **Date:** 2026-09-25
- **Issue:** [#360](https://github.com/hyperledger-identus/sdk-rust/issues/360)
- **Decision authority:** ADR 0102, ADR 0143, ADR 0144 and issue #360

## Context

Issue #358 produces a one-shot public-client Token Request with a zeroizing
form body and retains the exact public Credential Issuer, Authorization Server,
selected Credential Configuration and issuer-identification evidence. If each
consumer independently handles the HTTP response, status/body confusion,
unbounded fields, missing cache-suppression evidence, lineage loss and
continued retention of the authorization code or PKCE verifier become likely.

The SDK remains transport-neutral. It receives one final status, effective
Content-Type, Cache-Control and Pragma field value, and body from a caller that
owns HTTP execution and response provenance.

## Decision

1. Consume `AuthorizationCodeTokenRequest` exactly once through
   `try_bind_response` and erase its zeroizing form body before inspecting any
   remote response value.
2. Classify exact status `200` as success and exact `400` or `401` as OAuth
   error before validating headers or parsing the body. Reject every other
   status.
3. Parse a `200` body only with the existing bounded `TokenResponseCore` and a
   `400` or `401` body only with the existing bounded
   `TokenErrorResponseCore`. A `401` result must be exact `invalid_client`.
4. Require bounded `application/json`, a syntactically valid bare `no-store`
   Cache-Control directive and a syntactically valid bare `no-cache` Pragma
   directive on both success and error outcomes.
5. Apply independent positive caps to each effective header value and reuse
   the existing independently bounded success and error body policies.
6. Return one exclusive success/error outcome carrying the exact public
   request lineage and unchanged issuer-identification evidence. Do not accept
   replacement metadata, endpoint or configuration inputs.
7. Keep all new Debug, Display and stable append-only error surfaces free of
   request secrets, token values, response bodies and remote header values.
8. Generalize the existing private HTTP directive scanner without changing
   its earlier `no-store` behavior and add no dependency.
9. Leave HTTP execution, redirects, decompression, response origin, TLS,
   field combination, client authentication, DPoP, timeouts, cancellation,
   retry, token trust and storage to the consumer.
10. Defer Token Response Authorization Details correlation and selected
    Credential Configuration narrowing to focused successor
    [#362](https://github.com/hyperledger-identus/sdk-rust/issues/362).

## Consequences

Consumers can no longer accidentally choose the success parser from body shape
or detach a parsed response from the request that produced it. Failures are
deterministic in status, Content-Type, Cache-Control, Pragma and body order, and
the request secret lifetime ends before remote parsing begins.

The API proves bounded local classification and lineage only. It does not
prove that headers were combined correctly, that a response came from the
selected endpoint, that intermediaries honored cache directives, or that an
access token is authentic, current, authorized or safely persisted.

## Reconsideration and rollback

Reconsider accepted statuses or cache evidence only when a pinned standards
profile and interoperability evidence require it. Add transport adapters only
outside this protocol crate. Rollback removes the additive response module,
composite limits, diagnostics, tests and request helper while leaving request
construction and the existing response-core parsers intact.
