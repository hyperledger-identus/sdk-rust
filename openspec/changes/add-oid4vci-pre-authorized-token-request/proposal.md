# Change: construct a bounded OID4VCI Pre-Authorized Token Request

## Why

Issue #125 advances IDR-023 after Transaction Code input binding landed in
#123. The SDK now owns every mandatory Pre-Authorized Code Token Request
input, but headless consumers still have no deterministic, bounded, and
redaction-safe request value to hand to their transport adapters.

## What changes

- Add a consuming transition from the prepared input state to an owned
  Pre-Authorized Token Request value.
- Carry the already validated Token Endpoint plus static POST and media-type
  guidance.
- Encode the mandatory Final parameters in deterministic UTF-8
  `application/x-www-form-urlencoded` form, including `tx_code` exactly when
  present.
- Bound encoded output with checked preflight accounting and zeroize the
  sensitive body on drop.
- Advance the in-progress IDR-023 ledger pointer from completed child #123 to
  active child #125.

This change does not execute HTTP, add client identity/authentication or
optional authorization selectors, parse responses, establish trust, enforce
single use, or change a consumer repository.

## Capabilities

### New capabilities

- `oid4vci-pre-authorized-token-request`: bounded deterministic construction
  of the mandatory Final Pre-Authorized Code Token Request form.

### Modified capabilities

- `ssi-upstream-program`: keep IDR-023 in progress while its active child
  advances from #123 to #125.

## Impact

- Owner crate: `identus-oid4vci`.
- Public API: additive request-limit and request-value types plus one consuming
  transition.
- Wire behavior: additive deterministic UTF-8 form body and static transport
  guidance.
- Dependencies/features/targets: unchanged.
- Consumers: Oxid and Lace ID Portal remain read-only evidence.
- Rollback: revert one unpublished `develop` change before downstream
  adoption.
