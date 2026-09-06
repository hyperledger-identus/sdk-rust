# Change: expose the OID4VCI Final Nonce Endpoint from issuer metadata

## Why

Issue #133 advances IDR-023 after the bounded Credential Nonce Response landed
through #131. A headless wallet cannot request that response until it can
discover the optional Final `nonce_endpoint` in validated Credential Issuer
Metadata.

## What changes

- Add a redaction-safe `NonceEndpoint` value for the optional issuer metadata
  member.
- Validate the exact bounded value with the same HTTPS endpoint policy already
  used for the required Credential Endpoint.
- Reuse the existing Credential Endpoint byte limit as a shared endpoint-URL
  budget, preserving the public ten-argument limits constructor.
- Add field-specific static errors for oversize and unsafe Nonce Endpoints.
- Advance the in-progress IDR-023 ledger pointer from completed child #131 to
  active child #133.

This change does not construct a Nonce Request, execute HTTP, validate a Nonce
Response, establish trust or freshness, build proofs or Credential Requests,
or change a consumer repository.

## Capabilities

### New capabilities

- `oid4vci-nonce-endpoint-metadata`: optional, exact, bounded and
  redaction-safe Final Nonce Endpoint discovery.

### Modified capabilities

- `ssi-upstream-program`: keep IDR-023 in progress while its active child
  advances from #131 to #133.

## Impact

- Owner crate: `identus-oid4vci`.
- Public API: additive `NonceEndpoint` and metadata accessor.
- Wire behavior: additive interpretation of one optional metadata member.
- Dependencies/features/targets: unchanged.
- Consumers: Oxid and Lace ID Portal remain read-only evidence.
- Rollback: revert one unpublished `develop` change before downstream adoption.
