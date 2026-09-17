## ADDED Requirements

### Requirement: Context JSON is bounded by construction

Every public `ContextEntry::Object` SHALL contain an opaque `ContextObject`
validated under the DID JSON depth, node, property, collection, property-name,
and string budgets. No public unchecked or mutable raw-map path SHALL directly
create a context-object entry. Accepted object serialization SHALL remain the
exact JSON object representation.

#### Scenario: Hostile native context object is rejected

- **WHEN** a native caller transfers a 32,768-level JSON tree to the context-
  object constructor
- **THEN** validation returns the existing static depth error and dismantles
  the rejected owned tree iteratively without exhausting the process stack

#### Scenario: Accepted context object round trips

- **WHEN** a bounded inline JSON-LD context object is constructed or parsed
- **THEN** its document serialization and deserialization preserve the existing
  object/scalar/array wire forms and exact accepted values

### Requirement: Generic cardinality representation is policy-neutral

`OneOrMany<T>` SHALL preserve whether one non-empty value was represented as a
scalar or a non-empty array. It SHALL NOT apply the DID document item ceiling
inside its generic constructor or deserializer. Every validated DID owner SHALL
apply the applicable DID collection ceiling before return.

#### Scenario: Oversized context reaches the domain policy boundary

- **WHEN** a caller constructs 129 already-validated context entries in an
  array-shaped `OneOrMany`
- **THEN** structural construction succeeds, DID document construction returns
  `TooManyItems`, and destruction remains stack-safe

### Requirement: Native owned-JSON rejection cleanup is iterative

The SDK SHALL require every public DID constructor or builder `build` path that
takes ownership of caller-provided recursive JSON to arm the shared private cleanup boundary
before any validation branch. Failure SHALL dismantle owned arrays and objects
through one iterative worklist; success SHALL yield the exact validated
candidate. The implementation SHALL NOT use public custom `Drop`, unsafe code,
or an intentional leak.

#### Scenario: Another field fails before JSON validation

- **WHEN** a candidate owns hostile-depth JSON but an earlier non-JSON invariant
  fails first
- **THEN** the same boundary dismantles every owned JSON root iteratively and
  returns the existing redacted error
