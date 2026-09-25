# oid4vc-rust-reuse-assessment Specification

## Purpose
TBD - created by archiving change reassess-oid4vc-rust-reuse. Update Purpose after archive.
## Requirements
### Requirement: OID4VC candidates are decided per layer

The assessment SHALL classify existing OID4VCI behavior, OID4VP orchestration,
DCQL execution, and each full-framework candidate independently. A useful
closed mechanic SHALL NOT activate a framework, define an Identus public type,
or replace existing OID4VCI behavior without separate parity evidence.

#### Scenario: A narrow engine fits while a framework does not

- **WHEN** the DCQL engine passes its focused gate but a full framework fails coupling or release gates
- **THEN** only private DCQL reuse MAY be recommended for a separate production decision

### Requirement: The candidate executes outside release graphs

Exact `siros-dcql 0.3.0` SHALL be evaluated in a separately locked unpublished
fixture. It and its transitive types SHALL remain absent from the root
workspace, release dependency graph, public APIs, and ordinary fast CI.

#### Scenario: Ordinary SDK consumers build

- **WHEN** the root workspace or staged release graph is inspected
- **THEN** `siros-dcql` and its research-only dependencies SHALL be absent

### Requirement: SDK bounds and diagnostics remain authoritative

The fixture SHALL reject an oversized query before candidate parsing, map
candidate errors to bounded local categories, and return no candidate-owned
public result. It SHALL demonstrate valid matching and record tolerant
identifier and missing-metadata behavior as mismatches rather than policy.

#### Scenario: Verifier input is oversized or malformed

- **WHEN** a query exceeds the local byte limit or candidate parsing fails
- **THEN** the adapter returns a bounded redacted error without echoing query bytes or verifier identifiers

### Requirement: Evidence distinguishes executed and unrun targets

The assessment SHALL record exact provenance, license, MSRV, dependency cone,
unsafe/native reach, host and available target checks, maintenance, protocol
currency, compatibility, rollback, commands, and objective reconsideration
triggers. It SHALL NOT infer runtime or portable support from unrun checks.

#### Scenario: A target toolchain is unavailable

- **WHEN** a portable compile command cannot run in the evidence environment
- **THEN** that target is explicitly recorded as unrun and no support claim is made

### Requirement: Research does not activate production reuse

Merging the research SHALL NOT modify OID4VCI/OID4VP runtime behavior or create
a support claim. Production use SHALL require a separate issue and OpenSpec,
an Identus-owned facade, target evidence, and an updated ADR.

#### Scenario: The assessment merges

- **WHEN** its reviewed PR lands on `develop`
- **THEN** current SDK behavior is unchanged and the candidate remains research-only

