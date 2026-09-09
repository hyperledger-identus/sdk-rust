# ADR 0107: expose the OID4VCI Deferred Credential Endpoint

- **Status:** Accepted for implementation
- **Date:** 2026-09-09
- **Decision authority:** standing product mandate and IDR-023 issue #241
- **Related work:** issues #7, #133, #149, #239, and #241
- **Normative baseline:** OpenID4VCI 1.0 Final sections 9 and 12.2.4

## Context

The SDK parses a deferred Credential Response body but cannot discover the
optional endpoint to which a later Deferred Credential Request would be sent.
Credential Issuer Metadata currently treats `deferred_credential_endpoint` as
an unknown discarded extension even though the Final specification defines its
syntax and omission semantics.

## Decision

1. Recognize the optional metadata member in the existing strict bounded JSON
   scanner.
2. Retain a present value as a distinct `DeferredCredentialEndpoint` in
   zeroizing ownership and expose it only through an optional borrowed accessor.
3. Require the established absolute HTTPS-with-host policy; allow port, path
   and query while rejecting userinfo and fragments.
4. Apply `max_credential_endpoint_bytes` independently to each Credential,
   Nonce and Deferred Credential Endpoint, preserving the existing public
   limits constructor.
5. Return fieldless stable errors for an oversized or unsafe deferred endpoint
   and keep all caller content out of diagnostics.
6. Do not grant network authority or infer a fallback when the member is absent.

## Consequences

- A later issue can build a Deferred Credential Request from typed advertised
  metadata instead of an arbitrary URL.
- Existing metadata and limits callers remain source and wire compatible.
- Exact endpoint text, aggregate parser limits and decoded duplicate detection
  are preserved.
- No dependency, feature, unsafe, runtime or target surface is added.
- Request construction, HTTP, polling, scheduling, correlation, token and
  transaction validity, encryption, trust and product policy remain later work.

## Rejected alternatives

- A raw string accessor would blur validated and unvalidated endpoint state.
- Adding a separate public limit argument would break the existing constructor
  for an endpoint governed by the same URL ceiling.
- Adding a URL or OAuth client dependency would widen the cone without adding a
  required capability beyond the established validator.
- Constructing the Deferred Credential Request here would combine independent
  metadata and sensitive transaction/token state transitions.

## Rollback

Remove the additive field, type, accessor, errors, tests and canonical
capability. Existing callers remain compatible and the member returns to
bounded extension-discard behavior.
