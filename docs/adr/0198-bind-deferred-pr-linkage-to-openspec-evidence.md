# ADR 0198: Bind deferred PR linkage to OpenSpec evidence

- **Status:** Accepted
- **Date:** 2026-10-29
- **Decision owners:** SDK-Rust maintainers
- **Related:** issue #291, issue #388, PR #585, ADRs 0108 and 0127

## Context

Ordinary pull requests should close their owning issue. Some acceptance work,
however, cannot finish until after the protected merge produces an exact commit
or a natural scheduled run. Requiring a closing keyword in those cases forced
issues #276 and #388 through misleading close/reopen transitions.

## Decision

Keep closing linkage as the ordinary path. Permit deferred linkage only when
the pull-request body explicitly declares it, references the exact branch issue
without closing it, and binds a strict `post-merge-acceptance.json` sidecar in
the archived OpenSpec change at the exact PR head. Protected-base code validates
the bounded sidecar as untrusted data and never executes head code.

The sidecar must match the issue and enumerate the remaining gates. It reserves
later acceptance; it does not satisfy, waive, or automate those gates.

## Consequences

- Long-lived acceptance work keeps truthful issue state across integration.
- Reviewers can see exactly which evidence remains and why merge is not issue
  completion.
- Ordinary implementation retains the simpler closing path.
- Deferred PRs carry one small versioned sidecar and cannot rely on labels,
  milestones, comments, or free-form prose.
- GitHub settings, approvals, release authority, and downstream behavior remain
  unchanged.

## Alternatives rejected

- Mandatory close/reopen misrepresents lifecycle state.
- Any free-form `References` line is too weak and ambiguous.
- Mutable labels or milestones are not exact-head evidence.
- Executing head-owned validation code is unsafe under `pull_request_target`.

## Rollback

Revert the validator, template, and documentation. Existing deferred issues
remain ordinary open issues; no SDK or consumer data migration is required.
