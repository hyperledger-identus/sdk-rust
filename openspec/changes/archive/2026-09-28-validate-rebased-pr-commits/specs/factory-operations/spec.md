## ADDED Requirements

### Requirement: Pre-push validates only trusted contribution commits

The local pre-push hook SHALL resolve the exact current fetched
`refs/remotes/origin/develop` commit and require it to be an ancestor of every
outgoing local head. For an ordinary fast-forward update, it MAY validate only
the commits after the existing remote feature head when that head is both an
ancestor of the local head and a descendant of current protected `develop`.
For a first push or a legitimate rebase, it SHALL validate the feature commits
relative to current protected `develop`. It SHALL fail closed on missing
objects, a missing protected base, unrelated history, or ambiguous ancestry,
and SHALL NOT trust a caller-selected ref as a protected base.

#### Scenario: Ordinary feature update is pushed

- **WHEN** the existing remote feature head descends from current
  `origin/develop` and is an ancestor of the outgoing local head
- **THEN** pre-push validates only commits after that remote feature head

#### Scenario: Signed feature branch is rebased

- **WHEN** the old remote feature head shares protected history but is not an
  ancestor of a local head that descends from current `origin/develop`
- **THEN** pre-push validates commits after current `origin/develop` and does
  not require local verification of protected-base commits

#### Scenario: Rebased feature commit is unsigned

- **WHEN** any authored commit after current `origin/develop` lacks required
  local signature or DCO evidence
- **THEN** pre-push rejects the update before network mutation

#### Scenario: Feature branch is pushed for the first time

- **WHEN** Git reports a zero remote SHA and current `origin/develop` is an
  ancestor of the outgoing local head
- **THEN** pre-push validates every feature commit after that protected base

#### Scenario: Protected-base evidence is unsafe

- **WHEN** current `origin/develop` is missing, is not an ancestor of the local
  head, an input commit is missing or malformed, or old remote history is
  unrelated to the protected base
- **THEN** pre-push fails closed without selecting an arbitrary validation base
