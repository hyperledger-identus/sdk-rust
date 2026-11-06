# Close deferred-linkage policy gaps

## Why

PR #589 merged the first evidence-bound deferred issue-linkage policy at exact
head `1f73f6ea4dfe7c3671deb40e9fc019c4c607a576`. Its final review ledger still
identified three blocking gaps: code and HTML-comment parsing can disagree with
CommonMark authority, indented code can be accepted as live metadata, and the
required hosted policy does not bind its verdict to GitHub's authoritative
closing-issue connection. Issue #590 owns the corrective slice.

## What Changes

- Parse bounded pull-request Markdown so fenced and indented code never grants
  linkage authority and HTML comments are interpreted only outside code.
- Fail closed on unterminated fences or comments instead of accepting partial
  authoritative metadata.
- Fetch and validate bounded, cursor-paginated GitHub
  `closingIssuesReferences` in protected-base CI.
- Require ordinary `closes` linkage to contain the exact hosted repository
  issue and require `deferred` linkage not to contain it.
- Keep local preflight, guarded delivery, hosted policy, and terminal metrics
  aligned; document the unavoidable post-check merge-message observation
  boundary.
- Clarify exact standalone closing and reference syntax in the pull-request
  template and add adversarial mutation tests.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `factory-operations`: make issue-link authority code-aware and bind hosted
  verdicts to bounded GitHub closing-reference evidence.

## Impact

This is routine repository-local delivery hardening. It changes protected-base
factory scripts, workflow data collection, templates, tests, documentation,
and the canonical factory contract. It does not change Rust APIs, wire or
persisted behavior, dependencies, targets, releases, repository settings, or
downstream repositories.
