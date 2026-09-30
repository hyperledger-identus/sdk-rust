# language-adapter-mappings

## ADDED Requirements

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

Schema v1 SHALL bind one pinned language version and revision to that exact
patch interval. A wider patch, minor, or major interval SHALL require a later
schema with additional immutable revisions and differential evidence.

#### Scenario: SDK-TS evidence is pinned only to 8.1.4

- **WHEN** the seed mapping is validated
- **THEN** its interval is `>=8.1.4,<8.1.5` rather than a claim about later 8.x
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
