## ADDED Requirements

### Requirement: Publishable crate tests own their package inputs

A publishable crate SHALL keep every static input needed by its declared test
targets below the crate root. The canonical package listing SHALL include those
inputs, and repository conformance catalogs SHALL reference that same canonical
file rather than a mutable duplicate outside the package.

#### Scenario: Repository and package tests use one packet

- **WHEN** a conformance packet is both repository evidence and a crate test
  input
- **THEN** the catalog, test, and package archive resolve one regular
  crate-owned file with one recorded digest

#### Scenario: Test reaches above the package root

- **WHEN** a declared test target requires a static file outside its crate root
- **THEN** source-distribution validation rejects the candidate ownership model
