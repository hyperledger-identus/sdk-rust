# Define SDK-TS feature success criteria

## Why

TS1 already classifies all 24 normalized SDK-TS capabilities by target relation
and evidence state, but its human report does not yet provide the maintained
side-by-side feature view needed for roadmap steering. The milestone also lacks
an executable definition of success for the Rust-backed TypeScript SDK across
browser, Node.js, React web, and React Native consumers.

Without those contracts, "parity" can drift back toward identical historical
module names, an unqualified WASM claim for every JavaScript runtime, or a test
count that does not distinguish reused SDK-TS journeys from intentional Rust
deviations.

## What changes

- Extend each capability record with concise current Rust behavior, target
  TypeScript surface, applicable targets, per-target evidence state, and
  structured missing-proof dimensions. Current SDK-TS behavior remains joined
  from the pinned canonical inventory instead of being copied.
- Add a checked human comparison table that covers every capability exactly
  once and keeps known deviations visible.
- Define browser, Node.js, React web, and React Native target contracts without
  assuming that one WASM loading strategy works in every runtime.
- Define a layered CI graph where the Rust-core job runs first, an exact-head
  impact plan selects isolated target jobs, and the existing single protected
  `fast` verdict aggregates only applicable results.
- Define a thin, standards-oriented TypeScript facade that does not export the
  historical component names used by SDK-TS.
- Require an applicability inventory for SDK-TS E2E tests and reuse the same
  TypeScript journeys against the Rust-backed route where behavior is meant to
  match.
- Require Rust E2E/conformance evidence for additional, safer, or intentionally
  different behavior, plus selected TypeScript boundary smoke tests.
- Add explicit coverage, type-safety, maintainability, portable-core reuse,
  performance, and rollback success criteria.
- Extend offline validation and mutation coverage for the target and success
  contracts.

## Capabilities

### Modified capability

- `sdk-ts-parity-conformance`: maintain the cross-language feature table and
  define objectively reviewable TS1 exit criteria.

## Non-goals

No runtime source, binding, package, consumer, E2E suite, or public API is
implemented by this change. It does not activate support, publish npm or Rust
artifacts, select a React Native bridge, start mdoc, preserve historical SDK-TS
module names, change protected required checks, or claim that every SDK-TS test
is applicable.

## Delivery

Issue #594 remains the definition issue and #502 remains the implementation
program. This change is a behavior-neutral roadmap/checker update on the local
TS1 branch. Later capability trains implement the criteria and retain their own
target, security, conformance, release, and rollback authority.
