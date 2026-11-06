## ADDED Requirements

### Requirement: Maintained cryptographic dependency closure

The `identus-crypto` production and standalone fuzz dependency closures SHALL
contain no crate version rejected by the repository's current authoritative
RustSec policy. A dependency remediation SHALL preserve SDK-owned methods,
canonical key and signature encodings, cross-node verification behavior,
feature topology and supported targets. When a published public field exposes
an affected upstream type and an advisory-clean in-place substitution is not
possible, the source break SHALL be recorded in the consumer-change ledger and
a versioned migration SHALL be published. A RustSec ignore SHALL NOT substitute
for remediation without a separately accepted, time-bounded risk decision.

#### Scenario: a transitive cryptographic advisory is published

- **WHEN** the authoritative audit rejects a transitive arithmetic or curve crate
- **THEN** the coordinated upstream family SHALL be upgraded or replaced behind
  the existing SDK facade, and root plus fuzz audit gates SHALL pass without an ignore

#### Scenario: dependency implementation changes remain behavior-neutral

- **WHEN** a major upstream curve-family version is adopted for remediation
- **THEN** all existing key, signature, derivation, JWK/COSE, cross-language,
  JVM compatibility and portable-target evidence SHALL remain valid

#### Scenario: an upstream type is part of the release-candidate API

- **WHEN** a consumer directly constructs or reads a public key-wrapper tuple field
- **THEN** the next candidate SHALL identify the RustCrypto 0.14 source break and
  direct the consumer to SDK byte constructors/encoders or matching 0.14 types
