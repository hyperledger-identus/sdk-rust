# identus-platform-core-migration Specification

## ADDED Requirements

### Requirement: DID methods are owned by cohesive versioned capabilities

The SDK SHALL keep generic DID contracts method-neutral and SHALL package peer
DID, PRISM method, and PRISM VDR/indexing capabilities as independently
selectable release units. Donor component names SHALL remain provenance aliases
only. No generic crate SHALL depend on a method crate.

#### Scenario: peer DID profiles diverge

- **WHEN** current standardization selects numalgo 4 while an existing Identus
  consumer uses numalgo 2
- **THEN** the SDK exposes explicit current and compatibility profiles with
  separate evidence and no silent profile upgrade

#### Scenario: PRISM logic is reusable but effects are not

- **WHEN** deterministic method, VDR, or indexing behavior can be tested
  without a Cardano client, database, HTTP server, CLI, or node runtime
- **THEN** the behavior may be SDK-owned behind lower-layer contracts while
  the concrete effects remain NeoPRISM adapters

#### Scenario: a donor crate mixes change axes

- **WHEN** a NeoPRISM crate combines portable protocol rules with Cardano,
  persistence, or runtime concerns
- **THEN** the SDK adapts the smallest cohesive capability rather than copying
  the donor crate boundary

### Requirement: NeoPRISM reduction follows immutable differential adoption

Duplicate NeoPRISM behavior SHALL remain until a released SDK capability is
pinned and old/new paths process the same network snapshot with correctness and
QoS evidence.

#### Scenario: an SDK implementation passes unit tests

- **WHEN** the SDK crate is locally and host-CI green but no immutable
  NeoPRISM adoption receipt exists
- **THEN** downstream deletion remains unauthorized

#### Scenario: adoption is qualified

- **WHEN** both paths produce the same ordered operations and terminal DID/VDR
  state and record duration, CPU, and peak RAM within accepted thresholds
- **THEN** only the covered duplicate behavior becomes eligible for a separate
  NeoPRISM removal decision
