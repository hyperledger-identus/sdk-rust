# Change: parse a bounded OID4VCI Credential Error Response core

## Why

Issue #145 advances IDR-023 after the SDK gained a bounded immediate
Credential Response and request-bound success envelope. A headless wallet
still has to parse standardized Credential Endpoint payload errors without
importing an HTTP client, recovery policy, or legacy draft fields.

## What Changes

- Add positive limits for the complete error body, JSON shape, error code, and
  optional developer description.
- Parse one strict bounded object with required `error` and optional
  `error_description`, structurally checking and discarding extensions.
- Preserve exact valid error codes while classifying the seven values listed by
  OpenID4VCI 1.0 Final and representing every other valid code as `Extension`.
- Keep descriptions explicitly untrusted, retain strings in zeroizing storage,
  and keep diagnostics free of all remote content.
- Add static fieldless errors plus positive, negative, boundary, extension,
  redaction, and consumer-shaped tests.
- Record the public/wire/standard decision in ADR 0058 and advance the
  in-progress IDR-023 ledger pointer from completed child #143 to active child
  #145.

This change does not validate an HTTP envelope, parse RFC 6750 authentication
errors, decide remediation or retry behavior, accept superseded `c_nonce`
semantics, execute transport, verify credentials, or modify a consumer.

## Capabilities

### New Capabilities

- `oid4vci-credential-error-response-core`: bounded parsing and redaction-safe
  classification of an OID4VCI Final Credential Endpoint error body.

### Modified Capabilities

- `ssi-upstream-program`: keep IDR-023 in progress while its active child
  advances from #143 to #145.

## Impact

- Owner crate: `identus-oid4vci`.
- Public API: additive limits, exact-code wrapper, known-kind enum, and parsed
  response core.
- Wire behavior: strict bounded JSON, Final NQSCHAR strings, exact standard
  classification, and extension-compatible code retention.
- Errors: additive fieldless `oid4vci.*` diagnostics.
- Dependencies/features/targets: unchanged.
- Consumers: Oxid and Lace ID Portal remain read-only.
- Rollback: revert one unpublished `develop` change before downstream adoption.
