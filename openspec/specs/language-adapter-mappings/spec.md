# language-adapter-mappings Specification

## Purpose

Define the canonical, versioned compatibility boundary between SDK-Rust
capabilities and temporary language-SDK DTO, error, and lifecycle facades.
## Requirements
### Requirement: Rust contracts remain canonical

Every language-adapter mapping SHALL identify a canonical Rust crate,
module/symbol or public error code and its version origin. A language DTO,
exception, error string, or runtime convention SHALL NOT constrain the Rust
core; compatibility SHALL be implemented by an outer, versioned mapping.

#### Scenario: a legacy language DTO has an extra derived field

- **WHEN** the field can be calculated from the canonical Rust value
- **THEN** the mapping records it as derived instead of adding it to the Rust
  domain type

### Requirement: Mapping fidelity and unsupported behavior are explicit

Every mapping SHALL declare its direction, compatibility class, and lossless
or lossy fidelity. A lossy mapping SHALL name each lost distinction and define
deterministic unsupported or migration behavior rather than silently normalize
canonical data.

#### Scenario: a legacy DID URL cannot preserve an empty query

- **WHEN** reverse conversion would lose absent-versus-empty information
- **THEN** the mapping marks the case unsupported or routes it to an additive
  replacement shape without changing `identus-did`

### Requirement: Errors preserve stable codes and redaction

Error mappings SHALL retain the canonical stable Rust code, bounds, and
redaction policy even when a legacy language facade uses one exception class
or human message for multiple failures. Human error text SHALL NOT be a stable
compatibility identifier.

#### Scenario: two Rust parse failures map to one legacy class

- **WHEN** `did.invalid_did` and `did.invalid_did_url` both surface as
  `InvalidDIDString`
- **THEN** adapter observability still distinguishes the stable Rust code and
  rejected caller input remains redacted

### Requirement: Compatibility has a bounded lifecycle

Every active mapping SHALL reference a closed canonical-source record and
record its supported version window, deprecation phase, consumers, vector
evidence, exact selectors, migration action, replacement, observability,
fallback, rollback, and removal gate. The source record SHALL carry the source
revision, path, and content identities shared by all mappings using that file.
The window SHALL use an ordered `>=x.y.z,<x.y.z` interval containing the pinned
language version. Stable mapping IDs SHALL only evolve through explicit
versioning or replacement.

#### Scenario: the last supported consumer adopts the canonical shape

- **WHEN** the removal gate's evidence is satisfied
- **THEN** the legacy mapping may enter removal through its declared migration
  and rollback process rather than disappearing silently

### Requirement: Adapter bounds cannot weaken Rust policy

Every mapping SHALL resolve its input ceiling to a named literal public Rust
`usize` constant in the exact bounded bytes of its referenced canonical-source
record. The adapter ceiling MAY be stricter and SHALL NOT exceed that canonical
value.

#### Scenario: an adapter raises its accepted DID size

- **WHEN** the declared adapter ceiling exceeds `MAX_DID_BYTES`
- **THEN** validation fails before the mapping can be accepted

### Requirement: Registry and human view remain deterministic

The machine registry SHALL use a closed versioned schema and fail on unknown or
incoherent metadata. The repository SHALL retain a deterministic human-readable
rendering of every governed field, and validation SHALL fail when it drifts
from the registry. Required seed records SHALL NOT prevent additional
schema-conforming language mappings.

#### Scenario: a registry field changes without regenerating documentation

- **WHEN** the checked-in Markdown no longer equals deterministic rendering
- **THEN** the factory gate rejects the change

### Requirement: Canonical identities remain structurally distinct

Every mapping SHALL represent its Cargo package and canonical Rust API path as
separate fields. Error mappings SHALL represent the public stable error code as
an error identity and SHALL NOT concatenate that code, a Cargo package name, or
a duplicated Rust crate prefix into a synthesized source path.

#### Scenario: a stable DID error is rendered for review

- **WHEN** the mapping exposes `did.invalid_did` through
  `identus_did::Error::to_identus_error`
- **THEN** the human view shows Cargo package, Rust API path, and stable code as
  three distinct values

### Requirement: Field directions cannot exceed their mapping

The validator SHALL enforce a closed compatibility matrix between the mapping
direction and every field direction. A bidirectional mapping MAY contain
one-way or both-way fields; a one-way mapping SHALL contain only fields in that
same direction.

#### Scenario: a forward-only mapping contains a reverse field

- **WHEN** a `rust-to-language` mapping declares a `language-to-rust` or `both`
  field
- **THEN** validation fails with a bounded field-direction diagnostic

### Requirement: Every lossy mapping names its losses

Every mapping SHALL contain a closed loss-record collection. A lossless mapping
SHALL contain none. A lossy value or error mapping SHALL name at least one
unique lost distinction, its compatibility consequence, and required
mitigation. Unsupported-value behavior SHALL remain separate and every
unsupported ID SHALL resolve to a declared loss.

#### Scenario: two canonical errors share one legacy class

- **WHEN** an error mapping uses the legacy `InvalidDIDString` class for a
  canonical stable code
- **THEN** it records class coalescing as a loss and stable-code preservation as
  the mitigation even though the error remains representable

### Requirement: Canonical evidence paths stay inside the repository

