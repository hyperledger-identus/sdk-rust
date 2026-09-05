# jws-compact delta

## MODIFIED Requirements

### Requirement: Protected headers have a small closed validated surface

The protected header SHALL be one complete UTF-8 JSON object containing exactly
one string `alg` member, at most one string `typ`, and at most one key reference
selected from string `kid`, public object `jwk`, or array `x5c`. Unknown or
duplicate members, trailing JSON, wrong JSON types, a missing `alg`, or more
than one key-reference member SHALL be rejected. `alg` SHALL contain 1 through
64 visible ASCII bytes, SHALL be case-sensitive and SHALL NOT equal `none`.
`typ` and `kid`, when present, SHALL be non-empty UTF-8 strings without Unicode
control code points and SHALL fit the configured protected-header string bound.

An inline `jwk` SHALL use the accepted validated public-only JWK type and SHALL
reject private key material. An `x5c` chain SHALL contain 1 through 8 non-empty
standard-base64 certificate strings, each fitting the configured header-string
bound. Encoding SHALL stop once the configured decoded protected-header byte
limit is exceeded rather than allocate the complete oversized serialization.

The codec SHALL NOT infer that an algorithm is registered, asymmetric,
supported or compatible with a key. It SHALL NOT validate certificate paths or
trust and SHALL NOT accept `crit`, `b64`, key-attestation or trust-chain
semantics until a focused profile owns their validation.

#### Scenario: consumer-shaped common headers are reusable

- **WHEN** a header supplies Ed25519, legacy EdDSA or ES256, an optional
  explicit proof type, and at most one DID URL `kid`, public JWK or X.509 chain
- **THEN** the codec preserves the validated header without applying DID,
  proof-type, algorithm-selection, certificate or trust policy

#### Scenario: ambiguous header input is rejected

- **WHEN** a header duplicates a member, supplies multiple key references, uses
  an unknown member or wrong JSON type, omits `alg`, adds trailing JSON, carries
  private JWK material, uses an invalid X.509 chain, or exceeds a bound
- **THEN** parsing or encoding fails through a static header or size error

#### Scenario: unsecured algorithm is not representable

- **WHEN** a caller constructs or parses a protected header with `alg` equal to
  the case-sensitive value `none`
- **THEN** the SDK rejects it before an unverified compact value is created
