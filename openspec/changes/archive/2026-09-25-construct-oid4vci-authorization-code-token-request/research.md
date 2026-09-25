# Authorization Code Token Request research

Research class: routine
Research status: ready
Decision date: 2026-09-25
Source retrieval date: 2026-09-25
Research blockers: none

## Problem and existing implementation

`CorrelatedAuthorizationCode` already owns the exact Authorization Request
lineage, returned code and #356 issuer-identification evidence. That lineage
contains the selected Authorization Server metadata, selected offered
Credential Configuration, client identifier, redirect URI and SDK-bound PKCE
S256 verifier. The crate also owns a private exact
`application/x-www-form-urlencoded` encoder and the Pre-Authorized Code Token
Request demonstrates bounded secret-bearing output and transport neutrality.

The missing behavior is the one-shot Authorization Code exchange request.
Without it, consumers can exchange at a substituted endpoint, omit or change
the redirect URI, lose the verifier, incorrectly omit the public client ID, or
retain reusable code state after serializing the request.

## Normative sources

- [OpenID4VCI 1.0 Final](https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0-final.html),
  sections 6 and 6.1, retrieved 2026-09-25.
- [RFC 6749](https://www.rfc-editor.org/rfc/rfc6749.html), sections 3.2.1 and
  4.1.3, retrieved 2026-09-25.
- [RFC 7636](https://www.rfc-editor.org/rfc/rfc7636.html), section 4.5,
  retrieved 2026-09-25.
- [RFC 9700 / BCP 240](https://www.rfc-editor.org/rfc/rfc9700.html), section
  2.1.1, retrieved 2026-09-25.

OID4VCI Final delegates an Authorization Code Token Request to RFC 6749
section 4.1.3 and requires OAuth Security BCP conformance. RFC 6749 requires a
UTF-8 form POST containing exact `grant_type=authorization_code` and `code`,
the identical `redirect_uri` when the Authorization Request carried it, and
`client_id` for an unauthenticated client. RFC 7636 adds the exact retained
`code_verifier`. RFC 9700 requires public clients to use PKCE and recommends
S256; predecessor #352 already validates the verifier and derives S256.

## Public-client profile decision

The current typed predecessor does not model `token_endpoint_auth_method`, a
client secret, assertion key or another authentication method. Guessing a
confidential-client method would silently create an invalid or unsafe request.
This slice therefore implements only an explicitly documented unauthenticated
public-client profile and always sends `client_id`. A later capability must
model authenticated client methods before it can construct those requests.

The deterministic parameter order is the RFC 6749 base order followed by its
PKCE extension:

1. `grant_type=authorization_code`
2. `code`
3. `redirect_uri`
4. `client_id`
5. `code_verifier`

Parameter order is not an OAuth semantic requirement; fixing it creates a
stable exact-byte SDK contract.

## Compatibility and dependency evidence

The API is additive and unpublished. Existing request and response types,
wire parsing, constants, error-prefix discriminants, features, dependencies,
lockfile and supported-target claims remain unchanged. The internal consuming
decomposition is crate-private and preserves every public predecessor API.

## Candidate decisions

| Candidate | Disposition | Reason |
| --- | --- | --- |
| Existing private form codec | `adopt` | It already implements the required HTML form percent encoding, exact size calculation and zero-copy append path. |
| Existing `oauth2` dependency oracle | `reference-only` | Its request builder is useful differential evidence but owns transport/client-auth policy and does not preserve this SDK's typed lineage or exact bounded output contract. |
| `serde_urlencoded` or another form crate | `not-adopt` | A new public dependency cone is unnecessary for five fixed fields and would not remove the SDK-owned bounds/lineage policy. |
| Existing `CorrelatedAuthorizationCode` | `adopt` | It is the only valid predecessor and already binds state and optional RFC 9207 issuer evidence. |
| Existing server/input states | `adopt` | They own the exact Token Endpoint, client, redirect, verifier and selected configuration without reparsing. |
| HTTP client integration | `defer` | Transport authority remains outside this chain-neutral protocol crate. |

No dependency, feature, lockfile, target, unsafe/native or network capability
is needed. The implementation will move values from their existing owners and
discard sensitive predecessor state instead of cloning it into reusable
parallel objects.

## Security, privacy and maintenance evidence

The request form contains an authorization code and PKCE verifier and is
therefore zeroizing, explicitly exposed only for immediate transport, and
redacted from Debug. Construction consumes the correlated success so callers
cannot retry or exchange the same typed code state. The result retains public
issuer/server metadata and the selected configuration needed by a later
response-bound transition
but not the code, verifier, state, redirect URI, client identifier or complete
Authorization Request URI outside the already encoded ephemeral body.

The new limits independently cap the borrowed Token Endpoint and complete
encoded body. Earlier parsers remain bounded, but the request-specific endpoint
cap lets an adapter apply a tighter transport policy without reparsing or
normalizing the URI.

Issuer evidence is copied exactly as `VerifiedRfc9207` or `NotAdvertised`.
Construction does not upgrade absent RFC 9207 advertising into mix-up
protection and does not establish server trust, endpoint reachability,
authorization, token provenance or issuance success.

## Rejected or deferred candidates

Authenticated clients, Authorization Details at the Token Endpoint, resource
indicators, DPoP, attestation, HTTP, retries, response binding, token storage
and downstream adoption remain independently owned.

## Open questions and blockers

None for the bounded unauthenticated public-client request.

## Evidence commands

```text
scripts/factory research-ready construct-oid4vci-authorization-code-token-request
scripts/factory constraints-ready construct-oid4vci-authorization-code-token-request
cargo test -p identus-oid4vci --test authorization_code_token_request
cargo test -p identus-oid4vci
scripts/factory check
nix flake check
```
