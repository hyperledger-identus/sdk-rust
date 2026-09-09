# Local review

- **Review date:** 2026-09-09
- **Review angle:** correctness, security, privacy and recovery
- **Scope:** exact staged implementation diff after planning commit `91d7619`
- **Result:** passed after findings were resolved

## Resolved findings

1. The bootstrap root command could emit two paths because of shell operator
   grouping. It now uses an explicit conditional.
2. The Git configuration bootstrap omitted the `apply` subcommand. The wrapper
   now invokes the guarded command correctly and was exercised successfully.
3. Pi user-policy backup handling could follow an existing symlink. Config and
   backup inputs now require bounded regular files, with a known-bad symlink
   test.
4. Factory/CI/build diffs did not initially recommend the complete weekly slow
   backstop promised by the specification. Routing and its test now agree.
5. Git-hook modules executed their CLIs when imported. Both modules now keep
   reusable exports side-effect free through direct-run guards.
6. Metrics comment updates used a returned URL. Updates now reconstruct the
   repository-scoped API endpoint from a validated numeric comment identity.
7. Preflight accepted both action flags and did not reject a stale develop
   base before receipt creation. It now requires exactly one action and exact
   ancestry.
8. A worktree negative test depended on a surrounding Git checkout and failed
   in the isolated Nix source. Canonical containment is now a pure exported
   predicate with checkout-independent positive and negative cases.

## Remaining limitations

- The first end-to-end Pi-driven SDK slice is intentionally post-merge and
  issue-backed; no unmerged factory code is treated as the etalon.
- Weekly/manual slow target recommendations are evidence routing, not new
  per-PR required checks.
- Metrics retention is policy only in v1; no implicit destructive pruning job
  is installed.
- `main`, publishing, release and downstream adoption remain inactive.

No unresolved blocking finding remains.
