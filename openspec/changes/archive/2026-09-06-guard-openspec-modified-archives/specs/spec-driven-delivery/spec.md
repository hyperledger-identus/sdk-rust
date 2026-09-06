## MODIFIED Requirements

### Requirement: Verified changes preserve their specification history

A verified change SHALL sync its reviewed delta requirements into current
capability specs and SHALL be archived with its proposal, design and completed
tasks before the delivery is finalized.

Before archive, every active `MODIFIED` requirement SHALL either retain every
nonblank normalized line of its exact canonical requirement in order or carry
an `archive-intent.toml` acknowledgement that identifies the capability and
requirement, binds the exact normalized canonical block SHA-256, and gives a
nonempty replacement rationale. Missing, malformed, stale, duplicate or unused
acknowledgements SHALL fail closed. Rename plus modification SHALL compare with
the canonical source requirement according to OpenSpec's rename-first order.

#### Scenario: Verified change is finalized

- **WHEN** every change task and verification gate passes
- **THEN** its reviewed capability requirements exist under `openspec/specs/`
  and the completed change exists under `openspec/changes/archive/`

#### Scenario: Partial modified requirement would lose canonical content

- **WHEN** an active `MODIFIED` block omits or rewrites a canonical nonblank
  line without an exact reasoned acknowledgement
- **THEN** the factory check exits non-zero before the change can be archived
  and names the capability and requirement without printing their bodies

#### Scenario: Complete additive modified requirement is checked

- **WHEN** a modified block retains every canonical nonblank line in order and
  adds reviewed prose or scenarios
- **THEN** the preservation preflight succeeds without intent metadata

#### Scenario: Intentional replacement is acknowledged

- **WHEN** a rewrite has one matching intent entry with the exact canonical
  block hash and a nonempty rationale
- **THEN** the preservation preflight permits archive while the sidecar remains
  in the archived change as review evidence

#### Scenario: Replacement acknowledgement is stale or ambiguous

- **WHEN** an intent entry has the wrong canonical hash, is malformed,
  duplicated, or does not correspond to a destructive modified block
- **THEN** the preservation preflight fails closed and identifies the invalid
  entry

#### Scenario: Renamed requirement is also modified

- **WHEN** a delta renames an existing requirement and modifies the new header
- **THEN** the preservation preflight compares against the renamed canonical
  source before deciding whether intent metadata is required

#### Scenario: New capability is added

- **WHEN** a delta adds requirements to a capability with no canonical spec
- **THEN** the preservation checker accepts the absence of modified blocks and
  leaves new-capability structure to strict OpenSpec validation
