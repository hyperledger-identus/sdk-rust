# ADR 0170: make Rust contracts canonical and language adapters transitional

- **Status:** Accepted
- **Date:** 2026-09-30
- **Decision authority:** project-sponsor review recorded in issue
  [#497](https://github.com/hyperledger-identus/sdk-rust/issues/497)
- **Clarifies:** ADR 0162, ADR 0163, ADR 0169
- **Supersedes:** ADR 0169 only where its first SDK-TS canary wording could be
  read to make TypeScript DTOs or errors the SDK-Rust contract
- **Related:** ADR 0061, ADR 0110, ADR 0164, issues #420, #492, #493, #501,
  and #502

## Context

SDK-TS is the best current discovery baseline, but its released object shapes,
errors, private workspace layout, plugin host, and historical modules reflect
TypeScript and product evolution. Requiring Rust-backed behavior to preserve
those DTOs and errors would reverse the platform-core dependency direction.
It would turn a migration facade into the source of truth and make every other
binding inherit JavaScript-specific coupling.

The same problem appears in the preliminary ownership of private workspace
packages and DID behavior. Packaging AnonCreds, DIDComm, JWE, domain types, or
protobuf privately does not make them TypeScript capabilities. Protobuf is an
intentional Prism DID codec/codegen concern and the SDK-Rust developer shell
already provides the toolchain, but portable method semantics still require a
bounded implementation decision. Generic DID domain and service semantics are
already Rust responsibilities.

## Decision

### Canonical contract

SDK-Rust owns canonical reusable domain types, validated construction, error
taxonomy, staged verification/trust state, protocol state, serialization, and
port contracts. These contracts are idiomatic Rust and follow repository
rules for ownership, redaction, bounded inputs, portability, cohesion, and
dependency direction.

A language SDK may expose an idiomatic or legacy DTO/error facade when needed
for migration. The adapter is explicit, versioned, tested in both directions,
observable, documented with deprecation/removal gates, and removable without
changing the canonical Rust model. Unsafe legacy behavior is rejected rather
than simulated.

### Private workspace packages

Portable identity behavior found in a language-private package moves to an
appropriate `identus-*` crate or to a qualified Rust dependency behind an
Identus-owned facade. Language layers retain package loading, host APIs,
generated declaration ergonomics, and bounded compatibility translation.

The policy applies independently to domain models, protobuf codecs,
AnonCreds, DIDComm, JWE, and future embedded engines. Existing bundling is
compatibility evidence, not automatic dependency selection.

### DID ownership

SDK-Rust owns generic DID/DID URL values, documents, verification
relationships, services, resolution, dereferencing, registration, and method
dispatch. Portable DID-method codecs and deterministic semantics compose those
contracts. Peer DID is a required planned Rust method capability.

Method-specific Rust crates may own portable method semantics. Concrete ledger
observation/submission, network execution, storage engines, custody, and
product policy remain injected or downstream. For Prism this permits portable
protobuf/message and method semantics in SDK-Rust without importing Cardano
clients, indexer persistence, or NeoPRISM node composition.

### Quality evidence

Property, fuzz, benchmark, and differential tests are required engineering
evidence when applicable to the capability's risks. They are not normative
authority by themselves. Every implementation issue records each class as
required or justified not applicable; expensive exact-head campaigns may run
in slow/release lanes rather than every PR.

## Consequences

- SDK-TS can migrate without a flag-day API break, while new bindings are not
  forced to copy its historical shapes.
- Private npm/WASM implementation boundaries no longer decide the Rust crate
  graph.
- DID bindings can expand from the narrow value canary toward the complete
  generic DID domain and service surface in reviewed slices.
- Cross-language adapters carry measurable migration cost and an exit plan.
- Candidate engines still require their own standards, supply-chain, target,
  security, conformance, and resource decisions.

## Alternatives rejected

- Preserve SDK-TS DTOs and errors as the canonical contract: couples every
  target to one donor and makes later cleanup breaking by construction.
- Rewrite each language API immediately: creates unnecessary adoption risk and
  removes a useful compatibility runway.
- Keep private workspace engines language-owned: duplicates portable behavior
  and security review across SDKs.
- Put complete Prism/Cardano behavior in the generic SDK: violates the
  chain-neutral dependency boundary.
- Treat generative, fuzz, benchmark, and differential work as optional polish:
  leaves parser, state, resource, and interoperability risks until release.

## Verification and rollback

The machine SDK-TS inventory records the canonical owner, adapter policy,
private-workspace policy, DID ownership, and four quality classes. Its checker
and mutation tests fail if those controls disappear. Capability issues and the
adoption roadmap link the owning evidence.

Rollback reverts this planning decision before a dependent capability ships.
After a language adapter is released, rollback follows its versioned migration
contract; it never makes the donor DTO/error model canonical retroactively.
