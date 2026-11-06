# Support explicit deferred pull-request issue linkage

## Why

The contribution policy requires every issue branch pull request to close its
branch issue. That is correct for ordinary delivery, but it forces an
artificial close/reopen transaction when acceptance deliberately depends on
post-merge evidence such as a natural scheduled workflow, an exact protected
merge receipt, or independent approval. Issue #388 reproduced the same
friction originally recorded by issue #291: PR #585 had to close #388 for
policy compliance even though its final slow receipt and approval could exist
only after the protected merge.

## What Changes

- Keep issue-closing linkage as the explicit default for ordinary pull
  requests.
- Permit `deferred` linkage only when the pull-request body references the
  branch issue without closing it and names one archived OpenSpec
  `post-merge-acceptance.json` sidecar from the exact pull-request head.
- Validate the sidecar with bounded, duplicate-key-safe parsing and require it
  to match the branch issue, state a rationale, and enumerate the remaining
  acceptance gates.
- Reject ambiguous, mismatched, missing, commented, code-fenced, malformed, or
  unbound linkage evidence.
- Update the pull-request template, governance, factory operations, tests, and
  retrospective guidance without changing SDK behavior or GitHub settings.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `factory-operations`: support fail-closed closing versus deferred issue
  linkage for pull-request delivery.
- `ai-software-factory`: keep evidence-gated post-merge acceptance open without
  lifecycle churn.
- `spec-driven-delivery`: bind deferred acceptance to archived OpenSpec
  evidence while retaining issue-first delivery.

## Impact

This is behavior-neutral repository tooling. It changes pull-request metadata
validation, templates, governance documentation, tests, and OpenSpec
contracts. It does not change Rust APIs, dependencies, wire formats, target
support, releases, protected-branch settings, or downstream repositories.
