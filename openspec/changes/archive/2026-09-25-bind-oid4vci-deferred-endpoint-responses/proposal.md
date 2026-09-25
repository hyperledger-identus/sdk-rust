# Bind OID4VCI deferred endpoint responses

## Why

Issue #368 preserves the exact issuer, advertised Deferred Credential Endpoint,
bearer authority, transaction and originating proof count in a one-shot
Deferred Credential Request. The SDK still exposes only separate borrowed
success and payload-error validators, so a caller can bypass one-shot authority
lifetime and an issued response is not capped by the originating proof count.

## What changes

- consume one request-bound Deferred Credential Request into a closed
  status-first 200/202/400 outcome;
- reuse the existing bounded immediate, correlated-pending and deferred-error
  parsers rather than add another wire model;
- cap HTTP 200 credential cardinality by the exact originating proof count;
- retain the exact continuation authority only through a correlated HTTP 202;
- erase authorization and obsolete request bytes before terminal parsing and
  on every failure;
- preserve all borrowed structural validators unchanged;
- hand the remaining wallet-engine work to one focused successor.

## Non-goals

No HTTP execution or provenance, TLS, token validation/refresh/storage,
interval scheduling, retries, polling loop, credential verification/storage,
notifications, encrypted responses, RFC 6750 challenges, product policy,
consumer adoption, release, chain or Midnight behavior.

## Capabilities

### Modified capabilities

- `oid4vci-deferred-credential-http-response`: add one request-consuming closed
  classifier while preserving borrowed validators.
- `ssi-upstream-program`: advance IDR-023 through deferred response binding and
  retain an open focused successor.
