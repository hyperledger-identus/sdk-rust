## ADDED Requirements

### Requirement: Parser replacement preserves fail-closed behavior

A candidate parser SHALL NOT replace an SDK parser based only on nominal format
support or valid-input parity. Dependency research SHALL compare every
capability-relevant rejection class, including malformed syntax, invalid text
encoding, duplicates, incomplete or trailing input and resource ceilings. If
the candidate accepts, repairs, ignores or lossily transforms an input the
current specification rejects, adoption SHALL remain deferred or rejected
unless a separate reviewed specification change intentionally changes that
boundary.

#### Scenario: Browser parser is more permissive than protocol boundary

- **WHEN** a candidate browser-compatible parser preserves malformed escapes or replaces invalid text that the SDK rejects
- **THEN** an agent records the semantic mismatch and cannot adopt it by wrapping the result in an SDK-owned type

#### Scenario: Candidate and strict facade compose without weaker behavior

- **WHEN** differential evidence proves all specified valid and invalid classes plus resource limits remain equivalent or stronger
- **THEN** the parser may proceed through the remaining consumer-payoff, dependency, compatibility and security gates
