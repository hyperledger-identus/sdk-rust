# ADR 0052: expose the Final Nonce Endpoint without transport claims

- **Status:** Accepted for issue #133
- **Date:** 2026-09-07
- **Decision authority:** standing component authority under ADR 0004
- **Related work:** issues #7, #20, #131, #133; OpenID4VCI 1.0 Final sections 7 and 12.2.4

## Context

OID4VCI Final defines an optional `nonce_endpoint` in Credential Issuer
Metadata. A headless wallet needs that value before it can request the bounded
Credential Nonce Response already provided by the SDK. The URL has the same
HTTPS constraints as the existing required Credential Endpoint.

## Decision

1. Add only optional bounded Nonce Endpoint interpretation to the existing
   unsigned Credential Issuer Metadata parser in this slice.
2. Retain the exact HTTPS URL in a redaction-safe `NonceEndpoint`; accept port,
   path and query and reject userinfo or fragments.
3. Return `None` when omitted and document the Final meaning that the issuer
   does not require `c_nonce`; do not invent a fallback endpoint.
4. Apply `max_credential_endpoint_bytes` independently to Credential and Nonce
   Endpoint values. Preserve the existing public constructor and clarify its
   shared endpoint-URL purpose in documentation.
5. Add field-specific, content-free oversize and unsafe errors. Keep request
   construction, HTTP, trust, nonce lifecycle, proof processing and consumer
   adoption in later issue-first slices.

## Consequences

- Rust consumers can discover the exact optional Final endpoint without
  acquiring HTTP or orchestration dependencies.
- Endpoint presence is not evidence of reachability, trust, network safety,
  nonce origin, freshness or replay protection.
- The API is additive, unpublished, dependency-neutral and independently
  reversible before adoption.
