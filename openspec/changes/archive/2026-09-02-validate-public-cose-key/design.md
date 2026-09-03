## Context

The SDK already validates public JWKs for Ed25519, X25519, P-256 and
secp256k1, but has no compact equivalent for CBOR protocols. RFC 9052 defines
COSE_Key as a CBOR map, RFC 9053 defines OKP/EC2 parameters, and RFC 8812 adds
secp256k1. The boundary is parser- and crypto-adjacent: accepting a duplicate
label, private parameter or incompatible curve can change which key a protocol
uses.

The donor audit found no generic implementation to port. Apollo
`ccee22bcd693e618b9b8ff3e15ed6f9c9156c27c` and NeoPRISM
`8becb225132efb1d9302b2c5f6ed4d87b84e8685` contain no COSE key codec.
midnight-identity `427f8571950c42967a18726cbcbefecc19ef8d79`
contains future JOSE/COSE vocabulary but no reusable codec. Lace ID Portal
`804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` remains evidence-only because
repository license evidence is unresolved. Oxid
`bfe3b481568dc738f0732c2b27548fab8721fd95` uses Apache-2.0 `ciborium` 0.2.2
for a bounded Midnight credential parser; its product format remains
downstream and only informs the resource-bound discipline here.

The implementation therefore uses Apache-2.0 `coset` as a standards-oriented
wire codec and places SDK-owned validation around it. No donor code or fixture
is copied.

## Goals / Non-Goals

**Goals:**

- Make invalid supported-profile public COSE Key states unavailable after
  construction or parsing.
- Bound parser work and reject duplicate-label, trailing-data and private-key
  confusion.
- Preserve unknown public metadata while hiding codec-specific types.
- Emit stable deterministic bytes and interoperate with the existing JWK
  boundary without inventing curve arithmetic.
- Keep the feature independently buildable, wasm-safe and inexpensive enough
  for wallet and binding use.

**Non-Goals:**

- Private, symmetric or unsupported public keys; COSE Sign/Sign1, Encrypt or
  Mac; generic CBOR; algorithm or `key_ops` authorization; on-curve/subgroup
  checks; new cryptography; FFI; publication; consumer adoption; or the Apollo
  and NeoPRISM lifecycle decisions.

## Decisions

### Decision 1: an SDK-owned validated wrapper over `coset`

`PublicKeyCose` stores typed key material plus a private normalized CBOR value.
`coset::CoseKey` is a temporary interpretation used during validation, not the
round-trip source of truth, because its default-valued fields cannot represent
the presence of every valid common parameter. The public API exposes
constructors, typed accessors, bounded `from_cbor` and deterministic `to_cbor`;
it does not expose `coset` or `ciborium` types. This keeps the SDK contract
stable if the codec changes and prevents callers from bypassing validation by
mutating public wire fields.

`coset` is used rather than implementing a CBOR/COSE parser. It already models
RFC 9052, rejects duplicate top-level labels and trailing data, supports
registered and text labels, and uses `ciborium` without a runtime framework.
The SDK wrapper adds the missing profile, private-material, width and resource
checks.

**Rejected:** expose `coset::CoseKey` directly. Its intentionally general,
mutable fields admit unsupported/private shapes and leak a dependency API.
Hand-writing a COSE parser is also rejected as unnecessary security-sensitive
code.

### Decision 2: match the implemented JWK profile, including compressed EC2

`CoseKeyType` has `Okp` and `Ec2`. `CoseCurve` has Ed25519, X25519, P-256 and
secp256k1 with the assigned values 6, 4, 1 and 8. Coordinates are fixed
`[u8; 32]`; `CoseEcY` is either `Coordinate([u8; 32])` or `Sign(bool)` because
RFC 9053 and RFC 8812 allow both EC2 forms. OKP rejects `y`; EC2 requires it.

Constructors and the deterministic encoder always emit assigned integer
identifiers. The parser accepts the matching registered text spellings and
normalizes them to integers. Unsupported/private registry values remain a
future-profile decision rather than vocabulary-only variants.

This validates representation, not curve membership or usage policy.

### Decision 3: public-only and extension-preserving

Label `-4` is rejected before the value can enter `PublicKeyCose`. Known
structural labels are validated once, and up to 32 non-structural top-level
parameters are retained in the private wire representation. Only `kty`, `crv`,
`x`, and `y` are excluded from that count. Common `kid`, `alg`, `key_ops` and
Base IV members therefore share the bound with unknown members and are also
retained, including explicitly present empty byte strings where the RFC permits
them, but are not interpreted as trust, capability or authorization.

