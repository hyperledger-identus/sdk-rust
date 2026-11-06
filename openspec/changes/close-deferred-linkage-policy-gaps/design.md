# Design

## Context

Pull-request linkage is an authorization decision over untrusted Markdown and
provider state. The repository uses a deliberately narrow line grammar rather
than general Markdown semantics, but it must first remove non-authoritative
code and comments correctly. Hosted CI must additionally verify the relation
GitHub will apply at merge.

## Decisions

### Parse code before comments

Process the body line by line. While inside a fenced block, only a compatible
closing fence changes state; HTML delimiters are inert. Outside fences, skip
tab- and four-space-indented lines, remove HTML comments while retaining live
text around them, and recognize a fence only from the resulting visible line.
An unclosed fence or comment invalidates the entire authority parse so earlier
metadata cannot create a partial-success path.

### Validate one bounded hosted evidence document

The workflow captures GraphQL pages for the exact repository pull request and
head into a runner-temporary JSON array. Protected-base Node code validates:

- one to eight pages and at most 800 nodes;
- exact PR number and head SHA on every page;
- boolean `hasNextPage`, nullable/string cursor shape, unique monotonic cursor
  progress, and a terminal page;
- closed issue-node identity with safe positive numbers and exact repository
  names.

The validator exposes only normalized `{number, repository}` identities.
Closing mode requires the complete relation to contain exactly one entry: the
matching branch issue. Deferred mode requires zero matching branch issues;
other visible closing issues do not satisfy branch ownership.

### Preserve path-specific evidence boundaries

Local preflight continues to validate text and sidecar evidence before a PR
exists. Required hosted CI adds the authoritative relation. Guarded delivery
and terminal metrics retain their remote checks and share normalized validation
where practical. The template states the exact standalone syntax so authors do
not have to infer parser rules.

### Keep provider timing honest

The required check is a snapshot, not a lock. Guarded delivery rechecks the
exact hosted state immediately before protected merge; the retained merge
receipt verifies observed post-merge state. Direct manual mutation between
check and merge remains detectable but cannot be made atomic without a
repository setting or merge-queue decision outside this change.

## Error and privacy contract

Errors identify only malformed page, pagination, identity, or linkage-mode
invariants. They never print PR bodies, GitHub response pages, tokens, sidecar
rationale, acceptance identifiers, or merge messages.

## Rollback

Revert the policy, workflow collection, tests, template, docs, and canonical
spec together. No SDK artifact or consumer migration is required.
