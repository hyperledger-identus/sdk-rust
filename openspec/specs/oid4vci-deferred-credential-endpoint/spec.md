# oid4vci-deferred-credential-endpoint Specification

## Purpose
TBD - created by archiving change add-oid4vci-deferred-credential-endpoint. Update Purpose after archive.
## Requirements
### Requirement: Credential Issuer Metadata exposes the optional Final Deferred Credential Endpoint

The SDK SHALL recognize `deferred_credential_endpoint` as an optional member of
bounded `CredentialIssuerMetadata`. When present exactly once as a non-empty
JSON string, it SHALL be retained exactly as `DeferredCredentialEndpoint` and
exposed through `deferred_credential_endpoint() ->
Option<&DeferredCredentialEndpoint>`.

When absent, parsing SHALL succeed and the accessor SHALL return `None`,
denoting the Final rule that the issuer does not support the Deferred Credential
Endpoint. The SDK SHALL NOT infer a fallback endpoint.

#### Scenario: advertised endpoint remains exact

- **WHEN** valid metadata contains
  `"deferred_credential_endpoint":"https://issuer.example:8443/deferred?tenant=wallet"`
- **THEN** parsing succeeds and the accessor returns that exact URL

#### Scenario: omitted endpoint remains absent

- **WHEN** otherwise valid metadata omits `deferred_credential_endpoint`
- **THEN** parsing succeeds and the accessor returns `None`

### Requirement: Deferred Credential Endpoint validation is bounded and HTTPS-only

Each decoded Credential, Nonce or Deferred Credential Endpoint URL SHALL
independently fit
`CredentialIssuerMetadataLimits::max_credential_endpoint_bytes`. The existing
limit/accessor and ten-argument constructor SHALL remain source-compatible and
their documentation SHALL state the shared endpoint-URL policy.

A present Deferred Credential Endpoint SHALL be an absolute HTTPS URL with a
host. It MAY contain port, path and query components and SHALL NOT contain
userinfo or a fragment. Empty or mistyped members SHALL return
`InvalidMetadata`, values over the shared budget SHALL return
`DeferredCredentialEndpointTooLarge`, and invalid URLs SHALL return
`UnsafeDeferredCredentialEndpoint`.

#### Scenario: exact shared byte bound succeeds independently

- **WHEN** a valid Deferred Credential Endpoint has exactly the configured
  endpoint byte length
- **THEN** parsing succeeds without reducing another endpoint's budget

#### Scenario: invalid endpoint fails closed

- **WHEN** the member is empty, mistyped, malformed, non-HTTPS,
  userinfo-bearing, fragment-bearing or one byte over its decoded byte bound
- **THEN** parsing fails with the corresponding static metadata, oversize or
  unsafe endpoint error

### Requirement: Deferred endpoint metadata remains redaction-safe and transport-free

`DeferredCredentialEndpoint` SHALL retain its URL in zeroizing storage and
SHALL expose no Clone, Display, Serde, automatic network or FFI surface. Its
Debug output SHALL not contain the URL. New errors SHALL be fieldless and
bridge to stable static `oid4vci.deferred_credential_endpoint_too_large` and
`oid4vci.unsafe_deferred_credential_endpoint` codes/messages without input
content.

The member SHALL participate in existing decoded duplicate-name and aggregate
node checks. The change SHALL add no dependency, manifest, lockfile, feature,
unsafe, consumer, chain or product mutation and SHALL preserve supported target
compilation.

Success SHALL NOT prove retrieval provenance, issuer trust/control,
reachability, network safety, access-token or transaction validity, polling,
scheduling, HTTP, encryption, response correlation, storage or product policy.

#### Scenario: diagnostics contain no remote content

- **WHEN** a caller inspects endpoint/metadata Debug, direct errors or bridged
  errors
- **THEN** no endpoint URL, JSON, parser cause, offset or unknown value appears

#### Scenario: duplicate member and portable gates remain strict

- **WHEN** metadata duplicates a decoded `deferred_credential_endpoint` name or
  focused, workspace, factory, target, supply-chain and Nix gates run
- **THEN** duplicates fail and the valid implementation passes without
  dependency, feature, target or downstream drift

