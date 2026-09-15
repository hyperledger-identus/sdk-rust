# Research readiness

Research class: cryptography-security
Research status: ready
Decision date: 2026-09-16
Source retrieval date: 2026-09-16
Research blockers: none

## Problem and existing implementation

PR #296 hosted review identified exact public surfaces omitted from the audit at
`crates/did/src/registry.rs` and `crates/did/src/cache.rs`.
`DidMethodRegistryBuilder` retains up to
`MAX_DID_METHOD_REGISTRY_ENTRIES`. `CachedDidResolver` validates exact cache-key,
capacity, and TTL ceilings while calling injected `DidResolutionCache` and
`MonotonicClock` ports whose operational work cannot receive a truthful
runtime-neutral quota.

The JWK validator walks borrowed JSON iteratively, but an owned rejected
`serde_json::Value` still uses its recursive destructor on return. The SDK owns
that cleanup after `from_parts` is called, including when another profile or
member error occurs before resource validation.

## Normative sources

- Issue #168 and PR #296 hosted review threads are the directed correction.
- ADR 0125 and `openspec/specs/sdk-input-resource-governance/spec.md` require
  distinct implemented families and truthful ownership.
- `openspec/specs/crypto/spec.md` defines standalone JWK budgets and redaction.
- `crates/did/src/cache.rs`, `crates/did/src/registry.rs`, and
  `crates/crypto/src/jwk.rs` are the assessed implementation evidence.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Private guard with iterative dismantling | `adopt` | Closes every owned-map early return without public API change. | `serde_json::Value` guarantees non-recursive destruction. |
| Separate registry, cache-policy, and adapter rows | `adopt` | Existing limits and caller-owned QoS are distinct families. | Those surfaces are removed or ownership materially moves. |
| Add a recursive cleanup helper | `not-adopt` | It preserves the process-stack exhaustion path. | Never; it contradicts the boundary. |
| Impose portable cache backend time/byte quotas | `defer` | Generic ports do not own concrete storage or runtime semantics. | A supported concrete SDK adapter exists. |

## Compatibility and dependency evidence

The current implementation revision assessed is
`50d7101c1a258c7a9396c919d9ab5001adf5e0e3`. Consumer evidence is limited to
the repository's explicit Oxid, Midnight, midnight-identity, NeoPRISM, Lace,
and Apollo boundary; none changes this private cleanup or inventory correction.
Primary source URLs are the live
[issue](https://github.com/hyperledger-identus/sdk-rust/issues/168) and
[PR](https://github.com/hyperledger-identus/sdk-rust/pull/296), pinned to that
revision for implementation inspection.

The exact version remains unpublished `identus-crypto 0.1.0-rc.1`; all existing
features, including `jwk` and `jwk-thumbprint`, keep their definitions and
defaults. License and provenance remain the workspace Apache-2.0 policy and
the existing repository source; no donor source or fixture is copied. MSRV and
etalon evidence both remain Rust 1.98.1 for the active-development policy.
Native, WASM, Android aarch64, and iOS aarch64 target gates remain applicable.

The direct and resolved dependency cone is unchanged because no package or
feature edge is added. Supply-chain evidence therefore remains the existing
lockfile, cargo-deny, cargo-audit, SBOM, and source-distribution closure. Public
types, errors, wire and persistence forms are unchanged. The public facade
boundary remains `identus-crypto`; guard and dismantling helpers stay private.

Rollback reverts the private guard/test and inventory rows and restores broad
incomplete-audit disclosure. Maintenance, release and security posture remain
unpublished and unsupported: no publication, tag, support-tier activation, or
consumer migration occurs. Protocol or draft currency is unchanged because
this correction implements ownership/resource semantics rather than a protocol
revision.

## Security, privacy and maintenance evidence

The private guard consumes rejected arrays and objects through an explicit
work stack and never formats their keys or values. Accepted extension trees
remain limited to depth 16 and 1,024 nodes. No authored unsafe Rust or native
code is introduced. Inventory review triggers cover future cache, registry,
and recursive-carrier changes.

## Rejected or deferred candidates

Changing the public constructor to borrowed JSON, leaking rejected maps,
accepting hostile depth, adding a cleanup dependency, and treating generic
cache ports as SDK-budgeted are rejected. Concrete cache-adapter QoS remains
deferred to an adapter-owning slice.

## Open questions and blockers

None. Every hosted finding has a bounded in-repository correction and exact
verification path.

## Evidence commands

- `cargo test -p identus-crypto --test jwk --all-features`
- `scripts/check-input-resource-boundaries.py .`
- `scripts/tests/input-resource-boundaries.py`
- `scripts/factory check complete-input-boundary-review`
- pinned `crypto-candidate` preparation and `nix flake check --fallback`

These exact commands are planned but unrun against remediation code at
research time; results belong in `verification.md` before archive.

## Reconsideration triggers

- `serde_json::Value` guarantees non-recursive destruction.
- DID registry/cache surfaces or ownership are removed or materially changed.
- A concrete SDK cache adapter acquires enforceable storage/time semantics.
