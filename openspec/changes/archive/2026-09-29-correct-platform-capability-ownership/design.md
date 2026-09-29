# Design

## Canonical contract and adapter direction

The dependency direction is one-way:

```text
SDK-TS legacy/idiomatic API
        │ versioned translation, deprecation, host integration
        ▼
WASM or native binding facade
        │ narrow generated/owned binding DTOs
        ▼
canonical identus-* Rust domain, errors, protocol state, and ports
```

Rust types are designed for ownership, validated construction, redaction,
bounded inputs, staged trust, `Send`/`Sync` where required, and target-neutral
composition. A TypeScript adapter can preserve names, object layout, error
codes, and async ergonomics for migration, but the mapping is explicit,
versioned, tested, observable, and removable.

## DID ownership

`identus-did` remains the generic owner of DID/DID URL values, documents,
verification relationships, services, resolution, dereferencing,
registration, and method dispatch contracts. Method implementations compose
those contracts. Peer DID is the first missing portable method priority
because DIDComm depends on pairwise identifiers.

Prism protobuf/message and deterministic method semantics may move into a
focused Rust method crate after the existing source/provenance assessment.
Cardano observation, transaction submission, indexer persistence, and node
composition stay outside the generic SDK boundary.

The WASM path expands in slices rather than remaining value-only forever:

1. DID and DID URL values;
2. DID documents, verification relationships, and services;
3. resolver/dereferencer/registrar port facades;
4. method capabilities such as peer DID;
5. target adapters and consumer migration.

Each slice keeps a thin SDK-TS layer and separate compatibility mappings.

## DIDComm layout

```text
application protocol modules (versioned independently)
  issue-credential | present-proof | revocation-notification
  out-of-band | discover-features | coordinate-mediation
  message-pickup | basic-message | problem-report
                         │
                         ▼
portable protocol state + typed effects
                         │
                         ▼
DIDComm Messaging v2.1 pack/unpack/routing facade
                         │
              ┌──────────┴──────────┐
              ▼                     ▼
     DID/secret resolver ports   transport ports
```

An application protocol is never enabled by the engine implicitly. A
machine-readable catalog will pin its specification revision, PIURI, status,
roles, message types, state schema version, effect set, fixtures, and known
consumer compatibility.

## Portable agent runtime

The runtime is a small library above protocol state machines, not a framework
that owns Tokio, fetch, a database, custody, UI, or business policy.

Core concepts:

- `AgentEvent`: bounded, versioned input with correlation and causation IDs;
- `ProtocolInstance`: protocol/version/role plus opaque versioned state;
- `Transition`: deterministic state plus zero or more typed `EffectRequest`s;
- `EffectRequest`: send, resolve, load/store checkpoint, schedule/cancel timer,
  request user decision, or call a capability through a typed port;
- `EffectResult`: bounded success/failure returned to the same instance;
- `AgentRuntime`: fair bounded dispatcher with cancellation, deadlines,
  idempotency and explicit checkpoint sequencing;
- `Executor`: host adapter that performs effects and feeds results back.

The runtime owns ordering, transition atomicity, correlation, cancellation,
retry intent, resource budgets, and versioned checkpoint/event contracts. The
host owns threads/executor, wall-clock implementation, network/TLS, storage
engine, secure keys, consent UX, telemetry sink, and deployment lifecycle.

FFI-facing use favors data-oriented commands and handles rather than exporting
generic async Rust traits. A deterministic single-threaded executor is the
reference test host; Tokio, WASM promises, Swift concurrency, Kotlin
coroutines, and Node promises remain adapters.

## Browser and Node networking

Rust owns bounded request/response, redirect intent, callback correlation,
timeout/cancellation semantics required by a protocol, and the transport port.
The concrete adapter may be:

- Rust compiled to WASM over `web-sys`/browser fetch;
- Rust native for Node or server use; or
- a thin TypeScript `fetch`/WebSocket implementation calling the same binding
  port.

Selection is per target and capability. Protocol logic, validation, secrets,
and state transitions do not move into the adapter. Host policy—TLS roots,
proxy, DNS/private ranges, cookie/auth integration, decompression, redirect
execution, and platform observability—stays with the adapter.

