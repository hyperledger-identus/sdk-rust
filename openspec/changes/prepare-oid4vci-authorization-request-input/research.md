# OID4VCI Authorization Request input research

Research class: protocol
Research status: ready
Decision date: 2026-09-25
Source retrieval date: 2026-09-25
Research blockers: none

## Problem and existing implementation

The current implementation from issue #350 delivered exact offer/server
binding and preserves `issuer_state`, but no state owns the inputs required by
a later Authorization Request. Consumer evidence is the wallet-side flow under
parent #7 and the retained exact `oauth2` comparison fixture. The main hazard
is treating an independently supplied challenge as proof of a verifier
relationship. The SDK must also avoid importing an OAuth runtime for one
deterministic primitive.

## Normative sources

- [OpenID4VCI 1.0 Final](https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0-final.html),
  published 2025-09-16, sections 5.1, 12.3 and 13.2. The request follows OAuth
  Authorization Code behavior, includes offered `issuer_state`, chooses
  `authorization_details` or scope for credential intent, and applies OAuth
  security BCP.
- [RFC 6749](https://www.rfc-editor.org/rfc/rfc6749.html), sections 3.1.2 and
  4.1.1 plus Appendix A. It requires `response_type=code` and `client_id`,
  defines redirect/state behavior, forbids redirect fragments, and gives the
  `VSCHAR` grammar used here.
- [RFC 7636](https://www.rfc-editor.org/rfc/rfc7636.html), sections 4.1 through
  4.3 and Appendix B. A verifier is 43 through 128 unreserved ASCII
  characters; S256 is base64url-no-pad of SHA-256 over its ASCII bytes.
- [RFC 9700](https://www.rfc-editor.org/rfc/rfc9700.html), sections 2.1.1 and
  4.4. Public clients use transaction-specific PKCE, S256 is the non-exposing
  method, and response correlation/mix-up defenses remain required.
- ADR 0102 and the exact separately locked `oauth2 5.0.0` fixture. The crate is
  reproducible oracle evidence, not a production dependency.

## Candidate decisions

| Candidate | Decision | Evidence and boundary |
| --- | --- | --- |
| Accept caller verifier and challenge | `not-adopt` | Grammar-valid values can still be unrelated; this fails the transaction binding the API appears to promise. |
| Generate verifier and state in `identus-oid4vci` | `not-adopt` | Entropy adapters and lifecycle policy are outside this transport-neutral protocol slice. |
| Adopt `oauth2 5.0.0` | `oracle` | ADR 0102 records broad unconditional URL/clock/RNG/HTTP/JSON coupling and under-validated/panicking verifier construction. |
| Reimplement SHA-256/base64url | `not-adopt` | Established audited SDK primitives already exist; local crypto duplication violates reuse policy. |
| Use `identus-crypto` behind OID4VCI-owned types | `adopt` | Feature-minimal `hash` and `base64` reuse adds no external package and preserves a narrow public boundary. |
| Select configuration by caller string | `not-adopt` | It admits an unbounded comparison input and duplicates a validated value already owned by the offer. |
| Select configuration by index | `retain-local` | O(1), allocation-free selection returns the existing validated identifier and makes out-of-range failure explicit. |
| Choose scope now | `not-applicable` | Current bounded issuer metadata does not retain optional configuration scope. |
| Construct `authorization_details` now | `not-applicable` | Format/profile details and serialization belong to the next explicit wire contract. |

## Compatibility and dependency evidence

The transition is additive and does not parse or emit wire bytes. A direct
`identus-crypto` dependency uses `default-features = false` and features
`hash,base64`; `identus-jose` already reaches the same internal crate and
base64 family, while SHA-256 is already locked by the workspace. No external
version, native library, unsafe block, runtime, clock, RNG, HTTP client or
target-specific package is introduced.

The direct internal dependency cone becomes `identus-core`, `identus-crypto`
and `identus-jose`. This is intentional cohesion: OID4VCI owns PKCE semantics;
crypto owns only the hash and encoding primitives. No crypto type is exposed.

The pinned oracle is exact `oauth2 5.0.0` at source revision
`f3424b4b2190c83c6d031fdc71eed2351d49e0df`, MIT OR Apache-2.0, crates.io
checksum `51e219e79014df21a225b1860a479e2dcd7cbd9130f4defd4bd0e191ea31d67d`.
This exact release tag, checksum and license are its immutable provenance.
It declares MSRV Rust 1.65 and, with defaults disabled, previously resolved 68
normalized host/iOS/Android normal/build lines and 77 on WASM. The isolated
fixture passed exact Rust 1.98.1 host, WASM, iOS and Android compile evidence.
Its release source has no direct unsafe block, native link or build script, but
the resolved graph reaches platform/unsafe code; the separately locked fixture
passed the repository cargo-deny and cargo-audit supply-chain checks on the
recorded retrieval date. The project was active through 2026-02-22, but no
released feature split changes ADR 0102's production decision.

The public facade boundary is OID4VCI-owned: client, redirect, state, verifier
and challenge types expose borrowed strings only, while `identus-crypto`
remains a private implementation dependency. Public and wire compatibility is
therefore additive and no external type enters the API.

## Security, privacy and maintenance evidence

The verifier is fixed by RFC at 43 through 128 bytes, so oversized input is
rejected before hashing. Client identifier, redirect URI and state use
positive caller-selected byte ceilings. Retained state/verifier values are
zeroized and all new Debug/error surfaces are redacted/static. The Appendix B
known answer is required.

URI validation accepts absolute private-use schemes and loopback HTTP needed
by native clients, but rejects relative references, fragments and userinfo. It
does not prove client registration, exact callback equality, origin safety or
OS handler ownership.

## Rejected or deferred candidates

Authorization URL/query construction, reserved-parameter duplicate handling,
scope/authorization-details selection, PAR, authorization response parsing,
exact state comparison, issuer mix-up defense, redirect comparison, code
exchange, persistence and interoperability remain separate issues. This state
is not an authorization or CSRF-completion claim.

## Open questions and blockers

There is no blocker for this input-preparation slice. The successor must choose
scope versus `authorization_details` with explicit issuer metadata and format
evidence, and must define deterministic reserved-parameter serialization before
an Authorization Request is exposed.

Protocol and draft currency is OpenID4VCI 1.0 Final plus RFC 6749, RFC 7636 and
RFC 9700; no pre-final OID4VCI draft behavior is admitted. The reconsideration trigger
for `oauth2` is a released feature-sliced version satisfying ADR 0102's bounded
non-panicking facade criteria with demonstrated multi-mechanic payoff.
Rollback removes the additive module and direct feature-minimal internal
dependency; prior states and wire behavior remain unchanged.

## Evidence commands

```text
scripts/factory research-ready prepare-oid4vci-authorization-request-input
scripts/factory constraints-ready prepare-oid4vci-authorization-request-input
scripts/factory preflight prepare-oid4vci-authorization-request-input --issue 352 --write
cargo test -p identus-oid4vci
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features
RUSTDOCFLAGS='-D warnings' cargo doc --workspace --all-features --no-deps
scripts/factory check
```

Unrun evidence includes external server interoperability, browsers, mobile
redirect handlers, callback/runtime behavior, HTTP, PAR and downstream use.
