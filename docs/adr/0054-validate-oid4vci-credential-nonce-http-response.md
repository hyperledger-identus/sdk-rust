# ADR 0054: validate the Final Credential Nonce HTTP response before body use

- **Status:** Accepted for issue #137
- **Date:** 2026-09-07
- **Decision authority:** standing component authority under ADR 0004
- **Related work:** issues #7, #20, #131, #135, #137; OpenID4VCI 1.0 Final
  section 7.2; RFC 9110; RFC 9111

## Context

Final requires a Nonce Response to carry a 2xx status, use the
`application/json` media type, and include Cache-Control `no-store`. The SDK
already constructs the request and parses the bounded body, but exposes no
transition that checks those mandatory transport properties together.

## Decision

1. Add a request-bound, non-executing response validator that accepts the
   caller's effective status, Content-Type, Cache-Control and body values.
2. Wrap existing body limits with positive independent field-value bounds.
3. Validate one RFC-shaped media type and cache-directive list privately;
   accept case-insensitive `application/json` with valid parameters and require
   an unqualified case-insensitive `no-store` directive.
4. Borrow rather than consume the request so retry policy remains downstream.
   Retain no response metadata or raw body; return only the existing bounded
   response core.
5. Keep errors fieldless and diagnostics static. Leave HTTP execution, network
   and trust policy, DPoP, actual provenance, nonce lifecycle, proof and
   Credential Request semantics to later issue-first slices.

## Consequences

- Headless consumers can reject an invalid Final response envelope before
  exposing nonce semantics without adopting an SDK HTTP runtime.
- Success is caller-paired syntax evidence, not proof of origin, trust,
  freshness, replay safety or correct proof use.
- The additive unpublished API adds no dependency, feature, manifest,
  lockfile, FFI, consumer, chain, publication or release change.
