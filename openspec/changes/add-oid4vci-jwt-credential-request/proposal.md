# Change: construct a bounded OID4VCI Final JWT Credential Request

## Why

Issue #139 advances IDR-023 after the SDK gained matched offer/issuer metadata,
a bounded Token Response core, a Final nonce path, and holder-produced proof
JWTs. A headless wallet still has to join these validated values and serialize
the protected Credential Endpoint request by hand.

## What Changes

- Add an architecture-approved dependency from `identus-oid4vci` to
  `identus-jose` so the request accepts only holder-produced
  `Oid4vciProofJwt` values rather than arbitrary proof strings.
- Add positive limits for proof count, one compact proof, the complete JSON
  body, and the Authorization field value.
- Construct a transport-neutral request from matched offer/metadata, a bounded
  successful Bearer Token Response without unvalidated Authorization Details,
  one offered configuration index, and a non-empty ordered proof list.
- Emit deterministic JSON containing exactly `credential_configuration_id`
  and `proofs.jwt`, plus static POST/media guidance and zeroizing sensitive
  authorization/body accessors.
- Add fieldless errors and focused selection, state, limit, wire, secret and
  redaction tests.
- Record the dependency decision in an ADR and advance the in-progress IDR-023
  ledger pointer from completed child #137 to active child #139.

This change does not execute HTTP, validate token/proof trust or freshness,
interpret Authorization Details, support Credential identifiers, construct
proofs, encrypt requests/responses, parse Credential Responses, add format or
chain extensions, or modify a consumer.

## Capabilities

### New Capabilities

- `oid4vci-jwt-credential-request`: bounded construction of the unencrypted
  Final Credential Request for the supported configuration-ID/JWT-proof path.

### Modified Capabilities

- `ssi-upstream-program`: keep IDR-023 in progress while its active child
  advances from #137 to #139.

## Impact

- Owner crate: `identus-oid4vci`.
- Public API: additive request limits, request value, constants and one
  matched-state constructor.
- Wire behavior: deterministic unencrypted JSON and Bearer Authorization value;
  no network access.
- Errors: additive fieldless `oid4vci.*` diagnostics.
- Dependencies: add only the already-approved local `identus-jose` edge.
- Features/targets: unchanged.
- Consumers: Oxid and Lace ID Portal remain read-only.
- Rollback: revert one unpublished `develop` change before downstream adoption.
