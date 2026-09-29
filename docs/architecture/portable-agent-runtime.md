# Portable Rust agent runtime target

**Status:** target architecture; no supported runtime exists yet

**Decision:** [ADR 0171](../adr/0171-own-a-portable-effect-driven-agent-runtime.md)

**Owner:** [issue #498](https://github.com/hyperledger-identus/sdk-rust/issues/498)

## Purpose

SDK-Rust needs enough orchestration to run the same identity protocol safely
in a browser, Node, mobile app, native wallet, test harness, or service. It does
not need a second application framework. The target is a small deterministic
kernel that owns reusable state and effect semantics while the host owns I/O,
threads, storage engines, secure keys, user interaction, and deployment.

This design is informed by SDK-TS and SDK-Swift behavior, VCX's separation of
protocols/messages/wallets/agents, One Core's provider/service split, and the
dispatch/event/service behavior visible in Credo, Aries Framework Go, Cloud
Agent, and Mediator. Those implementations are oracles; none is the module
template.

## Responsibility boundary

```text
product / language SDK / service
  consent · policy · UI · lifecycle · tenancy · deployment
                         │
                         ▼
host executor and adapters
  network · store · clock · entropy · keys · telemetry · async runtime
                         │ typed EffectResult
                         ▼
portable identus agent kernel
  dispatch · ordering · correlation · cancellation · checkpoint contract
                         │ typed EffectRequest
                         ▼
versioned protocol state machines
  DIDComm applications · OID4VC workflows · future portable protocols
                         │
                         ▼
SDK-Rust domain and protocol primitives
```

## Minimal conceptual API

The names below describe responsibilities, not accepted public Rust syntax.

| Concept | Responsibility | Excludes |
| --- | --- | --- |
| `AgentEvent` | bounded versioned input with instance, correlation and causation identity | ambient callbacks and unbounded host objects |
| `ProtocolInstance` | protocol/version/role and versioned opaque state | product workflow or UI state |
| `Transition` | deterministic next state plus ordered effect requests | I/O execution and wall-clock reads |
| `EffectRequest` | typed request to send/resolve/load/store/schedule/ask/call | raw closure, generic JSON command bus, or secret serialization |
| `EffectResult` | bounded result correlated to exactly one request/instance | host exceptions leaking through the core |
| `Checkpoint` | versioned atomic state/event position | concrete database schema |
| `AgentRuntime` | fair bounded dispatch, cancellation, deadlines, idempotency and checkpoint sequencing | Tokio, browser event loop, service lifecycle or multitenancy |
| `Executor` | performs effects and returns results | protocol validation and transition logic |

## Invariants to specify before implementation

- One event advances one protocol instance atomically or leaves it unchanged.
- Effects have stable identifiers and cannot be applied to another instance.
- Cancellation and deadline behavior is explicit at every asynchronous edge.
- Retries are requested by protocol/runtime policy; adapters do not silently
  repeat non-idempotent effects.
- Queue count, event bytes, state bytes, effects per transition, nesting,
  checkpoint size, timer count, and transition work have explicit limits.
- Persistence recovery cannot replay an acknowledged non-idempotent effect
  without an idempotency/reconciliation contract.
- Debug, errors, events, checkpoints, and telemetry never expose raw secrets,
  tokens, credentials, link secrets, or user decisions by default.
- Unknown protocol/state versions fail closed and remain migratable.
- Host scheduling differences do not change deterministic transition results.

## Port families

| Port | Rust-owned contract | Host-owned policy |
| --- | --- | --- |
| transport | bounded request/message and response/result envelope | TLS, sockets/fetch, proxy, DNS, redirects, decompression, connection pooling |
| DID resolution | typed query/options/result and cancellation | method/network adapter, cache persistence, endpoint trust |
| storage | transaction/checkpoint/repository semantics | database, encryption at rest, backup, replication |
| clock/timer | monotonic/wall-time values and timer intent | system clock, event loop, wake-up mechanism |
| entropy/key | purpose-bound request and opaque handles | RNG, keychain/HSM, custody and authorization |
| user decision | bounded prompt/response contract | UI, accessibility, consent and product policy |
| telemetry | privacy-safe event schema | sink, sampling, retention and operations |

## Target adaptation

The deterministic single-threaded executor is the first proof because it makes
ordering and failure reproducible. Later proofs map the same fixture to:

- a native Rust async executor;
- a WASM promise/event-loop executor;
- data-oriented UniFFI commands for Swift/Kotlin hosts; and
- Node/browser adapters selected by issue #499.

The foreign interfaces exchange commands, bounded values, effect requests,
effect results, and opaque handles. They do not export arbitrary Rust traits or
require each language to reimplement protocol transitions.

## Research questions owned by #498

1. Is an event-sourced checkpoint, state snapshot, or hybrid the smallest
   portable recovery contract?
2. Which effect set is sufficient for the first two unrelated protocols?
3. How are atomic state/effect intent and non-idempotent external effects
   reconciled after crashes?
4. Which concurrency model preserves fairness without requiring `Send` on
   every WASM value?
5. Which public error categories survive FFI without leaking host details?
6. How are protocol/state schema migrations registered and rolled back?
7. Which metrics establish dispatch, checkpoint, memory, and E2E QoS budgets?

## Evidence plan

- property: ordering, correlation, idempotency, cancellation, and recovery
  invariants over generated transition sequences;
- fuzz: event decoding, state migration, effect-result correlation, and
  checkpoint recovery;
- benchmark: transition latency, queue throughput, allocation, checkpoint
  size/time, and target overhead; and
- differential: identical fixture results across the reference, native async,
  and WASM-shaped executors plus selected SDK/service oracles.

No runtime crate or public API should be created until #498 pins the first two
consumer protocols and proves that this kernel is smaller than duplicating
their own orchestration.
