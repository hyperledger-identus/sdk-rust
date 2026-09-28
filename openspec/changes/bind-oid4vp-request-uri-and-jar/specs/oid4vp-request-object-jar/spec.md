## ADDED Requirements

### Requirement: Signed JAR validation is a consuming cryptographic transition

The SDK SHALL validate a bounded compact signed Request Object with
`identus-jose`, a caller-owned suite allowlist, and an explicitly
algorithm-bound public key before exposing its payload.

#### Scenario: a supported signature verifies

- **WHEN** the protected algorithm matches the bound key, the suite is allowed, and the signature is valid
- **THEN** payload processing proceeds to signed-claim correlation

#### Scenario: the protected type is absent or wrong

- **WHEN** `typ` is not exactly `oauth-authz-req+jwt`
- **THEN** parsing terminates before any authorization claim is exposed

#### Scenario: algorithm, key, suite, or signature is invalid

- **WHEN** JOSE verification fails for any cryptographic reason
- **THEN** the result is a static invalid-signature category with no nested diagnostic

### Requirement: Verified claims are bounded and correlated

The SDK SHALL parse the complete verified payload under byte, depth, node,
member, and string limits, reject duplicate top-level names, and require exact
outer/inner client-id and optional wallet-nonce correlation.

#### Scenario: signed correlation succeeds

- **WHEN** the verified payload has one matching `client_id` and, if sent, one matching `wallet_nonce`
- **THEN** the SDK returns a redacted verified envelope owning the bounded exact payload

#### Scenario: the client identifier differs

- **WHEN** the signed `client_id` is missing, duplicated, non-string, or differs byte-for-byte from the outer value
- **THEN** validation terminates without exposing the payload

#### Scenario: a sent wallet nonce is not repeated exactly

- **WHEN** POST supplied `wallet_nonce` and the signed claim is missing, duplicated, non-string, or different
- **THEN** validation terminates without exposing the payload

#### Scenario: JSON resource or ambiguity limits fail

- **WHEN** the payload exceeds depth, node, member, or decoded-string limits or has duplicate top-level names
- **THEN** validation returns a static bounded-payload category

### Requirement: Cryptographic validity does not imply verifier trust

The SDK SHALL document and type the result as signature/correlation evidence
only and SHALL NOT claim client-key authorization, prefix trust, audience,
freshness, OAuth validity, DCQL validity, consent, or product authorization.

#### Scenario: a caller supplies a valid but unauthorized key

- **WHEN** the signature verifies with that key
- **THEN** the result makes no claim that the key is associated with or trusted for the client identifier

### Requirement: JAR diagnostics are stable and redacted

The SDK SHALL not render compact JWS, decoded payload, client id, nonce, key
material, key reference, or verifier-controlled canaries in public diagnostics.

#### Scenario: hostile JAR content fails

- **WHEN** a malformed or invalid signed object contains canaries
- **THEN** every public diagnostic contains only stable category metadata
