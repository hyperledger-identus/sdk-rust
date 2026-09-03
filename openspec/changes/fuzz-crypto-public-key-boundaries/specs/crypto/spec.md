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

#### Scenario: accepted binary values remain coherent under mutation

- **WHEN** sanitizer-guided mutation produces a COSE Key accepted by the
  public bounded parser
- **THEN** deterministic re-encoding, reparse equality, resource ceilings, and
  supported structural conversion SHALL hold without panic
