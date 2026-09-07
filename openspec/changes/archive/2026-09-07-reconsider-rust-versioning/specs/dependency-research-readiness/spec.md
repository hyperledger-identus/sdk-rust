## MODIFIED Requirements

### Requirement: MSRV selection is measurable and independently gated

The SDK SHALL select its public MSRV from measured dependency value, supported
consumer constraints and target evidence rather than an arithmetic average,
calendar, current-stable value or fixed release-distance formula. Edition,
primary validation compiler and forward-compatibility compiler SHALL remain
independent from the MSRV. An MSRV change SHALL occur only in a focused PR that
updates Cargo, Nix, machine policy, migration guidance and target evidence
together; an accepted candidate without that implementation SHALL NOT change
the effective compiler promise. A boundary adapter MAY declare a higher
crate-local MSRV only through a separate material decision and SHALL NOT raise
the generic core automatically.

#### Scenario: Candidate MSRV precedes activation

- **WHEN** research identifies Rust 1.89 as the next useful candidate while the
  repository still declares and gates Rust 1.85
- **THEN** repository policy continues to advertise Rust 1.85 until a focused
  activation issue proves dependency, consumer and supported-target value

#### Scenario: Primary stable compiler advances

- **WHEN** the pinned primary validation compiler advances independently
- **THEN** the public MSRV remains unchanged and every MSRV feature gate still
  runs on the declared floor

#### Scenario: Boundary adapter needs a newer compiler

- **WHEN** an accepted FFI or platform adapter cannot support the workspace
  MSRV for a measured reason
- **THEN** its focused ADR may define a higher crate-local floor without
  changing the core-crate promise
