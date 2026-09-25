# OID4VCI deferred continuation authority research

Research class: routine
Research status: ready
Decision date: 2026-09-25
Source retrieval date: 2026-09-25
Research blockers: none

## Problem and existing implementation

`JwtCredentialRequest` owns the exact Credential Endpoint, zeroizing bearer
Authorization field, zeroizing proof body and proof count. The #366 consuming
classifier drops the whole request before parsing every remote response. Its
HTTP 202 state therefore retains only the parsed transaction and proof count.

The existing `DeferredCredentialResponseCore::try_deferred_credential_request`
accepts detached issuer metadata and deliberately owns no access token. It is a
useful compatibility-level structural serializer, but cannot prove that its
endpoint or bearer authority came from the request that produced the response.

## Normative sources

- [OpenID4VCI 1.0 Final](https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0-final.html),
  sections 9, 9.1 and 12.2.4, retrieved 2026-09-25.

The Final Deferred Credential Endpoint is optional issuer metadata. A wallet
that uses it sends an HTTP POST with JSON `transaction_id` and presents an
access token valid for issuance of the previously requested credential. An
issuer omitting `deferred_credential_endpoint` does not support that endpoint.

## Candidate decisions

| Candidate | Disposition | Reason |
| --- | --- | --- |
| Retain full metadata and token response | `not-adopt` | It extends secret/data lifetime and couples continuation to unrelated fields. |
| Retain minimal issuer, optional deferred endpoint, Authorization and proof count | `adopt` | These are the exact authority/evidence needed by the continuation boundary. |
| Duplicate the bearer value for continuation | `not-adopt` | Moving the existing zeroizing value avoids an unnecessary secret copy. |
| Distinct request-bound Deferred Credential Request type | `adopt` | It makes bearer ownership and stronger lineage explicit without mutating the legacy structural type. |
| Existing bounded transaction serializer and limits | `adopt-and-reuse` | They already implement the required deterministic JSON and complete-body bound. |
| Detached replacement metadata, endpoint, token or transaction | `reject` | Replacement would defeat the request/response authority chain. |

## Compatibility and dependency evidence

The API is additive and unpublished. Existing structural request construction,
response parsing, output shapes, limits and errors remain exact. No dependency,
feature, manifest, lockfile, unsafe/native, stored-data or network change is
needed; portable compile remains applicable.

## Security, privacy and maintenance evidence

Status remains the first branch selector. For HTTP 200, 400 and unsupported
statuses, the complete request is dropped before any remote field is parsed.
For HTTP 202 only, the proof body and Credential Endpoint are dropped before
parsing while the minimal continuation capability survives. A parse or request
serialization error drops that capability. The resulting authorized request is
one-shot, non-Clone, redacted and exposes secrets only through explicit
`expose_sensitive_*` methods.

The issuer and Deferred Credential Endpoint are duplicated from already
validated metadata at request creation. The bearer Authorization allocation is
moved, never copied. The transaction comes only from the bounded HTTP 202 body.

## Rejected or deferred candidates

Full lineage retention, detached reconstruction, token refresh/storage and a
generic OAuth/HTTP dependency are rejected. HTTP execution/origin, TLS, token
validation, interval scheduling, retries, polling lifecycle, deferred response
binding, credential verification/storage and product policy remain deferred.

## Open questions and blockers

None for this bounded continuation-construction slice.

## Evidence commands

```text
scripts/factory research-ready preserve-oid4vci-deferred-continuation-authority
scripts/factory constraints-ready preserve-oid4vci-deferred-continuation-authority
cargo test -p identus-oid4vci --test credential_endpoint_http_response
cargo test -p identus-oid4vci --test deferred_credential_request
cargo test -p identus-oid4vci
scripts/factory check
```
