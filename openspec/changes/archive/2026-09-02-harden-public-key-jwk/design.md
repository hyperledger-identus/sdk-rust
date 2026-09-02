## Context

The current `Jwk` is the minimal NeoPRISM port from
`8becb225132efb1d9302b2c5f6ed4d87b84e8685:lib/apollo/src/jwk.rs`. It is useful
as an encoder result but not safe as an SDK boundary because every field is
public and the type does not validate profile coherence or wire input.

Two consumer implementations demonstrate the missing invariants:

- `midnight-identity@427f8571950c42967a18726cbcbefecc19ef8d79`
  validates construction and deserialization, preserves extensions and rejects
  private material in `crates/midnight-did-domain/src/did_document.rs`.
- `oxid@685f9670af4846d52697a4cfeb94779758ae1075` makes the key type and curve
  typed, fields private and coordinates canonical in
  `crates/identity/domain/src/lib.rs`.

Both are Apache-2.0. Apollo KMP is conformance evidence only. Lace ID Portal is
evidence only because repository-level license evidence is unresolved. No
source is bulk-copied.

Normative behavior comes from RFC 7517, RFC 7518 section 6.2.1, RFC 8037
section 2 and Appendix A.2, and RFC 8812 section 3.1.

## Goals / Non-Goals

**Goals:**

- Make invalid supported-profile JWK states impossible after construction or
  deserialization.
- Keep the type public-only and preserve RFC-compatible public extensions.
- Preserve exact curve encoder output and wasm/minimal-feature support.
- Give callers structured, non-secret errors without new crypto operations.

**Non-Goals:**

- On-curve/subgroup validation, signature or agreement policy, `alg`, `use` or
  `key_ops` interpretation, JWK thumbprints, JWS/JWT, COSE/CBOR, private JWKs,
  RSA/symmetric keys, additional curves, custody, publication or downstream
  migration.

## Decisions

### Decision 1: one validated public-key value type

`PublicKeyJwk` stores private `kty`, `crv`, `x`, `y` and `extensions` fields.
Construction uses explicit `new_okp` and `new_ec` helpers for normal callers
and `try_from_parts` for wire/adaptation code. All paths share one validator.
Read-only accessors are the only field surface.

The type stores `Base64UrlStrNoPad` coordinates, not decoded key objects. This
keeps JWK representation separate from curve parsing and avoids coupling DID
documents to a concrete crypto backend. Exact coordinate length is checked at
construction; on-curve validation remains the consuming curve operation's job.

**Rejected:** a public struct with a `validate` method. It permits invalid
values between construction and validation. Separate EC/OKP public enums would
make serde and FFI evolution more complex without adding an invariant beyond
the private-field constructor gate.

### Decision 2: typed, deliberately bounded profiles

`JwkKeyType` contains `Okp` and `Ec`. `JwkCurve` contains `Ed25519`, `X25519`,
`P256` and `Secp256k1`, and each curve declares its required key type. This
slice models only crypto capabilities that the crate can emit and test.
Midnight Jubjub and BLS profiles stay in `midnight-identity`; Ed448, X448, RSA
and symmetric keys require later evidence and implementation issues.

This is capability support, not usage policy. The type does not decide whether
a key may sign, agree, authenticate or issue credentials.

### Decision 3: validating serde with lossless public extensions

`PublicKeyJwk` serializes to the standard `kty`, `crv`, `x`, optional `y`
members plus a flattened deterministic `BTreeMap<String, serde_json::Value>`.
Deserialization first uses a private wire shape and then invokes the same
constructor as native callers. The private EC/OKP parameter `d` is rejected.
Structural names are reserved and cannot be supplied as extensions.

RFC 7517 requires unknown additional members to be ignored when not
understood. Preserving them is stronger for resolver round trips and is proven
by midnight-identity; the crypto layer does not interpret them. Callers remain
responsible for bounding the total untrusted JSON document size before serde.

**Rejected:** `deny_unknown_fields`, because it would reject interoperable JWKs
with `kid`, `alg`, `use`, `key_ops`, certificate metadata or private extension
names even when the SDK does not interpret those public values.

### Decision 4: replace the experimental API cleanly

`EncodeJwk::encode_jwk` returns `PublicKeyJwk`. The old `Jwk` alias is not kept:
the workspace is version `0.0.0`, publishing is disabled, no supported consumer
exists, and an alias would retain direct-field expectations without preserving
the old construction API. Curve encoders call infallible profile-specific
constructors over fixed-size bytes, so `encode_jwk` remains infallible.

### Decision 5: dedicated safe errors

`JwkError` reports profile, coordinate kind and lengths, never coordinate
contents. It maps to `crypto.invalid_jwk` through the existing redaction-safe
`IdentusError` bridge. Serde converts the same error to a data error whose text
also excludes input values. Public key coordinates are not secrets, but this
uniform rule prevents accidental future expansion into sensitive material.

## Threat Contract

**Assets:** private key material, key-profile integrity, canonical wire
identity and the invariant that a parsed value is structurally usable.

**Threats addressed:**

- private `d` material entering a public-key type;
- algorithm confusion through incompatible `kty` and `crv`;
- missing or unexpected coordinates;
- aliases from padding, invalid alphabet or non-zero trailing bits;
- truncated/extended coordinates; and
- constructor validation bypass through serde.

**Residual boundaries:** no curve arithmetic is performed, so structural
validity does not prove an EC point is on-curve or an OKP byte string is safe
for a particular operation. Extension semantics and whole-document resource
limits belong to the consuming protocol/parser. These boundaries are explicit
in API docs and tests.

## Test and Verification Strategy

- Pin the RFC 8037 Appendix A.2 Ed25519 public JWK exactly.
- Preserve all four current curve encoder coordinate bytes.
- Round-trip supported profiles plus unknown public extensions.
- Reject private `d`, incompatible profiles, missing/unexpected `y`, invalid
  alphabet, padding/non-canonical encoding, bad trailing bits and 31/33-byte
  coordinates through both constructors and serde.
- Verify error redaction, default and minimal features, wasm, conformance,
  workspace tests, docs, clippy, formatting, OpenSpec and factory gates.
- Perform a distinct misuse-resistance/security review after implementation;
  record residual risks and follow-ups rather than silently expanding scope.

## Migration Plan

1. Land this issue-linked specification and ADR as a signed commit.
2. Implement the type and migrate in-crate encoders/tests.
3. Sync the canonical crypto spec, complete evidence, review and archive the
   OpenSpec change.
4. Merge the green PR to `develop`. Generic DID Core adopts it separately.

Rollback is a normal PR revert. No persisted SDK representation or released
consumer is migrated in this change.
