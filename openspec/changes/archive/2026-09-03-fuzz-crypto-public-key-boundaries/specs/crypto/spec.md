## ADDED Requirements

### Requirement: Sanitizer-backed public-key representation fuzzing

The crypto capability SHALL provide separate sanitizer-backed targets for its
public JWK and public COSE Key representation boundaries. Targets SHALL accept
arbitrary bytes and treat rejection as valid. Every accepted JWK SHALL preserve
its validated value across serde round trips, produce deterministic canonical
RFC 7638 SHA-256 thumbprints, and preserve required public key material through
full-coordinate COSE conversion. Every accepted COSE Key SHALL preserve its
validated value across deterministic encoding and reparsing and SHALL preserve
required public material through supported JWK conversion. Compressed EC2
conversion SHALL remain an explicit failure.

Fuzz-only dependencies SHALL remain in an independent workspace outside every
published crate dependency cone. The targets SHALL NOT add algorithm, curve,
key-use, signature, custody, trust, method, chain, or product policy.

#### Scenario: hostile JWK bytes cannot violate public invariants

- **WHEN** arbitrary bounded bytes contain valid, invalid, non-UTF-8, private,
  shadowed, incompatible, malformed, padded, deeply extended, or oversized
  JSON Web Key text
- **THEN** deserialization SHALL reject it without panic or return a public key
  whose serde, coordinate, thumbprint, and structural conversion invariants hold

#### Scenario: hostile COSE bytes cannot violate public invariants

- **WHEN** arbitrary bounded bytes contain valid, invalid, tagged, trailing,
  duplicate, private, floating-point, deeply nested, wrong-width, compressed,
  or over-limit CBOR
- **THEN** parsing SHALL reject it without panic or return a public key whose
  resource, deterministic encoding, and structural conversion invariants hold

#### Scenario: fuzz tooling is not a crypto dependency

- **WHEN** production, minimal-feature, MSRV, mobile, WASM, or downstream
  dependency cones are built
- **THEN** cargo-fuzz, libFuzzer, sanitizer, and corpus transport support SHALL
  NOT be required by or exposed from `identus-crypto`

### Requirement: Reproducible bounded crypto fuzz campaigns

The repository SHALL expose one documented crypto command interface for
committed-corpus replay, deterministic fixed-run smoke, and time-boxed soak
modes through its pinned sanitizer compiler, runner, and runtime binding.
Pull-request and integration smoke SHALL fix seed, run count, input ceiling,
execution timeout, memory ceiling, mutation reload, and worker count. Scheduled
and manual soak SHALL remain separately bounded.

Original corpora and dictionaries SHALL cover standards-shaped public-key and
consumer-shaped representation boundaries without containing production key
material or asserting trust. Exact binary COSE seeds MAY use a documented
text-only transport decoded solely by the harness. Failure artifacts SHALL be
retained for triage; an accepted defect SHALL be minimized and promoted to
committed corpus and deterministic regression evidence. Performance SHALL be
recorded without a hardware-specific pass threshold.

#### Scenario: ordinary crypto fuzz CI is repeatable

- **WHEN** the same revision runs the pull-request crypto fuzz gate
- **THEN** both targets SHALL receive the same seed, run count, and resource
  limits through the pinned Nix environment and terminate deterministically

#### Scenario: longer search remains bounded and diagnosable

- **WHEN** scheduled or manually dispatched crypto soak finds a sanitizer or
  invariant failure
- **THEN** the target SHALL stop inside the documented envelope and preserve
  its untrusted artifact for minimization without custom logging of its bytes

## MODIFIED Requirements

### Requirement: JWK

The crate SHALL provide a public-only `PublicKeyJwk` with private fields,
typed `JwkKeyType` and `JwkCurve`, fallible constructors, read-only accessors,
validating JSON serialization/deserialization, and an `EncodeJwk` trait
(`encode_jwk() -> PublicKeyJwk`) implemented for every supported public key
type. The supported profiles SHALL be `OKP/Ed25519`, `OKP/X25519`,
`EC/P-256`, and `EC/secp256k1`. Every coordinate SHALL be canonical unpadded
base64url and decode to exactly 32 bytes. OKP profiles SHALL contain `x` and
SHALL NOT contain `y`; EC profiles SHALL contain both `x` and `y`. Native
construction and deserialization SHALL enforce the same invariants. The public
type SHALL reject the private `d` member. Additional public members SHALL
round-trip without being interpreted and SHALL NOT shadow `kty`, `crv`, `x`,
or `y`.

