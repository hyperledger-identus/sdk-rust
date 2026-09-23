# ADR 0136: permit verified GitHub synchronization merges

- **Status:** Accepted under sponsor direction
- **Date:** 2026-09-23
- **Issue:** [#342](https://github.com/hyperledger-identus/sdk-rust/issues/342)
- **Related:** ADR 0135 and issue #339
- **Review no later than:** the next contribution-policy revision

## Context

GitHub's supported **Update branch** merge path creates a platform-signed merge
commit whose subject and trailers are not contributor-controlled. On PR #340,
the independent DCO2 check, review and fast CI passed, but the repository-local
checker rejected GitHub's merge subject and absent DCO trailer. The pull request
was merged only through an override, contrary to the repository's normal
protected path.

The canonical DCO2 implementation skips merge commits before parsing their
trailers. Copying that unconditional behavior would be too broad because a
hand-authored merge can contain new conflict resolution or other content.

## Decision

1. Hosted contribution provenance distinguishes authored commits from a
   narrowly identified GitHub branch-synchronization merge.
2. The synchronization classification requires two distinct parents, the
   previous pull-request commit as first parent, the protected base or its
   ancestor as second parent, the exact base-into-head subject, GitHub's
   platform committer identity, and a signature GitHub reports as verified and
   valid.
3. Only Conventional Commit subject and DCO trailer checks are omitted for
   that record. Accepted-envelope signature validation remains mandatory.
4. Ordinary commits, malformed records, hand-authored merges and local range
   validation receive no exemption.
5. Base-policy self-evaluation remains separately owned by issue #339.

## Consequences

- Contributors and agents may use GitHub's documented Update branch merge path
  without rewriting platform-generated metadata.
- Authored web-editor and local commits retain all existing provenance rules.
- Hosted validation becomes slightly more complex and relies on GitHub event,
  API and ancestry evidence, but avoids an unconditional merge loophole.
- Pull-request review remains responsible for the complete resulting diff; the
  checker does not independently reproduce GitHub's merge tree.

## Verification and rollback

Replay PR #340's exact shape and mutation-test every classification field.
Revert this ADR and its implementation to restore the stricter behavior; no
published artifact, public API or persisted data requires migration.
