# did-resolution-module-ownership Specification

## Purpose

Define responsibility-cohesive private ownership for DID resolution and DID URL
dereferencing data while preserving public, wire, resource, error, cleanup,
runtime, and orchestration contracts.

## Requirements
### Requirement: DID resolution data has cohesive private owners

The `identus-did` crate SHALL separately own scalar value syntax, standard
operation metadata, DID document metadata, resolution result envelopes,
dereferencing result envelopes, and raw-wire preflight behind the existing
private `resolution` facade. Cross-owner visibility SHALL be no broader than
required for current semantic validation and cleanup composition.

#### Scenario: Maintainer changes one resolution responsibility

- **WHEN** a maintainer reviews scalar, metadata, envelope, dereferencing, or
  raw-wire behavior
- **THEN** the responsible private module SHALL contain that behavior without
  unrelated lifecycle mechanics or a forwarding-only public abstraction

#### Scenario: Code-health decomposition is evaluated

- **WHEN** the aggregate resolution module is split
- **THEN** no replacement SHALL exceed the authored-line attention threshold,
  broaden public/internal visibility, duplicate policy, or hide logic in macros

### Requirement: Resolution compatibility remains exact

The decomposition SHALL preserve every crate-root public name, type identity,
derive, method signature, trait implementation, const value, JSON member name,
nullable rule, accepted and rejected state, validation order, resource budget,
error projection, and hostile-value cleanup behavior. Parsing untrusted result
bytes SHALL continue to apply the size bound, raw-wire preflight, typed serde
conversion, and semantic validation in that order.

#### Scenario: Existing consumer upgrades across the refactor

- **WHEN** a consumer uses any existing DID resolution or dereferencing type,
  constructor, parser, accessor, trait, or limit constant
- **THEN** the same source path, signature, value, wire form, success/failure
  state, and deterministic redacted error SHALL remain available

#### Scenario: Hostile or over-budget JSON is rejected

- **WHEN** input contains duplicate decoded names, exceeds a configured wire or
  semantic budget, or contains deeply nested rejected extension values
- **THEN** the same error SHALL be returned and owned values SHALL be cleaned up
  iteratively without recursive-drop exposure

### Requirement: Runtime orchestration stays outside the model split

The decomposition SHALL NOT add I/O, network, blocking synchronization,
executor coupling, cache coordination, request coalescing, or cancellation
policy. Existing duplicate-safe concurrent fill semantics and resolver object
safety SHALL remain unchanged.

#### Scenario: Resolution is invoked through current orchestration

- **WHEN** registry, cache, dereferencing, or HTTP adapters consume resolution
  result models
- **THEN** dispatch, Send/Sync, cancellation, cache, and runtime behavior SHALL
  be identical to the pre-decomposition contract
