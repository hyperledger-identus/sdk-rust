## ADDED Requirements

### Requirement: Result validation preserves phase ownership and first-error precedence

Registration result-state validation MUST preserve the exact job-method,
state/job-shape, lifecycle-variant, document-metadata, and extension-policy
phase order and first error behind the unchanged public constructor.

#### Scenario: A result represents lifecycle progress

- **WHEN** a registration result is constructed
- **THEN** job-method correlation is checked before exhaustive lifecycle state
  validation, followed by document metadata and bounded extension policy
- **AND** terminal handles and public documents, action/wait continuation, and
  advisory wait retain their existing order and errors
- **AND** successful construction retains the caller's exact values without a
  new allocation class.

#### Scenario: Multiple result invariants fail

- **WHEN** one result violates invariants from two or more validation phases
- **THEN** the same existing first `RegistrationError` is returned
- **AND** no private predicate detail or caller-controlled value is exposed.
