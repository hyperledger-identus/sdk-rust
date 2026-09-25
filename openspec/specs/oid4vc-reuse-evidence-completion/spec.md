# oid4vc-reuse-evidence-completion Specification

## Purpose
TBD - created by archiving change complete-oid4vc-reuse-evidence. Update Purpose after archive.
## Requirements
### Requirement: Reuse evidence covers every requested seam

The report SHALL compare wire types, bounded parsing, OAuth mechanics, DCQL,
protocol state, JOSE, transport, signed request objects, client identifier
schemes, response modes, Digital Credentials API, encryption, nonce/replay,
trust injection, and credential-format abstraction for the SDK and viable
candidates.

#### Scenario: Reviewers evaluate build versus reuse

- **WHEN** the report is read
- **THEN** every seam has an explicit complete, partial, absent, oracle, or out-of-scope classification

### Requirement: Payoff and cost are measurable and qualified

The report SHALL record dependency names, source/deletion proxy, compile time,
RSS, allocation/resource behavior, target evidence, redaction, and rollback.
It SHALL identify proxies and unmeasured values rather than converting them to
production guarantees.

#### Scenario: A diagnostic is cited

- **WHEN** a reviewer reads a measurement
- **THEN** its command, host/compiler scope, value, and interpretation are available

### Requirement: Differential vectors are clean-room and isolated

The separately locked fixture SHALL execute independent positive and negative
Final-spec DCQL behavior through public APIs without copying candidate tests or
entering production graphs.

#### Scenario: Candidate semantics drift

- **WHEN** an exact locked behavior changes
- **THEN** a focused vector fails while root SDK behavior remains unchanged

### Requirement: Acceptance debt is explicit

The issue SHALL close only after every requested evidence item maps to the
report, fixture, ADR, or a truthful limitation.

#### Scenario: Follow-up merges

- **WHEN** the evidence PR lands
- **THEN** #391 can close without activating a production dependency
