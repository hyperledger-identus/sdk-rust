## MODIFIED Requirements

### Requirement: Dependency adoption is private and all-gates

A candidate engine SHALL be adopted only when one immutable release passes the
SDK's normative parity, valid and invalid behavior, exact Rust 1.99.1 compiler
floor and primary evidence, license/provenance, advisory, direct/resolved cone,
unsafe/native, host/portable-target, resource-bound, diagnostic-redaction,
secret-handling, maintenance, and concrete-consumer-payoff gates. Candidate
types and policies SHALL remain private behind Identus-owned staged states.

#### Scenario: Candidate implements RFC 9901 but exceeds the active source floor

- **WHEN** a current candidate requires Rust newer than 1.99.1 for the
  unpublished `0.2.x` line
- **THEN** it remains an oracle or deferred candidate and no production
  dependency or public candidate type is added
