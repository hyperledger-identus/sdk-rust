# Design: bounded canonical JWS Compact foundation

## Context

Issue #95 advances the JWS part of `IDR-004` from
`develop@8bde038306c0a56850ba5972c48e2553015a2b03`. RFC 7515 defines Compact
Serialization as three base64url-without-padding segments and defines the
signing input as the exact encoded protected-header and payload segments.
RFC 8725 requires applications to choose and verify algorithms rather than
trust the header. OpenID4VCI 1.0 Final then profiles this substrate with
`openid4vci-proof+jwt`, key-reference and claims requirements.

Oxid and Lace ID Portal independently repeat compact splitting, decoding and
signing-input construction. Their claim, DID, clock, algorithm and custody
behavior differs and remains outside this slice.

## Provenance and isolation

No donor code or fixture is copied. The implementation is independently
derived from the normative standards. Donor behavior is used only to construct
new conformance scenarios:

| Repository | Revision | Evidence | SHA-256 | License/classification |
| --- | --- | --- | --- | --- |
| Oxid | `bfe3b481568dc738f0732c2b27548fab8721fd95` | `crates/adapters/openid4vci/src/lib.rs` | `79122b8fc78251e50773a7effeeaf8162747411b9b89283d977e48a2b7f164af` | Apache-2.0; conformance-only |
| Oxid | same | `crates/adapters/openid4vci/src/laceid_portal_contract_tests.rs` | `22139569164d0189c2f2f243e0d356d7fa6f640bfe9b651e873a07fe9be786dc` | Apache-2.0; conformance-only |
| Lace ID Portal | `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` | `crates/issuer-services/src/oid4vp_verifier.rs` | `0ba6133b3a959958570f886765f69cb237c53112394ac0037ddaf1eb782b6254` | no repository license file at revision; behavior observation only |
| Lace ID Portal | same | `crates/issuer-services/src/tests.rs` | `91aa21d94304d2a6f2a9d459b68e08e23b2c614dedaeae1fb59b3be4d1d53134` | no repository license file at revision; behavior observation only |

Both donor checkouts remain read-only. Product values, fixtures and source
text do not enter SDK-Rust.

Preflight state is preserved as evidence rather than cleaned: Oxid is
`integration@bfe3b481568dc738f0732c2b27548fab8721fd95` with the pre-existing
untracked `.claude/` and `.pi/taskflows/` paths and a gone upstream; Lace ID
Portal is `main@804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` with the pre-existing
untracked `.pi-subagents/`, `.pi/` and `tmp/` paths.

Normative interpretation is pinned to RFC 7515
<https://www.rfc-editor.org/rfc/rfc7515.html>, RFC 8725
<https://www.rfc-editor.org/rfc/rfc8725.html>, and OpenID4VCI 1.0 Final
<https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0.html>.

## Decisions

### D1 — JOSE wire semantics live below protocol semantics

`identus-jose` is a credential-semantics member. It is reusable by future
credential formats and by protocol-semantics crates, while depending only on
foundation plus serialization libraries. Putting it in `identus-openid4vc`
would couple SD-JWT and other consumers to one protocol. Putting it in
`identus-crypto` would mix wire syntax and JWT-adjacent policy with key and
algorithm primitives.

The crate is experimental, has no default features and is not published.

### D2 — The public state is explicitly unverified

Parsing returns `UnverifiedCompactJws`. The type exposes the validated
protected header, arbitrary payload octets, signature octets, original compact
text and an exact borrowed signing-input byte slice. It offers no `verify`,
`claims` or truth-bearing convenience method. Custom Debug prints structural
lengths and non-secret algorithm/type metadata only; it omits payload,
signature, compact text and `kid`.

Encoding is staged through `JwsSigningInput`: construct canonical protected
header and payload segments, sign its exact bytes outside this crate, then
attach a non-empty bounded signature to produce `UnverifiedCompactJws`. This
shape supports a later signing port without granting the codec key custody.

### D3 — Limits precede allocation and have conservative defaults

