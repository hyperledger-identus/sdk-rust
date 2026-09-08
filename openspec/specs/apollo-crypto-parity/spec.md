# apollo-crypto-parity Specification

## Purpose
TBD - created by archiving change apollo-parity-manifest. Update Purpose after archive.
## Requirements
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

### Requirement: SDK crypto performance remains observable without a parity claim

The repository SHALL provide a stable-Rust, dependency-free diagnostic harness
for representative `identus-crypto` operations. A publishable run SHALL use at
least 20 samples after warm-up and record exact SDK revision, Rust compiler,
platform, CPU description, feature profile, per-operation batch sizes, median
and p95. Inputs SHALL be deterministic and setup SHALL remain outside timed
regions so entropy is not included accidentally.

#### Scenario: Weekly or manual baseline is published

- **WHEN** the slow benchmark job completes
- **THEN** it SHALL upload a machine-readable artifact tied to the exact commit
- **AND** the artifact SHALL identify itself as measurement-only
- **AND** it SHALL state that Apollo comparison is unavailable

#### Scenario: Publication metadata is incomplete

- **WHEN** revision, compiler, platform, CPU, features or the minimum sample
  count is absent
- **THEN** the runner SHALL fail without publishing a final artifact

### Requirement: Performance evidence does not become an implicit threshold

M2 performance evidence SHALL NOT impose a pass/fail latency threshold or make
a cross-machine, cross-language, constant-time, certification or product
latency claim. Any future threshold SHALL require a separate ADR based on
stable runner history.

#### Scenario: Timing changes between valid runs

- **WHEN** a later diagnostic produces different timing values
- **THEN** correctness and CI SHALL remain green unless a separately approved
  threshold policy is active

### Requirement: Portable-target parity closes on one policy-bound receipt

The Apollo parity ledger SHALL record one successful slow-workflow receipt for
the exact SDK closing revision. It SHALL name the Rust toolchain, portable
package and feature surface, support-policy source and exactly the WASM, iOS
and Android compile targets. Each portable target row SHALL match the
support-policy target ID, `compile-checked` tier, named target-specific gate,
packages, features, limitation, common closing revision and Actions run.

#### Scenario: Host-only evidence substitutes for a portable gate

- **WHEN** a portable row names `fast`, a host build or another target's gate
- **THEN** the validator SHALL fail and identify the target/gate mismatch

#### Scenario: Target evidence is stale or unrelated

- **WHEN** a portable row uses another revision, a non-success conclusion or a
  URI other than the declared GitHub Actions closing run
- **THEN** the validator SHALL fail before the parity report can claim closure

#### Scenario: Support policy changes after the closing receipt

- **WHEN** a portable target's toolchain, packages, features, tier, gate or
  limitation differs from the recorded closure
- **THEN** validation SHALL fail until a new exact target receipt is recorded

### Requirement: Human target report exposes evidence and limitations

The deterministic Markdown report SHALL show the closing revision, Rust
toolchain, portable packages, features and slow-run link. Every target row
SHALL show its evidence revision and URI. It SHALL continue to distinguish
compile-only WASM/mobile evidence from runtime, packaging and language-binding
support.

#### Scenario: Reviewer renders the target report

- **WHEN** the reviewer renders the current parity manifest
- **THEN** the output SHALL contain the exact portable closing revision and
  independently openable slow-run link
- **AND** WASM, iOS and Android SHALL remain labeled `compile-checked`
- **AND** language bindings SHALL remain `not-supported`
