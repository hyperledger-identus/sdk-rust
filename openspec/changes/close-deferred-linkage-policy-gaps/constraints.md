# Constraints and limitations

Impact class: routine
Decision status: not-required
Decision reference: not-required
Constraint blockers: none

## Existing entries affected

- `SDK-DELIVERY-001` remains effective: qualifying work is issue-linked,
  distinctly reviewed, and protected by required CI.
- `develop` remains the integration branch; release, publication, `main`,
  repository administration, and downstream authority are unchanged.
- Protected-base code remains the only executable authority in privileged
  pull-request policy.

## Introduced or changed constraints

- Linkage metadata inside fenced or indented code cannot authorize delivery.
- Unterminated code fences or HTML comments make linkage metadata invalid.
- Hosted policy requires a complete bounded authoritative closing-reference
  connection for the exact PR head before accepting either linkage mode.
- Closing mode requires the exact branch issue in that connection; deferred
  mode forbids it.

## Introduced or changed limitations

- Local preflight validates intended text and exact-head sidecars but cannot
  query a pull request that does not exist yet; hosted policy validates the
  resulting GitHub relation.
- A required check observes one remote snapshot. Guarded delivery rechecks
  before merge and post-merge verification detects drift, but the policy does
  not claim atomic control over manual merge-message mutation after the check.
- Issue #308 continues to own historical metric pagination and merged-PR
  fallback behavior beyond this required-policy repair.

## Consumer and product impact

None. No SDK artifact, API, wire behavior, dependency, target, support claim,
release candidate, or consumer implementation changes.

## Activation and rollback

Activation occurs when protected `develop` runs the corrected base policy.
Rollback reverts the parser, hosted evidence collection, tests, template, and
documentation together. Existing SDK data and release artifacts are unaffected.

## Evidence

Issue #590, PR #589's exact-head review ledger, parser mutations, hosted-page
fixtures, factory/bootstrap validation, independent review, and exact-head CI.
