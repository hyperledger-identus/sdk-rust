# cross-language-compatibility-foundation Specification

## MODIFIED Requirements

### Requirement: Consumer-visible changes are recorded before merge

The SDK SHALL record every additive, fixed, behavioral, deprecated, breaking,
or security-relevant consumer change with affected capabilities, packages,
mappings, old
and new behavior, migration action and window, release-note class, rollback,
and exact vector and quality evidence. Ledger validation SHALL reject dangling
references.

#### Scenario: a change is described as internal

- **WHEN** its public, wire, error, persistence, ABI, target, or runtime effect
  differs for a supported consumer
- **THEN** it is ledgered with its actual consumer-visible classification

A qualifying OpenSpec change SHALL additionally declare whether its effect is
consumer-visible or behavior-neutral. Factory validation SHALL reject
capability-mismatched, inactive, omitted, or disposition-inconsistent
references without inferring compatibility from a diff.

#### Scenario: governance does not change runtime behavior

- **WHEN** a qualifying governance change declares a behavior-neutral impact,
  cites no ledger ID, and provides a substantive bounded rationale
- **THEN** the canonical ledger remains empty and generated evidence reports no
  consumer-visible change
