# ADR 0098: establish the experimental UniFFI DID host foundation

- **Status:** Accepted
- **Date:** 2026-09-09
- **Decision authority:** issues #163, #222 and #226 under standing SDK mandate
- **Builds on:** ADR 0097
- **Related work:** issue #226

## Context

ADR 0097 selected a bounded DID/DID URL value slice and UniFFI 0.32 proc macros
after an isolated Swift/Kotlin proof. Production needs a cohesive crate and
versioned ABI before mobile packaging can proceed, but host execution alone
cannot justify a supported iOS/Android distribution claim.

## Decision

1. Add unpublished outer-boundary crate `identus-uniffi-did`. It depends on
   `identus-did` and exact UniFFI 0.32.0; no generic domain crate depends on or
   derives UniFFI types.
2. ABI version `1` exports only DID/DID URL parse functions, owned component
   records and closed errors carrying constant machine-readable codes.
3. Authored wrapper logic catches unexpected unwind and discards the payload
   before returning a constant internal failure. Caller input, domain details,
   secrets and panic payloads do not cross the return boundary.
4. All values are owned. Handles, borrowed memory, callbacks, futures,
   thread-affine state, storage, networking, keys and signing are excluded.
5. Use a separately locked exact-version bindgen tool. Do not unify CLI/template
   dependencies into the root runtime workspace cone.
6. Generate Swift and Kotlin twice from the release library, compare the full
   trees and normalized public API, then compile and run macOS Swift and
   Kotlin/JVM consumers.
7. Run those host checks plus deny/audit for the separately locked generator in
   the weekly slow macOS workflow. Preserve the Linux-only fast PR line during
   the current active-development phase.
8. Keep generated sources untracked and the component experimental. Preserve
   `SDK-LIM-002`; issue #222 must provide versioned XCFramework/SwiftPM and
   AAR/NDK plus iOS/Android runtime evidence before a supported FFI claim.

## Consequences

The SDK gains a reusable production-shaped native seam without contaminating
the DID domain model or claiming mobile delivery prematurely. Consumers can
review a compact ABI and stable error codes. UniFFI adds a material pre-1.0,
MPL-2.0 dependency cone with dependency-owned unsafe/native behavior; exact
pins, locks, supply-chain gates and host execution make that cost explicit.

The process-wide Rust panic hook is a host concern and can run before an unwind
is caught. This decision guarantees that authored wrapper return values discard
panic payloads; it does not claim control of a host application's global panic
logging policy.

The eight UniFFI runtime/generator packages use MPL-2.0. `deny.toml` grants
version-scoped exceptions only to that reviewed 0.32.0 family; MPL-2.0 is not
added to the repository-wide permissive-license allowlist.

## Compatibility and rollback

Breaking exported functions, record fields, error cases/codes or ownership
semantics requires a new ADR, ABI-version bump and consumer migration. Before
publication, rollback removes the crate, generator tool, tests and inventory
entry. `identus-did`, persisted identifiers and downstream repositories remain
unchanged.
