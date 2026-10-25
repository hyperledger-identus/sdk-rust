# language-adapter-mappings Specification

## ADDED Requirements

### Requirement: Canonical Rust source identity is content-bound offline

Every adapter mapping SHALL reference one closed canonical-source record. The
record SHALL bind a stable ID, owning issue, lifecycle state, Cargo package,
Git revision, repository path, regular-file mode, byte length, SHA-256 digest,
Git object format, and Git blob object ID. Validation SHALL recompute both
content identities from bounded non-symlink bytes without requiring `.git`, a
network, compiler, or parser.

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
