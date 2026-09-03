# ADR 0013: inject bounded DID caching and split clocks

- **Status:** Accepted
- **Date:** 2026-09-03
- **Decision authority:** issue #45, child of #5 / IDR-006
- **Related work:** #43, #44, #10, #41, #46 and #47

## Context

Resolution inputs/results and method dispatch are portable, but production
wallets need cache behavior without binding the DID domain to a system clock,
store, runtime or chain. Donors independently demonstrate injected clocks and
product caches but no reusable DID resolution cache contract. The current W3C
draft also includes `noCache` and explicitly distinguishes document/version
metadata from resolver-process caching information.

## Decision

1. `identus-core` owns bounded Unix, monotonic and duration millisecond values
   plus independent fallible `WallClock` and `MonotonicClock` ports.
2. DID caching uses monotonic time only. Entries are process-epoch values and
   do not serialize.
3. `ResolutionOptions` represents optional `noCache`; true bypasses cache reads
   and writes, while the control does not participate in cache identity.
4. `identus-did` owns a normalized bounded request key, immutable TTL/failure
   policy, typed entry/status, object-safe cache port and opt-in resolver
   decorator.
5. Cache keys include the exact DID and every other result-affecting option;
   nested JSON objects are recursively sorted before bounded encoding.
6. Positive/deactivated results and separately enabled `notFound` failures may
   be cached. Other failures may not. W3C metadata never invents a TTL.
7. Hits are fresh only within monotonic bounds and after request validation.
   Invalid, expired or clock-regressed entries are invalidated, never served.
8. Cache/clock failures follow explicit bypass or fail-closed policy.
9. Per-key and all-options-for-DID invalidation are supported. Backends declare
   and enforce capacity no greater than 4,096 entries.
10. Portable concurrent misses may duplicate upstream calls. Runtime-specific,
    cancellation-aware single-flight is deferred until proven by consumers.

## Consequences

- NeoPRISM and Midnight method resolvers can share one decorator without chain
  dependencies.
- Lace and Oxid gain foundation clock seams compatible with their existing
  dependency-injection direction, while concrete OS adapters remain theirs.
- Cache semantics are deterministic under fake time and do not become stale
  through wall-clock adjustment or process restart.
- Products retain explicit availability, eviction, rate-limit, storage,
  telemetry and finality choices.

## Provenance

NeoPRISM `d6ad1ec`, midnight-identity `427f857`, Lace ID Portal `804de0a`,
Oxid `bfe3b48` and Apollo `ccee22b` were inspected read-only. No donor source
or fixture is copied and no downstream tree is modified.

## Rollback

Revert issue #45's pull request. All behavior is opt-in, unpublished and owns
no persisted data or concrete adapter.
