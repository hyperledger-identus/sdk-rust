# DID resolution datetime-validation ownership

## ADDED Requirements

### Requirement: Resolution datetime validation preserves lexical and calendar ownership

The SDK MUST validate its bounded XML Schema whole-second UTC datetime profile
through private lexical and calendar owners that expose no parsed state and
preserve the existing public value, spelling, allocation, and error contract.

#### Scenario: A valid datetime is retained exactly

- **WHEN** any currently accepted bounded four-digit, astronomical-zero,
  negative, or extended-year datetime is passed through a borrowed, owned,
  string-trait, or Serde entry point
- **THEN** lexical recognition and calendar validation accept the same value
- **AND** the exact input spelling is retained without normalization.

#### Scenario: Lexical and calendar failures remain indistinguishable

- **WHEN** an input violates a byte, ASCII, terminal-zone, sign/year,
  separator/digit, month/day/leap, normal-time, or end-of-day invariant
- **THEN** every entry point rejects it with the same existing static invalid-
  datetime resolution boundary
- **AND** no failed input value or private parsed field is exposed.

#### Scenario: Validation remains bounded and cohesive

- **WHEN** the private owners scan and validate a candidate
- **THEN** work remains linear in the existing 128-byte maximum and numeric
  projection remains allocation-free
- **AND** no new ambient clock, locale, timezone, dependency, unsafe code,
  dynamic dispatch, metric waiver, or forwarding-only helper chain is added.
