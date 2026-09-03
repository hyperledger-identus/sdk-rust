# Change: add an injectable DID resolution cache and reusable clocks

## Why

The SDK now has bounded W3C resolution inputs/results, object-safe query ports,
and deterministic multi-method dispatch. Every production consumer also needs
freshness and cache behavior, but the inspected repositories either have no
resolution cache or embed product-specific clocks and caches. Without a shared
contract, NeoPRISM, midnight-identity, Lace and Oxid will recreate incompatible
keying, expiry, failure and invalidation semantics.

The current W3C DID Resolution draft permits generic and method-specific caches
and defines optional `noCache`. It does not define a universal TTL, and its
`nextUpdate` metadata describes a subsequent update of a resolved historical
document version. Treating that field as cache expiry would be incorrect and
could serve stale security material.

## What changes

- Add reusable, split `WallClock` and `MonotonicClock` ports and millisecond
  timestamp/duration values to `identus-core`.
- Add optional `noCache` to `ResolutionOptions` with its current W3C spelling
  and absent/false/true semantics.
- Add a bounded, normalized, redaction-safe cache key over a validated DID and
  all result-affecting resolution options except `noCache`.
- Add bounded policy, entry, status and object-safe cache-port types.
- Add an opt-in caching resolver decorator. Positive/deactivated results and
  explicitly enabled `notFound` failures are cacheable; other errors are not.
- Use monotonic expiry only, validate hits against the request, invalidate
  expired/regressed/invalid entries, and make cache failure behavior explicit.
- Record deterministic fake-clock/backend tests, concurrency behavior and
  release-mode performance evidence.

## Boundaries

- Existing `DidResolver` and `DidMethodRegistry` remain independently usable.
- No system clock, cache backend, async runtime, transport, VDR, method,
  product telemetry or chain policy enters a domain crate.
- The portable base allows duplicate concurrent fills. Cancellation-safe
  single-flight belongs in an optional runtime adapter/coordinator.
- Concrete persistence, distributed invalidation, HTTP cache headers,
  dereferencing recursion and registration lifecycle remain separate work.
- No downstream repository is modified and no donor source is copied.

## Delivery

Issue #45 precedes implementation. This change is delivered with ADR 0013,
signed+DCO commits, deterministic tests, performance evidence, a distinct
semantic/security/API review, full local Nix validation and exact-head hosted
CI before merge to `develop`.
