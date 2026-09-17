# Constraints and limitations

Impact class: routine
Decision status: not-required
Decision reference: not-required
Constraint blockers: none

## Existing entries affected

- `SDK-SUPPLY-001`: existing exact action, Cargo, Nix, and npm pins remain
  unchanged.
- `SDK-LIM-004`: the temporary Rust 1.98.1 fast/slow policy remains effective;
  this factory-only slice neither activates release compatibility nor adds PR
  matrices.
- ADRs 0003/0004: supervisor-owned eligible `develop` merge remains delegated,
  while releases, publication, settings, and `main` remain protected.
- ADR 0108: repository policy continues to outrank external factory guidance.

## Introduced or changed constraints

None. The change enforces existing PR metadata, merge, exact-head, protection,
and recoverability requirements through a local facade.

## Introduced or changed limitations

Superseded closeout does not prove semantic patch equivalence. It proves the
local deletion is recoverable because the exact head remains at the validated
remote branch, the original PR is closed, and the replacement PR merged to
`develop` while explicitly closing the owning issue. Network-unavailable
hosted evidence blocks merge and closeout. Local receipts cannot replace the
authoritative GitHub object or release receipt.

## Consumer and product impact

No SDK or downstream consumer contract changes. Agents receive earlier local
feedback and a safer, more auditable terminal delivery path.

## Activation and rollback

Activation requires deterministic tests, a real bounded Pi canary, factory and
OpenSpec gates, distinct local review, a signed/DCO PR, and green exact-head
hosted `fast` evidence. Rollback removes the new command/mode and returns to
manual supervisor operations without deleting historical receipts or refs.

## Evidence

Issue #320 and PR #319 supply measured process evidence. Existing effective
governance authorizes reversible repository-local factory maintenance. No new
material product, compatibility, security, license, certification, release, or
publication outcome is activated.