`JwsLimits` validates five positive maxima: complete compact text, decoded
protected header, decoded payload, decoded signature and each protected-header
string. Defaults are respectively 65,536, 4,096, 49,152, 1,024 and 2,048
bytes. `alg` has an additional invariant cap of 64 bytes.

The parser checks total encoded length, segment structure and each decoded
length estimate before allocating decoded storage. It checks actual decoded
length afterward. Arithmetic is checked. Custom limits make protocol-specific
budgets explicit; the SDK defaults are safety posture, not protocol truth.

An empty payload is valid because RFC 7515 defines base64url of an empty octet
sequence. The protected-header and signature segments must be non-empty.

### D4 — Canonical encoding removes aliases

Each segment uses the RFC 7515 URL-safe alphabet with no padding, whitespace or
line breaks. Parsing decodes and re-encodes every segment and requires exact
equality. Exactly two period delimiters are accepted. The original encoded
header and payload segments, not reserialized JSON, define the signing input.

The encoder serializes the header deterministically in `alg`, `typ`, `kid`
order. Parsing never promises to canonicalize JSON member order; it preserves
the accepted original compact value exactly.

### D5 — The first header surface is deliberately closed

`ProtectedHeader` requires `alg` and permits only optional `typ` and `kid`.
All must be JSON strings. A custom Serde map visitor rejects duplicate known
members, any unknown member, missing `alg` and trailing JSON. Algorithm values
must be visible ASCII, are case-sensitive, are at most 64 bytes and cannot be
`none`. Type and key identifiers must be non-empty valid UTF-8 without ASCII
control characters and fit the configured header-string limit.

The codec does not otherwise decide whether an algorithm is registered,
asymmetric, supported or suitable for a key. A later verifier must accept an
explicit caller allowlist and bind the chosen suite to the selected key, as
required by RFC 8725. Closed handling of `jwk`, `x5c`, `crit`, `b64` and other
members prevents silently ignoring security semantics. Each requires a focused
profile extension.

### D6 — Errors are static and redaction-safe

`JoseError` distinguishes invalid limits, excessive compact/header/payload/
signature sizes, compact structure, base64url encoding, protected-header JSON,
duplicate/unknown/missing header members, invalid header values and empty
signature. Variants carry no caller-controlled data. Display and the
`identus-core` bridge expose only static `jose.*` codes and the `jose`
capability.

### D7 — Evidence is layered

Unit and integration tests cover exact bounds, every rejection class, the RFC
7515 example and independently reconstructed Oxid/Portal shapes. A
deterministic matrix exercises varied bounded payload/signature lengths and
round-trips without adding a random/property-test dependency. An ignored
release test repeatedly parses a representative proof-shaped compact value and
prints operations per second without a machine-specific threshold.

Repository conformance continues to enforce workspace dependency declaration,
layer direction, publication denial and supported targets. A cargo-fuzz target
is retained for the repository-wide fuzz-tooling decision under #8.

## Risks and trade-offs

- A closed protected-header surface rejects otherwise valid JWS extensions.
  This is intentional fail-closed behavior; focused slices can add understood
  parameters with explicit mutual-exclusion and criticality rules.
- Owning the original compact text duplicates encoded and decoded data. It
  avoids lifetime-heavy public APIs and preserves exact verification input;
  limits cap the cost.
- JSON member order is not normalized on parse. Signature verification must
  use the exact received signing input, so normalization would be incorrect.
- The codec rejects unsecured JWS even though RFC 7515 describes `alg: none`.
  SDK-Rust has no accepted use case for an integrity-free value and RFC 8725
  documents the associated confusion risk.
- Deterministic property matrices are not coverage-guided fuzzing. They provide
  immediate bounded evidence while the fuzz harness remains a tracked
  follow-up.

## Migration and rollback

This is a new unreleased crate. Oxid and Lace remain unchanged, and no current
SDK crate depends on it. A focused revert removes the package, rulebook and
inventory entries without data migration. Proof builders, verifier suites,
OpenID profiles, consumer adoption and publication require separate issues.
