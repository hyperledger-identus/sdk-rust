## ADDED Requirements

### Requirement: Descriptor text is role-specific and bounded

The SDK SHALL provide distinct owned values for entity identifiers,
credential types, schema identifiers, schema versions, claim identifiers,
claim value-type hints, and claim-path segments. Each SHALL accept non-empty
UTF-8 with no leading or trailing whitespace and no control character. Entity
and schema identifiers SHALL accept at most 1,024 bytes, credential types at
most 256 bytes, and every other descriptor value at most 128 bytes. Validation
SHALL precede the one successful SDK-owned allocation, preserve accepted text
exactly, and reject invalid input without retaining or rendering it in errors.

#### Scenario: independent identifier dialects remain expressible

- **WHEN** adapters parse a DID, HTTPS URL, Midnight family identifier, W3C
  credential term, and SD-JWT VC type URI into their matching roles
- **THEN** every accepted value SHALL retain its exact spelling without a
  chain, format, or global-registry dependency

#### Scenario: unsafe descriptor strings fail closed

- **WHEN** a descriptor is empty, oversized, padded with whitespace, or
  contains control characters
- **THEN** construction SHALL return a static typed error without storing or
  echoing the rejected value

### Requirement: Claim paths and disclosure descriptors are bounded

The SDK SHALL define stable round-tripping disclosure modes `public`,
`selective`, `committed`, and `predicate_only`. A `CredentialClaimPath` SHALL
contain 1–16 validated path segments. A `CredentialClaimDescriptor` SHALL
contain one claim identifier, one path, a disclosure mode, a required flag,
and an optional value-type hint, but no claim value or commitment opening.

#### Scenario: unrelated claim models use the same descriptor

- **WHEN** a W3C/SD-JWT-shaped public claim and a Midnight committed or
  predicate-only claim are described
- **THEN** both SHALL use the same path and descriptor types without exposing
  claim values or format-specific path syntax

#### Scenario: invalid disclosure and path input is rejected

- **WHEN** a disclosure spelling is unknown or a path has zero or more than 16
  segments
- **THEN** construction SHALL fail with its stable redaction-safe error

### Requirement: Schema descriptors enforce bounded uniqueness

A `CredentialSchemaDescriptor` SHALL contain a schema identifier, optional
schema version, 1–16 unique credential types, and at most 64 claim descriptors.
Claim identifiers and complete claim paths SHALL each be unique within a
schema. Construction SHALL check collection bounds before duplicate scans and
SHALL require no registry, hash map, network, or format codec.

#### Scenario: Midnight and W3C schemas share one structural contract

- **WHEN** a Midnight birth-family schema and a W3C JSON-schema reference with
  claim descriptors are constructed
- **THEN** both SHALL preserve their identifiers, optional version, types,
  paths, disclosure modes, and required flags through the same API

#### Scenario: ambiguous schemas fail at construction

- **WHEN** types, claim identifiers, or complete claim paths are duplicated,
  or a required collection exceeds its bound
- **THEN** construction SHALL reject the schema with a stable typed error

### Requirement: Credential metadata is normalized and bounded

`CredentialMetadata` SHALL contain one issuer identifier, 0–16 unique subject
identifiers, 1–16 unique credential types, 0–16 schema descriptors with unique
schema identifiers, and optional `UnixTimestampMillis` valid-from and
valid-until bounds. When both bounds exist, valid-from SHALL be no later than
valid-until. Construction SHALL not imply that metadata was parsed from,
cryptographically bound to, or verified against a credential payload.

#### Scenario: three consumer shapes project through one metadata API

- **WHEN** Oxid/Midnight, Lace digital-passport, and W3C/SD-JWT-shaped adapters
  construct normalized metadata
- **THEN** each SHALL represent its issuer, optional subjects, types, schemas,
  and validity without chain, format, protocol, or product dependencies

#### Scenario: duplicate and reversed metadata is rejected

- **WHEN** subjects, types, or schema identifiers are duplicated, a collection
  exceeds its bound, types are empty, or valid-from is later than valid-until
- **THEN** construction SHALL fail rather than retain ambiguous metadata

### Requirement: Metadata remains privacy-safe and format-neutral

Credential metadata and schema descriptors SHALL contain no claim values,
proof bytes, private material, trust decision, display/localization content,
storage identifier, executable validator, or wire representation. Safe Debug
for `CredentialMetadata` SHALL not render issuer or subject identifiers and
errors SHALL not contain caller-controlled descriptor text.

#### Scenario: correlating identifiers do not enter diagnostics

- **WHEN** metadata containing known issuer and subject values is debugged, or
  invalid caller text is bridged to the shared error boundary
- **THEN** neither correlating value nor rejected input SHALL appear

### Requirement: Metadata errors use the stable SDK boundary

Every metadata/schema construction error SHALL map to `IdentusError` with
capability `credential`, kind `InvalidInput`, a stable static `credential.*`
code, and static public text.

#### Scenario: all metadata errors bridge without caller data

- **WHEN** every new descriptor, collection, duplicate, and validity error is
  converted and formatted
- **THEN** its capability, kind, code, and text SHALL match the contract and
  SHALL contain no caller-controlled value

### Requirement: Construction remains allocation-conscious

Scalar parsing SHALL validate borrowed text before one owned-string
allocation. Schema and metadata constructors SHALL retain caller-owned vectors,
check collection bounds before duplicate scans, and perform allocation-free
bounded duplicate comparisons. A manual release diagnostic SHALL report
construction throughput without a machine-dependent pass threshold.

#### Scenario: performance path is measurable without flaky correctness

- **WHEN** the ignored release diagnostic constructs representative metadata
  through the production API
- **THEN** it SHALL report elapsed time and throughput without adding a
  correctness timing assertion
