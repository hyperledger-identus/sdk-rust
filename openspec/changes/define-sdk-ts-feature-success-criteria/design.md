# Design

## Machine record remains primary

The TOML matrix remains the source of truth. Each capability gains concise
`rust_surface`, `target_surface`, `applicable_targets`,
`target_states`, `missing_dimensions`, and `missing_proof` fields. The existing
owner issue owns the missing-proof work. Current SDK-TS behavior is rendered by
joining the canonical inventory's immutable `outcome` and ownership fields;
the parity matrix does not create a second TypeScript source of truth. The
checked Markdown table renders those fields together with relation/state and
known deviations. Offline
validation requires exactly one row per capability, requires a target state for
every applicable target, and prevents a global `delivered` claim while any
applicable target is incomplete. It compares rendered cells with the machine
record so the human table cannot silently drift.

## Runtime targets

The matrix gains four closed target records:

- `browser`: WASM plus browser package/loading and real-browser E2E evidence;
- `nodejs`: WASM plus Node package/loading and runtime-specific E2E evidence;
- `react-web`: the browser artifact integrated through a client application,
  with SSR/hydration additionally routed through the Node.js contract;
- `react-native`: a dedicated New Architecture/TurboModule package over a
  qualified native binding, split into iOS and Android evidence under ADR 0163.

Every target is initially `target`, not `supported`. The target record names
runtime/version matrix, artifact/module format, loader or package subpath, host
ports, consumer fixture, support tier, CI lane, success evidence, and explicit
limitations. React Native remains blocked until its engine, New Architecture,
native adapter, iOS, and Android qualification are pinned.

Every target record cites `SDK-LIM-002` and/or `SDK-LIM-003` where applicable;
no initial target may be `supported`.

## Layered CI graph

The success contract defines one `core-rust` job and isolated downstream
target jobs within the one required `fast` policy signal. Each target record
names its job. A later workflow change will:

1. run the current Rust factory/build/lint/test verdict first;
2. run a fail-closed exact-head binding-impact plan over API/schema hashes,
   transitive bound crates, lockfiles, generated mappings, packages, contracts,
   and paths;
3. fan out selected browser, Node.js, React web, React Native, and package/type
   jobs with `fail-fast: false` and exact source/lock identity;
4. keep target caches, artifacts, logs, timeouts, retries, and statuses
   independent; and
5. emit inapplicable receipts for unselected targets without their toolchains;
   and
6. aggregate selected results into the existing one protected `fast` signal.

The core job result never depends on an adapter job. A red selected adapter job
prevents that target from being delivered and makes the protected aggregator
red, but does not erase green core evidence or suppress sibling results.
The core package set is structurally closed against inward binding/npm
dependencies. Public core API or mapping/generator changes select every
dependent target; unknown impact selects all target jobs.

## Success criteria

The matrix gains a closed ordered set of criteria for:

1. the portable Rust core and thin standards-oriented TypeScript facade;
2. target build/package/load/E2E evidence;
3. applicable SDK-TS E2E reuse through a complete donor-test ledger;
4. a separate Rust-owned evidence ledger for deviations and additions;
5. coverage and negative/resource-bound exercise;
6. strict TypeScript and validated Rust type safety;
7. shared core-crate semantics, generated mappings, and no duplicated semantics; and
8. size/performance, diagnostics, observability, fallback, and rollback; and
9. Rust-core-first CI with isolated, impact-routed target verdicts.

Each record names an objective pass condition, evidence classes, applicability,
bounded not-applicable policy, and non-claims. A not-applicable disposition
requires an owner, rationale, review/expiry trigger, and reconsideration rule.
Property, fuzz, benchmark, and differential obligations reference stable
quality-evidence declaration IDs rather than duplicating commands in prose.
The checker rejects missing IDs, target references, empty evidence, forbidden
legacy exported names, and a success claim without complete criteria.

## E2E applicability ledger

The criteria require, but this change does not create, a later test-level
ledger keyed by donor revision, source path, suite/test ID, and content hash.
Donor dispositions are `shared`, `adapted`, `platform-owned`, `superseded`, and
`excluded`; a separate Rust evidence ledger uses `rust-addition` and
`rust-deviation`. This prevents TS1 from hiding unsupported tests or counting
platform-only journeys as portable Rust failures.

## Verification

Focused checks cover the matrix checker and mutation suite, report parity,
strict OpenSpec validation, documentation build, and full factory structure.
The local review verifies that every capability description remains consistent
with the source inventory and that targets are described as future evidence,
not activated support.
