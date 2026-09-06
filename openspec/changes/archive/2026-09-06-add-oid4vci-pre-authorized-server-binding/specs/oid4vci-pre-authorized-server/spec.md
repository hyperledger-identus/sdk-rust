## ADDED Requirements

### Requirement: Pre-Authorized server binding is explicit and consuming

The SDK SHALL consume one `CredentialOfferWithMetadata` and one
`AuthorizationServerMetadataCore` to create a
`CredentialOfferWithPreAuthorizedServer` only after all requirements in this
capability succeed atomically. The result SHALL own both predecessor states and
expose them by reference without copying the Pre-Authorized Code or other
caller-controlled strings.

Success SHALL prove only cross-document server, grant, and Token Endpoint
agreement. It SHALL NOT prove metadata retrieval provenance, server or issuer
trust, endpoint reachability, client authorization/authentication, Transaction
Code satisfaction, request readiness, replay safety, or successful issuance.

#### Scenario: validated states advance without copying secrets

- **WHEN** the caller consumes a matched offer/issuer-metadata state and an
  eligible selected server core
- **THEN** one owned Pre-Authorized server state exposes both exact predecessor
  states and no bearer-adjacent value is copied

#### Scenario: any failed invariant is atomic

- **WHEN** any required cross-document invariant fails
- **THEN** no Pre-Authorized server state is returned

### Requirement: Issuer metadata and the grant constrain the selected server

The selected Authorization Server Metadata issuer SHALL exactly equal one of
the Credential Issuer Metadata's effective Authorization Servers. When the
issuer metadata omits `authorization_servers`, its Credential Issuer SHALL be
the sole effective server. When the offered Pre-Authorized Code grant contains
an `authorization_server` hint, that hint SHALL exactly equal the selected
server issuer.

The caller SHALL choose the candidate by supplying its already validated
metadata. The SDK SHALL NOT rank, discover, normalize, substitute, or fall back
between servers.

#### Scenario: omitted issuer list uses its exact single-server default

- **WHEN** issuer metadata omits `authorization_servers` and selected metadata
  uses the exact Credential Issuer identifier
- **THEN** server membership succeeds without inventing an advertised list

#### Scenario: listed or hinted server mismatch fails closed

- **WHEN** the selected issuer is absent from an advertised server list or
  differs from a present Pre-Authorized Code server hint
- **THEN** no bound state is returned and no alternative server is selected

### Requirement: Pre-Authorized grant support and Token Endpoint are required

The offer SHALL contain the OID4VCI 1.0 Final Pre-Authorized Code grant. The
selected server's effective grant list SHALL contain exactly
`urn:ietf:params:oauth:grant-type:pre-authorized_code`, and its metadata SHALL
contain a validated Token Endpoint.

An omitted `grant_types_supported` SHALL use the RFC 8414 defaults
`authorization_code` and `implicit` and therefore SHALL NOT satisfy this
requirement. The OID4VCI anonymous-access flag SHALL remain available through
the owned metadata but SHALL neither permit nor deny this binding.

#### Scenario: explicit support and endpoint complete the proof

- **WHEN** the offer carries the Pre-Authorized Code grant and the selected
  server explicitly advertises it with a Token Endpoint
- **THEN** the consuming transition succeeds regardless of the separately
  preserved anonymous-access flag

#### Scenario: absent or implicit-only capability fails closed

- **WHEN** the offer lacks the grant, server grants omit/default away from it,
  or the Token Endpoint is absent
- **THEN** a distinct static error identifies the failed invariant

### Requirement: Bound state remains redaction-safe portable and policy-neutral

The success state Debug representation SHALL expose no issuer, endpoint, grant,
Pre-Authorized Code, Transaction Code description, or retained JSON. Each
failure SHALL be fieldless and map to a stable static `oid4vci.*` code/message
without caller-controlled content.

The transition SHALL parse and allocate no untrusted data and SHALL operate
only over predecessor collections already limited by their contracts. The
normal dependency cone, feature set, Rust 1.85 MSRV, browser-WASM, Android
ARM64, and iOS ARM64 portability SHALL remain unchanged. No HTTP, async,
runtime, crypto, DID, storage, chain, or product dependency is permitted.

#### Scenario: sensitive canaries never enter diagnostics

- **WHEN** all predecessor values contain distinct canaries and the bound state
  plus every new error are formatted
- **THEN** no canary appears in any diagnostic

#### Scenario: portable policy-neutral gates remain green

- **WHEN** focused, workspace, factory, target/MSRV, supply-chain, and full Nix
  gates run
- **THEN** the transition passes without a dependency, manifest, feature,
  parser, limit, consumer, or product-policy change
