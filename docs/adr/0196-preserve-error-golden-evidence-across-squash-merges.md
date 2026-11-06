# ADR 0196: preserve error-golden evidence across protected squash merges

- **Status:** Accepted for issue #578 implementation
- **Date:** 2026-10-29
- **Decision authority:** issue #578 under ADRs 0003 and 0004
- **Related:** ADRs 0118, 0127, and 0195; PR #577
- **Applies to:** repository factory immutable-golden validation only

## Context

The immutable error-golden checker was designed when planning commits remained
in `develop` ancestry. The current guarded delivery path intentionally performs
a protected squash merge. PR #577 passed the feature-branch check, then merged
as single-parent commit `78bd1c01`; its signed planning commit `0f81f09c` is
therefore not an ancestor of `develop`, and the next `scripts/factory doctor`
fails.

Keeping feature branches forever does not restore ancestry and conflicts with
cleanup. Regenerating the receipt at the squash commit would falsely label
implementation as planning-only. Changing repository merge settings is outside
the repair and would couple evidence to administration.

## Decision

1. Pin the complete `preimplementation.json` bytes with a fixed SHA-256 in
   every immutable golden binding.
2. Retain the current exact base-to-contract-to-head, planning-only diff, and
   contract-blob proof whenever the contract commit is in current ancestry.
3. Permit a separate canonical-snapshot path only for an archived change when
   protected squash integration discarded contract ancestry.
4. The snapshot path requires the fixed base in current ancestry, the exact
   receipt hash, the exact archived planning golden, the exact stable golden,
   and the existing fixed golden hash. Each evidence file must also resolve
   byte-for-byte from its exact path in `HEAD`; working-tree copies alone are
   never canonical evidence.
5. If the named contract object is locally reachable, validate its base
   ancestry, planning-only changed paths, and golden blob even though it is not
   a `HEAD` ancestor. A reachable invalid object is a failure.
6. If branch cleanup makes the contract object unreachable, the exact canonical
   receipt and fixture snapshot remain sufficient. No remote ref, tag, network,
   private receipt, or GitHub setting becomes a dependency.
7. Active changes never use the snapshot path. Their planning commit must
   remain in current ancestry.
8. The fallback proves which reviewed receipt and fixture survived canonical
   integration; it does not claim that the squash commit itself was
   planning-only.

## Consequences

Protected squash integration and branch cleanup no longer make canonical
factory health depend on discarded feature history. Receipt mutation becomes
strictly harder for all bindings because every byte is pinned. The stronger
commit/diff proof remains active whenever the source object is available.

The canonical snapshot cannot reconstruct parentage intentionally discarded by
squash. That limitation is explicit and bounded to fixed compatibility
fixtures; general OpenSpec preflight still requires exact planning ancestry
before implementation.

## Alternatives rejected

Permanent feature branches or tags add external lifecycle dependencies without
restoring ancestry. Git bundles or stored patches add binary/history machinery
for a small fixed fixture. Merge-commit-only delivery changes protected
administration. Dropping ancestry without pinning the receipt would permit
silent contract-head drift. Reissuing the receipt after implementation would
destroy the planning boundary.

## Verification and rollback

Verification requires immutable receipt hashes for all bindings, both evidence
paths, pruned-history, uncommitted-copy and forged-evidence mutation tests,
factory and Nix gates, independent CLI review, and protected CI. Rollback is a
checker/test/docs revert; it reintroduces the known post-squash failure but
changes no SDK data or API.