When `jwk-thumbprint` is enabled, `PublicKeyJwk::thumbprint_sha256()` SHALL
return a typed `JwkThumbprint` computed according to RFC 7638. OKP hash input
SHALL contain only `crv`, `kty`, and `x`; EC input SHALL contain only `crv`,
`kty`, `x`, and `y`. Members SHALL be lexicographically ordered with no
whitespace and encoded as UTF-8. Extensions SHALL NOT affect the result. The
thumbprint SHALL expose immutable 32-byte SHA-256 digest access and canonical
unpadded base64url text. The serde, thumbprint, and structural JWK/COSE
invariants SHALL remain under the bounded sanitizer campaign.

#### Scenario: RFC 8037 Ed25519 public key is accepted exactly

- **WHEN** the RFC 8037 Appendix A.2 public JWK is deserialized
- **THEN** it SHALL produce `OKP/Ed25519`, preserve the exact `x`, omit `y`,
  and serialize to an equivalent public JWK without `d`

#### Scenario: curve encoders preserve their public coordinates

- **WHEN** Ed25519, X25519, P-256 or secp256k1 public keys call `encode_jwk()`
- **THEN** the result SHALL use the correct typed profile and SHALL contain
  the same canonical coordinate bytes as the public key encoding

#### Scenario: EC and OKP shapes are enforced

- **WHEN** an EC JWK omits `y`, an OKP JWK contains `y`, or `kty` and `crv`
  are incompatible
- **THEN** native construction and deserialization SHALL reject the value

#### Scenario: coordinates are canonical and full width

- **WHEN** a coordinate has padding, an invalid alphabet, non-zero trailing
  bits, or decodes to any length other than 32 bytes
- **THEN** native construction and deserialization SHALL reject the value

#### Scenario: private material is rejected

- **WHEN** a public JWK contains a `d` member
- **THEN** deserialization and extension-aware construction SHALL reject it
  without including the private value in an error

#### Scenario: unknown public extensions survive a round trip

- **WHEN** a valid public JWK contains additional public members such as
  `kid` or a collision-resistant extension name
- **THEN** deserialize/serialize SHALL preserve their JSON values while the
  crypto crate SHALL NOT interpret their policy

#### Scenario: RFC 8037 Ed25519 thumbprint matches exactly

- **WHEN** the RFC 8037 Appendix A.2 public JWK is thumbprinted
- **THEN** its digest SHALL equal
  `90facafea9b1556698540f70c0117a22ea37bd5cf3ed3c47093c1707282b4b89`
  and its base64url value SHALL equal
  `kPrK_qmxVWaYVA9wwBF6Iuo3vVzz7TxHCTwXBygrS4k`

#### Scenario: optional metadata cannot change key identity

- **WHEN** two JWKs have identical required key members but different `kid`,
  `alg`, `use`, or other public extensions
- **THEN** their SHA-256 JWK thumbprints SHALL be equal

#### Scenario: required key material changes key identity

- **WHEN** a required coordinate or supported curve differs
- **THEN** the SHA-256 JWK thumbprint SHALL differ

#### Scenario: canonicalization is fixed and bounded

- **WHEN** an OKP or EC JWK is thumbprinted
- **THEN** fixed JSON fragments and validated values SHALL stream directly
  into SHA-256 without a generic JSON canonicalizer or canonicalization heap
  allocation

#### Scenario: accepted wire values remain coherent under mutation

- **WHEN** sanitizer-guided mutation produces a JWK accepted by the public
  serde boundary
- **THEN** round-trip equality, coordinate shape, canonical thumbprint, and
  supported structural conversion SHALL hold without panic

### Requirement: validated public COSE Key boundary

The crate SHALL provide a public-only `PublicKeyCose` with private fields,
typed `CoseKeyType`, `CoseCurve` and `CoseEcY`, fallible constructors,
read-only accessors, bounded `from_cbor` parsing, deterministic `to_cbor`
encoding, and an `EncodeCose` trait implemented for every supported public key
type when the applicable features are enabled. Supported profiles SHALL be
`OKP/Ed25519`, `OKP/X25519`, `EC2/P-256` and `EC2/secp256k1`. Coordinates SHALL
be exactly 32 bytes. OKP SHALL contain `x` and no `y`; EC2 SHALL contain `x`
and either a 32-byte `y` or its registered boolean sign form. Label `-4`
private material SHALL always be rejected.

