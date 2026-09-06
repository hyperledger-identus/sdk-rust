# ADR 0051: parse the Credential Nonce Response without freshness claims

- **Status:** Accepted for issue #131
- **Date:** 2026-09-07
- **Decision authority:** standing component authority under ADR 0004
- **Related work:** issues #7, #20, #131; OpenID4VCI 1.0 Final section 7

## Context

OID4VCI Final moved Credential Issuer proof challenges to a dedicated optional
Nonce Endpoint. Existing Oxid/Portal evidence includes older deployment
profiles, but the Final response body requires only an opaque `c_nonce` string.
The SDK needs a reusable parse boundary before it can safely construct
proof-bearing Credential Requests.

## Decision

1. Add only a bounded `CredentialNonceResponseCore` body parser in this slice.
2. Treat `c_nonce` as any non-empty bounded JSON string. Do not require
   base64url, ASCII, a fixed size, a lifetime member, or an entropy estimate.
3. Hold the decoded value in zeroizing storage and expose it only through an
   explicitly sensitive proof-construction accessor.
4. Use the existing strict scanner to reject duplicate decoded names,
   malformed/trailing JSON, excessive depth/nodes, and invalid known fields;
   discard bounded unknown members.
5. Keep HTTP request/status/media/cache/DPoP behavior, metadata endpoint
   exposure, nonce generation, freshness/replay state, proof processing, and
   consumer adoption in later issue-first slices.

## Consequences

- Rust consumers gain one exact Final-compatible body value without inheriting
  donor-specific nonce formats.
- A parsed nonce is explicitly not evidence of unpredictability, freshness,
  origin, trust, uniqueness, or replay protection.
- The API is additive, unpublished, dependency-neutral, and independently
  reversible before adoption.
