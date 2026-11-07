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

### Requirement: TS1 maintains a side-by-side feature ledger

The TS1 machine record SHALL cover every normalized SDK-TS capability exactly
once and SHALL record current Rust behavior, intended
TypeScript surface, known deviations, applicable targets, per-target evidence
state, missing-proof dimensions, relation, evidence state, owner, and immutable
evidence. A capability SHALL NOT be globally delivered while any applicable
target remains incomplete. Current SDK-TS behavior SHALL be joined from the
pinned canonical inventory rather than copied into a second machine field. The
human feature table SHALL be checked against those records and SHALL NOT
silently omit blocked, excluded, platform-owned, or intentionally different
capabilities.

#### Scenario: implementation evidence changes

- **WHEN** a capability train adds, removes, supersedes, or activates evidence
- **THEN** the machine record and checked human row change together, preserving
  the distinction between current behavior, target behavior, and missing proof

### Requirement: The Rust-backed TypeScript SDK has target-specific evidence

TS1 SHALL target browser, Node.js, React web, and React Native consumers through
one versioned capability-oriented TypeScript contract over shared portable Rust
crate semantics and target-specific loaders. Identical APIs are not required
for platform-owned or inapplicable capabilities. The facade SHALL NOT expose
the case-insensitive historical names `Apollo`, `Castor`, `Mercury`, `Pluto`,
`Pollux`, or `Edge Agent` as new public exports, package paths, or primary
documentation categories. Private donor-history providers may retain them.
Each target SHALL
prove its own build, load, package, runtime E2E, resource, and rollback behavior
before support is claimed.

#### Scenario: React Native selects a bridge

- **WHEN** React Native integration is implemented
- **THEN** it uses the ADR 0163 dedicated New Architecture/TurboModule package
  over a qualified native binding with separate iOS and Android evidence, and
  browser WASM evidence alone is insufficient

#### Scenario: platform behavior is needed

- **WHEN** a flow needs storage, networking, lifecycle, UI, consent, or custody
- **THEN** TypeScript supplies a thin host adapter through explicit Rust ports;
  it may enforce generated outer allocation bounds and host policy before
  ingress, while Rust revalidates domain invariants and TypeScript does not
  duplicate cryptography, canonical parsing, formats, or protocol state

### Requirement: CI separates core and target verdicts

TS1 CI SHALL run the Rust-core job first. A fail-closed exact-head impact plan
SHALL select browser, Node.js, React web, React Native, package, and binding
jobs when exported API/schema fingerprints, transitive bound crates, relevant
lockfiles, generated mappings, packages, target contracts, or paths change.
Selected jobs SHALL use exact source/lock identity, `fail-fast: false`, and
isolated logs, caches, artifacts, timeouts, and failure domains. Unselected jobs
SHALL emit machine-readable inapplicable receipts without installing their
toolchains. A target failure SHALL NOT cancel, overwrite, or invalidate a
completed Rust-core job result or sibling target result, but SHALL fail the
existing single protected `fast` aggregator, block integration, and block the
affected target's delivery/support claim. Core-only changes SHALL NOT require a
TypeScript toolchain when the impact plan proves every target inapplicable.

Portable core packages and the `core-rust` package set SHALL have no inward
dependency on WASM, native binding, JavaScript, TypeScript, npm, React, or
platform package code. Binding and adapter packages SHALL depend inward on the
portable core only. A public core API, schema, mapping, or generator change
SHALL select every dependent target, and unknown impact SHALL select all target
jobs.

#### Scenario: one TypeScript binding fails

- **WHEN** the Rust-core gate passes and one downstream TypeScript binding or
  runtime target fails
- **THEN** the core result and unaffected target results remain green and
  reviewable, while the failed target remains undelivered and its changed-path
  policy gate remains red

#### Scenario: a Rust-only change is proposed

- **WHEN** impact classification proves that no binding, package, generated
  mapping, or target contract changed
- **THEN** the Rust-core gate is authoritative and CI does not manufacture a
  dependency on browser, Node.js, React, React Native, npm, or TypeScript tools

#### Scenario: a binding package enters the core dependency cone

