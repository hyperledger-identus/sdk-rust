# jose-jwe-encryption-port delta

## ADDED Requirements

### Requirement: production adapters require independently reproducible qualification

An SDK-provided JWE adapter SHALL be adopted only from an exact stable engine
whose selected graph passes the repository MSRV, license, unsafe-code,
dependency-cone, RustSec, redaction, resource and native/portable target gates.
Qualification SHALL independently encrypt with one public P-256 recipient and
decrypt with the corresponding private key without treating the SDK test double
as its own oracle.

#### Scenario: every mandatory gate passes

- **WHEN** the exact engine independently round-trips the bounded request and
  every mandatory production and target gate passes
- **THEN** a private adapter may be proposed without exposing engine DTOs or
  errors through the SDK API

#### Scenario: one mandatory gate fails

- **WHEN** the engine has an unresolved advisory, unsupported target, excessive
  coupling, unsafe-code violation, incompatible MSRV or public-recipient gap
- **THEN** production adoption is deferred without suppressing the gate, while
  separately isolated oracle evidence MAY be retained
