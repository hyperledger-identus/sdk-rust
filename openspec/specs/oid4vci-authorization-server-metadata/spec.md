# oid4vci-authorization-server-metadata Specification

## Purpose
TBD - created by archiving change add-oid4vci-authorization-server-metadata-core. Update Purpose after archive.
## Requirements
### Requirement: Authorization Server Metadata Core is explicitly partial

The SDK SHALL accept one complete UTF-8 Authorization Server Metadata JSON
object within positive configured byte, depth, and node limits. Every object at
every depth SHALL have unique decoded member names. The state SHALL retain the
exact JSON while interpreting only the bounded core in this specification and
preserving every unknown member losslessly.

Successful construction SHALL NOT assert complete RFC 8414 conformance. The
public type and documentation SHALL identify the value as a partial core, and
uninterpreted required or conditional RFC 8414 members SHALL remain outside
its validation claim. No retrieval, media-type, signature, origin, trust,
capability, or policy claim is made.

#### Scenario: consumer profile remains expressible without overclaim

- **WHEN** a bounded consumer-shaped document contains the interpreted core but
  omits an RFC 8414 member outside the projection
- **THEN** the core retains it without representing the document as completely
  RFC 8414-conformant

#### Scenario: unknown metadata remains exact

- **WHEN** the core document carries nested extensions or arbitrary-magnitude
  numeric values
- **THEN** exact retained JSON is unchanged and no unknown value is interpreted

### Requirement: Authorization Server identity and endpoints are bounded

The core SHALL require `issuer` as an RFC 8414 Authorization Server identifier:
HTTPS with non-empty host, optional port/path, and no userinfo, query, or
fragment. The caller SHALL supply the expected issuer used for future retrieval
derivation. It SHALL be syntactically valid, bounded, and identical to metadata
`issuer` under simple string comparison without normalization.

Optional `authorization_endpoint` and `token_endpoint` values SHALL be HTTPS
URLs with non-empty host, optional port/path/query, and no userinfo or fragment.
Their presence SHALL NOT assert reachability, safety, support, or trust.

#### Scenario: expected issuer matches exactly

- **WHEN** a valid core has an issuer byte-identical to the expected issuer
- **THEN** the typed issuer and any valid optional endpoints are exposed

#### Scenario: identity or endpoint confusion fails closed

- **WHEN** expected and metadata issuers differ by any spelling or an issuer or
  endpoint has unsafe URL components
- **THEN** no Authorization Server Metadata Core is returned

### Requirement: Grant advertisement and anonymous access preserve defaults

Optional `grant_types_supported` SHALL be a non-empty ordered array of unique,
non-empty bounded strings. The state SHALL preserve whether the array was
advertised. When absent, effective grant lookup SHALL expose the RFC 8414
default values `authorization_code` and `implicit` in that order without
inventing an advertised array.

Optional `pre-authorized_grant_anonymous_access_supported` SHALL be a boolean.
The state SHALL preserve whether it was advertised and expose an effective
default of false. Neither a grant value nor a true flag SHALL select a flow,
prove endpoint capability, or authorize anonymous access.

Defaults SHALL be 131,072 JSON bytes; depth 16; 1,024 nodes; 2,048 issuer
bytes; 2,048 bytes per endpoint; 256 bytes per grant value; and 32 grant values.
Every maximum SHALL be positive and configured depth SHALL not exceed 64.

#### Scenario: omitted values retain default provenance

- **WHEN** valid metadata omits grant types and anonymous-access support
- **THEN** accessors distinguish omission while exposing the RFC and OID4VCI
  effective defaults

#### Scenario: explicit values remain ordered and unambiguous

- **WHEN** valid metadata advertises distinct grant types and an anonymous flag
- **THEN** exact grant order and flag presence/value are retained, while empty,
  duplicate, mistyped, or excessive values fail closed

### Requirement: Authorization Server core diagnostics are redaction-safe and portable

The core state SHALL keep issuer, endpoints, grant values, exact JSON, and
errors from exposing caller-controlled content through `Debug`, `Display`, the
core error bridge, or Serde serialization. Every owned content-bearing string
SHALL be erased on drop. Public errors SHALL be fieldless, static, and use
stable `oid4vci.*` codes.

Tests SHALL cover official-shaped and independently reconstructed Oxid/Lace
evidence, optional endpoints, explicit/omitted grants, anonymous support,
exact byte/count/depth limits, every documented rejection class, unknown
extension retention, partial-conformance behavior, and diagnostic canaries.
The package SHALL remain portable across repository host, Rust 1.85 MSRV,
browser-WASM, Android ARM64, and iOS ARM64 without HTTP, async, crypto, DID,
storage, chain, or product code.

#### Scenario: sensitive canaries never enter diagnostics

- **WHEN** issuer, endpoint, grant, extension, and rejected strings contain
  distinct canaries and all public states/errors are formatted
- **THEN** no canary appears in any diagnostic

#### Scenario: portable core gates remain green

- **WHEN** focused, workspace, factory, target/MSRV, supply-chain, and full Nix
  gates run
- **THEN** the crate passes without expanding its normal dependency cone
