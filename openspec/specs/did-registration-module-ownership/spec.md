# did-registration-module-ownership Specification

## Purpose
TBD - created by archiving change decompose-did-registration-model. Update Purpose after archive.
## Requirements
### Requirement: Registration responsibilities have cohesive private owners

The SDK MUST keep bounded public extension data, request/lifecycle commands,
result-state validation, and registrar-port behavior in cohesive private owners
behind the unchanged crate-root registration vocabulary.

#### Scenario: Public extension data is validated

- **WHEN** callers construct registration public data from native or JSON input
- **THEN** the existing size, shape, private-member, reserved-member, string,
  node, and depth limits apply in the same order
- **AND** rejected hostile values are cleaned without recursive drop risk.

#### Scenario: A request reaches an adapter

- **WHEN** a create, update, deactivate, continue, or cancel request is built
- **THEN** method, DID/document, action/job, secret mode, collection, and
  idempotency invariants have already been enforced
- **AND** opaque values and debug output remain redacted.

#### Scenario: A result represents lifecycle progress

- **WHEN** a registration result is constructed or matched to a request
- **THEN** terminal/job, continuation, method, DID/document, metadata, handle,
  and advisory-wait invariants remain enforced with the existing errors.

#### Scenario: A registrar executes a request

- **WHEN** an adapter implements `DidRegistrar`
- **THEN** it retains the same object-safe runtime-neutral future contract
- **AND** dropping observation does not imply rollback or cancellation.

### Requirement: Decomposition is compatibility preserving

The SDK MUST preserve existing public paths, signatures, visibility, derives,
constness, limits, error projection, serialization, validation ordering,
cleanup, source distribution, features, dependencies, and target support.

#### Scenario: Consumers rebuild after the decomposition

- **WHEN** existing code imports registration vocabulary from `identus_did`
- **THEN** it compiles without source changes
- **AND** no private child module becomes a supported public path.

#### Scenario: Code-health evidence is refreshed

- **WHEN** the ownership move is complete
- **THEN** the registration hotspot is removed from the governed baseline
- **AND** no new over-threshold descendant or unrelated waiver is introduced.

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
