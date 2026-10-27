## ADDED Requirements

### Requirement: Maintained cryptographic dependency closure

The `identus-crypto` production and standalone fuzz dependency closures SHALL
contain no crate version rejected by the repository's current authoritative
RustSec policy. A dependency remediation SHALL preserve the crate's public API,
canonical key and signature encodings, cross-node verification behavior,
feature topology and supported targets. A RustSec ignore SHALL NOT substitute
for remediation without a separately accepted, time-bounded risk decision.

#### Scenario: a transitive cryptographic advisory is published

- **WHEN** the authoritative audit rejects a transitive arithmetic or curve crate
- **THEN** the coordinated upstream family SHALL be upgraded or replaced behind
  the existing SDK facade, and root plus fuzz audit gates SHALL pass without an ignore

#### Scenario: dependency implementation changes remain behavior-neutral

- **WHEN** a major upstream curve-family version is adopted for remediation
- **THEN** all existing key, signature, derivation, JWK/COSE, cross-language,
  JVM compatibility and portable-target evidence SHALL remain valid
