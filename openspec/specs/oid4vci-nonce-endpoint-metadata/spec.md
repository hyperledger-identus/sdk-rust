# oid4vci-nonce-endpoint-metadata Specification

## Purpose
TBD - created by archiving change expose-oid4vci-nonce-endpoint-metadata. Update Purpose after archive.
## Requirements
### Requirement: Credential Issuer Metadata exposes the optional Final Nonce Endpoint

The SDK SHALL recognize `nonce_endpoint` as an optional member of bounded
`CredentialIssuerMetadata`. When present exactly once as a non-empty JSON
string, it SHALL be retained exactly as `NonceEndpoint` and exposed through
`nonce_endpoint() -> Option<&NonceEndpoint>`.

When the member is absent, parsing SHALL succeed and the accessor SHALL return
`None`, denoting the OID4VCI Final rule that the Credential Issuer does not
require a `c_nonce` value. The SDK SHALL NOT infer a fallback endpoint.

#### Scenario: advertised endpoint remains exact

- **WHEN** valid issuer metadata contains
  `"nonce_endpoint":"https://issuer.example:8443/nonce?tenant=wallet"`
- **THEN** parsing succeeds and the accessor returns that exact URL

#### Scenario: omitted endpoint remains absent

- **WHEN** otherwise valid issuer metadata omits `nonce_endpoint`
- **THEN** parsing succeeds and the accessor returns `None`

### Requirement: Nonce Endpoint validation is bounded and HTTPS-only

Each decoded Credential or Nonce Endpoint URL SHALL independently fit
`CredentialIssuerMetadataLimits::max_credential_endpoint_bytes`. The existing
limit and ten-argument constructor SHALL remain source-compatible and their
documentation SHALL state the shared endpoint-URL policy.

A present Nonce Endpoint SHALL be an absolute HTTPS URL with a host. It MAY
contain a port, path and query, and SHALL NOT contain userinfo or a fragment.
Empty or mistyped members SHALL return `InvalidMetadata`, values over the
shared budget SHALL return `NonceEndpointTooLarge`, and invalid URLs SHALL
return `UnsafeNonceEndpoint`.

#### Scenario: exact shared byte bound succeeds

- **WHEN** a valid Nonce Endpoint has exactly the configured endpoint byte
  length
- **THEN** issuer metadata parsing succeeds without reducing the independent
  Credential Endpoint budget

#### Scenario: invalid endpoint fails closed

- **WHEN** `nonce_endpoint` is empty, mistyped, malformed, non-HTTPS,
  userinfo-bearing, fragment-bearing or one byte over its decoded byte bound
- **THEN** parsing fails with its corresponding static metadata, oversize or
  unsafe endpoint error

### Requirement: Endpoint metadata remains redaction-safe and transport-free

`NonceEndpoint` SHALL retain its URL in zeroizing storage and SHALL expose no
Clone, Display, Serde, automatic network or FFI surface. Its Debug output SHALL
not contain the URL. New errors SHALL be fieldless and bridge to stable static
`oid4vci.nonce_endpoint_too_large` and
`oid4vci.unsafe_nonce_endpoint` codes/messages without input content.

The endpoint SHALL participate in existing decoded duplicate-name and
aggregate node checks. The change SHALL add no dependency, manifest, lockfile,
feature, unsafe, consumer, chain or product mutation and SHALL preserve Rust
1.85, browser-WASM, Android ARM64 and iOS ARM64 portability.

Success SHALL NOT prove retrieval provenance, Issuer trust, reachability,
network safety, nonce origin/unpredictability/freshness/replay safety, HTTP
behavior, proof processing or Credential Request/Response correctness.

#### Scenario: diagnostics contain no remote content

- **WHEN** a caller inspects endpoint or metadata Debug, direct errors or
  bridged errors
- **THEN** no endpoint URL, JSON, parser cause, offset or unknown value appears

#### Scenario: duplicate member and portable gates remain strict

- **WHEN** metadata duplicates a decoded `nonce_endpoint` name or the focused,
  workspace, factory, target/MSRV, supply-chain and full Nix gates run
- **THEN** duplicates fail through the existing static error and the valid
  implementation passes without dependency, feature, target or downstream
  drift
