# ADR 0162: establish sdk-rust as the Identus platform core

- **Status:** Accepted
- **Date:** 2026-09-29
- **Decision authority:** issue
  [#415](https://github.com/hyperledger-identus/sdk-rust/issues/415)
- **Related:** ADR 0061, ADR 0097 through ADR 0101, ADR 0110, issues #163 and #223
- **Constraint impact:** material; adds authoritative ownership and migration
  boundaries without activating a binding support claim

## Context

Identus has substantial overlapping implementations in `sdk-ts`, `sdk-swift`,
and `sdk-kmp`. They independently implement combinations of cryptography,
DIDs, credentials, presentations, DIDComm, persistence, backup, edge-agent
protocols, and OpenID4VC behavior. This multiplies semantic drift, security
review, dependency management, fixes, conformance work, and release effort.

`sdk-rust` already owns reusable crypto and DID primitives, generic credential
and presentation contracts, selected OpenID4VC behavior, wallet storage ports,
and narrow experimental native/browser DID bindings. It is not yet a complete
replacement for the language SDKs. Messaging, concrete formats, storage
adapters, agent workflows, binding breadth, and supported distribution remain
incomplete.

## Decision

Make `sdk-rust` the authoritative implementation of reusable Identus domain
and protocol semantics. New portable behavior is implemented once in an
appropriate generic `identus-*` crate, or delegated to a reviewed upstream
dependency behind Identus-owned types. Language SDKs consume it through
isolated binding crates and remain responsible for:

- idiomatic language APIs and migration shims;
- package distribution and ecosystem tooling;
- platform key custody, persistence, networking, scheduling, permissions,
  lifecycle, and user-consent integration;
- product policy that cannot be expressed as a generic reusable contract; and
- temporary, explicit compatibility modes.

The architectural target is:

```text
product / application policy
           |
idiomatic platform facade and adapters
           |
WASM | UniFFI | React Native adapter boundary
           |
Identus-owned binding DTOs, errors and state machines
           |
generic identus-* capability crates
           |
private upstream engines and caller-owned ports
```

The Rust core remains chain-, product-, transport-, storage-, and runtime-
neutral. Binding frameworks do not enter generic domain crates. Secrets,
untrusted bytes, async work, cancellation, and ownership receive purpose-built
surfaces rather than automatic export of arbitrary Rust APIs.

`cloud-agent` and `mediator` are later service-decomposition programs, not SDK
binding conversions. Reusable domain/protocol pieces may move into `sdk-rust`,
but application APIs, persistence, operations, deployment, and service runtime
need separate architecture and migration decisions.

## Consequences

- Cross-platform security and conformance work can converge on one core.
- TypeScript, Swift, and Kotlin repositories are not automatically deleted;
  their duplicate portable implementations shrink while their language and
  platform responsibilities become explicit.
- Browser/Node, native Swift/Kotlin, and React Native need separate adapter and
  runtime evidence even when they expose the same capability.
- Capability gaps in Rust become product backlog rather than reasons to keep a
  second permanent semantic core.
- No present experimental binding becomes supported through this ADR.

## Alternatives rejected

- Archive all language SDKs after a repository-wide rewrite: creates a
  flag-day migration and loses platform integration and idiomatic APIs.
- Keep independent implementations and synchronize by documentation: retains
  semantic drift and duplicate security-critical work.
- Expose all Rust crates directly through one universal FFI: leaks unsuitable
  types and ignores different browser, native, and React Native runtimes.
- Port `cloud-agent` and `mediator` as part of the SDK migration: combines
  service operations and SDK compatibility into an unreviewable program.

## Verification and rollback

Each migrated capability must have a normalized inventory, binding contract,
shared conformance evidence, consumer rehearsal, change-ledger entry, rollback,
and target/runtime proof. Until those gates pass, its legacy implementation
remains authoritative for current consumers. Reverting this ADR removes the
program direction only; the bootstrap changes no runtime or package behavior.
