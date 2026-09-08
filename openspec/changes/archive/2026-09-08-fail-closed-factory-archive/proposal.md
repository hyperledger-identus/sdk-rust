## Why

`scripts/factory archive` currently trusts only the exit status of the pinned
OpenSpec command. OpenSpec can return zero after declining to archive a change,
for example when it detects a canonical-spec conflict. The wrapper then runs
the general factory check and prints `archived safely` even though the active
change was not moved. That success claim is false and can mislead an autonomous
delivery agent about durable specification state.

## What Changes

- Define explicit postconditions for a successful factory archive.
- Reject a dated-archive destination collision before invoking OpenSpec.
- Require the active change to disappear, the expected dated archive to exist,
  and all mandatory change artifacts to survive before reporting success.
- Add hermetic regression coverage for zero-exit/no-op, destination-collision,
  and successful state-transition behavior without parsing OpenSpec prose.
- Preserve the existing readiness, loss-prevention and post-archive factory
  validation gates.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `ai-software-factory`: archive completion becomes a verified state
  transition rather than an inference from a child process exit status.

## Impact

This changes only the repository-owned delivery facade and its contract tests.
It introduces no Cargo dependency, SDK API, protocol, wire-format, persisted
data, supported target or downstream consumer change. Existing automation that
encounters a no-op archive will now stop truthfully instead of continuing from
a false success receipt.
