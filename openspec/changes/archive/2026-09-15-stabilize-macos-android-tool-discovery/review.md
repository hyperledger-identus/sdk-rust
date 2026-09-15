# Local review

## Scope and identities

- Planning head: `026e6e04d8ec675667888cdbdea334020605861b`
- Reviewed implementation head: `432ace441d6f978aa2f277ba1797c260c99d998c`
- Review lenses: workflow authority, shell failure behavior, ambient tool
  discovery, policy-test strength, compatibility, and delivery sequencing

## Resolved findings

1. **Resolved — global markers admitted decoy evidence.** The first policy
   draft searched the whole slow workflow for required Android commands. A
   comment or unrelated step could therefore preserve a marker while the real
   installation step returned to ambient `PATH` discovery. The checker now
   isolates the named installation step, requires exact executable command
   lines in their safe order, and includes a comment-decoy mutation.
2. **Resolved — proposal described the already-split repairs as one PR.** The
   candidate determinism repair had already merged independently to retain
   factory stack depth zero. The delivery text now records this repair as a
   focused follow-on, matching the real history and branch state.
3. **Resolved — pre-merge tasks included impossible post-merge completion.**
   The checklist now prepares the reviewed archive and explicitly reserves the
   hosted canary as an issue-owned post-merge operation. It does not claim that
   hosted or natural-schedule evidence already exists.
4. **Resolved — closing-reference policy conflicts with long-lived evidence.**
   The contribution gate requires this issue branch's PR body to close #276,
   while the weekly-evidence contract keeps it open through the first natural
   run. Delivery therefore treats immediate post-merge reopening as part of the
   merge transaction. Issue #291 records the underlying policy mismatch for
   factory backlog polishing; this repair does not weaken the required gate.

## Result

No unresolved architecture, security, privacy, compatibility, or operations
finding remains. The workflow reads only GitHub's declared SDK variables,
quotes derived paths, checks the root and executable before mutation, preserves
the exact package set, and does not broaden `PATH` or repository permissions.
