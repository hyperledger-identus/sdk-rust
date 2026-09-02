## Context

The existing bootstrap policy deliberately retained human merge authority while
the factory contract was unproven. The factory now produces local readiness
evidence and stable CI statuses. The project sponsor has authorized any agent
or human to publish locally reviewed work and merge it into `develop` when the
repository gates are green, with an issue required for every pull request.

This delegation applies to integration, not product intent or release. `main`
remains reserved, publishing credentials remain protected, vulnerability
disclosure remains private and repository settings remain maintainer-owned.

## Goals / Non-Goals

**Goals:**

- make the delegated `develop` delivery authority explicit and consistent;
- require an issue link and a completed local review for every pull request;
- make failed, pending or missing required CI checks an absolute merge blocker;
- give agents a deterministic stop/go rule that works across LLM clients;
- enforce the PR evidence fields through a small, repository-owned CI check.

**Non-Goals:**

- direct pushes to `develop` or `main`;
- autonomous product scope, governance initiation or standards decisions;
- publishing crates, creating releases or promoting commits to `main`;
- bypassing branch protection, unresolved findings or security embargoes;
- mutating live repository settings in this documentation change.

## Decisions

### 1. The issue precedes the pull request

Every pull request references a real repository issue. If no suitable issue
exists, the contributor or agent creates one before opening the pull request.
The issue is the durable scope and outcome record; the PR is implementation and
evidence. Even administrative changes use a lightweight delivery-task issue.

### 2. Local review is a publication precondition

Implementation and a distinct local review pass complete before the branch is
published as a ready pull request. The review may be performed by a human or an
agent in a fresh review context and is recorded in the PR. This is a quality
gate, not a claim that the author can waive security, compatibility or
conformance findings.

### 3. Green required CI delegates merge authority on `develop`

A human or agent may merge a pull request into `develop` without a new
per-merge authorization only when it is non-draft, mergeable, current under the
repository rules, references its issue, has local review evidence, has no
unresolved blocking review and every required CI status is successful. Pending,
missing, cancelled or failing required checks prohibit merge. Branch-protection
bypass is never part of this delegation.

### 4. Protected authority remains human

The delegation ends at `develop`. Product scope acceptance, repository settings,
secret access, security disclosure, releases, package publication and promotion
to `main` still require explicit human maintainer authority. This keeps routine
integration fast without broadening high-impact authority.

### 5. A narrow PR-policy job makes evidence machine-checkable

`scripts/check-pr-policy.sh` validates the PR base, ready state, issue reference
and local review declaration using pull-request event fields passed through the
environment. A pinned GitHub workflow verifies that the referenced issue exists
in the repository and exposes the stable `pull-request-policy` status. It does
not execute PR body text and requires only read access. Shell fixtures cover
accepted and rejected inputs.

## Risks / Trade-offs

- **An existing issue can still cover the wrong scope** → CI verifies existence;
  reviewers and agents verify correspondence before merge.
- **Local agent review can miss systemic risk** → security, crypto, protocol and
  conformance stop conditions still require the named specialist review.
- **Green CI can be incomplete** → repository settings enumerate only real,
  stable required checks and new risk classes add gates before delegation is
  relied upon.
- **A compromised workflow could create false green** → workflows remain
  pinned, least-privileged and reviewable; no branch-protection bypass is
  delegated.

## Migration Plan

1. Add the policy contract, ADR, docs, templates, script, fixtures and workflow
   on an issue-linked feature branch from `develop`.
2. Pass local factory, shell, OpenSpec and Nix validation and record the review.
3. Open the PR against `develop`; merge it only when every existing CI gate is
   green under the newly delegated authority.
4. After merge, maintainers add `pull-request-policy` to the live `develop`
   ruleset once the status has succeeded and reconcile approval requirements
   with the documented desired state.

Rollback is a revert on `develop`. The prior requirement for explicit human
merge authorization becomes effective again if this policy is reverted.
