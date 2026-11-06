# language-adapter-mappings Specification

## ADDED Requirements

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

## MODIFIED Requirements

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

### Requirement: Version windows do not exceed pinned evidence

Every schema version SHALL bind each language mapping to one pinned language
version and revision and to that exact patch interval unless the schema
explicitly adds immutable revisions and differential evidence for a wider
window. A schema-version increase alone SHALL NOT widen compatibility claims.

#### Scenario: SDK-TS evidence is pinned only to 8.1.4

- **WHEN** the seed mapping is validated under schema v2
- **THEN** its interval remains `>=8.1.4,<8.1.5` rather than claiming later 8.x
  or 9.x releases
