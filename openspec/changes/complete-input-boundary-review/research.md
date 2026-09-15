# Research readiness

Research class: corrective
Research status: ready
Decision date: 2026-09-16
Source retrieval date: 2026-09-16
Research blockers: none

## Evidence and decision

PR #296 hosted review identified the exact public surfaces at
`crates/did/src/registry.rs`, `crates/did/src/cache.rs`, and
`crates/crypto/src/jwk.rs`. `DidMethodRegistryBuilder` retains up to
`MAX_DID_METHOD_REGISTRY_ENTRIES`. `CachedDidResolver` validates exact cache-key,
capacity, and TTL ceilings while calling injected `DidResolutionCache` and
`MonotonicClock` ports whose operational work cannot receive a truthful
runtime-neutral quota.

The JWK validator walks borrowed JSON iteratively, but an owned rejected
`serde_json::Value` still uses its recursive destructor on return. The SDK owns
that cleanup after `from_parts` is called. A private ownership guard with
iterative dismantling is adopted; changing the public signature, leaking the
map, accepting hostile depth, or adding a dependency is rejected.

## Compatibility and security

The change adds inventory evidence and private cleanup code only. Public types,
constants, errors, feature defaults, dependency edges, wire and persistence
forms are unchanged. Rejection remains redacted. Accepted extension trees stay
bounded to depth 16 and 1,024 nodes. No authored unsafe Rust or native code is
introduced.

## Reconsideration triggers

- `serde_json::Value` guarantees non-recursive destruction.
- DID registry/cache surfaces or ownership are removed or materially changed.
- A concrete SDK cache adapter acquires enforceable storage/time semantics.
