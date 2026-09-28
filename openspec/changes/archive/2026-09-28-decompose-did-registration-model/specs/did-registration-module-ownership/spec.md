# DID registration module ownership

## ADDED Requirements

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
