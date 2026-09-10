# reusable-module-extraction Specification

## Purpose
TBD - created by archiving change define-reusable-module-extraction-criteria. Update Purpose after archive.
## Requirements
### Requirement: A reusable module passes every hard architecture gate

Before source or behavior enters sdk-rust as a reusable module, its issue SHALL
prove all of the following:

- it owns a named standards- or domain-defined capability rather than chain,
  product, custody, consent, trust, UI, deployment, or concrete runtime policy;
- it has one primary responsibility and dominant reason to change, is
  independently useful and testable, and does not become a generic helper or
  umbrella package;
- it depends only downward on lower generic SDK layers, minimal ports, and
  separately accepted narrow engines, without a cycle or consumer dependency;
- Identus-owned types, stable redacted errors, lifecycle states, input limits,
  and secret ownership define its public boundary;
- effectful network, filesystem, database, executor, clock, and entropy
  behavior is injected through minimal core ports or isolated in separately
  selectable adapter crates, while chain and product policy remains downstream;
- minimal and enabled feature dependency cones, supported targets, reachable
  unsafe/native code, and portability limitations are explicit;
- untrusted work and allocation are bounded and parsed, validated, verified,
  and trusted states cannot be silently confused;
- exact donor revision, file history, license, transformation, normative
  source, fixtures, and compatibility evidence are recorded; and
- independent consumer evidence and independently reversible delivery are
  defined.

A failed hard gate SHALL NOT be offset by a score or unrelated strength.

#### Scenario: Small utility appears generic

- **WHEN** a donor helper has no named SDK capability, independent change axis,
  or second credible consumer use
- **THEN** its small size and generic Rust syntax do not qualify it for
  extraction

#### Scenario: Public API exposes a donor type

- **WHEN** an otherwise useful candidate requires a NeoPRISM or external
  framework type in its public signature
- **THEN** direct extraction fails and the candidate must be adapted behind an
  Identus-owned facade or assigned another disposition

### Requirement: Every candidate receives one finite source disposition

The assessment SHALL assign exactly one current disposition:

- `extract` for a coherent generic Rust source unit that passes every hard
  gate;
- `adapt` for reusable behavior whose source API, dependencies, effects, or
  states must be redesigned at the SDK boundary;
- `conformance-only` when behavior or fixtures are valuable but source and
  public model are not reusable;
- `remain-downstream` for chain, ledger, node, runtime, concrete storage,
  product, custody, consent, trust, UI, or deployment ownership; or
- `reject` when the candidate lacks a cohesive SDK responsibility or fails
  license, provenance, security, maintenance, dependency, or consumer-value
  requirements.

The assessment SHALL distinguish this source disposition from the delivery
action when sdk-rust already implements the capability.

#### Scenario: SDK already owns equivalent behavior

- **WHEN** a NeoPRISM source is classified `extract` but a compatible SDK
  capability already exists
- **THEN** the next action is compatibility comparison and downstream adoption,
  not creation of a duplicate SDK module

#### Scenario: Chain-specific package contains a generic-looking helper

- **WHEN** extracting the helper would create an unowned utility package with
  no independent consumer contract
- **THEN** the package remains downstream and the helper is rejected as a
  standalone SDK module

### Requirement: Reusable boundaries are orthogonal and cohesive

The extraction unit SHALL be the smallest independently useful capability that
can be selected without pulling unrelated behavior. Capabilities that share a
public invariant and always change, test, and release together MAY remain one
crate or module. A proposed split SHALL demonstrate reduced coupling rather
than merely increasing package count.

#### Scenario: Optional adapter has a separate change axis

- **WHEN** an HTTP or platform adapter depends on a runtime not required by the
  deterministic domain core
- **THEN** the adapter is isolated behind an opt-in crate or feature and the
  minimal core graph excludes that runtime

#### Scenario: Two types share one invariant

- **WHEN** separating related types would duplicate validation and force
  lockstep releases
- **THEN** they may remain in one cohesive capability despite being distinct
  source modules

### Requirement: Reusability has independent consumer evidence

A reusable public component SHALL demonstrate two independent consumer-shaped
uses through its public minimal feature surface. A foundational standards
primitive MAY instead record a reviewed exception naming two credible usage
paths and explaining why production adoption cannot precede the primitive.
SDK-local examples or conformance adapters SHALL be described as evidence, not
as downstream adoption.

#### Scenario: Only NeoPRISM uses the candidate today

- **WHEN** the candidate is a foundational standards primitive and a second
  production consumer has not yet adopted it
- **THEN** extraction proceeds only with a recorded exception, two credible
  usage paths, all other hard gates, and no claim of completed consumer adoption

#### Scenario: Consumer proof needs unrelated default features

- **WHEN** an example cannot use the capability without enabling an unrelated
  protocol, runtime, chain, or storage feature
- **THEN** the orthogonality gate fails until the dependency boundary is
  corrected

### Requirement: SDK delivery precedes downstream code removal

Each missing generic capability SHALL be delivered through its own sdk-rust
issue, OpenSpec contract, tests, review, and immutable merged revision. A
separately authorized NeoPRISM issue SHALL then pin that revision, adapt one
surface, run historical fixtures and target/dependency evidence, and record
rollback. NeoPRISM SHALL NOT remove its compatible local implementation before
that downstream evidence passes.

#### Scenario: SDK implementation is still an active branch

- **WHEN** NeoPRISM can reference only an unmerged SDK branch, moving tag, or
  local path
- **THEN** downstream deletion is blocked and the compatibility facade remains
  reversible

#### Scenario: One immutable SDK candidate passes NeoPRISM evidence

- **WHEN** the pinned SDK revision passes the mapped historical vectors,
  relevant workspace tests, feature/dependency comparison, and supported target
  checks
- **THEN** the authorized NeoPRISM slice may remove only the now-duplicated
  implementation covered by that evidence

### Requirement: Extraction preserves provenance and behavior without copying policy

Every adapted source file or fixture SHALL record its donor repository, exact
revision, path, file history and license, plus the transformation and
normative-source relationship. Normative standards SHALL outrank donor
behavior. Differential tests SHALL preserve intended compatibility while
rejecting donor behavior that violates SDK bounds, redaction, trust states, or
ownership rules.

#### Scenario: Donor fixture conflicts with an SDK security boundary

- **WHEN** a historical NeoPRISM fixture expects unbounded input, secret
  exposure, an ambiguous trust state, or chain policy inside a generic type
- **THEN** the SDK does not reproduce that behavior silently and records the
  compatibility decision and migration evidence

#### Scenario: Source license is known only at repository root

- **WHEN** file history or copied fixture provenance has not been checked at the
  exact revision
- **THEN** source or fixture extraction remains blocked even if the repository
  root declares Apache-2.0