The debug and error surfaces report only typed profile information, input
lengths and invariant names; they never render raw CBOR or extension values.
There is deliberately no extension mutation API in this slice. A later
protocol component can define typed metadata only when a consumer contract
requires it.

### Decision 4: bounded parsing and deterministic encoding

`from_cbor` rejects inputs over 4096 bytes before allocation, parses with a
depth limit of 16, requires exactly one untagged CBOR map and exact end of
input, and rejects duplicate map keys. It limits common-plus-unknown top-level
parameters to 32 by counting the normalized source map and excluding only
`kty`, `crv`, `x`, and `y`. Floating-point extension values are rejected because
the selected codec does not emit half-precision floats and therefore cannot
guarantee RFC 8949 preferred serialization for them.

`to_cbor` normalizes the supported `kty` and `crv` identifiers, uses definite
lengths and shortest integers, and recursively sorts each map using RFC 8949
section 4.2.1 length-first ordering: shorter deterministic key encodings
precede longer ones, with bytewise lexical order as the tie-breaker. Duplicate
nested map keys are rejected. The same byte sequence is produced for repeated
serialization of the same value.

The bounds are intentionally conservative for key objects and can be widened
by a later evidence-backed issue. They are not a generic CBOR document limit.

### Decision 5: key-material-only JWK conversion

Full-coordinate COSE keys convert to structural `PublicKeyJwk` values and vice
versa with exact coordinate bytes. Format-specific `kid`, `alg`, `key_ops`,
Base IV and extension metadata are not translated because their registries and
value domains differ. A compressed EC2 `y` fails conversion explicitly rather
than performing curve decompression inside a representation crate.

### Decision 6: feature and dependency shape

The additive `cose` feature enables optional, default-feature-disabled
`coset`. Default builds include `cose`; a COSE-only build does not require
serde, JSON, base64 or a curve backend. Curve `EncodeCose` implementations are
compiled only when both their curve feature and `cose` are enabled. JWK
conversion additionally requires `jwk`.

## Threat Contract

**Assets:** private key material, selected public-key identity, profile
integrity, canonical output identity and bounded parser resources.

**Threats addressed:**

- private scalar injection using label `-4`;
- duplicate-label and trailing-data smuggling;
- `kty`/`crv` confusion and malformed coordinate types or widths;
- treating a compressed EC2 point as a complete JWK;
- parser allocation/nesting exhaustion within an unbounded envelope;
- non-deterministic encodings changing hashes or signatures; and
- raw attacker-controlled values entering errors or debug output.

**Residual boundaries:** the type does not prove that coordinates are on the
declared curve, validate algorithm/key-operation policy, assign trust, or
translate format-specific metadata. Whole protocol messages impose their own
smaller limits. These boundaries remain explicit in API docs and tests.

## Test and Verification Strategy

- Pin hand-audited RFC 9052/9053 map encodings for OKP and EC2, plus RFC 8812
  secp256k1 values.
- Prove constructor/parser equivalence, registered text normalization,
  deterministic map ordering, extension retention and exact-end behavior.
- Reject every recorded malformed shape, private label, duplicate, tag,
  overflow, nesting and unsupported floating-value case.
- Prove all four curve encoders and full-coordinate JWK conversions preserve
  exact public bytes; reject compressed-to-JWK conversion.
- Verify COSE-only, curve-plus-COSE, default, all-feature, no-default, MSRV,
  WASM, conformance, docs, clippy, formatting, OpenSpec, Nix and supply-chain
  gates.
- Record a release-mode encode/parse throughput observation without creating a
  brittle timing assertion.
- Perform a distinct local misuse review and independent security review before
  integration.

## Pre-Implementation Semantic Review

- The slice is additive and independently reversible.
- Normative numeric identifiers and structural labels are unambiguous.
- The dependency is Apache-2.0 and meets the repository MSRV.
- Product CBOR, chain curves, policy, publication and downstream mutations are
  excluded.
- Parser limits, private-material rejection and residual curve validation are
  testable.
- No semantic blocker remains as of 2026-09-03.

## Migration Plan

1. Land this issue-linked contract and ADR as a signed, DCO-bearing commit.
2. Implement the wrapper, curve encoders, conversions and focused tests.
3. Run the target/security/performance evidence, sync the canonical crypto
   spec and archive the change after local review.
4. Merge the eligible PR to `develop`; downstream adoption stays separate.

Rollback is a normal PR revert. No published or persisted SDK contract is
migrated by this change.
