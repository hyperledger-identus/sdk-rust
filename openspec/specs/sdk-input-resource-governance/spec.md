# sdk-input-resource-governance Specification

## Purpose
Define an exhaustive, machine-checked ownership map for implemented SDK input
and work boundaries so typed limits are not confused with caller, transport,
deserializer, FFI, language-runtime, or concrete-adapter obligations.
## Requirements
### Requirement: Implemented SDK input boundaries are exhaustively inventoried

The repository SHALL contain one normative machine-readable inventory covering
every package classified `implemented` by the bootstrap inventory. Each
distinct input-boundary family SHALL have a stable identifier, package, public
surface summary, one ownership disposition, exact evidence, consumer impact,
and review triggers. `sdk-enforced` boundaries SHALL name explicit byte,
element, nesting, recursion, allocation, time, or work limits. Fixed/no-input,
caller-budgeted-work, outer-preallocation, and known-unbounded-compatibility
boundaries SHALL state why an SDK limit is inapplicable, cannot protect earlier
work, or requires a public compatibility migration.

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

#### Scenario: A public resource constant is omitted

- **WHEN** an implemented package declares a public resource constant with a
  `MAX_` or `MIN_` name segment, including `DEFAULT_MAX_`, that no inventory
  limit names
- **THEN** offline validation fails and names the package and omitted constant

### Requirement: Residual resource ownership remains explicit

The inventory and limitation index SHALL distinguish typed SDK validation from
allocation or scanning that may already have occurred in a caller, serde,
transport framework, FFI runtime, or JavaScript engine. Borrowed cryptographic
primitive inputs and generic async/storage ports MAY remain caller-budgeted
when the SDK does not retain them and owns no normative protocol budget. These
residuals SHALL identify the responsible layer and SHALL NOT be described as an
incomplete audit.

A retained public compatibility type MAY remain unbounded only when imposing a
limit requires an explicit public migration. It SHALL be inventoried as
`known-unbounded-compatibility`, named in the limitation index, and documented
as unsafe for direct hostile input until that migration occurs.

Native constructors that accept already-owned recursive JSON SHALL disclose
when validation errors can enter recursive destruction of the rejected tree.
The limitation and inventory SHALL require callers to use a bounded wire-slice
parser or establish an equivalent pre-entry depth bound until iterative cleanup
is comprehensive.

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

### Requirement: Distinct DID registry and cache ownership remains visible

The boundary inventory SHALL record the bounded DID method registry separately
from dispatched adapter work. It SHALL record portable DID resolution cache
key, declared-capacity, and TTL limits separately from caller-budgeted cache and
clock adapter allocation and QoS. Existing package-level coverage SHALL NOT be
treated as evidence that these distinct families are present.

#### Scenario: DID method registry is inspected

- **WHEN** an agent searches the inventory for retained method bindings
- **THEN** it SHALL find the exact 64-entry limit and registry source evidence

#### Scenario: DID resolution cache ownership is inspected

- **WHEN** an agent searches for portable cache policy and operational adapter
  work
- **THEN** it SHALL find SDK-enforced key/capacity/TTL limits and a separate
  caller-budgeted cache/clock adapter row

### Requirement: Known unbounded compatibility retention remains explicit

An implemented SDK type SHALL have a distinct
`known-unbounded-compatibility` inventory row when it retains unbounded caller
input for public compatibility. The row SHALL name the exact public surface,
source evidence, consumer guard, migration trigger, and effective limitation.
It SHALL NOT be
misrepresented as bounded, non-retaining, or protected by an outer allocator.

#### Scenario: Historical Multihash placeholder is inspected

- **WHEN** an agent audits `identus_did::Multihash`
- **THEN** the inventory and `SDK-LIM-007` SHALL disclose its unbounded opaque
  byte retention and require consumers to bound input before construction or
  serde until an explicit migration replaces the compatibility contract

### Requirement: Native DID JSON rejection cleanup remains consumer-guarded

The inventory and limitation index SHALL disclose that native DID constructors
accepting already-owned recursive JSON can recurse while destroying a rejected
hostile-depth tree. Accepted retained values SHALL remain resource-bounded, but
the hardened hostile-input path SHALL be a bounded wire-slice parser or an
equivalent caller-owned depth bound until rejection cleanup is iterative for
the complete native family.

#### Scenario: Native caller owns a hostile-depth JSON tree

- **WHEN** a caller would pass recursive JSON directly to a DID constructor
- **THEN** the caller SHALL pre-bound its depth or use the bounded slice parser
  so validation failure cannot enter unbounded recursive destruction
