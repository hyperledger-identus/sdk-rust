# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-29
Source retrieval date: 2026-09-29
Research blockers: none

## Problem and existing implementation

`scripts/git-hooks/local-policy.mjs` reads Git's pre-push update lines and uses
the existing remote branch SHA as the validation base whenever it is nonzero.
That is correct for an ordinary fast-forward update but not for a rebased
branch: `git rev-list old_remote..rebased_local` contains commits newly
reachable through the updated protected base. Local `git verify-commit` then
tries to verify trusted GitHub-created protected-base squash commits as if they
were contributor commits.

Issue #333 records the observed failure from PR #332 and requires current
`origin/develop` ancestry, first-push, fast-forward, rebase, malicious-history,
missing-base, and signed/unsigned evidence. The hosted policy already validates
the complete proposed PR history independently; this slice corrects the local
pre-network guard without weakening it.

## Normative sources

- Issue #333 and accepted contribution provenance policy in the canonical
  `factory-operations` specification.
- Git `pre-push` hook input and remote-old-value semantics:
  https://git-scm.com/docs/githooks#_pre_push
- Git `merge-base --is-ancestor`, `merge-base`, `rev-parse --verify`, and
  `rev-list A..B` behavior:
  https://git-scm.com/docs/git-merge-base,
  https://git-scm.com/docs/git-rev-parse, and
  https://git-scm.com/docs/git-rev-list
- Repository implementation at
  `develop@20e4f7ad6630cec476fbadf1b71bb2315aca7876`.

## Candidate decisions

| Candidate | Decision | Reason |
| --- | --- | --- |
| Always validate `remote_sha..local_sha` | `reject` | A rebase includes protected-base commits that are outside the authored contribution. |
| Always validate `origin/develop..local_sha` | `not-adopt` | Safe but needlessly revalidates already pushed feature commits on every ordinary fast-forward. |
| Use the remote head only when it is a protected-base-rooted ancestor; otherwise use current protected base after ancestry proof | `adopt` | Preserves the narrow fast-forward range and selects only current protected-base-relative feature commits after a rebase. |
| Trust a caller-supplied base/ref | `reject` | An arbitrary ref could hide unsigned contribution commits. |
| Fetch or query GitHub inside the hook | `reject` | Adds network mutation/latency and makes local enforcement nondeterministic. |
| Add a Git or signing dependency | `not-adopt` | Existing pinned Git, OpenSSH test tooling, and repository policy functions are sufficient. |

## Compatibility and dependency evidence

No Cargo, Nix, action, MSRV, target, feature, SDK API, wire format, crate, or
resolved dependency-cone change is required. The hook remains local and
repository-owned. Ordinary fast-forward updates keep their current narrow
range. First pushes and rebased pushes require the exact fetched protected base
to be an ancestor of the local head; missing or unrelated evidence fails before
network mutation.

## Security, privacy and maintenance evidence

Only exact lowercase commit IDs from Git hook input and the fixed
`refs/remotes/origin/develop` name enter the resolver. No shell evaluation,
dynamic ref name, network call, credential, wallet data, or private material is
introduced. A remote branch head is used as a base only when Git proves both
its ancestry to the local head and its descent from current protected
`origin/develop`. Divergent remote history must share ancestry with the
protected base before the hook classifies it as a rebase; unrelated history
fails closed.

Hermetic repositories exercise graph selection. An ephemeral test-only SSH
key may prove that a signed rebased feature commit passes and an unsigned one
fails; no key or generated fixture is retained. Rollback is a normal atomic
revert of resolver, tests, and specification.

## Rejected or deferred candidates

Remote API lookups, a bundled GitHub OpenPGP key, signature exemptions, and
hosted-only enforcement are rejected because they either expand trust or
remove useful local feedback. General multi-upstream and non-`develop` support
is deferred because the repository policy fixes the integration base.

## Open questions and blockers

None. The issue, Git graph semantics, and existing repository policy determine
the behavior without a new product or governance decision.

## Evidence commands

Planning evidence: issue and implementation inspection, factory doctor,
research readiness, constraint readiness, strict OpenSpec validation, and the
issue-bound preimplementation receipt. Implementation evidence will include
focused Node tests, full factory operations tests, `./scripts/factory check`,
the Nix factory contract, exact-diff review, and hosted `fast` CI. Those
implementation commands are unrun at planning time.
