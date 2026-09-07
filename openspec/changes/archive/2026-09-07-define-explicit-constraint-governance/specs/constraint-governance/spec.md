## ADDED Requirements

### Requirement: Material SDK constraints and limitations are explicitly indexed

The repository SHALL maintain one machine-readable index of material
cross-cutting constraints and known limitations. Every entry SHALL have a
unique stable identifier, kind, category, lifecycle state, concise summary,
scope, canonical source, authority, consumer impact, enforcement evidence,
owner, review triggers, activation path and rollback path. The index SHALL
reference rather than silently replace detailed canonical policy.

Kinds SHALL be limited to `hard`, `guardrail`, `budget` and `limitation`.
Lifecycle states SHALL be limited to `effective`, `target`, `deferred` and
`prohibited`. A target or deferred entry SHALL NOT be represented as an
effective compatibility promise.

#### Scenario: Maintainer inspects the current MSRV

- **WHEN** the machine-readable index and SDK support policy are validated
- **THEN** the effective MSRV entry equals the support-policy `toolchains.msrv`
  value and any future MSRV is visibly separate with state `target`

#### Scenario: Index entry omits its consequence

- **WHEN** an entry lacks consumer impact, enforcement, ownership, activation,
  rollback or another required field
- **THEN** the offline constraint checker exits non-zero and names the entry
  and missing field

#### Scenario: Entry points to a missing repository source

- **WHEN** an entry's canonical repository path does not exist
- **THEN** the offline constraint checker rejects the index

### Requirement: Materiality determines decision authority without format approvals

A constraint change SHALL be material when it changes an effective
consumer-visible compatibility floor, supported platform or feature,
public/wire/data commitment, product or chain boundary, security/privacy/crypto
posture, license obligation, certification claim, performance/resource budget,
publishing/release promise or irreversible migration. Routine formatting,
task decomposition, implementation detail, test organization and reversible
repository-local tooling SHALL remain under standing agent authority.

An exact recorded sponsor or responsible-maintainer direction SHALL be required
before a new or changed material outcome becomes implementation-ready, unless
that exact outcome is already authorized by an effective indexed constraint or
standing roadmap decision. Research and reversible non-activating preparation
MAY continue while direction is proposed.

#### Scenario: Change introduces a new compiler floor

- **WHEN** an active change proposes to make a higher MSRV effective and no
  exact decision reference authorizes that consumer outcome
- **THEN** structural research MAY continue but constraint readiness fails
  before implementation or integration

#### Scenario: Change contains only reversible implementation detail

- **WHEN** constraint impact is truthfully classified `routine` and no
  consumer-visible limitation or effective promise changes
- **THEN** the agent proceeds under standing authority without requesting a
  formatting or scope ceremony

### Requirement: Every qualifying change declares constraint impact

Every active OpenSpec change SHALL contain `constraints.md` with an impact
class, decision status, decision reference, blocker state and non-empty
sections covering affected entries, introduced or changed constraints,
introduced or changed limitations, consumer/product impact, activation and
rollback, and evidence. Impact SHALL be one of `none`, `routine` or `material`;
decision status SHALL be one of `not-required`, `proposed` or `directed`.

A ready material record SHALL use `directed`, cite an exact durable decision
reference and declare zero blockers. A proposed material record SHALL remain a
valid planning artifact but SHALL fail the pre-implementation and final
readiness gates.

#### Scenario: Active change omits constraint impact

- **WHEN** a qualifying OpenSpec change has no `constraints.md`
- **THEN** the factory structural check fails and names the change

#### Scenario: Material choice is still proposed

- **WHEN** `constraints.md` declares material impact with decision status
  `proposed`
- **THEN** ordinary structural validation passes but
  `factory constraints-ready`, `research-ready`, `ready` and `receipt` fail

#### Scenario: Directed material choice is complete

- **WHEN** all required sections are substantive, blockers are `none`, status
  is `directed` and the exact decision reference is present
- **THEN** constraint readiness passes without requiring a separate document
  format approval

### Requirement: Constraint activation, exceptions and removal are explicit

An effective material constraint SHALL NOT be silently weakened, strengthened,
removed or activated from target/deferred state. The deciding issue and ADR
SHALL state affected consumers, migration, enforcement change, activation
event and rollback. Exceptions SHALL name the constrained entry, scope, owner,
rationale, expiry trigger and security/maintenance cost; they SHALL NOT mutate
the base rule invisibly.

#### Scenario: Target becomes effective

- **WHEN** a target constraint is selected for activation
- **THEN** a focused issue, decision record, change-level material impact and
  updated enforcement evidence are required before the effective source changes

#### Scenario: Consumer needs a temporary exception

- **WHEN** evidence proves a supported consumer cannot meet an effective
  constraint for a bounded period
- **THEN** the exception records its exact scope, owner, exit trigger and cost
  while the original constraint remains visible

### Requirement: Pull requests expose constraint and limitation impact

The pull-request contract SHALL state constraint impact and limitations in
addition to issue, review and compatibility evidence. PR policy SHALL reject a
missing or unchanged-template declaration. Semantic review SHALL verify that
the declaration agrees with the spec, design, implementation and indexed
constraints; structural success SHALL NOT be represented as semantic approval.

#### Scenario: Pull request leaves constraint fields at template defaults

- **WHEN** the pull-request body omits constraint impact or leaves the
  limitations placeholder unchanged
- **THEN** the pull-request policy check fails before integration

#### Scenario: Pull request declares no impact but changes a material promise

- **WHEN** the machine-shaped PR fields pass but semantic review finds an
  effective compatibility or product limitation change
- **THEN** the finding blocks integration until the declaration and authority
  record are corrected
