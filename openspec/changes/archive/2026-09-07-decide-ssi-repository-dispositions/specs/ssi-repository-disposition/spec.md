## ADDED Requirements

### Requirement: Every SSI repository has an explicit disposition

The SDK SHALL record an ADR for every SSI repository named by the accepted
repository portfolio before broadening its recorded production, source-donor,
fixture, oracle, or reference use. The ADR records an immutable assessed revision and one of
`conditional-adopt`, `spike`, `oracle`, or `not-adopt`. The ADR SHALL define
allowed and prohibited uses, evidence limitations, and an objective activation
or reconsideration trigger.

#### Scenario: Agent begins work from a known SSI repository

- **WHEN** an implementation agent proposes to reuse a repository covered by the portfolio
- **THEN** it consults that repository's ADR and cannot broaden its allowed use without an evidence-backed superseding decision

### Requirement: Conditional adoption remains separately gated

A `conditional-adopt` disposition SHALL authorize only a focused, issue-linked
evaluation or adapter change. Production integration SHALL still prove exact
version/features, license/provenance, normative parity, effective Rust and
target compatibility, resolved dependency cone, reachable unsafe/native code,
security/privacy behavior, public/wire compatibility, and rollback.

#### Scenario: Conditional candidate is proposed as a dependency

- **WHEN** a component issue proposes one narrow crate or optional adapter from a conditional repository
- **THEN** the integration remains blocked until its repository ADR trigger and ADR 0061 adoption evidence are both satisfied

### Requirement: Oracle and not-adopt repositories stay out of release artifacts

Repositories classified as `oracle` or `not-adopt` SHALL NOT enter the
normal/build dependency closure or public API under the recorded decision.
Oracle use MAY include read-only source study, provenance-pinned fixtures, and
dev-only differential tooling when the owning issue records license and source
evidence.

#### Scenario: Differential evidence uses an oracle

- **WHEN** conformance tests compare SDK behavior with an oracle repository
- **THEN** the oracle remains outside production artifacts and normative standards outrank disagreements between implementations

### Requirement: Third-party frameworks do not own SDK public boundaries

Any adopted external implementation SHALL remain behind Identus-owned public
types, ports, errors, lifecycle states, input limits, redaction, and secret
handling. Third-party framework types SHALL NOT cross public Rust, UniFFI,
WASM, persistence, or product-policy boundaries without a separate public-API
ADR.

#### Scenario: Narrow upstream module is adopted

- **WHEN** a focused upstream module implements a standards mechanic
- **THEN** the SDK maps it through its own facade and retains trust, transport, storage, custody, consent, chain, and product policy outside that module

### Requirement: New production or source-donor use requires a durable decision

The SDK SHALL record an issue and ADR before a newly researched SSI repository
enters a production dependency graph or contributes source code. Ordinary
read-only source research SHALL remain permitted through an issue or research
record without first creating an ADR.

#### Scenario: Agent researches a new candidate

- **WHEN** an agent studies a repository that is not yet in the accepted portfolio
- **THEN** read-only evaluation may proceed, while production adoption or source donation waits for a durable disposition

### Requirement: Portfolio evidence is dated and refreshable

Each repository decision SHALL record the retrieval date and immutable revision
and SHALL distinguish upstream claims from SDK-verified evidence. A maintenance,
release, standard, security, packaging, or platform change MAY trigger a new
research issue but SHALL NOT silently alter the accepted disposition.

#### Scenario: Upstream satisfies a reconsideration trigger

- **WHEN** new upstream evidence satisfies the exact trigger in its ADR
- **THEN** an agent may propose a superseding ADR and focused integration issue rather than ignoring or editing the historical decision
