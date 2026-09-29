# Make linked-issue verification transient-safe

## Why

PR #472 passed repository-owned metadata validation and every other required
check, but its issue verification failed because GitHub GraphQL returned HTTP
499 while looking up existing open issue #470. A single provider transport
failure should not create avoidable delivery friction, while unverifiable or
incorrect issue identity must remain a hard failure.

## What changes

- Move repository-issue verification into one tested CI script.
- Use the GitHub REST issue representation so issue and pull-request records
  can be distinguished explicitly.
- Retry only recognized transient transport/service failures, at most twice
  after the initial attempt, with a short fixed delay.
- Keep missing records, pull requests, mismatched URLs, malformed inputs, and
  retry exhaustion fail-closed.

## Capability

### Modified capability

- `factory-operations`: hosted linked-issue verification becomes resilient to
  bounded transient GitHub API failures without relaxing issue linkage.

## Non-goals

No product, crate, dependency, contribution, signature, DCO, exact-head,
branch-protection, or workflow-trigger policy changes. No unbounded retry or
automatic workflow rerun is introduced.

## Delivery

Issue #474 owns this factory-only correction and must merge before PR #472 is
resynchronized against protected `develop`.
