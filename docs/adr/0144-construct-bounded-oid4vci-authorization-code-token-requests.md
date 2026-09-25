# ADR 0144: construct bounded OID4VCI Authorization Code Token Requests

- **Status:** Accepted
- **Date:** 2026-09-25
- **Issue:** [#358](https://github.com/hyperledger-identus/sdk-rust/issues/358)
- **Decision authority:** ADR 0083, ADR 0102, ADR 0143 and issue #358

## Context

Issue #356 correlates one Authorization Response and retains the exact code,
selected Authorization Server, redirect URI, public client identifier and PKCE
S256 verifier needed for code exchange. Leaving serialization to each consumer
would permit endpoint substitution, code reuse, verifier loss, redirect drift,
or omission of the unauthenticated public client's identifier.

The predecessor does not model a Token Endpoint authentication method, client
secret or assertion key. The SDK must not guess confidential-client behavior
or imply that a serialized request has been sent, authorized or trusted.

## Decision

1. Consume `CorrelatedAuthorizationCode` exactly once through
   `try_into_public_client_token_request`; the method name deliberately scopes
   the first profile to an unauthenticated public client.
2. Require the exact selected server's validated Token Endpoint and always
   emit the retained `client_id`, because RFC 6749 section 4.1.3 requires it
   for an unauthenticated client.
3. Emit exactly `grant_type=authorization_code`, `code`, `redirect_uri`,
   `client_id` and RFC 7636 `code_verifier` in deterministic RFC-base-plus-PKCE
   order using the existing private form codec.
4. Apply independent positive Token Endpoint and complete encoded-body byte
   caps. Calculate the exact body size with checked arithmetic before one
   exact-capacity allocation.
5. Move sensitive predecessor values into one zeroizing body. Discard state,
   PKCE challenge and Authorization Request URI state; retain no separately
   reusable code, verifier, redirect or client field.
6. Discard the consumed Credential Offer, including obsolete grant state, and
   retain only public issuer/server metadata plus the selected Credential
   Configuration needed for later response binding.
7. Preserve the exact #356 issuer-identification evidence. `NotAdvertised`
   remains an explicit absence of RFC 9207 evidence and is not upgraded into
   a mix-up-protection claim.
8. Keep the result transport-neutral and expose the form body only through an
   explicitly sensitive borrowed accessor. Debug and static append-only errors
   disclose no remote or sensitive value.
9. Add no dependency. Keep `oauth2 5.0.0` as a reference oracle rather than
   delegating this typed state, exact output or resource policy to it.
10. Defer authenticated client methods and request-bound Token Endpoint HTTP
    response handling; the latter continues in focused successor
    [#360](https://github.com/hyperledger-identus/sdk-rust/issues/360).

## Consequences

Wallet adapters receive a deterministic request description that cannot be
constructed before exact response correlation and cannot reuse the typed code
state afterward. A request-specific endpoint cap can be tighter than the
metadata parser's bound. Consumers still own transport bytes, DNS, TLS, HTTP,
timeouts, authentication policy, retries and response provenance.

This slice does not support confidential clients, Authorization Details at the
Token Endpoint, DPoP, token parsing changes, token trust/storage, authorization
or issuance completion.

## Reconsideration and rollback

Add authenticated profiles only after metadata and credential ownership are
modeled without exposing secrets. Reconsider a shared external form builder
only if it preserves exact deterministic output, positive bounds, zeroizing
ownership and the dependency policy. Rollback removes the additive request,
limits, diagnostics, private consuming helpers and evidence without changing
existing response correlation.
