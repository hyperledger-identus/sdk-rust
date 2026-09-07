# dependency-research-readiness Specification

## Purpose

Require proportionate, evidence-backed build-versus-adopt research before an
agent implements an SDK change, while keeping dependency and MSRV decisions
cohesive, reversible and explicit.

## Requirements
### Requirement: Qualifying changes record research before implementation

Every active OpenSpec change SHALL contain a research record that identifies
the current implementation, normative sources, candidate decisions,
compatibility and dependency evidence, security/privacy/maintenance evidence,
rejected or deferred alternatives, open blockers and commands actually run.
The record SHALL use a full assessment for foundational, protocol,
cryptography/security, storage and FFI changes and MAY use justified
not-applicable entries for routine work.

#### Scenario: Foundational change becomes research-ready

- **WHEN** an agent marks a foundational change ready for implementation
- **THEN** the factory verifies every full-assessment evidence category, at least one explicit candidate disposition and `Research blockers: none`

#### Scenario: Routine change has no adoption decision

- **WHEN** a routine change has no external standard, implementation or dependency choice
- **THEN** its research record may use `not-applicable` with concise supporting evidence and can pass the same gate

### Requirement: Implementation has a distinct research readiness gate

The factory SHALL provide a pre-implementation command that validates the
selected OpenSpec change and requires its research status to be ready with no
declared blocker. Structural draft checks SHALL accept a draft status so
research can be iterated, while implementation adapters SHALL stop before code
changes if the readiness command fails.

#### Scenario: Draft research is checked during planning

- **WHEN** an active change contains a structurally complete research record with `Research status: draft`
- **THEN** the structural factory check accepts the draft record and the selected research-readiness command rejects implementation

#### Scenario: Research-ready record passes

- **WHEN** the selected record has valid metadata, required evidence and no declared blockers
- **THEN** `scripts/factory research-ready <change>` exits successfully before implementation begins

### Requirement: Dependency decisions preserve SDK cohesion

An adopted dependency SHALL remain private behind an Identus-owned facade
unless a separate public-API decision explicitly proves re-export value. Its
integration issue SHALL record exact version/features, license/provenance,
MSRV and target evidence, resolved dependency cone, reachable unsafe/native
code, normative parity, public/wire compatibility, rollback and non-goals.

#### Scenario: Focused crate is approved for adoption

- **WHEN** research concludes that a focused crate reduces total correctness and maintenance risk
- **THEN** a linked issue defines an independently reversible integration without exposing dependency types or importing product, chain, transport, custody, storage or trust policy

### Requirement: Negative dependency decisions remain actionable

The repository SHALL maintain dependencies that are not adopted with the
assessed version/revision, finite reason codes, current alternative and an
objective reconsideration trigger. A future production issue SHALL update the
decision record when that trigger is satisfied rather than ignoring the prior
evidence.

#### Scenario: Rejected draft-era implementation becomes current

- **WHEN** a future release claims the SDK's pinned final standard and passes the recorded trigger
- **THEN** an agent may open a new research issue that updates the negative decision before proposing production integration

### Requirement: MSRV selection is measurable and independently gated

The SDK SHALL choose MSRV through a documented release-distance policy rather
than an arithmetic average of incomplete crate metadata. An MSRV change SHALL
occur only in a focused PR that updates Cargo, Nix, machine policy and target
evidence together; an accepted ADR without that implementation SHALL NOT
change the effective compiler promise.

#### Scenario: Rolling MSRV ADR precedes implementation

- **WHEN** ADR 0062 accepts stable-minus-three and identifies Rust 1.95 as the initial target
- **THEN** repository policy continues to advertise and gate Rust 1.85 until the focused MSRV implementation issue is merged
