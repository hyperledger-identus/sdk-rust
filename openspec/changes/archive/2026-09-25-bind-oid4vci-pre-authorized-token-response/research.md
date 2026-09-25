# Pre-Authorized Token Response research

Research class: routine
Research status: ready
Decision date: 2026-09-25
Source retrieval date: 2026-09-25
Research blockers: none

## Problem and existing implementation

`PreAuthorizedTokenRequest` owns the selected Token Endpoint, a zeroizing form
body containing the Pre-Authorized Code and optional Transaction Code, and a
boolean transaction-code marker. It currently discards the matched issuer,
server and offer scope during construction. `TokenResponseCore` and
`TokenErrorResponseCore` already provide bounded strict parsing and redacted
ownership. The Authorization Code flow already has a status-first HTTP
response binder and private strict HTTP-field grammar.

The complete retained Credential Offer JSON is not suitable lineage: it
contains the secret Pre-Authorized Code. Exact non-secret offer authority is
the ordered, duplicate-free set of configuration IDs already matched against
Credential Issuer Metadata. Retaining those IDs, issuer metadata and selected
Authorization Server Metadata preserves the required authority without
retaining the grant secret or accepting detached replacements.

## Normative sources

- [OpenID4VCI 1.0 Final](https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0-final.html),
  sections 6.2 and 6.3, retrieved 2026-09-25.
- [RFC 6749](https://www.rfc-editor.org/rfc/rfc6749.html), sections 5.1 and
  5.2, retrieved 2026-09-25.

OID4VCI delegates successful Token Responses to RFC 6749 and delegates Token
Error Responses to RFC 6749 section 5.2. It identifies `invalid_request` and
`invalid_grant` as the relevant additional Pre-Authorized Code failures. RFC
6749 specifies HTTP `200` success and `400` error responses using JSON and
requires successful responses to carry `Cache-Control: no-store` and
`Pragma: no-cache`.

The constructed request is explicitly an unauthenticated transport-neutral
request and issue #375 excludes client authentication. Therefore this slice
accepts only exact `200` and `400`. A possible authenticated-client `401`
`invalid_client` path requires separately designed client authority and is not
silently generalized from the Authorization Code public-client binder.

## Candidate decisions

| Candidate | Disposition | Reason |
| --- | --- | --- |
| Existing `TokenResponseCore` | `adopt` | Bounded zeroizing success ownership already exists. |
| Existing `TokenErrorResponseCore` | `adopt` | It already recognizes OAuth plus Final pre-authorized error codes. |
| Existing private HTTP-field grammar | `adopt` | Strict JSON and bare cache-directive validation needs no new dependency. |
| Complete retained Credential Offer JSON | `reject` | It contains the Pre-Authorized Code and exceeds least-authority lineage. |
| Ordered offered configuration IDs | `adopt` | They preserve exact matched offer scope without grant secrets. |
| Authorization Code response wrapper | `reference-only` | Its one-selected-configuration and RFC 9207 authority are flow-specific. |
| HTTP/client crates | `not-adopt` | This boundary receives extracted scalar metadata and performs no transport. |

No dependency, feature, manifest, lockfile, unsafe/native or network capability
is required.

## Security, privacy and maintenance evidence

Construction consumes the validated offer/server/input states and copies only
bounded public configuration IDs plus already validated metadata. Response
binding consumes the request and drops its zeroizing form body before remote
status/header/body validation. Neither the Pre-Authorized Code nor Transaction
Code survives in lineage. Outcomes have no Clone, Serde, Display, raw response
or network API, and Debug reveals no issuer, server, configuration, token,
header, body or endpoint value.

Status determines the parser. A `200` error object and `400` success object
fail the selected parser. Status is checked before headers; bounded Content-
Type, Cache-Control and Pragma are checked in that order before the body.
Applying the cache requirements to both branches is a conservative SDK policy
because error descriptions and extensions can also carry sensitive context.

## Compatibility and dependency evidence

The API and errors are additive and unpublished. Existing request accessors,
response cores, dependencies, features and target claims remain unchanged.
The binder proves bounded supplied metadata and exact typed lineage only. The
caller still owns TLS, origin, redirects, decompression, duplicate header
combination, timeouts, cancellation, retries, input-allocation erasure and
token trust.

## Rejected or deferred candidates

Authenticated-client `401` behavior, DPoP, refresh tokens as an active flow,
HTTP execution, transport provenance, retry/remediation policy, token
verification/storage and Credential Request construction remain independently
owned. A complete offer object and its secret-bearing JSON are explicitly
rejected as response lineage.

## Open questions and blockers

None for this bounded slice.

## Evidence commands

```text
scripts/factory research-ready bind-oid4vci-pre-authorized-token-response
scripts/factory constraints-ready bind-oid4vci-pre-authorized-token-response
cargo test -p identus-oid4vci --test pre_authorized_token_http_response
cargo test -p identus-oid4vci
scripts/factory check
nix flake check
```
