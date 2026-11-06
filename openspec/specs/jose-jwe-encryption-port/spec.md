# jose-jwe-encryption-port Specification

## Purpose
TBD - created by archiving change construct-oid4vp-encrypted-direct-post-response. Update Purpose after archive.
## Requirements
### Requirement: JOSE exposes one bounded engine-neutral encryption port

The SDK SHALL expose protocol-neutral P-256 ECDH-ES/A128GCM recipient,
request, compact-result and static error types without exposing an
implementation dependency. Plaintext, key identifier, public key and compact
output SHALL have independent finite limits and redacted diagnostics.

#### Scenario: an adapter encrypts accepted material

- **WHEN** a conforming adapter receives one validated SDK-owned request
- **THEN** it returns one bounded compact JWE matching the closed algorithm and
  recipient profile or one static failure category

#### Scenario: an adapter returns invalid material

- **WHEN** compact material is malformed, oversized, uses another algorithm,
  omits the required direct-agreement shape or carries a different key ID
- **THEN** the SDK rejects it without exposing the compact value or recipient
  material in diagnostics

### Requirement: the encryption port owns no protocol or engine policy

The port SHALL perform no HTTP, OIDC claim handling, trust, consent, storage,
telemetry or product policy and SHALL NOT expose third-party JOSE types. A
production engine SHALL remain a separately qualified private adapter.

#### Scenario: a protocol composes encryption

- **WHEN** a higher-level protocol supplies bounded plaintext and one public
  recipient
- **THEN** JOSE returns only validated cryptographic boundary material and
  makes no protocol, authentication or delivery claim

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