The parser SHALL reject inputs over 4096 bytes before decoding, use a nesting
limit of 16, require one untagged map with exact end of input, reject duplicate
labels and allow no more than 32 non-structural top-level parameters. Only
`kty`, `crv`, `x`, and `y` are structural for this bound; common parameters
such as `alg`, `key_ops`, `kid`, and Base IV count together with unknown public
parameters. Common and unknown public parameters SHALL survive parse/encode
without being interpreted or exposed as third-party wire types. Floating-point
extension values SHALL be rejected. The encoder SHALL normalize supported
`kty` and `crv` registry names to integers and emit definite-length,
shortest-form CBOR with recursively length-first map-key ordering: shorter
deterministic key encodings first, then bytewise lexical order for keys of
equal length. The resource, encoding, and structural conversion invariants
SHALL remain under the bounded sanitizer campaign.

#### Scenario: registered OKP and EC2 fixtures are accepted

- **WHEN** valid assigned-integer COSE keys for Ed25519, X25519, P-256 and
  secp256k1 are parsed
- **THEN** they SHALL produce the matching typed profile and exact public
  coordinate bytes

#### Scenario: registered text names normalize to integer identifiers

- **WHEN** a supported key uses the registered text spelling for `kty` or
  `crv`
- **THEN** parsing SHALL accept the profile and deterministic encoding SHALL
  emit the assigned integer identifiers

#### Scenario: EC2 supports full and compressed y forms

- **WHEN** an EC2 public key contains a 32-byte `y` or a boolean sign value
- **THEN** the typed value SHALL preserve the selected form and emit an
  equivalent deterministic key

#### Scenario: private and incompatible key shapes are rejected

- **WHEN** label `-4` is present, OKP contains `y`, EC2 omits `y`, or `kty`
  and `crv` are incompatible
- **THEN** parsing SHALL fail without rendering caller-controlled bytes

#### Scenario: coordinate types and widths are enforced

- **WHEN** `x` is not a byte string, `y` has the wrong CBOR type, or a public
  coordinate is not exactly 32 bytes
- **THEN** parsing SHALL reject the key

#### Scenario: parser resources and message boundaries are enforced

- **WHEN** input exceeds 4096 bytes, nesting exceeds 16, a tag or trailing
  item is present, or more than 32 common-plus-unknown parameters are supplied
- **THEN** parsing SHALL reject the input before returning a key

#### Scenario: duplicate labels cannot be smuggled

- **WHEN** a top-level or retained nested map repeats a deterministically
  equivalent key
- **THEN** parsing or deterministic encoding SHALL reject it

#### Scenario: public extensions round trip deterministically

- **WHEN** a valid key contains bounded common or unknown public parameters,
  including explicitly present empty common byte strings, without
  floating-point values
- **THEN** repeated `to_cbor` calls SHALL return identical bytes with RFC 8949
  length-first map ordering and parsing those bytes SHALL retain equivalent
  parameters

#### Scenario: curve encoders preserve public key bytes

- **WHEN** Ed25519, X25519, P-256 or secp256k1 public keys call
  `encode_cose()`
- **THEN** the result SHALL use the correct typed profile and exact public
  coordinate bytes

#### Scenario: full-coordinate JWK conversion is lossless for key material

- **WHEN** a supported structural JWK converts to COSE and back, or a
  full-coordinate COSE key converts to JWK and back
- **THEN** key type, curve and public coordinate bytes SHALL be unchanged,
  while format-specific metadata SHALL NOT be inferred

#### Scenario: compressed EC2 does not masquerade as a JWK

- **WHEN** conversion of a sign-bit EC2 key to `PublicKeyJwk` is requested
- **THEN** conversion SHALL fail explicitly without performing curve
  decompression

#### Scenario: accepted binary values remain coherent under mutation

- **WHEN** sanitizer-guided mutation produces a COSE Key accepted by the
  public bounded parser
- **THEN** deterministic re-encoding, reparse equality, resource ceilings, and
  supported structural conversion SHALL hold without panic
