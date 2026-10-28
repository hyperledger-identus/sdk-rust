# oid4vp-request-object-jar Specification

## Purpose
Define a consuming cryptographic transition from an unverified compact signed
Request Object to bounded signature and correlation evidence without implying
verifier trust.
## Requirements
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

The SDK SHALL type signature/correlation evidence separately from audience,
freshness, and replay evidence and SHALL NOT claim client-key authorization,
prefix trust, consent, or product authorization at either stage.

#### Scenario: a caller supplies a valid but unauthorized key

- **WHEN** the signature verifies and security policy accepts the Request Object
- **THEN** the result still makes no claim that the key is authorized or trusted for the client identifier

### Requirement: JAR diagnostics are stable and redacted

The SDK SHALL not render compact JWS, decoded payload, client id, nonce, key
material, key reference, or verifier-controlled canaries in public diagnostics.

#### Scenario: hostile JAR content fails

- **WHEN** a malformed or invalid signed object contains canaries
- **THEN** every public diagnostic contains only stable category metadata

### Requirement: Request Object audience follows discovery mode

The SDK SHALL require a bounded `aud` claim and compare it exactly with the
caller-supplied discovery audience before returning security evidence.

#### Scenario: static discovery succeeds

- **WHEN** `aud` contains exactly `https://self-issued.me/v2` and all other policy checks pass
- **THEN** the security transition may proceed

#### Scenario: dynamic discovery succeeds

- **WHEN** `aud` contains the exact caller-supplied discovered issuer and all other policy checks pass
- **THEN** the security transition may proceed without consulting Request Object `iss`

#### Scenario: audience differs

- **WHEN** `aud` is not exactly one value equal to the expected value byte-for-byte
- **THEN** validation fails with a static audience-mismatch category

### Requirement: Temporal claims are deterministic and bounded

The SDK SHALL require integral `iat`, honor integral `exp` and `nbf`, and apply
finite maximum-age and allowed-skew policy using a caller-owned clock.

#### Scenario: an object is fresh

- **WHEN** `iat`, optional `nbf`, and optional `exp` are within policy at the injected time
- **THEN** validation proceeds to atomic replay acceptance

#### Scenario: an object is stale, future, expired, or not yet valid

- **WHEN** any temporal boundary fails after allowed skew
- **THEN** validation stops with the corresponding static category before replay state changes

### Requirement: Replay acceptance is one caller-owned atomic transition

The SDK SHALL invoke a caller-owned replay guard exactly once after all local
validation and SHALL return security evidence only when that guard atomically
accepts the bounded event.

#### Scenario: replay is accepted

- **WHEN** local policy passes and the guard atomically accepts the event
- **THEN** the SDK returns redacted security evidence eligible for typed routing

#### Scenario: replay is rejected or the guard is unavailable

- **WHEN** the guard rejects the event or cannot decide
- **THEN** validation fails closed without exposing payload or identifiers in diagnostics

### Requirement: Typed routing requires security evidence

The SDK SHALL expose DCQL and authorization-routing successors from consumed
security evidence rather than directly from signature/correlation evidence.

#### Scenario: a caller has only signature evidence

- **WHEN** the caller has not completed audience, temporal, and replay validation
- **THEN** the public typed API does not permit routing to authorization processing
