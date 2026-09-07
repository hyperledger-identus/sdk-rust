# ADR 0053: construct a transport-neutral Final Credential Nonce Request

- **Status:** Accepted for issue #135
- **Date:** 2026-09-07
- **Decision authority:** standing component authority under ADR 0004
- **Related work:** issues #7, #20, #133, #135; OpenID4VCI 1.0 Final section 7.1

## Context

OID4VCI Final defines a Credential Nonce Request as an HTTP POST to the
optional `nonce_endpoint` advertised by Credential Issuer Metadata. The
endpoint is unprotected, so a wallet need not supply an access token. The SDK
already validates this endpoint but intentionally owns no HTTP client.

## Decision

1. Construct `CredentialNonceRequest` only from validated
   `CredentialIssuerMetadata`; do not expose a raw URL constructor.
2. Own an exact duplicate of the advertised endpoint in zeroizing storage so
   the request does not borrow the broader metadata value.
3. Describe only the Final wire inputs: `POST`, a zero-length byte body, and no
   access-token requirement. Do not add content type, generic headers, bearer
   tokens, or HTTP execution.
4. Fail omitted endpoints with fieldless `NonceEndpointRequired` and stable
   `oid4vci.nonce_endpoint_required` metadata. Keep Debug output data-free.
5. Leave network policy, status/header/media validation, response correlation,
   trust, nonce lifecycle, proof and Credential Request semantics to later
   issue-first slices.

## Consequences

- Headless consumers can map one validated request description into their own
  transport and network-policy adapter without re-parsing metadata.
- Construction proves neither endpoint safety beyond syntax nor successful or
  trustworthy nonce acquisition.
- The additive API introduces no dependency, feature, manifest, lockfile, FFI,
  consumer, chain, publication, or release change.
