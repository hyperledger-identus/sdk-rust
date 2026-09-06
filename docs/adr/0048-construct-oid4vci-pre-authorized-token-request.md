# ADR 0048: construct the mandatory OID4VCI Pre-Authorized Token Request form

- **Status:** Accepted for issue #125
- **Date:** 2026-09-07
- **Decision authority:** standing autonomous component authority in ADR 0004
- **Related work:** issues #7, #20, #123, and #125

## Context

The SDK now binds a Final Credential Offer, issuer metadata, an eligible
Authorization Server, and optional Transaction Code input into a validated
predecessor state. A headless wallet still needs exact mandatory Token Request
bytes it can pass to its chosen transport without reimplementing protocol
parameter presence, spelling, escaping, limits, and secret hygiene.

OpenID4VCI 1.0 Final section 6.1 defines the Pre-Authorized Code parameters and
uses the OAuth Token Endpoint request rules. RFC 6749 requires UTF-8
`application/x-www-form-urlencoded` bodies. The request includes bearer-like
material, so exposing it is necessary for transport but must be unmistakably
sensitive.

## Decision

1. Add a consuming transition from
   `CredentialOfferWithPreAuthorizedTokenInput` and positive
   `PreAuthorizedTokenRequestLimits` to `PreAuthorizedTokenRequest`.
2. Emit exactly one `grant_type`, one `pre-authorized_code`, and optional
   `tx_code` in that deterministic order. Include `tx_code` exactly when the
   predecessor owns it.
3. Encode UTF-8 octets using RFC 6749 Appendix B form rules: alphanumerics plus
   `*`, `-`, `.`, and `_` stay literal, space becomes `+`, and other octets use
   uppercase `%HH`.
4. Compute exact encoded length with checked arithmetic before allocating.
   Make a positive independent body limit default to 16,384 bytes; distinguish
   invalid limits from oversized or arithmetically impossible output.
5. Consume and drop the predecessor after encoding. Store the validated Token
   Endpoint and body in zeroizing allocations.
6. Expose endpoint, static `POST`, static media type, byte count, and
   Transaction Code presence normally. Expose exact body bytes only through
   `expose_sensitive_form_body`, documented as a secret-bearing boundary.
7. Provide no Clone, Display, or Serde contract. Keep Debug and errors
   data-free.
8. Defer HTTP execution, client identity/authentication, optional
   `authorization_details`/`scope`/`resource`, responses, trust, retries/replay
   control, and downstream adoption.

## Consequences

- Headless consumers receive portable request bytes without taking a runtime
  or HTTP dependency.
- Deterministic output supports fixtures and cross-language adapters without
  implying that servers may depend on parameter order.
- One unavoidable copy creates the wire representation; original and encoded
  secret allocations are both erased on their respective drops.
- The public API and wire behavior are additive and unpublished; dependencies,
  features, manifests, parsers, lockfile, and target policy remain unchanged.

## Rejected alternatives

- **Expose raw Pre-Authorized and Transaction Codes separately.** Rejected
  because it expands the secret-reading surface and makes each consumer
  responsible for normative presence and encoding.
- **Depend on an HTTP or URL form serializer.** Rejected because the three-field
  profile is small, deterministic, independently testable, and does not justify
  expanding the runtime dependency cone.
- **Retain the full predecessor in the request.** Rejected because it would
  keep duplicate source and encoded secrets alive after construction.
- **Send the HTTP request in the same transition.** Rejected because transport,
  client authentication, retry/replay behavior, and response processing are
  separate policy and security boundaries.
