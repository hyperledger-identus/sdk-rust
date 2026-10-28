# oid4vp-request-object-jar Delta Specification

## MODIFIED Requirements

### Requirement: Cryptographic validity does not imply verifier trust

The SDK SHALL type signature/correlation evidence separately from audience,
freshness, and replay evidence and SHALL NOT claim client-key authorization,
prefix trust, consent, or product authorization at either stage.

#### Scenario: a caller supplies a valid but unauthorized key

- **WHEN** the signature verifies and security policy accepts the Request Object
- **THEN** the result still makes no claim that the key is authorized or trusted for the client identifier

## ADDED Requirements

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
