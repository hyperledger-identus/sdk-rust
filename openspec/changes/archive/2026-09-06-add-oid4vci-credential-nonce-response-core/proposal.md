# Change: parse a bounded OID4VCI Credential Nonce Response core

## Why

Issue #131 advances IDR-023 after the successful and error Token Response cores
landed through #127 and #129. Proof-bearing Credential Requests require a
fresh Credential Issuer challenge, but headless Rust consumers have no bounded
Final-spec value for the dedicated Nonce Endpoint response.

## What changes

- Add strict bounded parsing of the OID4VCI Final Credential Nonce Response.
- Preserve the required opaque `c_nonce` value without imposing a
  deployment-specific encoding or length profile.
- Retain the nonce only in zeroizing storage behind an explicitly sensitive
  proof-construction accessor and keep diagnostics content-free.
- Structurally validate and semantically discard bounded unknown members.
- Advance the in-progress IDR-023 ledger pointer from completed child #129 to
  active child #131.

This change does not expose the metadata endpoint, construct or execute HTTP,
validate response status/headers/cache behavior, prove nonce unpredictability
or freshness, bind a proof, construct a Credential Request, or change a
consumer repository.

## Capabilities

### New capabilities

- `oid4vci-credential-nonce-response-core`: bounded, opaque,
  redaction-safe parsing of the Final Credential Nonce Response.

### Modified capabilities

- `ssi-upstream-program`: keep IDR-023 in progress while its active child
  advances from #129 to #131.

## Impact

- Owner crate: `identus-oid4vci`.
- Public API: additive nonce-response limits, opaque nonce, and partial response
  core.
- Wire behavior: additive strict validation of required `c_nonce`; unknown
  fields are checked under the same structural budgets and discarded.
- Dependencies/features/targets: unchanged.
- Consumers: Oxid and Lace ID Portal remain read-only evidence.
- Rollback: revert one unpublished `develop` change before downstream adoption.
