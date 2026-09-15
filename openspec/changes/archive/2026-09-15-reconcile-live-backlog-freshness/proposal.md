## Why

The canonical SSI backlog is structurally valid but is not currently safe as
the sole selector for an autonomous worker. At
`develop@523c64d4ff71d97f71c44b55c99594f065896b56`, `IDR-004` points to
closed foundation child #95 and `IDR-023` points to closed delivery child #250
while both rows remain `in_progress`. The offline checker cannot discover this
because network access is deliberately excluded from the required fast lane.

Issue #285 owns a bounded control-plane correction. It preserves deterministic
offline CI, repairs the two active owner references, and gives the Desktop
supervisor an explicit live audit before work selection or Pi delegation.

## What changes

- Move `IDR-004` ownership to focused completion-audit issue #286 and
  `IDR-023` ownership to its still-open component epic #7.
- Add an explicit, read-only `factory backlog-live` command that compares the
  canonical ledger with live GitHub issue state.
- Require every referenced issue to be visible and every `in_progress` owner
  to be open, while allowing closed delivery evidence for `specified` and
  `delivered` rows.
- Add hermetic snapshot tests for live-state success and failure behavior.
- Document the supervisor selection gate and reconcile the completed native
  DID binding receipts in GitHub.
- Replace deprecated Nix platform predicates without changing dependency or
  target behavior.

## Capabilities

### Modified capabilities

- `ssi-upstream-program`: adds live active-owner freshness before autonomous
  selection and repairs the current active owner references.
- `factory-operations`: exposes a bounded network-explicit supervisor audit
  while preserving the offline required CI contract.

## Non-goals

- No automatic issue mutation, closure, prioritization or implementation.
- No new SDK runtime behavior, public API, dependency or target claim.
- No repository setting, release, publication, `main` or consumer mutation.
- No attempt to infer delivery completeness from issue state alone.

## Delivery

Issue #285 owns this routine, reversible factory-maintenance slice from
`develop@523c64d4ff71d97f71c44b55c99594f065896b56`. The PR targets `develop`.
