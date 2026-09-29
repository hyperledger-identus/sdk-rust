# Characterize and decompose list traversal evidence

## Why

After #438, `crates/wallet-conformance/src/list.rs::run` is the only remaining
wallet-conformance function attention signal: 82 production SLOC, cognitive
complexity 11, and cyclomatic complexity 17. It combines async adapter calls
with five deterministic pagination invariants. Existing tests do not bind each
failure projection independently, so characterization must precede movement.

## What changes

- Add closed test-only faults for page bounds, duplicate entries, membership,
  cursor progress, and bounded termination.
- Keep async list calls and continuation orchestration in the private scenario
  coordinator.
- Give bounded page evidence, cursor history, observed membership, and final
  report construction one private state owner.
- Preserve every public path, operation count, static step/kind, diagnostic,
  dependency, target, and runtime-neutral boundary.
- Refresh canonical code-health evidence without weakening unrelated signals.

## Capability

### Modified capability

- `wallet-storage-conformance`: the private paginated-list scenario gains a
  characterized deterministic evidence boundary while its public contract and
  adapter authority remain unchanged.

## Non-goals

No storage or pagination redesign, sorting guarantee, cursor interpretation,
adapter, executor, synchronization, public API, dependency, feature, wire or
persisted format, allocation/performance promise, or consumer migration.

## Delivery

Issue #441 owns this slice after #438. Planning and an immutable
preimplementation receipt precede production edits. Characterization,
public/source compatibility, focused/workspace tests, strict lint/docs,
code-health, factory, portable/Nix, protected exact-head CI, archive, and
published metrics gate completion.
