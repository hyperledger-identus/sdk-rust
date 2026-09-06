# Change: bind bounded OID4VCI Transaction Code input

## Why

Issue #123 advances IDR-023 after the Pre-Authorized Code server-binding state
landed in #121. OID4VCI Final requires a `tx_code` Token Request parameter
exactly when the Credential Offer contains a `tx_code` object, but the SDK has
no state that can accept this bearer-adjacent input without losing its
presence contract or secret-lifecycle boundary.

## What changes

- Add a consuming transition from `CredentialOfferWithPreAuthorizedServer`
  and optional caller-owned Transaction Code input to an explicit prepared
  input state.
- Require exact agreement between offered `tx_code` object presence and input
  presence.
- Bound a present input independently, reject an empty value, and move it into
  zeroizing storage before validation.
- Keep the code opaque and prevent public diagnostics or serialization from
  exposing it.
- Advance the in-progress IDR-023 ledger pointer from completed child #121 to
  active child #123.

This change does not serialize or send a Token Request, validate a Transaction
Code with an Authorization Server, interpret advertised mode/length as an
authentication decision, identify or authenticate a client, establish trust,
or change a consumer repository.

## Capabilities

### New capabilities

- `oid4vci-transaction-code-input`: bounded, redaction-safe presence binding
  for optional Pre-Authorized Code Transaction Code input.

### Modified capabilities

- `ssi-upstream-program`: keep IDR-023 in progress while its active child
  advances from #121 to #123.

## Impact

- Owner crate: `identus-oid4vci`.
- Public API: additive limit and prepared-state types plus one consuming
  transition.
- Wire behavior: none; no serializer, HTTP request, or new Serde contract.
- Dependencies/features/targets: unchanged.
- Consumers: Oxid and Lace ID Portal remain read-only evidence.
- Rollback: revert one unpublished `develop` change before downstream
  adoption.
