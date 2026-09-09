# Constraint and limitation impact

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/215
Constraint blockers: none

## Existing entries affected

`SDK-LIM-002` remains effective: this research does not activate an FFI
surface. `SDK-LIM-003` remains effective because cross-compilation does not
become device runtime or packaging support. `SDK-SEC-001` continues to forbid
authored unsafe code, while dependency-owned unsafe/native reach is recorded
as adoption evidence. `SDK-SEC-003` continues to require DID/DID URL input
bounds. `SDK-COMPAT-002`, `SDK-COMPAT-004` and `SDK-COMPAT-005` require the
Rust 1.98.1 etalon. `SDK-ARCH-001` keeps chain and product policy out of the
binding facade.

## Introduced or changed constraints

No effective repository-wide value changes. ADR 0097 will constrain a future
native binding implementation to an isolated crate, proc-macro definitions,
library-mode generation, SDK-owned DTOs/errors and domain crates with no
UniFFI dependency. Generator dependencies and generated sources must remain
outside the production runtime cone unless a later implementation issue
demonstrates why they are required.

## Introduced or changed limitations

- Native host smoke evidence is not iOS/Android device, packaging, store or
  certification support.
- Generated Swift/Kotlin API snapshots are research evidence, not stable ABI.
- React Native, Node and browser React remain unsupported and require separate
  versioned runtime/bundler decisions.
- This value-only spike proves no object ownership, concurrency, callback,
  async cancellation, panic containment for arbitrary code or secret handling.

## Consumer and product impact

There is no current consumer migration, public API, wire value, persisted
state or product behavior change. The chosen direction allows future Swift and
Kotlin consumers to share generic DID syntax without coupling the SDK core to
mobile frameworks. Oxid, Midnight, NeoPRISM, Lace and Apollo remain unchanged.

## Activation and rollback

The research decision becomes repository guidance after its issue-linked PR
merges to `develop`. Production activation remains #163 and its native child
issue; it must replace `SDK-LIM-002` only with runtime/package evidence and a
versioned contract. Reverting this research PR removes the fixture and decision
without code, data or consumer rollback.

## Evidence

Evidence includes pinned versions/revisions/licenses, exact normal and build
dependency cones, reachable unsafe/native inventory, Rust 1.98.1 build/tests,
generated output hashes/diff, Swift and Kotlin runtime tests, stable redacted
error assertions, root manifest/lock invariance and a distinct local review.
