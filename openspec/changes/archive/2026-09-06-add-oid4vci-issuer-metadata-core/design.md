# Design: bounded OID4VCI Credential Issuer Metadata core

## Context

Issue #117 continues B09 / `IDR-023` from
`develop@78860abcffee5f2a10a5377ba3bce17bc67c164b`. OpenID4VCI 1.0 Final
sections 12.2.1 and 12.2.4 define the unsigned Credential Issuer Metadata
members and exact issuer comparison. Section 4.1.1 constrains an offered
`authorization_server` hint.

The Final HTML was published 2025-09-16, retrieved 2026-09-06, and hashed as
`f123c3178cacd27688b15b762098a045e9eb35eccfe2f5f18a357c3815e06ba7`.
Errata and later revisions require a separate compatibility review.

## Provenance and isolation

No donor source or fixture is copied. Final-shaped and independently
reconstructed consumer-shaped inputs are test evidence.

| Repository | Revision | Evidence | SHA-256 | Classification |
| --- | --- | --- | --- | --- |
| Oxid | `bfe3b481568dc738f0732c2b27548fab8721fd95` | `crates/adapters/openid4vci/src/lib.rs` | `79122b8fc78251e50773a7effeeaf8162747411b9b89283d977e48a2b7f164af` | Apache-2.0; behavior only |
| Oxid | same | `crates/adapters/openid4vci/src/portal.rs` | `cd1a398af4eac8ffb20562478c37ec973fa8ccf268ea7acf0b6f4ad27bafdb0a` | Apache-2.0; behavior only |
| Lace ID Portal | `925ec8d04882eabd4ac7b784c70fc2f0c152faae` | `crates/issuer-http/src/well_known.rs` | `0d5908f54fe1d56960c20ffca6680848e0ba638c52d8958dd9b373a08376a4eb` | no repository license evidence; behavior observation only |
| Lace ID Portal | same | `crates/issuer-integration/fixtures/openid4vci-final/positive/credential-issuer-metadata.json` | `7c8562a13310722ba2b554daaa1fd5f9e44e7757c6c2689ada0f85425e39aa71` | no repository license evidence; shape observation only |

Oxid has pre-existing untracked `.claude/` and `.pi/taskflows/`. Lace's current
integration base is `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` with pre-existing
untracked `.pi-subagents/`, `.pi/`, and `tmp/`. Neither checkout may be changed.

## Decisions

### D1 — validate exact unsigned metadata as a distinct state

`CredentialIssuerMetadata::parse` accepts exact UTF-8 JSON plus an expected
Credential Issuer Identifier and positive `CredentialIssuerMetadataLimits`.
The expected identifier represents the identifier used to derive a future
retrieval URL; the metadata's required `credential_issuer` must match it by
simple string comparison without normalization. The exact JSON is retained in
a zeroizing owned string and unknown members remain untouched.

The parser independently enforces byte, JSON depth/node, issuer, endpoint,
Authorization Server, configuration ID, format, and count limits. Duplicate
decoded property names at every level are rejected before semantic use.
Defaults are 131,072 JSON bytes, depth 16, 1,024 nodes, 2,048 issuer bytes,
2,048 endpoint bytes, 2,048 bytes per Authorization Server, 16 Authorization
Servers, 256 bytes per configuration ID, 128 bytes per format, and 128
configurations.

### D2 — parse only the cross-protocol metadata core

The state validates and exposes:

- the required HTTPS `credential_issuer` identifier;
- optional non-empty, duplicate-free `authorization_servers`, each with RFC
  8414 issuer syntax;
- the required HTTPS `credential_endpoint`, allowing port/path/query but no
  userinfo or fragment; and
- required `credential_configurations_supported` entries, each object-shaped
  and carrying a non-empty bounded `format` string.

The configuration identifier and format are opaque strings. Format-specific
members, proof algorithms, display, claims, optional endpoints, encryption,
batch, notification, and signed metadata remain unparsed but lossless. This is
not a claim that any credential format is supported.

### D3 — distinguish advertised and effective Authorization Servers

The metadata state preserves whether `authorization_servers` was absent. When
absent, the Credential Issuer Identifier is the single effective Authorization
Server. A present array must be non-empty and duplicate-free. Accessors expose
the advertised list and deterministic effective lookup without inventing a
selected server.

### D4 — make cross-document agreement a consuming transition

`CredentialOfferWithGrants::try_with_metadata` consumes both inputs and returns
`CredentialOfferWithMetadata` only when:

1. metadata and offer issuer strings are identical;
2. every offered configuration ID is a metadata key; and
3. every grant `authorization_server` hint appears in a metadata list with at
   least two entries.

This implements the Final's prohibition on using a hint otherwise. The SDK
does not choose a grant or a server when no hint exists. The matched state owns
both prior states so exact offer and metadata JSON remain available through
explicit accessors.

### D5 — use static redacted failures and zeroizing content

All caller-controlled metadata strings and exact JSON use zeroizing ownership.
Public debug output contains only safe counts, booleans, and type names. Types
implement neither `Display` nor Serde serialization. Fieldless errors separate
invalid limits/JSON/fields, bounds, endpoint syntax, expected-issuer mismatch,
offer issuer mismatch, missing offered configuration, and invalid grant hint.

### D6 — preserve the dependency cone and portability

The selective bounded scanner is extended rather than adding a general
deserializer or network stack. `uriparse` remains the syntax dependency. The
crate keeps no default features and remains compile-checked on Rust 1.85,
browser-WASM, Android ARM64, and iOS ARM64.

## Review record before implementation

- **Architecture/API:** distinct parse and consuming match states prevent raw
  JSON or unmatched metadata from crossing the boundary; no protocol choice is
  implied.
- **Standards:** simple-string issuer comparison, optional/non-empty AS array,
  issuer-as-default, safe endpoint syntax, required configuration map, and
  multi-AS hint matching are represented explicitly.
- **Security/privacy:** no network authority or trust is accepted; limits,
  duplicate rejection, zeroization, and redacted diagnostics cover the new
  untrusted input.
- **Resource:** every collection and decoded string has an explicit positive
  maximum; the inherited hard JSON depth cap remains 64.
- **Compatibility:** APIs are additive and experimental; consumer divergence
  is retained as evidence rather than normalized into product policy.

No pre-implementation blocker remains.

## Risks and trade-offs

- Selective rescanning costs CPU but preserves staged semantics and the exact
  original JSON without exposing a generic mutable tree.
- Validating only `format` inside configuration entries intentionally leaves
  format-profile validity to later profile crates; the type name and docs must
  not overclaim full metadata conformance.
- Endpoint syntax is not origin trust, SSRF safety, reachability, or TLS
  validation; adapters retain those responsibilities.
- The strict non-empty format value is an early usability invariant on an
  opaque required identifier in this experimental API.

## Migration and rollback

The API is additive in an unpublished crate and has no downstream dependency.
A focused revert removes metadata/matching modules, limits/errors/tests/ADR and
spec delta while retaining all three prior Credential Offer states.
