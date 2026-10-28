# ssi-upstream-program Specification

## ADDED Requirements

### Requirement: Remaining portable DID methods have independent profiles and owners

The SDK SHALL plan `did:key`, `did:jwk`, and `did:web` as independent,
selectable method crates with pinned normative revisions, explicit first
profiles, bounded inputs, SDK-owned DTOs and errors, and focused issue owners.
Independent implementations MAY provide differential evidence but SHALL NOT
become production dependencies unless a later decision proves an equivalent
profile, acceptable dependency cone, target support, and lower lifecycle cost.

`did:key` and `did:jwk` SHALL remain effect-free. `did:web` SHALL separate pure
method transformation and document binding from retrieval through a minimal
injected bounded-fetch port. Concrete HTTP, executor, DNS, TLS, redirect,
proxy, cache, retry, SSRF, and product-trust policy SHALL remain outside the
portable method core.

#### Scenario: static method implementation begins

- **WHEN** `did:key` or `did:jwk` enters implementation
- **THEN** it uses the existing generic DID and public cryptographic boundaries
  without adding network, storage, secret-provider, runtime, or global-registry
  effects

#### Scenario: web method retrieves a document

- **WHEN** a `did:web` resolver needs remote document bytes
- **THEN** it invokes a bounded caller-supplied fetch port and does not select a
  concrete HTTP client or network-security policy

#### Scenario: a third-party method implementation is available

- **WHEN** a Rust crate implements some or all of a selected DID method
- **THEN** it remains an oracle unless its exact profile, bounds, dependency
  cone, public boundary, and supported-target evidence pass a focused adoption
  decision

#### Scenario: planning hands off to delivery

- **WHEN** planning issue #555 integrates
- **THEN** `IDR-040` references #557 and advances in order to #558 and #559 as
  each independently evidenced method slice is delivered
