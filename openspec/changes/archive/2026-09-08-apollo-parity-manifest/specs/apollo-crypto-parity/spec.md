## ADDED Requirements

### Requirement: Apollo crypto parity has one executable evidence ledger

The repository SHALL maintain one machine-readable manifest for the pinned
Apollo cryptography comparison. It SHALL identify immutable Apollo and
sdk-rust baselines, the delivery issue, parent milestone issue, human report,
assessment date, green baseline CI receipt, audited capabilities and mapped
vector/evidence suites.

#### Scenario: Canonical manifest is complete

- **WHEN** the factory validates the repository
- **THEN** every capability audited for the pinned Apollo baseline SHALL occur
  exactly once with an allowed disposition
- **AND** summary counts SHALL equal the parsed capability rows

#### Scenario: Baselines and evidence are immutable and self-consistent

- **WHEN** a capability or vector points to source, test, delivery or CI
  evidence
- **THEN** GitHub source links SHALL contain the corresponding declared full
  revision and path
- **AND** parity evidence SHALL use the exact declared green sdk-rust CI
  receipt

### Requirement: Parity dispositions fail closed on missing evidence

Each capability SHALL use exactly `parity`, `sdk-exceeds`,
`accepted-difference` or `gap`. `parity` and `sdk-exceeds` SHALL name local
executable vector/evidence mappings, immutable SDK tests and delivery receipts.
`accepted-difference` SHALL state rationale, consumer impact, tracking issue
and an objective reopen trigger. `gap` SHALL name a tracking issue and SHALL
block an M2-closing zero-gap summary.

#### Scenario: Unsupported status is rejected

- **WHEN** a row uses another status or omits required disposition evidence
- **THEN** the validator SHALL fail with the row ID and reason

#### Scenario: Accepted difference remains actionable

- **WHEN** an Apollo convenience, platform or public helper is intentionally
  not mirrored
- **THEN** its row SHALL explain current consumer impact and the evidence that
  would reopen the decision

### Requirement: Vector mappings point to executable local selectors

Every vector/evidence entry SHALL record provenance kind, source repository,
full revision, path, license, transformation, expected result, immutable source
URI, safe repository-relative SDK test path, one or more selectors and an
immutable SDK test URI. Capability vector IDs SHALL resolve to these entries.

#### Scenario: Local test evidence drifts

- **WHEN** a referenced test file is missing, leaves the repository, or no
  longer contains a declared selector
- **THEN** factory validation SHALL fail before accepting the parity claim

#### Scenario: Capability references an unknown vector

- **WHEN** a capability names an absent vector ID
- **THEN** factory validation SHALL fail and identify the capability/vector

### Requirement: Human parity report is rendered from the ledger

The validator SHALL offer a deterministic Markdown rendering containing the
declared baselines, computed disposition summary, capability table and vector
table. Discussion #178 SHALL receive that generated content as the human view;
the Discussion SHALL NOT replace the versioned manifest as source of truth.

#### Scenario: Reviewer renders the current report

- **WHEN** the reviewer runs the documented render command twice without
  changing the manifest
- **THEN** both outputs SHALL be byte-identical and contain every capability
  and vector exactly once
