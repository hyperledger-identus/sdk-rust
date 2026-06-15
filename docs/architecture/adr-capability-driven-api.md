# ADR: Capability-Driven API Strategy

Status: Accepted

Date: 2026-06-15

## Context

The Rust SDK is intended to replace duplicated SDK semantics and become the
core used by TypeScript, Swift, Kotlin, React, React Native, browser, mobile,
server, Mediator, Cloud-Service, and NeoPRISM composition. The existing SDKs
are valuable source evidence, but they were built with different language
constraints, naming history, runtime assumptions, and platform-specific
ergonomics.

Trying to derive a universal API by taking the union or intersection of all
existing SDK methods would create the wrong Rust API:

- A union would preserve too many historical seams, duplicate capabilities, and
  wrapper-specific conveniences.
- An intersection would lose important capabilities that exist in only one
  source today, such as platform storage decisions, plugin workflows, or
  protocol-specific helpers.
- Both approaches would bias the core toward legacy naming and language
  constraints instead of SSI domain concepts, type safety, and portable
  protocol state.

Conformance tests should therefore not define a universal cross-SDK public API.
They should define capability contracts: inputs, roles, state transitions,
expected outputs, typed errors, security properties, fixture coverage, and
binding parity expectations.

## Decision

The best-fit `sdk-rust` API is capability-driven and layered:

1. Domain primitives: strongly typed identifiers, keys, DID values, DID URLs,
   credential formats, presentation requests, message types, status references,
   trust anchors, and error codes.
2. Ports: traits for signing, key agreement, DID resolution, storage, transport,
   time, entropy, HTTP, VDR access, status resolution, and trust resolution.
3. Protocol state machines: DIDComm, mediation, issue credential, present
   proof, revocation notification, OID4VCI, OID4VP, SIOPv2, HAIP, and OpenID
   Federation flows.
4. Capability services: issuer, holder, verifier, peer, wallet, mediator,
   credential verifier, presentation verifier, OpenID4VC issuer, OpenID4VC
   wallet, trust policy, and VDR services.
5. Binding facades: JSON-compatible DTOs, opaque handles, typed result
   envelopes, and target-specific async/event ergonomics for TypeScript, Swift,
   Kotlin, React, React Native, browser, Node, and server packages.

The Rust API should expose the smallest stable surface that composes these
layers without erasing type information. Language wrappers can add ergonomic
method names, builders, hooks, or platform callbacks, but cannot own SSI
semantics.

## API Selection Criteria

An API belongs in the Rust core when it satisfies most of these criteria:

- It represents an SSI domain concept or protocol state, not a wrapper
  convenience.
- It is needed by at least one product role: issuer, holder, verifier, peer,
  mediator, wallet, service, or administrator.
- It can be tested by deterministic fixtures without mandatory Docker or
  external services.
- It benefits from Rust type safety, explicit lifetimes of secret handles,
  memory safety, or strict error modeling.
- It should behave identically across TypeScript, Swift, Kotlin, React,
  React Native, browser, mobile, and server targets.
- It can be expressed through typed ports when the implementation depends on
  platform I/O, storage, network, ledger, or device facilities.

An API belongs in a wrapper when it is only:

- UI or framework ergonomics.
- Platform callback shape.
- Package-specific builder syntax.
- React hook composition.
- Swift/Kotlin naming adaptation.
- JavaScript promise/event convenience.
- Test-runner or demo scaffolding over Rust-owned behavior.

## Recommended Public Rust Shape

The first stable Rust surface should be organized around these capability
families:

| Capability family | Primary crate | Public shape |
|---|---|---|
| Core model and errors | `identus-core` | typed result envelopes, redaction-safe diagnostics, capability ids |
| Cryptography | `identus-crypto` | key handles, signer ports, key agreement ports, JWK/JOSE/COSE models |
| DID | `identus-did` | `Did`, `DidUrl`, method profiles, resolver ports, VDR ports |
| Trust and status | `identus-trust` | status resolver, status verifier, trust policy, trust evidence store |
| Credentials | `identus-credentials` | credential model, format registry, issuer/verifier services |
| Presentations | `identus-presentations` | request model, selection, disclosure, verifier services |
| Messaging | `identus-messaging` | DIDComm message model, protocol state machines, mediation and pickup |
| OpenID4VC | `identus-openid4vc` | issuance, presentation, SIOPv2, HAIP, federation state machines |
| Wallet | `identus-wallet` | wallet records, secure-store handles, backup/restore, inventory |
| Agent | `identus-agent` | issuer, holder, verifier, peer workflows over ports and state machines |
| Adapters | `identus-adapters` | HTTP, storage, secure storage, VDR, transport, proximity, platform ports |
| Bindings | `identus-bindings` | DTOs, opaque handles, target manifests, extension registry |

This shape lets conformance ask, "Can this capability run correctly and safely
through Rust and each wrapper?" instead of, "Does every wrapper expose the same
method name?"

## Conformance Strategy

Conformance must be capability-first:

- Catalog entries name the specification, capability family, owner crate,
  support stage, conformance mode, and backlog task.
- Fixtures exercise roles, messages, credentials, presentations, state
  transitions, typed errors, privacy rules, and redaction policy.
- Wrapper API inventories are compatibility gates, not API design inputs.
- Migration matrices explain where existing SDK behavior moves, but do not
  freeze Rust public API names.
- Binding tests prove that wrapper calls dispatch to Rust-owned semantics and
  replay the same fixtures.
- Public API stabilization requires capability coverage, not surface parity
  alone.

The first API conformance suites should therefore be:

1. Domain primitive tests for parsing, normalization, typed errors, and
   redaction-safe display.
2. Port contract tests for deterministic in-memory adapters.
3. Protocol transcript replay for DIDComm and OpenID4VC.
4. Credential and presentation vector replay, including negative cases.
5. Role workflow tests for issuer, holder, verifier, peer, wallet, mediator,
   and service composition.
6. Binding facade tests for DTO round trips, opaque handles, error parity, and
   no-secret-leak behavior.

## Consequences

- The Rust API can be idiomatic, typed, and stable without copying historical
  wrapper shapes.
- Conformance becomes a behavioral contract that works across languages and
  products.
- Legacy SDK APIs remain useful as evidence, migration targets, and wrapper
  compatibility checks.
- New capabilities such as Open Badges, ISO mdoc, HAIP, OpenID Federation,
  Digital Credentials API, secure storage, and proximity exchange can enter
  the model without waiting for all legacy SDKs to expose identical APIs.
