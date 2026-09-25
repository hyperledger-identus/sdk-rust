# Authorization Code Token Response research

Research class: routine
Research status: ready
Decision date: 2026-09-25
Source retrieval date: 2026-09-25
Research blockers: none

## Problem and existing implementation

`AuthorizationCodeTokenRequest` already owns the exact selected Token Endpoint,
Credential Issuer Metadata, Authorization Server Metadata, selected Credential
Configuration and #356 issuer-identification evidence. Its form body is the
only remaining owner of the authorization code, redirect URI, client ID and
PKCE verifier. `TokenResponseCore` and `TokenErrorResponseCore` already provide
strict bounded JSON parsing and redacted zeroizing ownership, but intentionally
prove no HTTP or request correlation.

The crate has transport-neutral HTTP envelope precedents for credential,
deferred-credential and nonce endpoints. They validate status before headers
and body, reuse the private strict `application/json` parser, and delegate JSON
to an existing bounded core. The nonce boundary also validates a bounded bare
`no-store` cache directive. No current type binds a Token Endpoint HTTP result
to an Authorization Code request.

## Normative sources

- [OpenID4VCI 1.0 Final](https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0-final.html),
  sections 6.2 and 6.3, retrieved 2026-09-25.
- [RFC 6749](https://www.rfc-editor.org/rfc/rfc6749.html), sections 5.1 and
  5.2, retrieved 2026-09-25.
- [RFC 9110](https://www.rfc-editor.org/rfc/rfc9110.html), sections 8.3 and 15,
  retrieved 2026-09-25.
- [RFC 9111](https://www.rfc-editor.org/rfc/rfc9111.html), sections 3 and 5.2,
  retrieved 2026-09-25 as cache-semantics context.

OID4VCI delegates successful Token Responses to RFC 6749 and delegates Token
Error Responses to RFC 6749 section 5.2. RFC 6749 defines exact HTTP `200` for
success and `400` for errors, while permitting `401` for `invalid_client`; both
bodies use `application/json`. Token-bearing responses require
`Cache-Control: no-store` and `Pragma: no-cache`. RFC 9110 makes status the
primary response classification and defines case-insensitive media types with
parameters. RFC 9111 confirms that `no-store` prevents compliant caches from
storing the response but is not itself a complete privacy mechanism.

## Status and header policy decision

The SDK classifies status before reading any header or body semantic:

1. `200` selects `TokenResponseCore`.
2. `400` selects `TokenErrorResponseCore`; `401` selects the same parser but
   is accepted only when the parsed error is exact `invalid_client`.
3. Every other status fails with one static diagnostic.

The selected class cannot be overridden by body shape. A `200` error object
therefore fails the success parser, and a `400`/`401` success object fails the
error parser. A non-`invalid_client` error on `401` fails a separate static
status/error mismatch diagnostic. This prevents ambiguous success/error
branches and follows the OAuth-defined HTTP contract.

Both branches require a bounded valid `application/json` Content-Type, a
bounded valid Cache-Control field containing a bare `no-store`, and a bounded
valid Pragma field containing a bare `no-cache`. Applying the conservative
cache boundary uniformly avoids retaining developer descriptions or extension
data from a token exchange when an implementation emits them on an error.
Header validation order is Content-Type, Cache-Control, then Pragma; body
parsing happens last. Exact values never enter diagnostics.

## Candidate decisions

| Candidate | Disposition | Reason |
| --- | --- | --- |
| Existing `TokenResponseCore` | `adopt` | It owns bounded strict success JSON, zeroizes tokens and already ignores bounded extensions as required. |
| Existing `TokenErrorResponseCore` | `adopt` | It owns bounded OAuth error syntax, classification and redacted untrusted metadata. |
| Existing private HTTP field codec | `adopt-and-generalize` | The strict JSON and bare-directive scanners avoid a new parser/dependency while preserving current nonce behavior. |
| Existing credential/deferred envelopes | `reference-only` | Their status and ownership shapes are useful, but their payload types and endpoint semantics must not couple to token exchange. |
| `http`, `headers`, `mime` or HTTP client crates | `not-adopt` | The boundary accepts already-extracted scalar metadata; adding a transport dependency would widen effects and the dependency cone. |
| `oauth2` response API | `reference-only` | It combines transport/client policy and does not retain this SDK's exact OID4VCI lineage or explicit resource limits. |

No dependency, feature, manifest, lockfile, unsafe/native or network capability
is needed.

## Security, privacy and maintenance evidence

The binding method consumes the request for success, protocol error and local
validation failure. It destructures and drops the zeroizing request body before
validating or copying the remote response, so authorization code and verifier
material do not survive remote parsing. Only public issuer/server/configuration
lineage and the exact `VerifiedRfc9207` or `NotAdvertised` evidence continue.

Successful access/refresh tokens remain inside `TokenResponseCore`; error
metadata remains inside `TokenErrorResponseCore`. The outcome and lineage have
redacted Debug implementations and no Clone, Display, Serde, raw response or
network API. Header/body errors are fieldless and static.

The SDK validates caller-supplied final response metadata and borrowed body
text only. The caller still owns response origin, TLS, redirects,
decompression, aggregate transport/header bytes, duplicate HTTP field
combination, timeouts, cancellation and erasure of its input allocation.

## Compatibility and dependency evidence

The API and errors are additive and unpublished. Existing response-core,
request, HTTP envelope and wire behavior remains unchanged. The private
directive scanner generalization preserves the existing `has_bare_no_store`
contract. No public serialized shape, dependency, feature or target claim
changes. WASM, Android ARM64 and iOS ARM64 remain compile-only evidence.

## Rejected or deferred candidates

HTTP execution, confidential-client handling, DPoP, WWW-Authenticate parsing,
retry/remediation policy, token introspection/verification, token persistence,
Authorization Details-to-offer correlation, Credential Request construction
and downstream adoption remain independently owned.

## Open questions and blockers

None for the bounded response-binding slice.

## Evidence commands

```text
scripts/factory research-ready bind-oid4vci-authorization-code-token-response
scripts/factory constraints-ready bind-oid4vci-authorization-code-token-response
cargo test -p identus-oid4vci --test authorization_code_token_http_response
cargo test -p identus-oid4vci
scripts/factory check
nix flake check
```
