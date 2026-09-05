# JWS signature capabilities Specification

## ADDED Requirements

### Requirement: Supported algorithms are closed, case-sensitive and fully bound

The SDK SHALL represent the accepted JOSE signing algorithms as a closed
case-sensitive set containing `Ed25519`, `ES256`, and an explicitly deprecated
compatibility value `EdDSA`. `Ed25519` SHALL mean EdDSA using only the Ed25519
parameter set. `ES256` SHALL mean ECDSA using P-256 and SHA-256. `EdDSA` SHALL
also be restricted to Ed25519 in this compatibility surface and SHALL never be
pre-registered by the recommended constructor.

Every verification key SHALL be bound by construction to exactly one of these
algorithms. The binding SHALL require an OKP/Ed25519 JWK for `Ed25519` or
legacy `EdDSA`, and an EC/P-256 JWK for `ES256`. If the public JWK contains an
`alg` member, it SHALL be a string exactly equal to the selected algorithm.
The protected header, bound key and selected verifier suite SHALL name the
same algorithm before any cryptographic operation occurs.

#### Scenario: fully specified algorithms are recommended

- **WHEN** a caller creates the recommended suite registry
- **THEN** it contains `Ed25519` and `ES256` and does not contain legacy
  `EdDSA`

#### Scenario: legacy EdDSA is explicit compatibility

- **WHEN** a caller needs an RFC 8037-era value with `alg` equal to `EdDSA`
- **THEN** the caller must explicitly add the legacy Ed25519-only suite and
  bind the selected Ed25519 key to that legacy algorithm

#### Scenario: algorithm confusion fails before crypto

- **WHEN** header, key binding, JWK `alg`, key curve, or registered suite do
  not all select the same accepted algorithm
- **THEN** verification fails through a static algorithm or key error without
  invoking a mismatched cryptographic primitive

### Requirement: External signing receives only exact public message bytes

The SDK SHALL provide a synchronous, object-safe `JwsSigner` capability that
reports one accepted algorithm and receives only the exact public JWS signing
input. It SHALL return an exact 64-byte signature array or a small static
redaction-safe failure class, making every other output width unrepresentable.
It SHALL expose no raw key material, key export, provider handle format, async
runtime, network transport or custody policy.

Signing a `JwsSigningInput` through this capability SHALL require the
protected-header algorithm to equal the signer's algorithm. The returned
signature SHALL be exactly 64 bytes for all algorithms in this slice. Software
adapters SHALL borrow accepted `identus-crypto` private-key types and SHALL
emit strict Ed25519 or raw big-endian `R || S` ES256 signatures.
Before invoking the signer, the SDK SHALL prove that a 64-byte signature fits
both the configured decoded-signature bound and complete compact bound.

#### Scenario: an external signer receives byte-exact input

- **WHEN** a caller signs a prepared `JwsSigningInput` through a capability
- **THEN** the capability receives exactly `JwsSigningInput::as_bytes()` and
  a successful 64-byte response produces an `UnverifiedCompactJws`

#### Scenario: signer mismatch and provider failures fail closed

- **WHEN** the signer reports another algorithm, rejects the operation, or is
  unavailable
- **THEN** signing fails through a static error and no compact value is
  returned

#### Scenario: impossible fixed output avoids signer side effects

- **WHEN** the configured signature or compact bound cannot contain the
  fixed 64-byte signature and its unpadded base64url representation
- **THEN** signing fails through the applicable static size error before the
  external signer is invoked

### Requirement: Verification is an allocation-bounded explicit state transition

The SDK SHALL provide a synchronous object-safe signature-suite capability
and a registry with a positive caller-selected capacity no greater than 16.
Registration SHALL reject duplicate algorithms and additions above capacity.
Registry membership SHALL be the caller's explicit algorithm allowlist.

Verification SHALL borrow an `UnverifiedCompactJws`, find exactly one suite
by the case-sensitive protected-header algorithm, and pass that suite the
exact `UnverifiedCompactJws::signing_input()` bytes, the decoded signature
octets, and the already algorithm-bound public JWK. Only a successful suite
result SHALL construct an owned `VerifiedCompactJws`. The verified type SHALL
retain the accepted algorithm and original compact value but SHALL NOT claim
key authorization, DID relationship, claims validity, freshness or trust.

#### Scenario: exact received signing input is verified

- **WHEN** a parsed compact value used non-canonical JSON member ordering that
  is valid under the bounded codec
- **THEN** verification passes the original first two encoded segments to the
  selected suite without reserializing the header or payload

#### Scenario: unlisted and duplicate algorithms fail closed

- **WHEN** the header algorithm has no registered suite or a caller attempts
  to register the same algorithm twice
- **THEN** the operation fails through a static registry error without trying
  another algorithm

#### Scenario: parse success cannot construct verified state

- **WHEN** a compact value is only parsed or a signature is only attached
- **THEN** the public result remains `UnverifiedCompactJws` and cannot become
  `VerifiedCompactJws` without successful registry verification

### Requirement: Built-in Ed25519 and ES256 suites use accepted crypto primitives

The built-in Ed25519 suite SHALL reconstruct only an Ed25519 public key from
the bound OKP coordinate and use strict Ed25519 verification. It SHALL reject
every signature whose length is not 64 bytes. The same primitive SHALL serve
the fully specified `Ed25519` suite and the explicitly registered legacy
`EdDSA` compatibility suite.

The built-in ES256 suite SHALL reconstruct only a validated P-256 public point
from the bound EC coordinates. It SHALL interpret the JWS signature as exactly
64 bytes containing 32-byte unsigned big-endian `R` followed by 32-byte
unsigned big-endian `S`; it SHALL NOT accept DER at the JOSE boundary.
`identus-crypto` SHALL expose fixed-width P-256 sign/verify operations so JOSE
does not duplicate primitive implementation or parse the existing DER API.

#### Scenario: RFC Ed25519 compatibility vector verifies

- **WHEN** the RFC 8037 Appendix A compact value and public key are processed
  with an explicitly registered legacy suite
- **THEN** strict verification succeeds over the RFC signing input

#### Scenario: ES256 wire format is exact

- **WHEN** the software P-256 adapter signs a prepared value
- **THEN** the JWS signature is exactly 64 raw `R || S` bytes and the built-in
  suite verifies it, while DER encoding and every other length are rejected

#### Scenario: key points and signatures are validated by reviewed primitives

- **WHEN** JWK coordinates do not form an accepted public key or a signature
  does not verify
- **THEN** the built-in suite fails through a static invalid-key or
  invalid-signature error without exposing coordinates or signature bytes

### Requirement: Diagnostics remain redaction-safe and runtime-neutral

All new errors SHALL contain only static categories or non-sensitive lengths
and SHALL never include signing input, compact text, payload, signature, key
coordinates, private material, `kid`, provider errors or JWK extension values.
Debug output for bound keys and verified values SHALL omit those values.

The capability traits SHALL not depend on an async runtime. Remote, HSM,
secure-element and agent integrations MAY schedule or adapt the synchronous
operation at an outer layer; transport, cancellation and retry policy remain
outside this crate until two real consumers establish a shared contract.

#### Scenario: diagnostics omit caller-controlled cryptographic values

- **WHEN** every signing, registry, binding and verification failure plus the
  bound and verified types are formatted
- **THEN** known canary inputs, signatures, coordinates, `kid` and provider
  details are absent

#### Scenario: portable targets retain the same capability surface

- **WHEN** the accepted MSRV, browser WASM, Android ARM64 and iOS ARM64 gates
  compile the affected crates
- **THEN** no async, OS, network, product or chain dependency is required
