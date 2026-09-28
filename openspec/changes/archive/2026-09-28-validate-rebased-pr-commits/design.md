# Design

## Trusted graph resolver

Add one exported local-policy resolver that accepts a repository path plus the
exact outgoing local and remote SHAs and returns a closed result containing the
selected base or static errors.

1. Resolve the outgoing local SHA as that exact commit.
2. Resolve only the fixed `refs/remotes/origin/develop^{commit}` protected base.
3. Require the protected base to be an ancestor of the local head.
4. On a first push, select the protected base.
5. On an ordinary update, select the remote branch head only when it is an
   ancestor of the local head and a descendant of the current protected base.
6. When the remote head is no longer an ancestor, require it to share history
   with protected `develop`, classify the update as a rebase, and select the
   current protected base.
7. If an unusual fast-forward graph contains the protected base only after the
   remote head, select the protected base rather than importing protected
   commits into the contributor range.

Exact commit resolution and ancestry checks use argument-vector Git calls;
there is no shell or dynamic ref expression. The pre-push driver accumulates
resolver errors across outgoing refs and invokes the existing
`validateCommitRange` only for a successful selection.

## Test graph

Focused tests create temporary repositories with a fixed
`refs/remotes/origin/develop` and explicit commit graphs:

- first push selects protected `develop`;
- an ordinary fast-forward selects the prior remote feature head;
- a rebased branch selects new protected `develop` and excludes the old
  protected squash commit from local verification;
- a signed rebased feature commit verifies, while an unsigned one fails;
- an unrelated remote head, missing protected ref, non-ancestor local head, and
  malformed/missing commit object fail closed.

The signature fixture uses a temporary SSH key and repository-local allowed
signers file. The factory derivation includes nixpkgs-pinned OpenSSH explicitly,
so host and hermetic evidence exercise the same capability. No generated key
enters the repository.

## Risks and mitigations

- A stale `origin/develop` could select an older base: doctor/delivery refresh
  remains required, and the resolver never claims remote currency.
- Treating any divergent history as a rebase could hide commits: the current
  protected base must be an ancestor of the outgoing head and the old remote
  must share ancestry with it; otherwise resolution fails.
- Fast-forward behavior could regress: a dedicated graph proves the remote
  feature head remains the selected base when all trust predicates hold.
- Error handling could terminate the whole hook opaquely: the resolver returns
  bounded static errors that the driver reports through the existing prefix.

## Rollback

Revert the resolver, driver call, tests, and delta specification atomically.
The previous conservative false rejection returns; hosted policy and repository
history are unaffected.