- **WHEN** dependency validation finds an inward edge from a portable core
  package or core CI package set to binding or platform code
- **THEN** core isolation fails before the Rust-core verdict can pass

### Requirement: Applicable SDK-TS E2E journeys are reused

Every discovered SDK-TS E2E test SHALL be identified by donor revision, path,
suite/test ID, and content hash and SHALL have exactly one of `shared`,
`adapted`, `platform-owned`, `superseded`, or `excluded`, with a reason,
applicable targets, and hermetic/external dependency classification. Discovered
count SHALL equal ledger count; unledgered skip, focus, quarantine, or omission
SHALL fail. A `shared` journey SHALL execute one TypeScript assertion body
against legacy and Rust-backed private provider factories on every applicable
selected target. An `adapted` journey SHALL preserve the consumer outcome while
recording the exact profile or platform-boundary change.

#### Scenario: a donor test is inapplicable

- **WHEN** a test depends on a superseded profile, platform-only behavior, or an
  intentionally retired contract
- **THEN** it is adapted or excluded with a reviewable reason rather than
  silently skipped or counted as a Rust parity failure

### Requirement: Rust owns evidence for additional and deviated behavior

Additional, safer, or intentionally different portable behavior SHALL have
Rust-owned normative, negative, resource-bound, and E2E/conformance evidence,
plus property, fuzz, benchmark, or differential dispositions where applicable.
That evidence SHALL be recorded separately as `rust-addition` or
`rust-deviation`, not as a donor-test disposition.
Selected TypeScript tests SHALL prove binding, DTO, error, async, and host-port
mapping without becoming the normative source for portable semantics.

#### Scenario: Rust follows a newer profile

- **WHEN** Rust deliberately supersedes SDK-TS behavior
- **THEN** Rust proves the selected profile and incompatibility boundaries,
  TypeScript proves the versioned adapter or migration behavior, and the
  feature ledger records the deviation and rollback

### Requirement: TS1 has measurable quality gates

Every delivered capability SHALL map each owned public operation to contract or
E2E exercise and SHALL prove measured coverage for owned changed executable
code, negative/resource-bound behavior, strict TypeScript types, validated Rust
domain invariants, mapping-drift detection, shared-core reuse, accepted target
size/performance budgets, bounded diagnostics, observability, fallback, and
rollback. Each coverage receipt SHALL pin tool/version, base/head,
include/exclude denominator, numerator, target/profile, and artifact. The
default changed-code targets SHALL be at least 80 percent line coverage and 70
percent branch coverage where the selected tooling reports those measures
reliably, with no regression below an established higher package baseline.
Rust surfaces without reliable branch coverage SHALL provide condition,
mutation, negative-selector, and public-operation mapping evidence.
Unsupported measures SHALL NOT be replaced by an invented percentage.

TypeScript SHALL compile with pinned strict flags; owned public declarations
SHALL expose no explicit `any`, SHALL narrow `unknown` at ingress, and SHALL
map numbers/bigints, bytes, null/undefined, errors, lifecycle/free, Promises,
cancellation, and `AbortSignal` deterministically. Drift snapshots and runtime
wire validation SHALL fail closed.

Performance SHALL remain measurement-only until a target has an accepted
budget record. Before support is claimed, the record SHALL pin runner and
protocol, compressed/uncompressed artifact size, cold/warm startup p50/p95,
representative latency/throughput/memory measures, variance, and regression
rules.

Each evidence class SHALL either reference an executable artifact or stable
quality-evidence declaration ID, or carry an owner, rationale, review/expiry
trigger, and reconsideration rule for a bounded not-applicable disposition.

#### Scenario: an adapter duplicates portable semantics

- **WHEN** review or drift tests find portable validation, cryptography, format,
  or protocol-state logic reimplemented in TypeScript
- **THEN** the capability is not delivered until the logic moves behind the
  Rust contract or is explicitly classified as platform-owned

#### Scenario: a public TypeScript type escapes validation

- **WHEN** a new public adapter exports `any`, exposes an unvalidated wire value
  as a domain value, or maps Rust errors through an unversioned string
- **THEN** type-safety and maintainability criteria fail before activation
