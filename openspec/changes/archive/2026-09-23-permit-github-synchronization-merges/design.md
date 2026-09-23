# Design

## Hosted record and context

The workflow adds `committerName`, `committerEmail`, `committerActor`,
`parentShas` and `treeSha` to every commit record and supplies `PR_BASE_SHA`,
`PR_BASE_REF` and `PR_HEAD_REF` to the checker. All new values come from the
signed pull-request event or GitHub API; none is supplied by pull-request prose.

`validateHostedCommits` receives this context and classifies each record before
authored-metadata validation. The function keeps its exact-head and duplicate-
SHA checks.

## Synchronization classification

A record is a GitHub synchronization merge only when:

1. it has exactly two distinct hexadecimal parent SHAs;
2. its first parent equals the immediately preceding pull-request record;
3. its second parent equals or is an ancestor of the event's base SHA;
4. its subject is exactly `Merge branch '<base-ref>' into <head-ref>`;
5. its committer is exactly `GitHub <noreply@github.com>` and its API actor is
   `web-flow`; and
6. GitHub reports its signature as verified with reason `valid`; and
7. `git merge-tree --write-tree` reproduces the exact recorded tree from the
   two parents without a conflict.

The ancestor test uses the already complete checkout and `git merge-base
--is-ancestor`; failure to execute or a missing commit returns false. Allowing
an ancestor, rather than requiring exact equality, prevents a previously valid
synchronization commit from becoming invalid only because `develop` advanced
again before the next pull-request event.

The first record cannot be exempt because it has no preceding pull-request
commit. A malformed field, unexpected parent order, unexpected subject,
different committer, unverified signature, merge conflict or tree mismatch
makes the record ordinary. Reproducing the tree excludes a web conflict
resolution that introduces new content under otherwise similar metadata.

## Validation split

`validateCommitEvidence` receives an explicit `authoredMetadata` boolean that
defaults to true. Conventional subject and DCO checks run only for authored
metadata. Signature provenance runs unconditionally. The hosted loop is the
only caller allowed to pass false, after the synchronization classification.

The local `validateCommitRange` path keeps the default. This intentionally
fails closed for local merges rather than guessing that an arbitrary two-parent
commit came from GitHub.

## Tests

Unit tests cover the exact PR #340 shape and reject wrong parent count, order,
base ancestry, merge tree, committer, subject, signature and missing context. Existing
ordinary PGP/SSH, DCO, subject and exact-head tests remain unchanged. A fixture
repository supplies the ancestor relation so the test exercises the real git
query rather than a mocked boolean.

Workflow tests assert that the API projection and event environment contain
the new fields. A replay fixture for PR #340 proves the prior failure is green
without changing the ordinary-commit result.

## Documentation and decision record

ADR 0136 records the exception and its bounds. `CONTRIBUTING.md` and `DCO.md`
clarify that authored commits retain every existing rule, while a GitHub-
generated branch synchronization merge is handled structurally by hosted CI.
The canonical `factory-operations` specification receives an additive
requirement, so no existing requirement is replaced and no archive intent is
needed.
