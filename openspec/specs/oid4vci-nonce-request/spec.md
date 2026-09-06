# oid4vci-nonce-request Specification

## Purpose
TBD - created by archiving change add-oid4vci-nonce-request. Update Purpose after archive.
## Requirements
### Requirement: Final Credential Nonce Request binds validated metadata

The SDK SHALL construct an owned `CredentialNonceRequest` only through
`CredentialIssuerMetadata::try_nonce_request` when the bounded metadata
advertises a validated `NonceEndpoint`. The request SHALL retain that exact URL
in an independently owned `NonceEndpoint` and expose it through a typed borrow.

Metadata that omits `nonce_endpoint` SHALL remain valid, but attempting to
construct a request from it SHALL fail with the fieldless
`NonceEndpointRequired` error and stable
`oid4vci.nonce_endpoint_required` code/message. The SDK SHALL NOT infer a
fallback endpoint or expose an arbitrary-string request constructor.

#### Scenario: advertised endpoint becomes an owned request

- **WHEN** bounded issuer metadata advertises
  `https://issuer.example:8443/nonce?tenant=wallet`
- **THEN** request construction succeeds and retains that exact endpoint after
  the metadata borrow ends

#### Scenario: omitted endpoint cannot produce a request

- **WHEN** otherwise valid issuer metadata omits `nonce_endpoint`
- **THEN** request construction fails with the static missing-endpoint error
  without changing the validity of the metadata

### Requirement: Final transport guidance is exact and least-authority

`CredentialNonceRequest` SHALL expose HTTP method `POST`, an exactly empty byte
body, and `access_token_required() == false`. The method and body SHALL also be
available as static `NONCE_REQUEST_HTTP_METHOD` and `NONCE_REQUEST_BODY`
constants. The request SHALL carry no access token, Authorization value,
request parameters, content type, generic header map or automatic HTTP client.

Success SHALL prove only construction of the OpenID4VCI Final section 7.1
request shape. It SHALL NOT prove DNS, TLS, redirect or private-network safety;
endpoint reachability or issuer control/trust; response status, media type,
cache-control or DPoP headers; response provenance/correlation; nonce
generation, unpredictability, freshness, expiry, reuse or replay safety; proof
or Credential Request correctness.

#### Scenario: request has the canonical Final shape

- **WHEN** a request is constructed from metadata with a Nonce Endpoint
- **THEN** its method is `POST`, its body length is zero, and it requires no
  access token

#### Scenario: transport authority remains external

- **WHEN** a caller receives a constructed request
- **THEN** it still supplies its own HTTP and network policy and cannot obtain
  a bearer token, header map or execution result from the request value

### Requirement: Request diagnostics and portability remain isolated

The request SHALL expose no Clone, Display, Serde, FFI or raw URL constructor.
`CredentialNonceRequest` and `NonceEndpoint` Debug output SHALL contain no
endpoint URL, metadata JSON or other remote value. New errors SHALL be
fieldless and bridge to static redaction-safe metadata.

The change SHALL add no dependency, manifest, lockfile, feature, unsafe,
consumer, chain or product mutation. The runtime cone SHALL remain
`identus-core`, `serde_json`, `uriparse` and `zeroize`, and the change SHALL
preserve Rust 1.85, browser-WASM, Android ARM64 and iOS ARM64 portability.

#### Scenario: diagnostics contain no remote content

- **WHEN** a caller inspects request/endpoint Debug or direct/bridged errors
- **THEN** no endpoint, JSON, parser cause, header or unknown value appears

#### Scenario: portable policy-neutral gates remain green

- **WHEN** focused, workspace, factory, target/MSRV, supply-chain and full Nix
  gates run
- **THEN** the request passes without dependency-cone, feature, target or
  downstream drift
