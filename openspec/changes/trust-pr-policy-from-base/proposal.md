# Trust pull-request policy from the protected base

## Why

The hosted pull-request policy currently checks out the pull request merge tree
and executes the checker, configuration, and workflow logic supplied by the
same contribution being judged. A contribution can therefore weaken its own
gate. The policy must instead execute only artifacts from the immutable base
revision while treating pull-request commits and metadata as untrusted data.

## What changes

- Load the workflow through `pull_request_target` for pull requests to
  `develop`.
- Check out the event's exact base SHA with credentials disabled and verify the
  resulting checkout identity before running any repository code.
- Fetch only the exact pull-request head object required for Git history
  validation; never check out or execute the head tree.
- Retain explicit read-only permissions, pinned actions, and API-sourced commit
  records.
- Add source-level regression tests for the trust boundary and document it in
  ADR 0166 and the factory contract.

## What does not change

This slice does not compare base and head policy strength, change contribution
rules, introduce secrets, alter repository rulesets, select a signature
envelope, or grant write authority. Those concerns remain #339 or
administrator-owned follow-up work.

## Capabilities

### Modified capabilities

- `ai-software-factory`: make the protected base revision the executable trust
  root for pull-request integration policy.

## Authority

Issue #341, the standing autonomous-delivery mandate, and GitHub's official
`pull_request_target` security guidance.
