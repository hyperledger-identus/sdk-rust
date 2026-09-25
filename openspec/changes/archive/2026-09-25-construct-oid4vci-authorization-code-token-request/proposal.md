# Construct the OID4VCI Authorization Code Token Request

## Why

The SDK can now build an Authorization Request and correlate its bounded
query-mode success response, but consumers still have to copy the returned
code and retained PKCE inputs into an OAuth Token Request. Reimplementing that
transition risks code reuse, verifier loss, redirect mismatch, client
misidentification, unbounded form output, or exchange at a different server.

## What changes

- Consume one `CorrelatedAuthorizationCode` exactly once.
- Construct the deterministic form body for an unauthenticated public client
  using the retained authorization code, redirect URI, client identifier and
  PKCE verifier.
- Require and retain the exact selected server's validated Token Endpoint.
- Apply independent positive Token Endpoint and complete-body byte caps before
  returning a request.
- Preserve least-authority issuer/server/configuration lineage and the exact #356
  issuer-identification evidence while discarding reusable code, verifier,
  state, Authorization Request URI state and the consumed Credential Offer.
- Add stable diagnostics, normative/exact-byte/boundary/redaction tests, ADR
  0144, architecture evidence and a focused successor.

## What does not change

No HTTP, confidential-client authentication, client secret, assertion, DPoP,
PAR/JAR/JARM, Authorization Details narrowing at the Token Endpoint, response
parsing change, browser/callback routing, persistence, retry, discovery,
trust, consumer, chain or Midnight-specific behavior is added.

## Capabilities

### New capabilities

- `oid4vci-authorization-code-token-request`: bounded one-shot construction of
  the unauthenticated public-client Authorization Code Token Request.

### Modified capabilities

- `oid4vci-error-contracts`: append static request-limit diagnostics without
  changing prior entries.
- `ssi-upstream-program`: record #358 as the current IDR-023 delivery slice and
  keep the engine open under a focused successor.

## Authority

Issue #358, OpenID4VCI 1.0 Final section 6.1, RFC 6749 sections 3.2.1 and
4.1.3, RFC 7636 section 4.5, RFC 9700 section 2.1.1, and the standing SDK
delivery mandate.
