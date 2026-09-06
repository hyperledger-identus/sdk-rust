# Design: bounded OID4VCI Authorization Server Metadata core

## Context

Issue #119 continues B09 / `IDR-023` from
`develop@1d94a634994ab4da7d5e902216e8f47106e4bbce`. OpenID4VCI 1.0 Final
sections 12.2.4 and 12.3 rely on RFC 8414 Authorization Server Metadata and add
the `pre-authorized_grant_anonymous_access_supported` boolean with a default of
false. RFC 8414 section 2 defines issuer, endpoint, grant, and other metadata
members; section 3 defines retrieval and exact issuer comparison.

The Final HTML was published 2025-09-16, retrieved 2026-09-06, and hashed as
`f123c3178cacd27688b15b762098a045e9eb35eccfe2f5f18a357c3815e06ba7`.
RFC 8414 text was retrieved 2026-09-06 and hashed as
`16c816e4e0fdbffb7e910ff3017867bf39debe9cb7f52f5cbc508a052ed660e8`.
Later errata or profiles require a separate compatibility review.

## Provenance and isolation

No donor source or fixture is copied. Official-shaped and independently
reconstructed consumer-shaped inputs are test evidence.

| Repository | Revision | Evidence | SHA-256 | Classification |
| --- | --- | --- | --- | --- |
| Oxid | `5ba38b9bbc9326c294b353daaf2a074eca18c22f` | `crates/adapters/openid4vci/src/lib.rs` | `79122b8fc78251e50773a7effeeaf8162747411b9b89283d977e48a2b7f164af` | Apache-2.0; behavior only |
| Oxid | same | `fixtures/laceid-portal/76e8edf394a4cb37ca822037272d543c68f25f71/openid4vci-final/positive/authorization-server-metadata.json` | `3514a5d3acb75eceb79923960e3463af451741fc770b9cec60fa0f9c466ab7a1` | Apache-2.0; shape only |
| Lace ID Portal | `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` | `crates/issuer-http/src/well_known.rs` | `4dff93d21e02b221598c325a8afdc9ff7311b3c5041727652cea9ecac81419b1` | behavior only |
| Lace ID Portal | `76e8edf394a4cb37ca822037272d543c68f25f71` | `crates/issuer-http/src/well_known.rs` | `0d5908f54fe1d56960c20ffca6680848e0ba638c52d8958dd9b373a08376a4eb` | immutable Final shape only |

Oxid is clean at preflight. Lace has pre-existing untracked `.pi-subagents/`,
`.pi/`, and `tmp/`. Neither checkout may be changed. The repositories are
Apache-2.0 family projects, but no source or fixture is imported by this slice.

## Decisions

### D1 — name and document a partial projection

`AuthorizationServerMetadataCore::parse` accepts exact UTF-8 JSON, an expected
Authorization Server identifier, and positive limits. The `Core` suffix and
API documentation state that success validates only the interpreted members;
it does not prove complete RFC 8414 conformance. In particular,
`response_types_supported` and conditional relationships outside this core are
uninterpreted.

This boundary preserves the established consumer shape without lowering or
misstating RFC 8414. A later full metadata validator can consume the same exact
JSON under a broader contract.

### D2 — bind issuer identity exactly

The required `issuer` and caller-supplied expected issuer both use the existing
RFC 8414 HTTPS identifier grammar. They must match by simple string comparison
without URL normalization. This prevents metadata mix-up while making no claim
about retrieval provenance or server trust.

### D3 — expose only later-flow prerequisites

The core interprets:

- optional HTTPS `authorization_endpoint` and `token_endpoint`, allowing
  port/path/query but no userinfo or fragment;
- optional non-empty, ordered, duplicate-free `grant_types_supported`; and
- optional boolean `pre-authorized_grant_anonymous_access_supported` with an
  effective default of false while preserving whether it was advertised.

An omitted grant list remains distinguishable from an explicit list. Effective
lookup applies the RFC 8414 default `authorization_code`, `implicit` only when
the list is absent. Opaque explicit values, including the OID4VCI
Pre-Authorized Code grant identifier, are not capability or selection claims.

### D4 — bound and retain the exact document

The parser retains the exact JSON in zeroizing ownership and selectively scans
only the core fields. Unknown members and arbitrary-magnitude numbers remain
lossless. Duplicate decoded names at every object depth fail before semantic
use.

Defaults are 131,072 JSON bytes, depth 16, 1,024 nodes, 2,048 issuer bytes,
2,048 bytes per endpoint, 256 bytes per grant value, and 32 grants. Every
maximum is positive and configurable depth cannot exceed 64.

### D5 — use static redacted failures

Caller-controlled strings and retained JSON use zeroizing ownership. Public
debug output includes only safe type, presence, and count state. Types
implement neither `Display` nor Serde serialization. Fieldless errors separate
invalid limits/JSON/fields, bounds, issuer mismatch, endpoint syntax, and grant
array ambiguity without echoing input.

### D6 — preserve portability and dependencies

The existing selective scanner and `uriparse` validation helpers are reused.
The normal cone stays `identus-core`, `serde_json`, `uriparse`, and `zeroize`.
Rust 1.85, browser-WASM, Android ARM64, and iOS ARM64 remain required.

## Review record before implementation

- **Architecture/API:** the standalone core is the smallest prerequisite for
  later flow selection and does not couple it to offer ownership or I/O.
- **Standards:** exact issuer binding, RFC defaults, zero-element omission, and
  the OID4VCI anonymous-access default are explicit; the partial type cannot be
  confused with full RFC 8414 validation.
- **Security/privacy:** untrusted unsigned metadata gains duplicate, type,
  resource, URL, zeroization, and diagnostic-redaction controls without trust.
- **Resource:** every decoded string and collection is independently bounded;
  the inherited hard JSON depth cap remains 64.
- **Compatibility/provenance:** API is additive and experimental; no consumer
  material is copied and consumer divergence is recorded rather than hidden.

No pre-implementation blocker remains.

## Risks and trade-offs

- A partial projection accepts documents that a future complete RFC 8414
  validator may reject. The type name, docs, and absence of a conformance claim
  are therefore security-relevant.
- HTTPS syntax does not establish TLS validation, SSRF safety, reachability, or
  endpoint capability; adapters retain those responsibilities.
- Repeated selective scanning costs CPU but avoids a generic public value tree
  and preserves exact unknown JSON.

## Migration and rollback

The API is additive in an unpublished crate and has no downstream dependency.
A focused revert removes this core while leaving offer and issuer-metadata
states untouched.
