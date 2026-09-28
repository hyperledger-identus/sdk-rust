# oid4vci-bounded-json-ingress Specification

## Purpose

Define the private ownership and behavioral invariants for bounded OID4VCI JSON
parsing without coupling protocol-specific limits, errors, or object semantics.

## Requirements
### Requirement: OID4VCI JSON uses one bounded private ingress

The `identus-oid4vci` crate SHALL route every existing bounded JSON parser
through one private scanner and root-object admission contract. That contract
SHALL preserve exact byte consumption, depth and node accounting, duplicate
member rejection, iterative traversal, zeroizing decoded-string ownership,
and static redaction-safe errors. It SHALL NOT expose parser mechanics, add an
ambient runtime, construct an unbounded recursive intermediary, or become a
cross-protocol JSON framework.

#### Scenario: Protocol object is accepted through shared admission

- **WHEN** any existing Credential Offer, metadata, token/nonce, or credential
  response parser receives a valid bounded root object
- **THEN** the shared ingress SHALL admit and count the root, invoke exactly
  one protocol-owned parser, and require complete input after whitespace

#### Scenario: Hostile JSON fails under the existing contract

- **WHEN** input is malformed, incomplete, duplicate-bearing, too deep, or
  exceeds its node budget
- **THEN** it SHALL fail with the same existing static error precedence and
  SHALL NOT leak input, offsets, retained secrets, or parser causes

### Requirement: Protocol JSON ownership follows independent change axes

The SDK SHALL place Credential Offer/grant, metadata, token/nonce, and
credential-response field records and grammars in cohesive private modules.
Each module
SHALL retain its protocol-specific required fields, limits, cardinality,
uniqueness, branch exclusions, exact-value ownership, and error selection.
Private facade re-exports MAY preserve existing crate-local caller paths.

#### Scenario: Parser implementation moves between private modules

- **WHEN** a protocol parser is decomposed from the former aggregate file
- **THEN** its public caller, accepted and rejected wire forms, limit source,
  error variant and precedence, retained value, dependency cone, and target
  support SHALL remain unchanged

#### Scenario: Similar syntax has different protocol authority

- **WHEN** two objects use similar JSON mechanics but different limits, errors,
  normative sections, or change cadence
- **THEN** only scanner mechanics SHALL be shared and protocol policy SHALL
  remain in its owning module
