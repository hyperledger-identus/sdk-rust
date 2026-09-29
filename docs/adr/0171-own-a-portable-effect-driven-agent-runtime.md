# ADR 0171: own a portable effect-driven Rust agent runtime

- **Status:** Accepted as target architecture; implementation remains issue-first
- **Date:** 2026-09-30
- **Decision authority:** project-sponsor review recorded in issue
  [#497](https://github.com/hyperledger-identus/sdk-rust/issues/497)
- **Related:** ADR 0170; issues #418, #419, #498, #499, #500, and #502
- **Constraint impact:** material; changes the preliminary TypeScript-only
  runtime and networking disposition without claiming current support

## Context

SDK-TS combines protocol workflows with jobs, events, fetch, lifecycle,
plugins, and user interaction. Keeping that complete bundle in TypeScript
would duplicate reusable orchestration in every language SDK. Moving it
literally to Rust would instead import browser policy, product lifecycle, and
an ambient async runtime into the generic core.

SSI implementations provide useful design evidence. VCX separates protocol,
message, credential, ledger, wallet, and agent crates. Procivis One Core
separates providers from orchestration services. Credo, SDK-TS, Aries agents,
Cloud Agent, and Mediator demonstrate dispatch, events, persistence, transport,
and protocol coordination, but none of their framework boundaries is
normative for SDK-Rust.

Browser and Node networking has the same ambiguity. Protocol crates must own
request/response meaning and bounds, while host environments must own actual
I/O and platform policy. The implementation language of the host adapter is a
target decision, not a reason to move protocol behavior out of Rust.

## Decision

SDK-Rust will provide a small library runtime with these concepts:

- bounded, versioned events carrying correlation and causation identifiers;
- protocol instances identified by protocol, version, role, and versioned
  opaque state;
- deterministic transitions that return new state and typed effect requests;
- effect requests/results for message transport, DID resolution, persistence,
  timers, user decisions, and capability calls;
- explicit cancellation, deadlines, fairness, idempotency, retry intent,
  checkpoint sequencing, and resource budgets; and
- stable redacted errors and privacy-safe event/telemetry projections.

The runtime will not require Tokio or another async executor. It will not own
TLS, sockets/fetch, a database, secure keys, custody, consent policy, UI,
telemetry storage, multitenancy, or deployment lifecycle. Those are injected
ports and host executors.

The reference proof is a deterministic single-threaded executor. Native async,
WASM promises, Swift concurrency, Kotlin coroutines, Node promises, server
runtimes, and production stores/transports are adapters. FFI surfaces use
data-oriented commands and opaque handles rather than exporting generic Rust
async traits.

Rust protocol crates own bounded request/response types, validation,
correlation, redirect and timeout/cancellation intent, and transport ports.
Browser/Node adapters own TLS roots, proxy and DNS/private-address policy,
cookies/auth integration, decompression, redirect execution, actual timeouts,
and host observability. A concrete adapter may be Rust/WASM, native Rust, or
thin TypeScript after issue #499 compares target evidence.

## Consequences

- One deterministic protocol state implementation can run in wallets, mobile
  apps, Node, browsers, services, and tests.
- Target executors remain idiomatic without becoming protocol authorities.
- Effects and checkpoints become explicit test and FFI boundaries.
- Runtime design requires careful queue/resource, replay, persistence, secret,
  and cancellation analysis before implementation.
- SDK-TS remains a valid host and migration facade; it stops owning portable
  orchestration semantics.

## Alternatives rejected

- Keep the runtime entirely in each language SDK: duplicates state, retry,
  persistence, and security behavior.
- Port SDK-TS `Agent` and `JobManager` literally: imports donor framework and
  browser policy.
- Require Tokio in public contracts: makes WASM, mobile, embedded, and foreign
  executors unnecessarily expensive.
- Implement networking entirely in Rust for every target now: chooses
  dependency and host-policy outcomes before evidence.
- Implement protocol flows in thin TypeScript adapters: recreates the
  duplication this platform-core program exists to remove.

## Verification and rollback

Issue #498 owns the exact runtime API and multi-executor proof. Issue #499 owns
the per-target networking decision. The same protocol fixture must run through
the deterministic reference executor and target-shaped executors. Property,
fuzz, benchmark, and differential evidence follows issue #501.

This ADR activates target architecture only. Until an implementation issue
passes its gates, the messaging/agent placeholder remains unsupported. A
failed runtime experiment can be removed without changing existing SDK-TS or
service behavior.
