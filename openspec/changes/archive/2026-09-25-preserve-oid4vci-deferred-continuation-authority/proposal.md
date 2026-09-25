# Preserve OID4VCI deferred continuation authority

## Why

Issue #366 classifies a consumed Credential Request and binds an initial HTTP
202 response to its proof count, but discards the issuer, Deferred Credential
Endpoint and bearer authority needed for the next request. Reintroducing those
values as detached caller inputs would break the request lineage established by
the preceding slices.

## What changes

- retain the smallest continuation capability in a JWT Credential Request;
- preserve it only through the exact request-bound HTTP 202 outcome;
- consume that outcome into a distinct authorized Deferred Credential Request;
- reuse the existing bounded transaction-body serializer and request limits;
- preserve the legacy structural request constructor unchanged;
- update #366's secret-lifetime contract and governing evidence;
- hand later deferred response binding to one focused successor.

## Non-goals

No HTTP execution or provenance, TLS, token validation or refresh, storage,
interval scheduling, retries, polling loop, credential verification/storage,
trust, consumer, release, chain, Midnight or product behavior.

## Capabilities

### Modified capabilities

- `oid4vci-credential-endpoint-response`: retain minimal continuation authority
  only for a successfully parsed request-bound HTTP 202 response.
- `oid4vci-deferred-credential-request`: consume that bound response into an
  authorized request without accepting replacement authority.
- `ssi-upstream-program`: keep IDR-023 in progress under a focused successor.
