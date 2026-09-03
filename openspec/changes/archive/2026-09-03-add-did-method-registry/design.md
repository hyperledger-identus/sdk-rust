## Context

The W3C DID Resolution Candidate Recommendation Draft dated 28 August 2026
defines `methodNotSupported` when a resolver does not support the DID method and
`featureNotSupported` when a requested feature is not supported. The SDK query
ports already return those standards-shaped envelopes. This slice needs only a
deterministic composition root between validated method names and independent
port implementations.

Immutable evidence inspected before implementation:

- NeoPRISM `d6ad1ecade80757f08da4f9101d14c2fb1a4d02b`, Apache-2.0:
  one `DidResolver` implementation boundary; no generic method registry.
- midnight-identity `427f8571950c42967a18726cbcbefecc19ef8d79`,
  Apache-2.0: one Midnight resolver/registrar boundary; method-specific state
  and VDR semantics are rejected from the SDK registry.
- Lace ID Portal `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1`,
  evidence-only: `Arc<dyn ...>` registry demand is useful, while vector scan,
  runtime macros and unresolved license provenance are not adopted.
- Oxid `bfe3b481568dc738f0732c2b27548fab8721fd95`, Apache-2.0:
  one boxed-future resolver injection seam; product errors and Midnight
  provenance are rejected.

No donor code or fixture is copied and no downstream tree is modified.

## Goals / Non-Goals

**Goals:**

- Compose up to 64 validated DID methods without chain knowledge.
- Reject duplicate ownership at construction time.
- Keep the built registry immutable, cheaply cloneable and safe for concurrent
  reads without locks.
- Route resolution and optional dereferencing by exact method name.
- Preserve the query ports' single W3C result/error channel.
- Make registered method ordering deterministic and observable.

**Non-Goals:**

- Runtime registration, plugins, unloading, priorities or fallback chains.
- Cache, freshness, clock, retry, timeout, telemetry or circuit breaking.
- DID URL dereferencing algorithms, recursive resolution or authorization.
- HTTP/content negotiation/SSRF behavior or method discovery over a network.
- DID Registration, storage, chain/VDR implementations or product trust.

## Decisions

### Decision 1: separate immutable binding, builder and registry

`DidMethodBinding` owns a validated `DidMethod`, an `Arc<dyn DidResolver>` and
an optional `Arc<dyn DidUrlDereferencer>`. A binding requires resolution but
keeps dereferencing optional because dereferencing remains at risk and method
implementations may support resolution only.

`DidMethodRegistryBuilder` is the only mutation surface. Registering a binding
rejects an already-owned method and rejects a 65th entry. `build` freezes the
entries into an `Arc<BTreeMap<String, DidMethodBinding>>`; clones therefore
share an immutable deterministic lookup table and concurrent dispatch requires
no mutex. An empty registry is valid and deterministically reports unsupported
methods.

The map key is the exact validated method spelling. A `String` key allows
allocation-free lookup by the borrowed `&str` exposed by `Did` and `DidUrl`.
Method adapters cannot register aliases or wildcard handlers in this slice.

### Decision 2: expose redaction-safe setup failures

`RegistryError` distinguishes duplicate method ownership from capacity
exhaustion without carrying the attacker- or configuration-supplied name.
`Error::InvalidRegistry` bridges through stable code
`did.invalid_method_registry`; duplicate ownership maps to `Conflict`, while
capacity exhaustion maps to `InvalidInput`. Invocation never returns this setup
error because a built registry is immutable and valid.

### Decision 3: implement the existing query ports directly

`DidMethodRegistry` implements `DidResolver` and `DidUrlDereferencer` rather
than defining a second dispatcher interface. A registered resolver receives
the original borrowed DID/options and its W3C result is forwarded unchanged.
An unknown DID method returns `methodNotSupported`. A known method without a
dereferencer returns `featureNotSupported`; an unknown dereferencing method
still returns `methodNotSupported`.

This keeps chain adapters responsible for producing valid result envelopes and
does not add an outer `Result`, adapter-error taxonomy or async runtime. Deeper
result integrity and raw-wire hardening remain #41.

### Decision 4: deterministic introspection is narrow

The registry exposes count, emptiness, exact membership, dereferencing support,
and a sorted iterator of method-name strings. It does not expose or downcast
the registered trait objects. This is enough for composition diagnostics and
tests without turning the registry into a service locator.

## Threat Contract

**Assets:** unique method ownership, correct dispatch, immutable composition,
bounded setup cost, result-channel consistency and chain-neutrality.

**Threats addressed:** duplicate shadowing, nondeterministic precedence,
unbounded handler registration, method-prefix confusion, mutable-registry
races, unsupported-method ambiguity and leakage of configuration values through
the stable error bridge.

**Residual boundaries:** registered adapters are trusted to honor their port
contract; the registry does not catch panics, validate ledger proofs, apply
timeouts, prevent network SSRF, enforce result freshness, authorize resources,
or choose wallet trust policy.

## Test and Verification Strategy

- Reject duplicate registrations and a 65th entry with stable redacted errors.
- Assert sorted introspection and exact `prism` versus prefix-shaped method
  routing.
- Route PRISM and Midnight resolution through `Arc<dyn DidResolver>` and route
  optional dereferencing through `Arc<dyn DidUrlDereferencer>`.
- Assert W3C `methodNotSupported` and `featureNotSupported` results.
- Exercise cloned registries concurrently without mutation or a runtime.
- Record release-mode lookup/dispatch throughput without a CI threshold.
- Run focused, workspace, MSRV, mobile/WASM, docs, lint, supply-chain,
  OpenSpec, factory and full Nix gates before an exact-head review.

## Migration Plan

1. Land this issue-linked OpenSpec delta and ADR before implementation.
2. Add the setup types, error bridge, port implementations and conformance
   tests without touching downstream repositories.
3. Record performance, complete an independent semantic/security review, sync
   the canonical spec and archive the change.
4. Merge only after exact-head hosted CI is green.
5. Consume the registry from #10 and later method/adoption work separately.
