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

### Requirement: Compiler-floor selection is phase-appropriate and measurable

The SDK SHALL select its compiler floor from measured dependency value,
supported consumer constraints, target evidence and delivery phase rather than
an arithmetic average or release-distance formula. During the temporary
unpublished active-development phase authorized by discussion #172, the
workspace floor, primary compiler and compatibility etalon SHALL all be exact
Rust 1.98.1 and no lower-version compatibility SHALL be claimed. Dependency
research SHALL still record each candidate's declared and observed compiler
requirements so a later release decision has evidence.

A release candidate SHALL require a focused compatibility decision that uses
named consumer and target evidence, updates Cargo, Nix, machine policy,
migration guidance and CI together, and resolves all weekly slow-lane failures.
A boundary adapter MAY declare a different crate-local floor only through a
separate material decision and SHALL NOT change generic core automatically.

#### Scenario: Dependency requires a recent compiler during active development

- **WHEN** a cohesive dependency requires Rust no newer than the exact 1.98.1
  workspace floor and passes all other adoption gates
- **THEN** no artificial lower-MSRV lane blocks research or implementation

#### Scenario: Release candidate preparation begins

- **WHEN** the project proposes a release candidate or reaches 2026-12-08
- **THEN** a focused decision selects and enforces a consumer-driven compiler
  matrix before any artifact can be published

#### Scenario: Boundary adapter needs a distinct compiler constraint

- **WHEN** an accepted FFI or platform adapter cannot share the workspace floor
  for a measured reason
- **THEN** its focused ADR may define a crate-local constraint without silently
  changing the generic core promise

### Requirement: Production dependencies require concrete consumer payoff

A technically suitable crate SHALL NOT become a production dependency solely
because it passes compiler, license, maintenance, security, target, feature and
dependency-cone gates. Its adoption decision SHALL identify a current SDK
capability or named consumer, the normative behavior it requires, and the local
implementation, demonstrated correctness risk or material maintenance burden
the crate replaces. Without that evidence, the candidate SHALL remain
conditional or deferred and SHALL NOT be added to Cargo or used to expand a
public SDK type.

#### Scenario: Cohesive crate has no current consumer

- **WHEN** a narrow standards crate passes every technical adoption gate but no current SDK capability consumes its semantics
- **THEN** the decision records exact reusable evidence and an objective activation trigger without adding the dependency or inventing public policy

#### Scenario: A named consumer activates reconsideration

- **WHEN** a focused issue pins a normative profile that needs the candidate and defines its policy, resource, compatibility and migration boundaries
- **THEN** research refreshes the evidence and may propose an independently reversible private integration behind Identus-owned types

### Requirement: Parser replacement preserves fail-closed behavior

A candidate parser SHALL NOT replace an SDK parser based only on nominal format
support or valid-input parity. Dependency research SHALL compare every
capability-relevant rejection class, including malformed syntax, invalid text
encoding, duplicates, incomplete or trailing input and resource ceilings. If
the candidate accepts, repairs, ignores or lossily transforms an input the
current specification rejects, adoption SHALL remain deferred or rejected
unless a separate reviewed specification change intentionally changes that
boundary.

#### Scenario: Browser parser is more permissive than protocol boundary

- **WHEN** a candidate browser-compatible parser preserves malformed escapes or replaces invalid text that the SDK rejects
- **THEN** an agent records the semantic mismatch and cannot adopt it by wrapping the result in an SDK-owned type

#### Scenario: Candidate and strict facade compose without weaker behavior

- **WHEN** differential evidence proves all specified valid and invalid classes plus resource limits remain equivalent or stronger
- **THEN** the parser may proceed through the remaining consumer-payoff, dependency, compatibility and security gates
