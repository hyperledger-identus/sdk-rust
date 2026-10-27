# did-prism Specification

## ADDED Requirements

### Requirement: Bounded PRISM identifier profile

The SDK SHALL provide immutable canonical and long-form `did:prism` values.
The canonical suffix SHALL be a lowercase 64-hex SHA-256 digest. A long form
SHALL contain unpadded base64url initial state whose decoded bytes are a valid
bounded, projectable create operation and whose digest equals the canonical
suffix.

#### Scenario: local long-form identity is self-certifying

- **WHEN** valid initial-state bytes are encoded into a long-form identifier
- **THEN** parsing SHALL verify its digest and retain exact operation bytes

#### Scenario: unrelated or oversized state is rejected

- **WHEN** state exceeds its operation budget, is not a create operation, has malformed base64url or does not match the suffix
- **THEN** parsing SHALL fail with a redacted method error before unbounded allocation

### Requirement: Exact bounded protobuf envelope

The method SHALL accept at most 96,000 raw operation bytes and 131,072 signed
envelope bytes. It SHALL reject malformed protobuf and duplicate
security-relevant singular or operation-oneof fields before typed decoding.
Unknown fields SHALL remain bounded and exact inbound operation bytes SHALL be
the only source for signature verification and operation hashes.

#### Scenario: unknown fields do not change signed bytes

- **WHEN** a valid envelope includes a bounded unknown protobuf field
- **THEN** the exact nested operation bytes SHALL remain available and SHALL NOT be reconstructed by re-encoding

#### Scenario: duplicate singular fields fail closed

- **WHEN** an envelope repeats `signed_with`, `signature`, `operation` or a recognized operation kind
- **THEN** parsing SHALL reject it before signature or state processing

### Requirement: Validated PRISM create operation

A create operation SHALL contain at most 50 uniquely identified keys and 50
uniquely identified services, at least one secp256k1 master key, supported key
usages/curves, bounded service values and at most 127 unique exact-text
absolute context URIs other than the SDK-added DID Core context. Validated URI
values SHALL retain their signed wire representation. Master, revocation and VDR keys SHALL remain protocol
state but SHALL NOT appear in a projected DID document.

#### Scenario: maximum finite initial state is portable

- **WHEN** a create operation satisfies every field, cardinality, character, byte and aggregate wire limit
- **THEN** native and WASM parsing SHALL produce the same typed operation and canonical hash

#### Scenario: omitted or duplicate required data is rejected

- **WHEN** required create data, a key representation or master key is absent, or IDs/contexts are duplicated
- **THEN** construction and parsing SHALL fail without retaining caller values in errors

### Requirement: Ordered atomic PRISM state updates

Update actions SHALL execute in signed wire order against a cloned candidate.
Any failed action or final invariant SHALL discard the complete candidate.
Identifiers SHALL remain reserved after removal using at most 4,096 compact
retired identifiers per kind. Retired full values SHALL be discarded. The
50-key and 50-service limits SHALL count active entries only, and a successful
update SHALL retain at least one active master key.

#### Scenario: retired identifier cannot be reused

- **WHEN** an update removes a key or service and a later action adds the same identifier
- **THEN** the complete later update SHALL fail and leave state unchanged

#### Scenario: historical churn does not consume active capacity

- **WHEN** more than 50 and no more than 4,096 lifetime identifiers exist but no more than 50 are active
- **THEN** the state SHALL remain valid while retaining all tombstones

#### Scenario: retained history exceeds the portable profile

- **WHEN** a valid non-terminal operation would retire more than 4,096 key or service identifiers
- **THEN** reduction SHALL return a distinct resource-limit result that history adapters treat as a resolution failure rather than an invalid operation to skip

#### Scenario: terminal deactivation ignores retired-history capacity

- **WHEN** a valid deactivation extends state whose retired-identifier budget is full
- **THEN** the state SHALL deactivate and discard active values and obsolete tombstones

#### Scenario: one failed action rolls back earlier actions

- **WHEN** an earlier action succeeds and a later action in the same update fails
- **THEN** keys, services, contexts and last-operation hash SHALL equal the pre-update state

### Requirement: Deterministic DID document projection

Active state SHALL project to `identus-did` values in deterministic insertion
order. Supported key usages SHALL map to their DID Core verification
relationships and public JWKs. Services and contexts SHALL use generic bounded
domain types, and aggregate document limits SHALL be enforced before candidate
state is accepted. Deactivated state SHALL return a standards-shaped result and
method metadata without active document content.

#### Scenario: repeated projection is stable

- **WHEN** the same state is projected repeatedly on native and WASM targets
- **THEN** the ordered domain values and canonical JSON representation SHALL be identical

#### Scenario: protocol-only keys are not disclosed as document methods

- **WHEN** active state includes master, revocation or VDR keys
- **THEN** those keys SHALL participate in protocol validation but SHALL NOT be emitted as verification methods or relationships

### Requirement: Runtime-neutral PRISM method integration

The crate SHALL provide a local long-form `DidResolver` and a `prism`
`DidMethodBinding`. Local initial-state resolution SHALL not call a network or
clock. Published short-form history lookup SHALL be supplied through a
chain-neutral port and SHALL not introduce Cardano or persistence types.

#### Scenario: long-form resolution is local

- **WHEN** a valid unpublished long-form DID is resolved
- **THEN** its initial state SHALL be validated and projected without transport, storage or ledger dependencies

#### Scenario: short-form history is absent

- **WHEN** a canonical short DID is resolved without a history provider
- **THEN** the resolver SHALL return a standards-shaped not-found or unsupported outcome rather than inventing network behavior
