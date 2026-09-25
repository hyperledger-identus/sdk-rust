# Prepare Bounded OID4VCI Authorization Request Inputs

## Why

The SDK now binds an offered Authorization Code grant to one capable
Authorization Server, but wallets must still assemble security-sensitive
client, redirect, CSRF and PKCE values without a typed state. Accepting an
unrelated caller-provided PKCE challenge would allow verifier/challenge drift,
while importing a broad OAuth client would violate the accepted ADR 0102
dependency decision.

## What changes

- Consume the server-bound Authorization Code state and select one offered
  Credential Configuration by bounded index.
- Validate an OAuth client identifier, absolute fragment-free redirect URI,
  caller-generated CSRF state and RFC 7636 code verifier.
- Derive the canonical S256 challenge from the retained verifier with
  feature-minimal `identus-crypto` SHA-256 and base64url primitives.
- Preserve the selected configuration, offer `issuer_state`, selected server,
  client inputs and PKCE pair under one owned, redacted predecessor state.
- Add static append-only diagnostics, RFC vectors, ADR and architecture
  evidence.

## What does not change

No entropy generation, Authorization Request serialization, URL query
mutation, `authorization_details` or scope emission, PAR, browser/callback,
authorization response, redirect comparison, code exchange, HTTP, discovery,
DNS, TLS, trust, persistence, product policy or chain behavior is added.

## Capabilities

### New capabilities

- `oid4vci-authorization-request-input`: bounded caller-input validation,
  configuration selection and PKCE S256 derivation for a later serializer.

### Modified capabilities

- `oid4vci-error-contracts`: append static diagnostics for the new transition
  without changing the immutable v1 prefix or prior live rows.
- `ssi-upstream-program`: record #352 as the current IDR-023 slice and require
  a focused successor before integration.

## Authority

Issue #352, OpenID4VCI 1.0 Final sections 5.1 and 12.3, RFC 6749 sections
3.1.2 and 4.1.1 plus Appendix A, RFC 7636 sections 4.1 through 4.3 and Appendix
B, RFC 9700 sections 2.1.1 and 4.4, ADR 0102, and the standing SDK delivery
mandate.
