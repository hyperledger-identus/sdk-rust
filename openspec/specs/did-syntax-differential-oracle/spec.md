# did-syntax-differential-oracle Specification

## Purpose
TBD - created by archiving change add-iota-did-syntax-oracle. Update Purpose after archive.
## Requirements
### Requirement: an independent DID syntax oracle remains isolated

The SDK SHALL run exact published `identity_did 1.5.1` only from a separately
locked unpublished conformance fixture. Candidate packages, models and errors
SHALL NOT enter root runtime manifests, release artifacts or public Identus APIs.

#### Scenario: dependency graphs are inspected

- **WHEN** the root and oracle graphs are compared
- **THEN** every IOTA Identity package appears only in the oracle fixture lock

### Requirement: differential cases retain normative authority

Every oracle case SHALL declare an identifier and expected class. Shared
acceptance SHALL preserve exact input spelling. Any implementation divergence
SHALL be explicitly expected and adjudicated against W3C DID Core 1.0, RFC 3986
or an Identus resource/compatibility constraint; agreement alone SHALL NOT
change the SDK contract.

#### Scenario: the corpus executes

- **WHEN** DID and DID URL positive, negative and boundary cases are run
- **THEN** every result matches its pinned class without exposing candidate
  diagnostics or redefining SDK errors

### Requirement: oracle value and cost are reviewable

The evidence SHALL record immutable provenance, exact features, license, MSRV,
direct/resolved cone, unsafe/native reach, advisories, target compile results,
maintenance posture, limitations and exact commands. The ADR SHALL decide
whether to retain or remove the executable oracle and state a refresh/removal
trigger.

#### Scenario: final review is completed

- **WHEN** behavior and supply-chain evidence is complete
- **THEN** the repository contains a reproducible decision without activating
  production dependency or target support

