# presentation-error-contracts Specification

## Purpose
TBD - created by archiving change decompose-presentation-error-contracts. Update Purpose after archive.
## Requirements
### Requirement: Presentations owns a lean private error contract

`identus-presentations` SHALL own a crate-private compile-time record containing
only the stable code and single static message required by current presentation
errors. `InvalidInput` and `presentation` SHALL remain centralized invariants.
No dependency, feature, allocation, runtime input, unsafe/native code,
serialization, FFI, retryability, public API or protocol behavior SHALL be
introduced.

#### Scenario: uniform metadata is not duplicated

- **WHEN** any current presentation error is converted
- **THEN** its private record supplies code/message while the shared conversion supplies the uniform kind/capability

### Requirement: Catalogue ownership follows presentation responsibilities

Records SHALL be grouped privately into request/query, candidate matching,
disclosure selection, artifact assembly and lifecycle catalogues with every
current variant in exactly one group.

#### Scenario: one domain is reviewable alone

- **WHEN** a reviewer inspects artifact errors
- **THEN** all nine artifact-owned records are visible without reading request, candidate, disclosure or lifecycle mappings

### Requirement: Routing and inventory are compile-exhaustive

One private wildcard-free router SHALL map every `PresentationError` variant to
exactly one record. The same private list SHALL produce its test-only inventory;
it SHALL NOT generate any public declaration.

#### Scenario: an unmapped variant fails

- **WHEN** a variant is added without a record/router entry
- **THEN** compilation fails rather than selecting a default contract

### Requirement: Exact pre-refactor behavior is immutable

The presentation compatibility oracle SHALL be a planning golden captured from
`develop@105308771dbebceb473b99d9eb82b0fa0178ab09` with exactly 48
unique rows and pin constant identity/visibility, code, kind, capability,
local/public/full display and source behavior. Stable and planning copies SHALL
be byte-identical and fixed-hash bound. Active/archive ambiguity, missing state,
mutation, path escape, or any symlinked component SHALL fail closed.

#### Scenario: all current rows match

- **WHEN** the refactored catalogue is tested
- **THEN** every row remains public, InvalidInput, presentation-owned, exact-message and source-free

### Requirement: Public and target boundaries remain unchanged

The public presentation error surface SHALL remain exact, including the enum,
order, derives, non-exhaustive marker, public constants/paths, `From`,
`Display`, `Error`, and `pub const fn to_identus_error`. It SHALL remain
free of any Serde/binding/wire surface. Direct target compilation is
evidence only and SHALL NOT create a runtime/device/binding support promise.

#### Scenario: compatibility diff is empty

- **WHEN** base and head public/dependency inventories are compared
- **THEN** no public item, manifest, feature, dependency or lockfile delta exists

### Requirement: Maintainability evidence is truthful

The change SHALL report largest router/function, mapping lines, decision count,
and total relevant production lines. It SHALL NOT claim decision deduplication:
the existing and selected designs each own 48 behavioral decisions.

#### Scenario: cohesion improves without false compression

- **WHEN** before/after evidence is reviewed
- **THEN** the largest router is at most 60 lines, five catalogues are independently discoverable, and any total-line increase is disclosed

