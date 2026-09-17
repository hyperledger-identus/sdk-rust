## ADDED Requirements

### Requirement: Retained key-reference alternatives are construction-bounded

Every public `kid` and `x5c` key-reference alternative SHALL carry an opaque
value validated under positive `JwsLimits`. Raw `String` and `Vec<String>`
payloads SHALL NOT directly inhabit those alternatives. Named fallible
constructors SHALL reject invalid or excessive input before returning a retained
key-reference value. Reuse under a tighter protected-header limit SHALL be
revalidated and fail closed.

#### Scenario: exact-limit native key references remain interoperable

- **WHEN** a caller constructs a non-empty control-free `kid` exactly at the
  configured string ceiling or an `x5c` chain of one through eight valid
  decodable standard-base64 entries within all configured ceilings
- **THEN** construction succeeds and protected-header serialization/parsing
  preserves the existing JOSE member and exact accepted text

#### Scenario: direct or tighter-limit bypass is unavailable

- **WHEN** a caller supplies an empty, control-containing, one-byte-oversized,
  malformed-base64, empty-chain, or ninth-certificate input, or reuses a value
  beneath a tighter builder limit
- **THEN** construction or builder validation fails through a static redacted
  error before signing and no raw public variant constructor can retain it
