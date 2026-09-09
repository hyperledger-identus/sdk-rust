# ADR 0103: do not adopt published Aries Askar as an SDK storage adapter

- **Status:** Accepted
- **Date:** 2026-09-09
- **Decision owners:** issue #162 and parent #151
- **Constraints:** `SDK-ARCH-001`, `SDK-ARCH-002`, `SDK-SEC-001` through
  `SDK-SEC-003`, `SDK-COMPAT-001` through `SDK-COMPAT-005`,
  `SDK-DELIVERY-001`, `SDK-LIM-001`, `SDK-LIM-003`, `SDK-LIM-005` through
  `SDK-LIM-007`, `SDK-LIM-009`

## Context

Aries Askar is an actively maintained encrypted SSI record/KMS store. It could
avoid reimplementing encryption-at-rest and database mechanics, but its
published top-level package combines storage with a broad cryptographic package
and exposes backend/session semantics that must not become the generic SDK
contract. Its entries also have no public revision token, while the Identus
storage ports require compare-and-swap revisions.

Published 0.4.6 is the only immutable crates.io candidate. Upstream tags 0.5.0,
but no corresponding crate is available. A source branch is not accepted as a
production or research dependency merely to obtain newer code.

## Decision

1. Classify published `aries-askar 0.4.6` as `not-adopt` for SDK production
   dependencies. Preserve the nested fixture as reference evidence only.
2. The fixture proves technical adaptability: an SDK-owned revision envelope
   and collision-free name mapping pass all 16 exact-record `SecretStore`
   operations over encrypted in-memory SQLite, including stale-write and
   stale-delete preservation.
3. That proof does not overcome the package boundary. Even with defaults off
   and only `sqlite`, the host normal/build tree contains 192 unique rendered
   package/version lines and the lock contains 256 packages. The top-level
   crate unconditionally selects broad `askar-crypto` key suites; storage-only
   consumers cannot feature-slice them away.
4. The public record model has no revision token. A conforming adapter must own
   and persist revisions, transactional compare-and-swap logic, namespace
   encoding and closed error lowering. Askar therefore does not remove the
   highest-risk SDK-specific storage mechanics.
5. The exact graph compiles for host, iOS arm64 and Android arm64 when explicit
   Xcode/NDK C toolchains are supplied. WASM fails before SQLite at
   `getrandom 0.2.17`, so it is a native-only candidate. These are compile
   receipts, not runtime support.
6. `cargo deny` passes the selected feature graph. `cargo audit --deny warnings`
   reports RUSTSEC-2023-0071 for `rsa 0.9.10` in the lock even though
   `cargo tree --target all -i rsa` proves it is not reachable under selected
   features. Research may record that nuance; production admission still
   requires a clean policy decision rather than silently ignoring it.
7. Keep Askar, SQLite, Tokio/SQLx, candidate types and errors out of generic
   crates, root locks, fast CI and public APIs. Treat unpublished 0.5.0 and main
   as maintenance/reference evidence only.
8. The fixture's process-local revision counter and `String` value type prove
   interface compatibility only. They do not establish restart-safe revisions,
   cancellation recovery, durable secret zeroization or production custody.

## Consequences

The SDK avoids a 192-line native/runtime/crypto dependency cone for an adapter
that still needs SDK-owned revision and transaction policy. Consumers may
implement their own Askar adapter outside the SDK using the fixture as a
reference, but no support or compatibility follows from this evidence.

The research is independently reversible and does not alter consumers or
release artifacts.

## Reconsideration

Open a new bounded issue only when a published immutable release:

- feature-slices encrypted storage from unrelated KMS algorithms and FFI;
- exposes a stable atomic revision or compare-and-swap primitive, or a named
  consumer proves the remaining SDK-owned envelope is acceptable;
- has a feature-aware clean advisory policy and explicit native target matrix;
- proves persistent migration, cancellation, crash recovery and secret-key
  ownership for a named consumer; and
- materially improves on a consumer-owned adapter after measured size and
  performance evidence.
