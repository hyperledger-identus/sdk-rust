# Permit verified GitHub branch-synchronization merges

## Why

PR #340 used GitHub's supported **Update branch** action to bring `develop`
into `fix/issue-338`. GitHub produced verified merge commit `39d20dcc`, with
the platform-generated subject `Merge branch 'develop' into fix/issue-338`
and no DCO trailer. The canonical DCO2 check passed, but the repository-local
`pull-request-policy` rejected the subject and missing trailer. The source
change, review and fast CI were otherwise green, so the local gate converted a
normal GitHub contribution flow into pressure to bypass protection.

## What changes

- Extend hosted commit evidence with parent SHAs and committer identity, and
  bind validation to the pull request base SHA/ref and head ref.
- Recognize only a GitHub-verified, canonical base-into-head synchronization
  merge whose first parent is the preceding pull-request commit and whose
  second parent is the current base or one of its ancestors.
- For that structural record only, omit Conventional Commit subject and DCO
  trailer validation while retaining accepted-envelope signature validation.
- Keep local commit-range validation fail-closed because it lacks trusted
  pull-request event context.
- Document the distinction between authored commits and platform-generated
  synchronization metadata in the contribution policy and ADR 0136.

## Capabilities

### Added capabilities

- `factory-operations`: hosted contribution provenance admits narrowly
  identified GitHub branch-synchronization merge commits without waiving
  authored-commit rules.

## Non-goals

No unconditional exemption for merge commits. No exemption for GitHub web
editor commits, conflict-resolution commits that do not match the structural
contract, unsigned or unverified commits, or hand-authored merges. No change
to branch protection, the canonical DCO2 app, accepted signature envelopes,
review authority or issue #339's base-policy hardening.

## Delivery

Issue #342 owns the change. It is part of the `0.1.0-rc.1` final lap and must
merge before the protected release SHA is frozen and slow evidence is run.
