# Research

Research class: routine
Research status: ready
Decision date: 2026-09-23
Source retrieval date: 2026-09-23
Research blockers: none

## Problem and existing implementation

`scripts/ci/contribution-policy.mjs` applies Conventional Commit, exact DCO
and signature rules uniformly to every record returned by the pull-request
commits API. `.github/workflows/pull-request-policy.yml` currently retains the
commit SHA, message, author identity, GitHub verification object and actor, but
not parent SHAs or committer identity. The checker therefore cannot distinguish
an authored commit from a platform-generated synchronization merge.

PR #340 provides an exact regression fixture. Commit
`39d20dccd05c520d4cb4f06d7413755231235bff` has two parents: prior PR head
`44206da491b1190db783e8c635ffc8c8361a6463` and protected `develop` head
`e7002e8d662b4efd9207b1ba9df0c16a9988c279`. GitHub is the committer with API
actor `web-flow`, the OpenPGP signature is verified and valid, and the subject is the canonical
base-into-head merge subject. The local policy rejected only the non-
conventional subject and absent DCO trailer. The independent DCO2 status passed.

## Normative sources

- GitHub Docs, *Keeping your pull request in sync with the base branch*,
  retrieved 2026-09-23:
  <https://docs.github.com/en/pull-requests/how-tos/create-pull-requests/keeping-your-pull-request-in-sync-with-the-base-branch>.
  GitHub explicitly supports **Update branch** by merging the base branch into
  the pull-request branch or rebasing it.
- CNCF DCO2 source at
  `14a6f0096a594f736ab368e3985c07ada7bf5154`,
  `dco2/src/dco/check/mod.rs`, retrieved 2026-09-23:
  <https://github.com/cncf/dco2/blob/14a6f0096a594f736ab368e3985c07ada7bf5154/dco2/src/dco/check/mod.rs>.
  The canonical check classifies merge commits before sign-off parsing and
  skips their DCO requirement. The repository-local checker is therefore the
  source of the observed stricter behavior.
- GitHub REST API commit representation, retrieved 2026-09-23:
  <https://docs.github.com/en/rest/commits/commits>. The hosted record exposes
  parent SHAs, author/committer identity, commit tree and signature
  verification needed for a structural classification.
- Repository evidence: PR #340, workflow run `35865841906`, job
  `107197071068`, and exact commit `39d20dcc`.

No third-party source or fixture is copied. These sources define behavior and
provide public evidence only.

## Candidate decisions

- `retain-local`: keep the repository checker and add a narrow hosted-only
  synchronization classification. Chosen. It removes the false negative while
  preserving repository-owned, offline-testable policy.
- `adopt-dco2-only`: delete the repository DCO check and rely solely on the
  installed app. Rejected because the local hook and combined hosted checker
  also enforce signature, subject, branch and exact-head invariants.
- `exempt-all-merges`: match DCO2's broad merge exemption. Rejected because an
  arbitrary content-bearing merge would also bypass subject and DCO checks.
- `require-rebase-update`: tell contributors to select GitHub's rebase option.
  Rejected because rebase rewrites commit identities and signatures and does
  not solve the supported merge-update path.

## Compatibility and dependency evidence

The change adds no dependency, Rust item, Cargo feature, target, wire format,
secret surface or consumer API. It modifies one Node checker, one workflow
record, policy documentation, tests and factory specifications. Rust MSRV,
primary compiler, crate archives and dependency cones are unchanged.

The hosted record shape is internal and additive. Existing ordinary records
remain valid when the new pull-request context is supplied by the workflow.
The CLI fails closed if that context is absent, so no caller silently gains an
exemption. Local range validation remains unchanged and requires normal commit
metadata for every local merge.

## Security, privacy and maintenance evidence

The exemption is contextual, not message-only. It requires two distinct
parents, GitHub's verified signature and platform committer identity, the
canonical base/head subject, the prior pull-request record as first parent,
and a second parent that is the current base or its ancestor. Signature
envelope enforcement still runs. A malformed or partial record is ordinary and
therefore fails the existing subject/DCO rules.

Residual risk: GitHub's verification and committer identity remain the hosted
trust anchor, as already accepted in ADR 0135. GitHub conflict-resolution flows
can also create merge commits; the parent, subject and identity boundaries
reduce accidental admission, while the full pull-request diff and review remain
mandatory. A future exact merge-tree reproduction check may narrow this
further if GitHub's merge strategy can be reproduced portably without false
negatives.

Rollback is a normal revert. It restores the stricter false-negative behavior
without changing published artifacts or persisted data.

## Rejected or deferred candidates

An unconditional merge exemption is rejected because a content-bearing merge
can carry authored conflict resolution. Independent merge-tree reproduction is
deferred: it would strengthen the classification, but GitHub's exact merge
strategy and runner Git compatibility need separate false-negative evidence
before becoming a required contribution gate. Replacing DCO2, changing branch
protection and implementing issue #339 are rejected as unrelated scope.

## Open questions and blockers

None blocking. The residual dependence on GitHub's verified identity and the
absence of merge-tree reproduction are explicit limitations, not hidden
assurance claims. A false positive or false negative in the structural
classifier triggers reconsideration before the next policy revision.

## Evidence commands

- `scripts/factory research-ready permit-github-synchronization-merges`
- `scripts/factory constraints-ready permit-github-synchronization-merges`
- `openspec validate permit-github-synchronization-merges --type change --strict`
- `node --test scripts/tests/factory-operations.mjs`
- `scripts/factory check`
- `actionlint .github/workflows/pull-request-policy.yml`
- applicable factory-contract and text Nix checks
- replay of PR #340's hosted commit record against the new checker

Rust, target, fuzz and benchmark gates are not applicable because no Rust,
manifest, dependency, protocol, parser or release artifact changes.
