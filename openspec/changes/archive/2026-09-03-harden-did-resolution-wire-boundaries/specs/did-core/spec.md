## MODIFIED Requirements

### Requirement: Typed common DID document metadata

`DidDocumentMetadata` SHALL type the common `created`, `updated`, `deactivated`,
`nextUpdate`, `versionId`, `nextVersionId`, `equivalentId` and `canonicalId`
properties and preserve bounded proof/method-specific extension JSON. Datetimes
SHALL implement the bounded XML Schema 1.1 `dateTime` lexical intersection that
is ASCII, adjusted to UTC with a terminal `Z`, and has whole seconds only.
Version ids SHALL be non-empty printable ASCII; equivalent-id sets SHALL be
non-empty and unique; extensions SHALL NOT shadow common properties.

#### Scenario: complete adjusted datetime profile is accepted

- **WHEN** metadata uses a calendar-valid four-digit, extended, negative, or
  astronomical-zero XML Schema year and a normal or `24:00:00Z` whole-second
  UTC time within the public byte limit
- **THEN** construction and serde SHALL preserve the exact datetime spelling

#### Scenario: non-adjusted or malformed datetime fails closed

- **WHEN** a datetime has an invalid calendar day/year spelling, unnecessary
  leading year zero, numeric offset, absent timezone, fraction, non-ASCII byte,
  invalid end-of-day time, or exceeds its byte limit
- **THEN** construction and serde SHALL reject it with the stable redaction-safe
  resolution boundary

### Requirement: Bounded open DID URL dereferencing results

The capability SHALL provide an immutable serialized DID URL dereferencing
result with `didUrlDereferencingMetadata`, optional `content` and
`contentMetadata`. Success SHALL have bounded JSON content and no error;
failure SHALL have an error, no content and empty content metadata. Open content
SHALL preserve semantic JSON and offer typed construction/access for DID
documents, verification methods, services and URIs without using a closed enum.
Arbitrary native content SHALL remain owned by an explicit binding value with
an exact media type and bounded opaque bytes, with no guessed text/base64
conversion into the core JSON envelope.

#### Scenario: known and future JSON resources remain representable

- **WHEN** dereferenced content is a DID document, verification method, service,
  URI or method-defined JSON resource
- **THEN** it SHALL round-trip semantically and known projections SHALL validate
  through their existing domain types without freezing the at-risk standard

#### Scenario: arbitrary native bytes remain binding-owned

- **WHEN** an HTTP or local binding returns a non-JSON native representation
- **THEN** that binding SHALL pair an exact media type with bounded opaque bytes
  and SHALL NOT make the core result guess UTF-8, base64, transport, or retrieval
  policy

### Requirement: Uniform bounded resolution validation

Raw resolution and dereferencing JSON SHALL be limited to 512 KiB before
deserialization. The raw entry points SHALL reject duplicate decoded object
names recursively before typed deserialization and SHALL preflight at most 64
nested containers, 16,384 JSON values, 128 members per object, and 128 KiB of
decoded names held by simultaneously open objects. Open collections SHALL
contain at most 128 entries; extension maps at most 64 entries with names no
longer than 256 bytes; open trees at most 4,096 nodes and 32 levels; and
arbitrary strings at most 64 KiB. Media types, datetimes, version ids and
problem-detail strings SHALL have smaller documented limits. Every failure
SHALL map to a stable redaction-safe DID resolution error.

#### Scenario: duplicate names fail before semantic collapse

- **WHEN** any raw resolution or serialized dereferencing object repeats a
  decoded property name at the envelope, metadata, error, document, content,
  content-metadata, or extension level
- **THEN** the raw entry point SHALL reject it without selecting a value or
  disclosing the name, value, position, or input bytes

#### Scenario: raw scanner resource limits fail deterministically

- **WHEN** bounded-size raw JSON exceeds scanner depth, node, per-object member,
  or live decoded-name ceilings
- **THEN** it SHALL fail before typed deserialization with a static resolution
  reason and stable public `did.invalid_resolution` boundary

#### Scenario: native construction cannot bypass semantic limits

- **WHEN** the same representable excessive metadata/content tree is supplied
  through a constructor or semantic JSON deserialization
- **THEN** both paths SHALL reject it under the same semantic resource policy

#### Scenario: bounded result parsing remains observable

- **WHEN** representative result envelopes are parsed repeatedly in release mode
- **THEN** fixed work units, bytes processed, and normalized scanner overhead
  SHALL be recorded without a machine-specific CI timing threshold

## ADDED Requirements

### Requirement: Pinned resolution boundary conformance

The DID Core capability SHALL use the W3C DID Resolution v1 Candidate
Recommendation Snapshot dated 6 August 2026 as its compatibility baseline and
SHALL track later drafts by immutable revision. Deterministic generated tests
SHALL cover every public resolution scalar parser, all RFC 9457 error kinds,
resolution/dereferencing states, raw-entry and constructor/serde equivalence,
and every documented JSON ceiling. Portable assertions from the official
implementation-report suite SHALL be mapped without importing HTTP or method
behavior.

#### Scenario: moving draft changes are classified before adoption

- **WHEN** a later editor or Candidate Recommendation Draft revision is audited
- **THEN** normative result/binding changes SHALL require an explicit
  compatibility decision while non-normative changes SHALL NOT silently alter
  the SDK wire contract

#### Scenario: official portable result assertions are executable

- **WHEN** successful, invalid-DID, unsupported-method, or deactivated result
  vectors are evaluated
- **THEN** required members, returned/document DID equality, failure metadata,
  and standard error URLs SHALL match the pinned W3C assertions without an HTTP
  server or DID-method dependency

### Requirement: Explicit legacy resolution error migration

Strict result JSON SHALL accept only the current RFC 9457-shaped W3C error
object. A recognized legacy keyword SHALL migrate only through the explicit
adapter helper, and unknown keywords SHALL fail.

#### Scenario: legacy producers cannot create dual wire semantics

- **WHEN** a NeoPRISM, Midnight, Lace, or Oxid adapter receives a legacy keyword
- **THEN** it SHALL deliberately map a recognized keyword to the exact standard
  URL object before SDK construction rather than relying on permissive raw JSON
