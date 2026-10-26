# ssi-upstream-program Specification

## MODIFIED Requirements

### Requirement: Repository boundaries remain directional

The SDK SHALL own chain-neutral SSI types, ports, protocol engines,
cryptographic utilities, portable DID-method semantics, and conformance
evidence. It SHALL NOT own Midnight or Compact runtime behavior, concrete
PRISM/Cardano ledger integration, wallet/product policy, trust decisions,
consent, custody, UI, deployment or concrete product storage.

Method-specific behavior MAY be SDK-owned when it forms a cohesive,
independently selectable capability that is deterministic, effect-isolated,
bounded, target-qualified, and useful outside one node composition. This does
not make the generic SDK layer depend on that method or move concrete chain,
storage, service, or runtime effects into the portable crate.

The canonical SSI backlog SHALL identify a focused specification owner for
IDR-040 rather than program issue #20. Implementation SHALL proceed through
focused child issues in their recorded dependency order.

#### Scenario: Generic behavior is found in a consumer

- **WHEN** equivalent behavior is useful to at least two independent consumers
- **THEN** its source is classified for a focused SDK slice without adding a
  dependency from the SDK to the consumer

#### Scenario: Source behavior is chain- or product-specific

- **WHEN** behavior depends on Midnight, Compact, a Cardano client, concrete
  PRISM/Cardano ledger operations, product policy, custody, UI, deployment or
  concrete persistence
- **THEN** it is classified `remain-downstream` and excluded from the portable
  SDK dependency graph; a separate accepted ADR may define an independently
  reusable adapter but cannot move the concrete effect into a portable core

#### Scenario: portable method behavior is found in a chain node

- **WHEN** PRISM method, VDR, or deterministic indexing behavior can satisfy
  the hard reuse gates without a Cardano client, database, service, or node
  runtime
- **THEN** it may be adapted into a method-specific SDK crate while concrete
  effects remain downstream

#### Scenario: method planning becomes implementation-ready

- **WHEN** ADR 0174, source pins, profiles, crate boundaries, and child issues
  are accepted
- **THEN** IDR-040 is `specified`, while its production capabilities remain
  undelivered until their individual evidence and adoption receipts pass

#### Scenario: a donor implementation is available

- **WHEN** SDK-TS, NeoPRISM, or a Rust peer library contains candidate behavior
- **THEN** the exact revision is used as compatibility evidence and does not
  become normative architecture or an automatic dependency
