# Design: bind a Pre-Authorized Code server

## Context

Issue #121 continues B09 / `IDR-023` from
`develop@04f5dc8bfe7df4f8089751af19e18fcf2f9410ae`. OpenID4VCI 1.0 Final
allows Credential Issuer Metadata to name one or more Authorization Servers and
allows an offered grant to disambiguate among them. A wallet must use metadata
to determine whether its selected server supports the offered grant. RFC 8414
defines omitted `grant_types_supported` as only `authorization_code` and
`implicit`, so omission cannot support this transition.

The Final HTML was published 2025-09-16 and has SHA-256
`f123c3178cacd27688b15b762098a045e9eb35eccfe2f5f18a357c3815e06ba7`.
RFC 8414 text has SHA-256
`16c816e4e0fdbffb7e910ff3017867bf39debe9cb7f52f5cbc508a052ed660e8`.

## Provenance and isolation

No donor source or fixture is copied. Official-shaped and independently
reconstructed consumer-shaped inputs are behavior evidence.

| Repository | Revision | Evidence | SHA-256 | Classification |
| --- | --- | --- | --- | --- |
| Oxid | `5ba38b9bbc9326c294b353daaf2a074eca18c22f` | `crates/adapters/openid4vci/src/portal.rs` | `d1a1d975ca8a740da1d51b6d93627a18811b96c9cccf0832ed92c86c9e7d48cb` | Apache-2.0; behavior only |
| Oxid | same | `crates/adapters/openid4vci/src/portal_internal_tests.rs` | `a1160157356ef3d030993ac50b0c0ba7c6c45da1ed7ee66f634a855526c8354f` | behavior only |
| Oxid | same | immutable Lace-derived Authorization Server Metadata fixture | `3514a5d3acb75eceb79923960e3463af451741fc770b9cec60fa0f9c466ab7a1` | Apache-2.0; shape only |
| Lace ID Portal | `76e8edf394a4cb37ca822037272d543c68f25f71` | `crates/issuer-http/src/routes_issuer.rs` | `c284250def5b6ee1dff407015d888f7ae7e7c18bb9d9ba6a4c468ab2324fa93c` | behavior only |
| Lace ID Portal | same | `crates/issuer-http/src/well_known.rs` | `0d5908f54fe1d56960c20ffca6680848e0ba638c52d8958dd9b373a08376a4eb` | shape only |

Oxid is clean at preflight. Lace is at
`804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` with pre-existing untracked
`.pi-subagents/`, `.pi/`, and `tmp/`. Neither checkout may be changed.

## Decisions

### D1 — the caller selects, the SDK proves eligibility

`CredentialOfferWithMetadata::try_with_pre_authorized_server` consumes its
receiver and one already parsed `AuthorizationServerMetadataCore`. Success
returns `CredentialOfferWithPreAuthorizedServer`, which owns both states. The
caller chooses which metadata to present; the SDK neither ranks candidates nor
falls back to another server.

### D2 — bind every applicable identifier exactly

The selected metadata issuer must equal one of the issuer metadata's effective
Authorization Servers. When `authorization_servers` was omitted, the sole
effective server is the Credential Issuer. If the Pre-Authorized Code grant has
an `authorization_server` hint, it must equal the selected server exactly.
Existing predecessor states have already validated identifier syntax and the
offer/issuer-metadata hint rules.

### D3 — require explicit grant support and the conditional endpoint

The offer must carry the Final Pre-Authorized Code grant. The selected server's
effective grant list must contain
`urn:ietf:params:oauth:grant-type:pre-authorized_code`, and a Token Endpoint
must be present. Because the RFC 8414 omitted-list default contains only
`authorization_code` and `implicit`, omitted grants fail. The result proves
cross-document agreement, not reachability, trust, client eligibility, or a
request-ready authentication policy.

### D4 — preserve least authority and redacted ownership

The transition does not parse or allocate attacker-controlled data. It moves
the predecessor states and exposes them by reference. Five fieldless errors
identify the failed invariant with static `oid4vci.*` codes/messages. Debug
output for the success state is data-free; retained predecessor values keep
their existing zeroizing ownership.

### D5 — keep the boundary chain-neutral and portable

No dependency, manifest, feature, JSON, limit, HTTP, runtime, crypto, DID,
storage, chain, or product surface changes. Rust 1.85, browser-WASM, Android
ARM64, and iOS ARM64 remain required.

## Review record before implementation

- **Architecture/API:** a consuming wrapper is the smallest state proof and
  preserves the existing staged ownership model without duplicating secrets.
- **Standards:** issuer membership, exact grant hint, explicit Pre-Authorized
  Code support, RFC 8414 omission semantics, and Token Endpoint presence are
  stated independently.
- **Security/privacy:** the transition closes server mix-up and unsupported
  endpoint/grant paths; it adds no trust, network, or secret-handling authority.
- **Resource:** all inputs are previously bounded and the transition performs
  linear scans over already bounded collections without allocation.
- **Compatibility/provenance:** API is additive and experimental; no consumer
  code or fixture is copied and consumer repositories remain read-only.

No pre-implementation blocker remains.

## Risks and trade-offs

- The partial Authorization Server Metadata core may still omit fields a later
  full RFC 8414 validator requires. This state proves only the five explicit
  invariants above.
- Passing metadata is an explicit selection, not trusted discovery. A consumer
  must still bind retrieval, TLS, SSRF, trust, and client policy.
- A server advertising the grant and endpoint can still reject a future
  request. Protocol responses remain a later state transition.

## Migration and rollback

The API is additive in an unpublished crate and has no downstream dependency.
A focused revert removes the transition while leaving each predecessor state
and its exact parsing behavior untouched.
