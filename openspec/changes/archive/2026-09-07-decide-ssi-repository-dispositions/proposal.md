## Why

The SDK already has a general narrow-engine/owned-facade rule, but agents still
have to reinterpret broad SSI repositories each time they start a component.
The sponsor supplied a wider repository inventory and requested one durable
decision per repository: conditionally adopt, spike, use as an oracle, or reject.

## What Changes

- Add a pinned evidence snapshot for twelve Rust SSI repositories.
- Add one small ADR per repository with permitted uses, prohibited coupling,
  and an objective reconsideration trigger.
- Specify a canonical repository-disposition contract for future agents.
- Refine repository-specific dispositions in ADR 0061 and the negative ledger
  without adding a dependency or activating a conditional roadmap capability.

## Capabilities

### New Capabilities

- `ssi-repository-disposition`: Requires an explicit, evidence-pinned decision
  before an SSI repository becomes a dependency, source donor, fixture source,
  differential oracle, or architectural reference.

### Modified Capabilities

None.

## Impact

This is an architecture and dependency-policy documentation change linked to
issue #175 and discussion #174. It creates no runtime/public API, target,
release, or downstream change. Future adoption remains separately issue-first.