## Service evolution

Phase 1 pins existing Cloud Agent and Mediator versions and runs black-box E2E
flows against SDK-Rust protocol behavior. Phase 2 builds in-process reference
services from public SDK crates and test adapters. Phase 3 proves deployable
service composition with persistence, tenancy, operations, performance,
migration, and rollback. Only then can a separate decision deprecate an
existing service.

## Quality evidence

- property tests prove invariants across generated valid and invalid domains;
- fuzz tests target untrusted parsers, decoders, transition/event ingress, and
  panic/resource-safety boundaries;
- benchmarks establish parsing, crypto, transition, E2E latency, allocation,
  CPU, memory, code-size, and throughput baselines where relevant; and
- differential tests compare official vectors, qualified libraries, SDK-TS,
  SDK-Swift, existing services, and legacy compatibility adapters.

These tests are required engineering evidence but are not normative authority
by themselves. Fast CI runs deterministic bounded representatives; expensive
campaigns run in slow/release lanes with exact-SHA receipts.

## Adoption milestone model

Milestones are capability outcomes, not calendar buckets or source-repository
parity percentages. Each milestone closes only when it has:

1. exact standards/profile and owned Rust contract;
2. dependency and target decisions;
3. implementation with bounded errors/resources and required quality evidence;
4. binding or host adapter for the named adoption path;
5. immutable conformance and compatibility packet;
6. one consumer-shaped or service E2E proof with rollback; and
7. release-candidate and migration evidence, without implying publication.

The roadmap uses these dependency-ordered deliverables:

| Milestone | Deliverable | Adoption proof |
| --- | --- | --- |
| A0 — contract correction | canonical ownership, adapter, test, and milestone rules | machine inventory rejects donor-shaped regressions |
| A1 — shared compatibility foundation | vector catalog, migration ledger, adapter/error mapping schema, risk-routed quality plan | SDK-TS DID value adapter consumes the shared packet |
| A2 — portable DID platform | full DID domain/service binding sequence plus peer DID; Prism portable semantics separately bounded | SDK-TS DID facade and one native binding use Rust without duplicated DID semantics |
| A3 — credential format engines | RFC 9901 base, pinned SD-JWT VC profile, legacy adapter policy, and AnonCreds 1.0 facade decision/delivery in separate slices | cross-language issuance/presentation vectors pass |
| A4 — DIDComm v2.1 core | qualified engine facade, DID/secret ports, pack/unpack/routing, attachment/resource/error contracts | interop with an immutable existing Mediator/agent endpoint |
| A5 — portable agent kernel | deterministic event/transition/effect runtime and checkpoint contract | the same protocol fixture runs in single-threaded, native async, and WASM test executors |
| A6 — DIDComm application protocols | versioned catalog and staged basic/OOB/discovery, issue, present, revoke, mediation/pickup state machines | selected existing Cloud Agent/Mediator E2E flows pass |
| A7 — host and language adapters | browser/Node network decision, WASM/UniFFI surfaces, thin SDK-TS then Swift adapters | named browser, Node, iOS, and Android canaries pass where selected |
| A8 — language SDK adoption | SDK-TS capability-by-capability migration, Swift parity migration, KMP compatibility plan | released-language APIs use Rust-backed behavior with measured rollback |
| A9 — composable services | reference edge-agent/mediator service composition from public SDK crates | operational, persistence, tenancy, QoS, migration, and rollback parity packet |

SD-JWT and AnonCreds remain separate implementation trains within A3;
DIDComm protocols remain separate slices within A6. A milestone never forces
unrelated capabilities into one PR or release. Work may overlap when dependency
contracts are stable, but an adoption claim cannot skip its predecessor proof.

## Verification

The inventory checker will require the canonical owner, adapter policy,
private-workspace policy, DID ownership, and all four quality evidence classes.
Mutation tests will fail if any of those controls regress. Human review checks
that dispositions, issues, ADRs, milestone dependencies, and prose agree.
