## Context

The repository facade owns the promise that `factory archive` safely completes
an OpenSpec lifecycle. Its preflight is already fail-closed for readiness and
lossy modified requirements, but its mutation boundary equates an OpenSpec zero
exit with completion. The general post-call factory check permits valid active
changes, so it cannot distinguish a completed archive from a no-op.

## Goals and non-goals

Goals:

- make archive success depend on directly observed repository state;
- reject deterministic destination collisions before mutation;
- preserve all required change artifacts and semantic validation;
- cover failure and success behavior hermetically on macOS and Linux shells.

Non-goals:

- parse or stabilize upstream console messages;
- reimplement OpenSpec's canonical-spec merge;
- automatically roll back arbitrary partial upstream mutation;
- change SDK runtime, dependency or public surfaces.

## Decisions

### Capture one deterministic destination

Before invoking OpenSpec, compute the local ISO date and the exact dated archive
path for the requested change. Reject an existing path before mutation. This
keeps the result bound to the requested identity and avoids accepting an
unrelated concurrent archive.

### Assert the complete state transition

After a zero exit, require the active change directory to be absent and the
expected destination to be a directory. Require `.openspec.yaml`, `proposal.md`,
`research.md`, `constraints.md`, `design.md`, and `tasks.md` as regular files
and `specs` as a directory. Only then run the existing broad factory validation
and print the success message.

### Test behavior rather than output prose

Extend the hermetic fixture so its fake OpenSpec first returns zero without
mutation and later performs the expected move. Assert exit status, filesystem
state and the wrapper-owned success marker. Never match upstream error prose.

## Risks and mitigations

- **Midnight boundary:** a legitimate next-day archive can fail the captured
  path assertion. The failure is conservative and retryable; no false success
  occurs.
- **Partial upstream mutation:** the wrapper cannot safely reverse it. The new
  checks stop immediately and leave observable evidence for recovery.
- **Upstream naming change:** the contract test fails and forces an explicit
  facade update rather than silently weakening completion evidence.

## Rollback

Revert the wrapper and contract-test changes. No package, runtime or migration
state is involved, although rollback knowingly restores the reproduced false
success behavior.
