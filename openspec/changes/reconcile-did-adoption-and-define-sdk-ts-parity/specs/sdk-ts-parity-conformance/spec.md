## ADDED Requirements

### Requirement: Every SDK-TS capability has an explicit parity relation

The program SHALL classify every normalized SDK-TS capability as exactly one
of `semantic-equivalent`, `compatible-adapter`, `platform-owned`,
`intentionally-different`, `deferred`, or `retired`. The relation SHALL state
the target responsibility boundary and SHALL NOT imply delivery.

#### Scenario: TypeScript retains platform behavior

- **WHEN** a capability owns browser, Node, npm, plugin-host, lifecycle,
  permission, UI, or consent integration
- **THEN** it may be `platform-owned` while its portable port and security
  boundary remain explicit, without requiring a duplicate Rust implementation

#### Scenario: Rust follows a newer selected profile

- **WHEN** SDK-Rust intentionally differs from SDK-TS because of a newer
  standard, safer contract, or accepted Identus profile
- **THEN** the capability is `intentionally-different` and records the exact
  deviation, migration, compatibility, and rollback behavior

### Requirement: Conformance state is independent from parity relation

Every capability SHALL record one of `delivered`, `partial`, `blocked`,
`excluded`, or `not-applicable` plus its exact evidence and missing gates. A
report SHALL group records by relation and state and SHALL NOT publish one
repository-wide parity percentage.

#### Scenario: a Rust implementation exists without SDK-TS adoption

- **WHEN** SDK-Rust has implementation and conformance evidence but SDK-TS has
  not routed the capability through the binding or compatibility adapter
- **THEN** the record remains `partial` and names the missing canary, consumer,
  observability, and rollback evidence

#### Scenario: parity is not meaningful

- **WHEN** a capability is accepted as platform-owned
- **THEN** semantic parity may be `not-applicable`, but the host-port,
  security/resource, ownership, and no-duplication evidence remains required

### Requirement: TS1 progresses capability by capability

The TS1 milestone SHALL depend on the shared A1 evidence foundation and SHALL
allow each capability to advance only after its own Rust, binding, profile,
target, and consumer prerequisites are satisfied. It SHALL NOT wait for all
SDK-Rust features or reproduce historical language-SDK modules.

#### Scenario: the first canary is selected

- **WHEN** DID and DID URL syntax have stable Rust, vector, mapping, WASM,
  package, and rollback evidence
- **THEN** issue #492 may deliver the opt-in SDK-TS facade without claiming
  parity for DID documents, methods, credentials, messaging, storage, or agent
  orchestration

### Requirement: Parity evidence is deterministic and reviewable

The parity record SHALL cover every normalized inventory capability exactly
once and SHALL retain immutable source identities, profile/authority,
conformance dimensions, known deviations, evidence links, owner issue, next
action, consumer proof, rollback, and non-claims. Offline validation SHALL fail
on missing, contradictory, unknown, duplicated, or silently divergent records.

#### Scenario: one capability is omitted or overstated

- **WHEN** a record is missing, claims delivery without complete required
  dimensions, marks a non-platform capability `not-applicable`, or declares an
  intentional difference without a deviation
- **THEN** validation fails before merge

### Requirement: Implementation uses local-first capability trains

TS1 implementation SHALL be delivered as cohesive capability trains normally targeting
5,000-10,000 meaningful changed lines. A train SHALL include the implementation
and its applicable adapter, vector, migration, documentation, and test evidence
under one rollback boundary. Generated, vendored, lockfile, and evidence-only
churn SHALL NOT count toward the target, and unrelated work SHALL NOT be added
to reach it.

#### Scenario: a capability train is assembled

- **WHEN** implementation advances through signed internal checkpoints
- **THEN** targeted local checks run at those checkpoints, the complete
  applicable local gate passes before the candidate push, and protected hosted
  CI provides final confirmation rather than the primary discovery loop

#### Scenario: one train cannot remain a coherent review unit

- **WHEN** work has independent ownership, security or release authority,
  rollback, or cannot be reviewed safely as one change
- **THEN** it is split even when either resulting PR is outside the preferred
  changed-line range
