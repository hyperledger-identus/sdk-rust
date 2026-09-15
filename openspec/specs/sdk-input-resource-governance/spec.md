# sdk-input-resource-governance Specification

## Purpose
TBD - created by archiving change audit-sdk-input-resource-boundaries. Update Purpose after archive.
## Requirements
### Requirement: Implemented SDK input boundaries are exhaustively inventoried

The repository SHALL contain one normative machine-readable inventory covering
every package classified `implemented` by the bootstrap inventory. Each
distinct input-boundary family SHALL have a stable identifier, package, public
surface summary, one ownership disposition, exact evidence, consumer impact,
and review triggers. `sdk-enforced` boundaries SHALL name explicit byte,
element, nesting, recursion, allocation, time, or work limits. Fixed/no-input,
caller-budgeted-work, and outer-preallocation boundaries SHALL state why an SDK
limit is inapplicable or cannot protect earlier work.

#### Scenario: An implemented package is omitted

- **WHEN** an implemented runtime package has no boundary row
- **THEN** offline validation fails and names the missing package

#### Scenario: A placeholder is presented as audited runtime code

- **WHEN** a placeholder or verification-only package appears as a boundary
- **THEN** offline validation fails instead of implying implemented support

#### Scenario: An SDK-enforced row lacks a limit or evidence

- **WHEN** a row claims SDK enforcement without an explicit limit or existing
  repository evidence
- **THEN** offline validation fails

### Requirement: Residual resource ownership remains explicit

The inventory and limitation index SHALL distinguish typed SDK validation from
allocation or scanning that may already have occurred in a caller, serde,
transport framework, FFI runtime, or JavaScript engine. Borrowed cryptographic
primitive inputs and generic async/storage ports MAY remain caller-budgeted
when the SDK does not retain them and owns no normative protocol budget. These
residuals SHALL identify the responsible layer and SHALL NOT be described as an
incomplete audit.

#### Scenario: Consumer passes an already allocated owned value

- **WHEN** the typed constructor rejects a value above its SDK limit
- **THEN** the inventory still warns that the caller or deserializer allocation
  occurred before validation

#### Scenario: Primitive message work has no protocol budget

- **WHEN** a caller hashes, signs, verifies, or authenticates a borrowed message
- **THEN** the inventory assigns its byte/CPU budget to that caller without
  imposing an arbitrary cryptographic primitive limit

### Requirement: Resource-boundary governance is deterministic and offline

The repository SHALL provide a standard-library-only bounded validator and
focused mutation suite. The factory structural contract SHALL invoke both.
Validation SHALL derive the package population from the canonical bootstrap
inventory, reject unknown schema/dispositions/fields, duplicate IDs, invalid or
missing evidence, missing package coverage, and limits inconsistent with their
disposition without network, Cargo, donor, or consumer access.

#### Scenario: Canonical inventory is checked

- **WHEN** factory validation runs offline
- **THEN** the resource-boundary checker and mutation suite pass deterministically

#### Scenario: Inventory is malformed or oversized

- **WHEN** the inventory exceeds its file/row/value bounds or contains unknown
  structure
- **THEN** validation fails closed before processing unbounded data
