# quality-evidence-declarations Specification

## Purpose
TBD - created by archiving change establish-quality-evidence-routing. Update Purpose after archive.
## Requirements
### Requirement: Every capability declares all four quality classes

Each active quality declaration SHALL contain exactly one property, fuzz,
benchmark, and differential obligation. Every obligation SHALL be explicitly
`satisfied`, `required`, or `not-applicable`; omission SHALL be invalid.

#### Scenario: a capability omits benchmark evidence

- **WHEN** its declaration contains only property, fuzz, and differential
  obligations
- **THEN** offline validation fails instead of inferring that benchmarking is
  unnecessary

### Requirement: Satisfied evidence is exact and class-specific

A satisfied obligation SHALL name exact safe selectors, a non-executed command,
target, risk, budget, lane/cadence, freshness policy, immutable source or run
receipt, evidence revision/date, owner, and the detail keys required for its
class. A generic CI URL, coverage percentage, or prose claim SHALL NOT satisfy
an obligation.

#### Scenario: a fuzz receipt lacks its corpus and bounds

- **WHEN** the declaration names a successful workflow but omits the corpus,
  seed, sanitizer, or resource limits
- **THEN** validation rejects the incomplete fuzz evidence

### Requirement: Not-applicable outcomes are reviewed risk decisions

A `not-applicable` obligation SHALL contain a bounded risk rationale, named
owner, and reviewing issue. It SHALL NOT contain executable commands,
selectors, targets, receipts, evidence revisions, freshness budgets, or open
debt.

#### Scenario: a parser benchmark has no consumer outcome

- **WHEN** the responsible issue explains that no comparable operation,
  environment, or performance budget exists
- **THEN** benchmark may be recorded as reviewed not applicable without
  inventing a measurement

### Requirement: Evidence debt and freshness fail visibly

Required but unsatisfied evidence SHALL be marked as open debt with a named
owner and positive issue. Satisfied time-bounded evidence SHALL be current at
the declaration evaluation date. Missing, stale, failed, or unowned evidence
SHALL NOT be represented as satisfied.

#### Scenario: a weekly fuzz receipt exceeds its freshness budget

- **WHEN** its completion date is older than the declared maximum at the
  registry evaluation date
- **THEN** validation fails until a current receipt is recorded or the
  obligation becomes explicit owned debt

### Requirement: Quality routes preserve the delivery-line policy

Executable obligations SHALL use a closed deterministic route/cadence pair:
focused/on-change, fast/per-pull-request, slow/weekly-or-manual, or
release/per-candidate. A not-applicable outcome SHALL use
none/not-applicable. The rendered plan SHALL be deterministic and SHALL NOT
execute stored commands or change branch-protection requirements.

#### Scenario: fuzz evidence is expensive but required

- **WHEN** the declaration routes its bounded sanitizer campaign to the slow
  weekly/manual line
- **THEN** the pull request retains the single required fast gate while the
  fuzz requirement remains visible production-promotion evidence

### Requirement: The DID seed reuses stable vector identities

The first declaration SHALL cover generic DID/DID URL syntax and reference
existing #420 vector IDs. Property, fuzz, and differential evidence SHALL bind
to current repository selectors; benchmark SHALL carry the reviewed
not-applicable decision. Vector payload SHALL NOT be copied into the quality
registry.

#### Scenario: a referenced DID vector is renamed or removed

- **WHEN** a declaration points to a vector ID absent from the local canonical
  catalog
- **THEN** offline validation rejects the dangling quality evidence

### Requirement: Stable declaration IDs coordinate later work

Issue and OpenSpec/evidence templates SHALL reference stable declaration IDs
rather than duplicating commands and dispositions. IDs SHALL NOT be silently
repurposed; replacement SHALL be explicit and acyclic. A semantic schema or
registry change SHALL increment its version.

#### Scenario: a later change consumes DID quality evidence

- **WHEN** #422 or a downstream canary records its quality dependency
- **THEN** it references `did.syntax.quality.v1` and independently evaluates
  current promotion evidence
