# Design: bounded OID4VCI Credential Offer grants

## Context

Issue #115 continues B09 / `IDR-023` from
`develop@bbfbbd77a68a619a616346aa9d3f31285b6effb3`. OpenID4VCI 1.0 Final
sections 3.5 and 4.1.1 define grant advertisement and Transaction Code input
requirements; section 6.1 defines how those values later enter a token request;
section 13.6 describes bearer replay and phishing threats. RFC 8414 section 2
defines the Authorization Server issuer identifier syntax referenced by the
Final specification.

The Final HTML was published 2025-09-16, retrieved 2026-09-06, and hashed as
`f123c3178cacd27688b15b762098a045e9eb35eccfe2f5f18a357c3815e06ba7`.
Errata and later revisions require a separate compatibility review.

## Provenance and isolation

No donor source or fixture is copied. Official examples and independently
reconstructed consumer-shaped inputs are test evidence.

| Repository | Revision | Evidence | SHA-256 | Classification |
| --- | --- | --- | --- | --- |
| Oxid | `bfe3b481568dc738f0732c2b27548fab8721fd95` | `crates/adapters/openid4vci/src/lib.rs` | `79122b8fc78251e50773a7effeeaf8162747411b9b89283d977e48a2b7f164af` | Apache-2.0; behavior only |
| Oxid | same | `crates/adapters/openid4vci/src/portal.rs` | `cd1a398af4eac8ffb20562478c37ec973fa8ccf268ea7acf0b6f4ad27bafdb0a` | Apache-2.0; behavior only |
| Lace ID Portal | `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` | `crates/issuer-services/src/credential_offer.rs` | `35629c3462af6d83210f046c9adabda6c00d75cc247df111f17387d8e62f5b60` | no repository license evidence; behavior observation only |

Oxid has pre-existing untracked `.claude/` and `.pi/taskflows/`. Lace has
pre-existing untracked `.pi-subagents/`, `.pi/`, and `tmp/`. Neither checkout
may be changed by this slice.

## Decisions

### D1 — use a consuming validation transition

`CredentialOffer::try_into_grants` consumes the core-semantic offer and returns
`CredentialOfferWithGrants`. The new state retains the prior offer and exact
JSON while exposing optional typed known grants. A caller can therefore observe
absence, an empty grants object, either known alternative, or both without the
SDK choosing a flow or claiming that any flow is usable.

Unknown grant names and unknown members inside known grants remain in the exact
retained JSON and are skipped semantically. The Final requires wallets to
ignore unrecognized top-level parameters and allows extension grant types.

### D2 — preserve secret-bearing values behind explicit accessors

Opaque issuer state, Pre-Authorized Code, Authorization Server identifier, and
Transaction Code description values use owned zeroizing strings. Public types
have custom content-redacted `Debug` and neither `Display` nor Serde output.
The grant-validated offer exposes safe presence/count/enum/integer shape in
debug output but never caller-controlled text.

### D3 — keep grant limits separate and exact

`CredentialOfferGrantLimits` has positive maxima for decoded issuer-state
bytes, Pre-Authorized Code bytes, Authorization Server identifier bytes,
Transaction Code description bytes, and advertised Transaction Code length.
Defaults are 2,048; 4,096; 2,048; 1,200; and 64 respectively. The description
also obeys the Final limit of 300 Unicode scalar values. The byte limit bounds
allocation while the character limit implements the wire contract.

Present opaque string values are non-empty. Transaction Code length is a
positive, base-10 JSON integer without sign, fraction, or exponent and cannot
exceed the configured maximum. These rules fail unusable instructions early
without validating the future user-supplied Transaction Code itself.

### D4 — model Transaction Code presence and defaults explicitly

An absent `tx_code` means no Transaction Code is requested. A present object,
including `{}`, becomes `TransactionCodeRequirements`; its optional raw mode,
length, and description remain independently observable. `effective_input_mode`
returns `numeric` when the member is absent, matching the Final default, while
`input_mode` preserves whether the issuer stated it explicitly.

Only `numeric` and `text` are accepted. Unknown members remain opaque. Lace's
current `tx_code: null` is recorded as legacy-negative consumer evidence
because Final says presence denotes an object, including an empty object.

### D5 — validate Authorization Server syntax, defer metadata agreement

Both known grants may carry an RFC 8414 issuer identifier: HTTPS, non-empty
host, optional port/path, and no userinfo/query/fragment. The exact string is
retained. Whether the hint is allowed, matches one of multiple issuer-metadata
entries, supports the chosen grant, or is trustworthy is deferred until a
metadata-matching state has both inputs.

### D6 — extend the bounded scanner and static errors

The implementation performs another selective pass through the already
transport-bounded JSON. It validates known grant objects and skips unrelated
values lexically, preserving the prior depth/node/duplicate-name bounds and
arbitrary-magnitude unknown number compatibility without materializing a
generic JSON tree. New fieldless errors distinguish invalid limits, known
grant shapes, state/code/identifier bounds, and Transaction Code rules.

## Risks and trade-offs

- A third bounded scan costs CPU but preserves explicit validation states and
  avoids an unbounded or lossy generic JSON model.
- Strict non-empty strings and positive Transaction Code length reject values
  that are syntactically typed but unusable; the crate is experimental and
  this behavior is captured in its capability contract.
- Grant validation does not protect a bearer Pre-Authorized Code from capture.
  Zeroization and redaction reduce accidental retention; replay resistance,
  separate-channel Transaction Codes, expiry, and one-time use remain issuer
  and later protocol-state responsibilities.
- An Authorization Server identifier is syntax, not permission or trust.

## Migration and rollback

The API is additive in an unpublished crate and has no downstream dependency.
A focused revert removes the grant module, limits/errors/tests/ADR/spec delta
and returns `CredentialOffer` grants to opaque-only status.
