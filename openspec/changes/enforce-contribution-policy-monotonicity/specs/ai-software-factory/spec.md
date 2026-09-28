## ADDED Requirements

### Requirement: Contribution policy changes are monotonic

The protected-base checker SHALL parse the exact proposed contribution policy
as bounded untrusted data under a closed versioned schema. It SHALL allow an
identical or strictly stronger policy and fail every relaxation before merge.
It SHALL NOT execute code from the proposed tree or accept a self-authorizing
waiver.

#### Scenario: Proposed policy is strictly stronger

- **WHEN** every changed allowance or exemption shrinks, enforcement flag
  strengthens, or numeric ceiling decreases under the versioned partial order
- **THEN** the base-owned monotonicity check succeeds

#### Scenario: Proposed policy relaxes an invariant

- **WHEN** any allowance or exemption expands, enforcement flag weakens, or
  numeric ceiling increases
- **THEN** the check fails and identifies the relaxed field

#### Scenario: Proposed policy is malformed or ambiguous

- **WHEN** the exact head policy is missing, oversized, duplicate-keyed,
  malformed, unsupported, or contains an unknown field
- **THEN** bounded base-owned parsing fails closed without executing head code
