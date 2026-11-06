# factory-operations

## MODIFIED Requirements

### Requirement: Pull-request issue linkage has explicit completion semantics

Every issue-branch pull request SHALL classify its branch-issue linkage as
`closes` or `deferred`. Ordinary delivery SHALL use `closes` and a standalone
closing keyword for the exact branch issue. Deferred linkage SHALL be accepted
only when the body contains a standalone non-closing reference to the exact
branch issue and names one valid archived OpenSpec post-merge acceptance
sidecar from the exact pull-request head. Metadata inside comments, inline code,
or fenced code SHALL NOT authorize linkage.

Indented CommonMark code SHALL NOT authorize linkage. HTML comment delimiters
inside fenced or indented code SHALL NOT change live comment state. An
unterminated fence or HTML comment SHALL invalidate linkage metadata rather
than permit a partial authoritative parse.

Deferred linkage SHALL reject closing keywords for the branch issue in the
pull-request title, authoritative body, exact-head commit messages, guarded
merge body, and observed merged commit message, including repository-qualified
references and every closing-keyword form supported by GitHub. A closing
keyword in live list-item continuation text SHALL be rejected even when the
continuation begins with four spaces or a tab.

The required hosted protected-base policy SHALL additionally bind its verdict
to a complete, bounded, cursor-paginated snapshot of GitHub's authoritative
closing-issue relation for the exact pull request and head. `closes` SHALL
require that the relation contain exactly one entry and that entry be the
matching repository branch issue;
`deferred` SHALL require that relation not to contain the branch issue. Missing,
malformed, stale, cyclic, ambiguous, or over-bound hosted evidence SHALL fail
closed without printing provider response bodies.

The protected-base validator SHALL treat head content only as bounded untrusted
data. It SHALL require the deferred sidecar to use the closed schema, match the
branch issue, state the deferred disposition and rationale, and enumerate the
remaining acceptance gates. It SHALL NOT execute pull-request-head code or
interpret deferral as completion.

Terminal metrics identity validation SHALL accept either the exact repository
closing linkage or the exact authoritative deferred reference for the recorded
issue and pull-request head.

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
  only inside an HTML comment, inline code span, or fenced code block
- **THEN** it does not satisfy pull-request linkage policy

#### Scenario: Deferred closing text continues a list item

- **WHEN** a deferred body places a closing keyword in an immediately indented
  continuation of a live list item
- **THEN** policy rejects the body because GitHub renders that continuation as
  live text rather than an indented code block

#### Scenario: Apparent linkage is indented code

- **WHEN** a metadata or closing line occurs only inside a tab- or four-space-
  indented code block
- **THEN** it does not satisfy pull-request linkage policy or alter comment
  state

#### Scenario: An open HTML comment ends on an indented line

- **WHEN** an HTML comment is already open and a later line contains its
  closing delimiter after four spaces or a tab
- **THEN** the delimiter ends the comment because that line remains part of
  the HTML block rather than starting an indented code block

#### Scenario: Markdown authority is unterminated

- **WHEN** a pull-request body ends inside an HTML comment or fenced code block
- **THEN** linkage validation fails without accepting metadata that appeared
  before the unterminated construct

#### Scenario: Hosted closing relation matches closing mode

- **WHEN** the complete bounded hosted relation for the exact PR head contains
  the exact repository branch issue and textual closing linkage is valid
- **THEN** required protected-base policy accepts closing mode

#### Scenario: Hosted relation would close a deferred issue

- **WHEN** textual deferred evidence is valid but GitHub's authoritative
  closing relation contains the branch issue
- **THEN** required protected-base policy fails before merge

#### Scenario: Hosted relation is incomplete or unsafe

- **WHEN** pagination is missing, stale, cyclic, malformed, ambiguous, or
  exceeds the declared page or node bound
- **THEN** required protected-base policy fails without accepting textual
  metadata alone or printing the provider payload
