# factory-operations

## ADDED Requirements

### Requirement: Pull-request issue linkage has explicit completion semantics

Every issue-branch pull request SHALL classify its branch-issue linkage as
`closes` or `deferred`. Ordinary delivery SHALL use `closes` and a standalone
closing keyword for the exact branch issue. Deferred linkage SHALL be accepted
only when the body contains a standalone non-closing reference to the exact
branch issue and names one valid archived OpenSpec post-merge acceptance
sidecar from the exact pull-request head. Metadata inside comments or fenced
code SHALL NOT authorize linkage.

Deferred linkage SHALL reject closing keywords for the branch issue in the
pull-request title, authoritative body, exact-head commit messages, guarded
merge body, and observed merged commit message, including repository-qualified
references and every closing-keyword form supported by GitHub.

The protected-base validator SHALL treat head content only as bounded untrusted
data. It SHALL require the deferred sidecar to use the closed schema, match the
branch issue, state the deferred disposition and rationale, and enumerate the
remaining acceptance gates. It SHALL NOT execute pull-request-head code or
interpret deferral as completion.

#### Scenario: Ordinary pull request closes its branch issue

- **WHEN** a ready pull request declares `closes` and contains a standalone
  closing keyword for its exact branch issue
- **THEN** local and hosted policy validation accept the linkage without a
  deferred sidecar

#### Scenario: Post-merge acceptance is explicitly deferred

- **WHEN** a ready pull request declares `deferred`, references its exact branch
  issue without closing it, and binds a valid matching sidecar in its archived
  OpenSpec change at the exact head
- **THEN** local and hosted policy validation accept the linkage while the issue
  remains open for the named evidence

#### Scenario: Deferred evidence is absent or mismatched

- **WHEN** the sidecar is missing, malformed, oversized, outside the allowed
  archive path, belongs to another issue, or omits its remaining gates
- **THEN** policy validation fails closed without printing the sidecar contents

#### Scenario: A later delivery surface would close a deferred issue

- **WHEN** the pull-request title, body, exact-head commit history, merge body,
  or observed merge commit contains a closing keyword for the deferred issue
- **THEN** policy or guarded delivery rejects the candidate before accepting
  the merge receipt

#### Scenario: Apparent linkage is non-authoritative prose

- **WHEN** an issue field, linkage mode, close, reference, or sidecar path occurs
  only inside an HTML comment or fenced code block
- **THEN** it does not satisfy pull-request linkage policy
