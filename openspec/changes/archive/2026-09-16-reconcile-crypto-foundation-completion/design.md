# Design

## Completion model

Create one human-readable architecture report whose rows map each issue #286
comparison class to implementation, vector/test/CI evidence, disposition,
exact revision, and limitation. Reuse immutable receipts rather than rewriting
the Apollo parity manifest, whose baseline is intentionally frozen to M2.

Mark IDR-004 delivered because functional and conformance acceptance is
complete. Keep four independent axes visibly false: registry release,
production adoption, supported bindings/runtime targets, and Apollo/NeoPRISM
lifecycle. Keep #298 and #299 as non-functional hardening work; their existence
does not turn already evidenced crypto/JOSE behavior back into an absent
capability.

## Repository changes

- Add `docs/architecture/crypto-foundation-completion.md` as the exact audit.
- Change only IDR-004's `delivery_status` from `in_progress` to `delivered`.
- Add a canonical SSI-program requirement describing the completion boundary.
- Update roadmap/B08 prose from active implementation to delivered foundation
  with separate release/adoption owners.
- Preserve the executable Apollo manifest and its pinned M2 revisions.

## Remote reconciliation

After merge, post the merged report and decision to discussion #178. Reconcile
epic #8 by recording every child closed and moving its publication statement
to IDR-011/protected release authority before closing it as an implementation
epic. The supervisor performs these mutations; a Pi worker has no GitHub
authority.

## Risks and mitigations

- `delivered` could be read as released: every summary repeats publication and
  support limitations and the canonical requirement forbids that inference.
- Open hardening could be hidden: #298/#299 are table rows with explicit scope.
- Frozen parity evidence could become stale: the audit binds its own base and
  points to the immutable M2 baseline rather than rewriting history.
- Epic closure could lose remaining work: bindings, release, adoption, input
  hardening, and downstream lifecycle retain independent open owners.

## Rollback

Revert the report, backlog state, roadmap text, and canonical requirement.
Remote discussion/issue reconciliation receives a correcting comment if the
merged decision is later reverted. No runtime or consumer migration is needed.
