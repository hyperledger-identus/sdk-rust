# OID4VCI proof JWT capability

## ADDED Requirements

### Requirement: Final-profile holder proof construction

The JOSE capability SHALL provide a holder-side builder for the
`openid4vci-proof+jwt` proof defined by OpenID4VCI 1.0 Final Appendix F.1. It
SHALL require one accepted asymmetric algorithm, exactly one key reference, one
non-empty bounded Credential Issuer audience and an explicit integer issuance
time. The emitted protected `typ` SHALL equal `openid4vci-proof+jwt`.

#### Scenario: identified holder input is canonical

- **WHEN** an identified client prepares a proof with a DID URL `kid`, audience,
  issuance time and server nonce
- **THEN** the protected header and claims SHALL contain exactly the selected
  algorithm, proof type, `kid`, `iss`, `aud`, `iat` and `nonce` values and the
  builder SHALL expose the exact bytes supplied to the signer

#### Scenario: anonymous pre-authorized input omits issuer

- **WHEN** an anonymous pre-authorized client prepares an otherwise valid proof
- **THEN** the claims SHALL omit `iss` by construction rather than serialize a
  null, empty or inferred client identifier

### Requirement: Exclusive bounded proof key references

The protected-header model SHALL represent at most one `kid`, public `jwk` or
non-empty `x5c` chain. It SHALL preserve the existing `ProtectedHeader::new`
`kid` API, reject ambiguous wire input and unknown members, reject private JWK
material, and bound an X.509 chain to at most eight non-empty standard-base64
entries under the configured header and complete-token byte ceilings.

#### Scenario: each standard reference has one wire shape

- **WHEN** callers construct otherwise identical headers using `kid`, public
  JWK and X.509 chain references
- **THEN** each SHALL serialize and parse as its one matching JOSE member with
  no second key-reference member present

#### Scenario: ambiguous or unsafe references fail closed

- **WHEN** a header contains multiple reference members, an empty or excessive
  certificate chain, malformed certificate base64, a private JWK, or an unknown
  member
- **THEN** parsing or construction SHALL fail with a static error before any
  signer is invoked

### Requirement: Claims and resource bounds precede signing

Issuer, audience and nonce strings SHALL be non-empty, control-free and no
larger than a positive caller-selected claim-string ceiling. Header, payload,
signature and complete compact bounds SHALL reuse `JwsLimits`. Every locally
detectable invalid input, unsupported key/algorithm combination or impossible
fixed signature size SHALL fail before external signing.

#### Scenario: attacker-controlled claims cannot trigger a provider

- **WHEN** any claim or key reference violates its bound or the resulting
  compact value cannot contain the fixed signature
- **THEN** preparation or signing preflight SHALL fail and a recording external
  signer SHALL observe zero calls

### Requirement: Typed staged proof state

Preparation SHALL return a distinct OID4VCI signing-input type. Only successful
use of an algorithm-matched `JwsSigner` SHALL produce the signed OID4VCI proof
type. Neither type SHALL claim signature verification, DID authorization,
freshness, replay acceptance or trust, and Debug/errors SHALL not expose claim,
key, certificate, compact, payload or signature values.

#### Scenario: signed is not verified

- **WHEN** an accepted signer returns a signature over the exact prepared bytes
- **THEN** the result SHALL expose the compact proof for transport while its
  type and documentation SHALL make no issuer-verification or authorization
  claim

### Requirement: Portable profile boundary

The holder builder SHALL remain synchronous, safe Rust, runtime-neutral,
chain-neutral and product-neutral. It SHALL use only existing inward JOSE and
crypto capabilities, introduce no DID, HTTP, storage, clock, custody or
consumer dependency, and compile under the repository Rust 1.85 and portable
target matrix.

#### Scenario: consumer evidence remains isolated

- **WHEN** Oxid-shaped anonymous/DID-key behavior and Lace's deferred-proof gap
  inform conformance cases
- **THEN** the cases SHALL be independently reconstructed without modifying or
  importing either consumer repository
