# Change: parse a bounded immediate OID4VCI Final Credential Response

## Why

Issue #141 advances IDR-023 after the SDK gained a bounded Final Credential
Request constructor. A headless wallet still has to parse the successful
immediate response by hand before it can pass opaque credentials to its
format-specific verification boundary.

## What Changes

- Add positive limits for response bytes, JSON depth/nodes, top-level members,
  credential count, credential-entry members, one retained credential value,
  aggregate retained credential bytes, and the optional notification ID.
- Parse the unencrypted Final immediate response with a non-empty ordered
  `credentials` array whose entries each contain one string or object
  `credential` value.
- Retain exact credential JSON in zeroizing ownership, expose the representation
  kind and decoded string value when applicable, and expose an optional opaque
  `notification_id` only through an explicitly sensitive accessor.
- Bound and discard unknown extension members, reject duplicate names, and
  reject the mutually exclusive deferred `transaction_id` branch and any
  top-level `interval`.
- Add fieldless errors and focused shape, order, exact-retention, resource,
  extension, duplicate, deferred-branch, and redaction tests.
- Record the public/error/standard interpretation in ADR 0056 and advance the
  in-progress IDR-023 ledger pointer from completed child #139 to active child
  #141.

This change does not validate HTTP metadata, parse error or deferred responses,
decrypt responses, interpret or verify credential formats, correlate a request,
execute notifications, store credentials, or modify a consumer.

## Capabilities

### New Capabilities

- `oid4vci-immediate-credential-response-core`: bounded transport-neutral
  parsing of the unencrypted Final immediate Credential Response body.

### Modified Capabilities

- `ssi-upstream-program`: keep IDR-023 in progress while its active child
  advances from #139 to #141.

## Impact

- Owner crate: `identus-oid4vci`.
- Public API: additive response limits, response/credential values and
  credential-kind enum.
- Wire behavior: strict bounded parsing with exact opaque credential retention;
  no serialization or network access.
- Errors: additive fieldless `oid4vci.*` diagnostics.
- Dependencies/features/targets: unchanged.
- Consumers: Oxid and Lace ID Portal remain read-only.
- Rollback: revert one unpublished `develop` change before downstream adoption.
