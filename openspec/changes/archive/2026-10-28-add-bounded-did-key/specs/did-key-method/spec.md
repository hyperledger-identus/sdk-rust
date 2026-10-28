# did-key-method delta

## ADDED Requirements

### Requirement: did:key creation and parsing are canonical and bounded

The SDK SHALL expose `did:key` through an independently selectable effect-free
crate. The first profile SHALL accept only canonical base58-btc `z` method
identifiers carrying direct Ed25519, X25519, P-256, or secp256k1 public-key
multicodecs. Creation SHALL consume typed caller-owned public keys and parsing
SHALL validate canonical varints, exact key lengths, curve points, re-encoding,
and public text/decoded byte ceilings. Ed25519 public keys SHALL additionally be
rejected when their field encoding is non-canonical or the point is weak.
X25519 public keys SHALL additionally be rejected when their field encoding is
non-canonical or they cannot produce a contributory shared secret.

#### Scenario: a supported public key creates a canonical DID

- **WHEN** a caller supplies one supported SDK public-key value
- **THEN** creation emits the fixed codec, canonical public bytes, base58-btc
  fingerprint, and a `did:key` value that parses back to the same public key

#### Scenario: non-canonical or unsupported input is received

- **WHEN** an identifier uses `u`, an overlong varint, unknown/private codec,
  wrong key length, invalid curve point, non-canonical or weak Ed25519 key,
  non-canonical or low-order X25519 key, trailing data, or non-canonical text
- **THEN** parsing fails with a stable redacted category and returns no key

#### Scenario: an input exceeds a public boundary

- **WHEN** method-specific text exceeds 128 bytes or decoded material could
  exceed 67 bytes
- **THEN** parsing or resolver dispatch rejects it before cloning, proportional
  decoding, or document projection

### Requirement: did:key resolution is deterministic and relationship-specific

Resolution SHALL project exactly one `Multikey` verification method using the
canonical fingerprint. Direct X25519 keys SHALL appear only in `keyAgreement`.
Ed25519, P-256, and secp256k1 keys SHALL appear in authentication,
assertionMethod, capabilityInvocation, and capabilityDelegation. Resolution
SHALL NOT derive another key or perform I/O.
The resolver SHALL return a context-free document for `application/did+json`
and add the DID Core and Multikey contexts for `application/did+ld+json`.

#### Scenario: a signing key is resolved

- **WHEN** a canonical Ed25519, P-256, or secp256k1 DID is resolved
- **THEN** the document controller and verification method are the DID and the
  one method is referenced from the four signing/capability relationships

#### Scenario: a direct agreement key is resolved

- **WHEN** a canonical X25519 DID is resolved
- **THEN** the one method is referenced only from `keyAgreement`

#### Scenario: registry composition is requested

- **WHEN** a caller explicitly registers the method with the generic registry
- **THEN** create/parse/resolve completes locally with the same canonical
  document while unrelated SDK users receive no new global default

#### Scenario: JSON-LD representation is negotiated

- **WHEN** the resolver receives `application/did+ld+json`
- **THEN** its document includes the DID Core and Multikey contexts while the
  default `application/did+json` representation remains context-free

### Requirement: did:key remains custody-neutral and independently evidenced

The crate SHALL own no private key, entropy, signing, custody, persistence,
network, executor, clock, or product policy. Normative and differential
fixtures SHALL be pinned and attributed without oracle dependencies. Property,
fuzz, benchmark, E2E, dependency, native, MSRV, no-default, WASM, iOS, Android,
strict Clippy, rustdoc, and workspace evidence SHALL be recorded before the
unreleased capability becomes active.

#### Scenario: quality evidence is incomplete

- **WHEN** any required evidence family or provenance record is absent
- **THEN** the change remains incomplete and cannot claim delivered support
