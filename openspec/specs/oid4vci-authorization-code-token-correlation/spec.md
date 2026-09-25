# oid4vci-authorization-code-token-correlation Specification

## Purpose
TBD - created by archiving change correlate-oid4vci-token-authorization-details. Update Purpose after archive.
## Requirements
### Requirement: Credential authorization is exact to selected request lineage

The SDK SHALL consume one request-bound Authorization Code Token success and
reuse the existing bounded Token Authorization Details validator. Every
recognized `openid_credential` entry SHALL reference the exact Credential
Configuration selected before authorization. Any recognized mismatch SHALL
fail before duplicate-entry ambiguity is considered.

After configuration equality, the response SHALL contain exactly one
recognized entry. More than one matching entry SHALL fail as ambiguous. The
one entry's source-ordered unique `credential_identifiers` SHALL become the
only typed Credential Dataset authority in the correlated state.

#### Scenario: exact selected configuration advances

- **WHEN** one recognized entry names the exact selected configuration and has
  valid bounded unique dataset identifiers
- **THEN** correlation returns those identifiers in source order with the
  exact request lineage and Token Response core

#### Scenario: missing details fail in the reused parser

- **WHEN** the bound success omits Authorization Details or has no recognized
  credential entry
- **THEN** correlation fails with the existing static response-local error

#### Scenario: unrequested configuration fails closed

- **WHEN** any recognized entry names a configuration other than the exact
  selected configuration
- **THEN** correlation fails with the static configuration-mismatch diagnostic

#### Scenario: repeated matching entries are ambiguous

- **WHEN** two or more recognized entries all name the exact selected
  configuration
- **THEN** correlation fails instead of selecting one by position

### Requirement: Extensions remain bounded and least authority

Unknown authorization-detail types and unknown fields SHALL remain governed by
the existing response byte/depth/node and explicit Authorization Details
limits. They SHALL be counted and ignored and SHALL NOT confer credential
authority or alter exact selected-configuration comparison.

#### Scenario: unrelated bounded details coexist

- **WHEN** one exact recognized entry coexists with bounded unknown detail
  types and fields
- **THEN** the exact entry advances and only the unknown-type count survives

### Requirement: Correlated token and dataset authority are redacted

The correlated state SHALL retain the exact public issuer/server/configuration
lineage and issuer-identification evidence, existing zeroizing Token Response
core, one matched entry's zeroizing identifiers and the bounded unknown-type
count. It SHALL expose only borrowed deliberate accessors and SHALL NOT expose
Clone, Display, Serde, raw parts or public constructors.

Debug and every error SHALL omit tokens, response JSON, dataset identifiers,
configuration, issuer/server/endpoint and remote values.

#### Scenario: lineage and identifiers remain deliberate

- **WHEN** correlation succeeds
- **THEN** lineage and evidence equal the consumed #360 success and dataset
  identifiers are available only through the correlated borrowed iterator

#### Scenario: aggregate diagnostics remain redacted

- **WHEN** the correlated state and all correlation errors are formatted
- **THEN** no token, identifier, configuration, endpoint or remote canary is
  present

### Requirement: Authority and compatibility remain narrow

The capability SHALL add no dependency, feature, lockfile, unsafe/native,
network, storage, consumer, chain, product, release or publication behavior.
It SHALL preserve the existing response-local public Authorization Details
API and SHALL NOT construct Credential Requests or claim token, dataset,
issuer, server or issuance trust.

#### Scenario: correlation is a pure local transition

- **WHEN** response-local data is correlated
- **THEN** no DNS, TLS, HTTP, clock, entropy, persistence, proof, retry or trust
  effect occurs
