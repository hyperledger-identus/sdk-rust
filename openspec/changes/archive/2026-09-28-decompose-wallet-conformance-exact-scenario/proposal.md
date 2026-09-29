# Decompose the exact-store conformance scenario

## Why

`crates/wallet-conformance/src/lib.rs::run_exact` owns 250 production SLOC,
cognitive complexity 18, and cyclomatic complexity 45. It mixes six storage
lifecycle invariants with aggregate evidence counting. The canonical
code-health report classifies this exact hotspot as `decompose`, and issue #438
is the implementation owner left by #270.

## What changes

- Keep the crate root as the public facade and retain every public path.
- Give exact-record fixtures, driver mechanics, lifecycle phases, and evidence
  counting one private module owner.
- Give list fixtures and bounded traversal one separate private owner instead
  of leaving both scenarios in the crate facade.
- Characterize the exact 16-call successful transcript and existing static
  failures before changing production ownership.
- Preserve operation order, conditional mutation semantics, redaction,
  runtime neutrality, and the existing report.
- Refresh canonical code-health evidence without relaxing unrelated signals.

## Capability

### Modified capability

- `wallet-storage-conformance`: exact-record lifecycle behavior gains cohesive
  private phase ownership and an auditable operation transcript while its
  public contract remains unchanged.

## Non-goals

No new storage semantics, public API, adapter, executor, synchronization,
single-flight, cancellation, pagination redesign, dependency, feature, wire
format, performance promise, or consumer migration.

## Delivery

Issue #438 owns this slice. Planning and exact-head preimplementation evidence
precede production edits. Characterization, public/source compatibility,
wallet/workspace tests, strict lint, code-health, factory, portable/Nix, and
protected exact-head CI evidence gate integration.