Every canonical source path SHALL be repository-relative, free of traversal
and symlink components, resolve strictly inside the repository root, name a
bounded regular file under the declared Cargo package's source tree, and
resolve its named literal public Rust bound exactly once. The canonical vector
catalog SHALL meet the same repository-containment and no-symlink rule. Invalid
evidence SHALL fail with bounded diagnostics rather than an uncaught exception.
The selected bound symbol SHALL also be named by at least one referenced
canonical vector's source locator, preventing an unrelated constant in the same
crate from weakening the mapped API's policy.

#### Scenario: a parent directory redirects to a fake constant

- **WHEN** any component of a canonical bound path is a symlink outside the
  repository
- **THEN** validation rejects the path before reading or accepting its value

#### Scenario: a DID mapping borrows the DID URL ceiling

- **WHEN** the DID mapping selects `MAX_DID_URL_BYTES` even though its vectors
  evidence `MAX_DID_BYTES`
- **THEN** validation rejects the same-crate bound substitution

### Requirement: Mapping records remain unique and type exact

Schema and owner integers SHALL be actual TOML integers rather than booleans or
floats. Rust and language field identities SHALL each be unique within a value
mapping, stable Rust error codes SHALL be unique within an error mapping, and
mapping state SHALL permit only its documented deprecation phases.

#### Scenario: removed state retains a transitional phase

- **WHEN** a mapping declares `state = "removed"` with a non-removed phase
- **THEN** validation fails rather than rendering contradictory lifecycle data

### Requirement: Version windows do not exceed pinned evidence

Every schema version SHALL bind each language mapping to one pinned language
version and revision and to that exact patch interval unless the schema
explicitly adds immutable revisions and differential evidence for a wider
window. A schema-version increase alone SHALL NOT widen compatibility claims.

#### Scenario: SDK-TS evidence is pinned only to 8.1.4

- **WHEN** the seed mapping is validated under schema v2
- **THEN** its interval remains `>=8.1.4,<8.1.5` rather than claiming later 8.x
  or 9.x releases

### Requirement: Mapping vectors resolve through the canonical catalog

Every vector ID referenced by a mapping SHALL resolve exactly once in the
canonical cross-language catalog, match the mapping capability, and include the
mapping language target. A value mapping SHALL reference successful outcomes;
an error mapping SHALL reference outcomes matching one of its stable Rust error
codes. Multiple mappings MAY reference the same catalog vector because the
mapping is a consumer of shared evidence, not its owner.

#### Scenario: an additive Swift mapping reuses DID evidence

- **WHEN** the mapping references an existing DID vector whose targets include
  Swift
- **THEN** validation accepts the shared reference without inventing or
  duplicating a vector ID

### Requirement: Mutation fixtures follow registry evidence

The mutation harness SHALL materialize every unique safe canonical source path
and the canonical vector catalog named by its test registry. Adding a mapping
that uses another valid source file SHALL NOT require a hardcoded fixture-list
edit.

#### Scenario: a new mapping names a second Rust source file

- **WHEN** the positive additive fixture is assembled
- **THEN** the harness copies that exact safe source path and validates the
  registry without relying on the DID source filename

### Requirement: Canonical Rust source identity is content-bound offline

Every adapter mapping SHALL reference one closed canonical-source record. The
record SHALL bind a stable ID, owning issue, lifecycle state, Cargo package,
Git revision, repository path, regular-file mode, byte length, SHA-256 digest,
Git object format, and Git blob object ID. Validation SHALL recompute both
content identities from bounded non-symlink bytes without requiring `.git`, a
network, compiler, or parser. When local Git metadata contains the declared
revision, validation SHALL also require it to be an ancestor of `HEAD`. An
active record SHALL
require the revision/path pair to resolve to the declared blob object ID. A
source-changing squash PR MAY instead use `pending-revision` only when the
prior revision differs and `HEAD` resolves the changed path to the declared
blob; a follow-up SHALL rotate the record to the resulting squash revision and
restore `active`.

PR validation SHALL reject an active record when its source path/blob differs
from the immutable PR base, SHALL require an active revision to be reachable
from that base, and SHALL require a pending record to name that exact base.
Validation on `develop` SHALL accept a pending record only when its revision is
the direct parent of `HEAD`, preventing any later change from landing before
activation. These protected modes SHALL fail if the declared revision object
is unavailable rather than falling back to content-only validation.

#### Scenario: a canonical source file drifts

- **WHEN** source bytes differ from the reviewed manifest while mapping records
  remain unchanged
- **THEN** validation fails on byte length, SHA-256, or Git blob identity before
  the mapping can be used as compatibility evidence

#### Scenario: multiple mappings share one Rust file

- **WHEN** DID value and stable-error mappings bind symbols from the same
  canonical source file
- **THEN** they reference one source ID and a source update rotates provenance
  once rather than duplicating revision and digest maintenance

#### Scenario: validation runs from a Git-less source archive

- **WHEN** the repository files are available without Git object metadata
- **THEN** the same mandatory provenance checks pass or fail using the exact
  source bytes and declared hash algorithms

#### Scenario: source formatting changes

- **WHEN** formatting changes exact source bytes without changing a Rust API
- **THEN** the shared source digest rotates explicitly while mapping semantics
  and stable mapping IDs remain unchanged

#### Scenario: a plausible revision does not contain the declared path

- **WHEN** local Git metadata exists and an active record's revision cannot
  resolve the source path to the recorded blob
- **THEN** validation fails even when the current file bytes match both hashes

#### Scenario: squash merge changes canonical source bytes

- **WHEN** a source-changing PR cannot know its future protected squash commit
- **THEN** it uses the bounded `pending-revision` transition and the immediate
  follow-up activates the squash revision without leaving `develop` invalid
