# factory-operations delta

## MODIFIED Requirements

### Requirement: Managed worktrees have single ownership and safe lifecycle

The factory SHALL use one canonical managed path per issue and SHALL audit
capacity before creation. Cleanup SHALL require an exact path and expected
head and SHALL refuse primary, current, dirty, locked, broad, symlinked or
ambiguously delivered targets.

A superseded unmerged pull request MAY release only its clean registered
worktree when its exact head remains preserved by the matching remote branch,
the original PR is closed against `develop`, and a named replacement PR is
merged to `develop` with an explicit closing reference to the original branch
issue. This recovery path SHALL NOT delete local or remote branch refs and
SHALL NOT claim semantic patch equivalence.

#### Scenario: Exact merged worktree is closed

- **WHEN** the named PR's remote head, merged state, base and expected local
  head match a clean unlocked canonical managed worktree
- **THEN** an explicit execute action may remove that one worktree

#### Scenario: Superseded recoverable worktree is closed

- **WHEN** the original closed PR, preserved remote branch head, branch issue,
  merged replacement PR and clean canonical local worktree all match
- **THEN** an explicit execute action may remove only that worktree while
  retaining both branch refs

#### Scenario: Cleanup target is ambiguous

- **WHEN** any ownership, path, head, cleanliness, lock or merge proof is
  missing
- **THEN** cleanup fails without deleting files or refs

#### Scenario: Superseded recovery evidence is incomplete

- **WHEN** remote recovery, issue-linkage or replacement-merge proof is missing
- **THEN** superseded cleanup fails without deleting files or refs

## ADDED Requirements

### Requirement: Supervisor delivery is file-backed and exact-head

The factory SHALL provide a local pull-request metadata preflight that reads a
bounded regular body file and applies the same base, readiness, issue, review,
constraint, limitation, title, branch, and closing-link validators used by
hosted CI before remote creation.

A supervisor-owned squash merge SHALL require explicit execution, the exact
hosted PR head, an open ready PR targeting `develop`, successful required
checks, and a bounded regular message file containing real multiline text and
an exact DCO trailer. It SHALL use the normal protected GitHub merge path and
SHALL fail closed when any hosted state is missing, pending, malformed, or
inconsistent.

After a successful merge, the factory SHALL retain one owner-private immutable
receipt binding repository, PR, original exact head, merge commit, base,
hosted merge time, merge-message SHA-256, and verification results. It SHALL
reject a conflicting receipt and SHALL NOT include tokens, PR body text,
checks output, prompts, transcripts, or provider/model data.

#### Scenario: Candidate PR metadata is malformed locally

- **WHEN** the body file would fail either hosted pull-request policy layer
- **THEN** local preflight exits non-zero with the hosted validator diagnostics
  before creating or changing a PR

#### Scenario: Candidate is eligible for protected merge

- **WHEN** the exact head, ready state, base, required checks, mergeability,
  body-file safety, real multiline shape, and DCO trailer all pass
- **THEN** dry validation reports eligibility and an explicit execute action
  may use the normal protected squash-merge path

#### Scenario: Hosted merge completes

- **WHEN** GitHub reports the expected PR merged to `develop` from the exact
  validated head
- **THEN** one immutable privacy-bounded receipt is retained below the Git
  common directory

#### Scenario: Delivery evidence is stale or unsafe

- **WHEN** the PR head changed, checks are not successful, the body file is
  unsafe or escaped, the PR is draft/closed/wrong-base, mergeability is not
  proven, or a retained receipt conflicts
- **THEN** the factory fails without merging, overwriting evidence, or deleting
  any worktree or branch
