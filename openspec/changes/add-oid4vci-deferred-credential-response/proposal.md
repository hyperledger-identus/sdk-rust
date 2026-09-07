# Change: parse a bounded OID4VCI deferred Credential Response core

## Why

Issue #149 advances IDR-023 after immediate and payload-error Credential
Response branches became independently bounded. A headless Wallet still needs
to recognize the Final deferred body without treating its transaction handle
as trusted or converting its positive JSON-number interval through a lossy
floating-point type.

## What Changes

- Add positive limits for complete JSON, structure, members, the decoded
  transaction identifier, and the exact interval number lexeme.
- Add a bounded duplicate-safe parser requiring `transaction_id` and positive
  JSON-number `interval` together.
- Preserve the transaction identifier and interval lexeme exactly in owned
  zeroizing storage, while keeping both out of diagnostics and Debug.
- Accept positive integer, fraction, and exponent forms without floating-point
  conversion; reject zero, negative, non-number, malformed, and oversized
  forms.
- Reject `credentials` and `notification_id` in this deferred branch, traverse
  and discard bounded unique extensions, and retain no extension data.
- Record the interpretation in ADR 0060 and advance the canonical IDR-023
  pointer from #147 to #149.

This change does not validate HTTP metadata, construct or execute a Deferred
Credential Request, correlate or invalidate transactions, schedule polling,
process encrypted responses, parse authorization errors, verify or store
credentials, notify an Issuer, or change a consumer.

## Capabilities

### New Capabilities

- `oid4vci-deferred-credential-response-core`: bounded Final deferred response
  body parsing with exact least-authority values.

### Modified Capabilities

- `ssi-upstream-program`: keep IDR-023 in progress while its active child
  advances from #147 to #149.

## Impact

- Owner crate: `identus-oid4vci`.
- Public API: additive limits, deferred core, opaque transaction identifier,
  and exact positive interval value.
- Wire behavior: strict complete JSON object with required deferred members,
  forbidden immediate-only members, and bounded ignored extensions.
- Errors: additive fieldless `oid4vci.*` diagnostics.
- Dependencies, features, targets, consumers, publication, and release:
  unchanged.
