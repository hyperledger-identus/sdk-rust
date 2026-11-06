# did-jwk-method Specification

## Purpose
TBD - created by archiving change add-bounded-did-jwk. Update Purpose after archive.
## Requirements
### Requirement: did:jwk creation and parsing are public-only and bounded

The SDK SHALL expose `did:jwk` through an independently selectable effect-free
crate. The first profile SHALL accept only canonical unpadded base64url method
identifiers carrying bounded, duplicate-free UTF-8 JSON for Ed25519, X25519,
P-256, or secp256k1 public JWKs. Creation SHALL consume an SDK-owned
`PublicKeyJwk`. Parsing SHALL preserve the exact supplied DID while validating
carrier encoding, JSON structure, private-member absence, supported profile,
actual public-key material, `use` compatibility, `key_ops` absence, and public
work ceilings. Creation SHALL enforce the same serialized JSON and carrier
ceilings as parsing, so every created DID remains parseable by this profile.

#### Scenario: a supported public JWK creates a deterministic DID

- **WHEN** a caller supplies one supported public SDK JWK within the method's
  serialized JSON and carrier ceilings
- **THEN** creation emits deterministic compact JSON, canonical unpadded
  base64url, and a `did:jwk` value that parses to the same semantic public JWK

#### Scenario: a valid alternate JSON serialization is parsed

- **WHEN** canonical base64url carries a bounded duplicate-free JWK whose
  whitespace or member order differs from SDK creation
- **THEN** parsing preserves the exact DID and public extensions without
  claiming a one-key/one-DID mapping or requiring serialization equality

#### Scenario: secret, ambiguous, or unsupported input is received

- **WHEN** JSON contains a private member, duplicate object name, unsupported
  key profile, invalid curve point, weak or aliased OKP key, `key_ops`,
  unsupported `use`, incompatible explicit curve/use combination, malformed
  UTF-8/JSON, padding, or a non-canonical carrier
- **THEN** parsing fails with a stable redacted category and returns no JWK

#### Scenario: an input exceeds a public boundary

- **WHEN** carrier text exceeds 21,846 bytes, decoded JSON exceeds 16,384
  bytes, or JSON exceeds 16 levels, 1,024 nodes, 64 top-level members, or the
  inherited public-extension budgets
- **THEN** creation, parsing, or resolver dispatch rejects it before avoidable
  decoding, cloning, typed construction, cryptographic validation, or
  projection

### Requirement: did:jwk resolution is deterministic and use-specific

Resolution SHALL project exactly one `JsonWebKey2020` verification method at
`#0` using the complete semantic public JWK. An absent `use` SHALL reference
the method from authentication, assertionMethod, capabilityInvocation,
capabilityDelegation, and keyAgreement. `use=sig` SHALL omit keyAgreement and
`use=enc` SHALL include only keyAgreement. `kid` and other extensions SHALL NOT
change the fragment or relationship policy. Resolution SHALL preserve the
supplied DID and perform no I/O. The absent-`use` rule is the pinned method's
compatibility projection and SHALL NOT be interpreted as a claim that each
curve performs every relationship's operation.

#### Scenario: an absent-use JWK is resolved

- **WHEN** a supported JWK without `use` is resolved
- **THEN** the one `#0` method is referenced from all five relationship
  families exactly as specified by the pinned method

#### Scenario: an explicitly scoped JWK is resolved

- **WHEN** a signing key with `use=sig` or X25519 key with `use=enc` is resolved
- **THEN** the document contains respectively the four signing/capability
  relationships or only keyAgreement

#### Scenario: registry composition is requested

- **WHEN** a caller explicitly registers the method with the generic registry
- **THEN** create, parse, and resolve complete locally with the same document
  while unrelated SDK users receive no new global default

#### Scenario: JSON-LD representation is negotiated

- **WHEN** the resolver receives `application/did+ld+json`
- **THEN** its document includes DID Core and JWS 2020 contexts while the
  default `application/did+json` representation remains context-free

### Requirement: did:jwk remains custody-neutral and independently evidenced

The crate SHALL own no private key, entropy, generation, signing, proof or
trust selection, custody, persistence, network, executor, clock, or product
policy. Normative and differential fixtures SHALL be pinned and attributed
without oracle dependencies. Property, fuzz, benchmark, E2E, dependency,
native, MSRV, no-default, WASM, iOS, Android, strict Clippy, rustdoc, and
workspace evidence SHALL be recorded before the unreleased capability becomes
active. Delivery SHALL separately prove the registry E2E behavior and portable
target compilation, and SHALL use manifest/public-API dependency guards plus a
repository-diff audit to prove prohibited surfaces remain absent.

#### Scenario: quality evidence is incomplete

- **WHEN** any required evidence family or provenance record is absent
- **THEN** the change remains incomplete and cannot claim delivered support

#### Scenario: a prohibited surface enters the slice

- **WHEN** the manifest, public API, or repository diff introduces private-key
  custody, effects, bindings, consumers, publication, or `did:web`
- **THEN** the stop-boundary audit fails and the change cannot be delivered
