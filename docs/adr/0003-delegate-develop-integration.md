# ADR 0003: delegate issue-linked, CI-gated `develop` integration

- **Status:** Accepted
- **Date:** 2026-09-02
- **Decision authority:** explicit project-sponsor direction
- **Supersedes:** the `develop` merge restrictions in the initial governance
  packet and AI Software Factory bootstrap
- **Related work:** sdk-rust issue #16

## Context

The bootstrap governance required agents to stop before pushing or merging and
required blanket human approvals on `develop`. That was appropriate while the
branch model and factory checks were being established, but it creates a manual
handoff after an agent has implemented, locally reviewed and verified a bounded
slice.

The project is explicitly AI-first. Its routine integration path should allow
agents to finish work without weakening the controls around product scope,
security disclosure, publishing, repository administration, releases or the
reserved `main` branch. A corresponding issue is required so each pull request
has a durable outcome and scope record outside the implementation branch.

## Decision

1. Every pull request has a corresponding repository issue. A human or agent
   creates the issue before the pull request when no suitable issue exists.
2. Pull requests target `develop`; direct pushes to `develop` and `main` remain
   prohibited.
3. After implementation and a distinct local review pass, a human or agent may
   push the focused branch and open a ready pull request without separate
   per-push authorization.
4. A human or agent may merge into `develop` without separate per-merge
   authorization only when the pull request is non-draft, mergeable, current
   under repository rules, linked to its issue, locally reviewed, free of
   unresolved blocking reviews and green on every required CI gate.
5. A missing, pending, cancelled or failing required gate prohibits merge.
   Branch-protection bypass is not delegated.
6. Human maintainers retain authority over product scope acceptance, repository
   administration, security disclosure, secret use, publication, releases and
   promotion to `main`.
7. Risk-specific specialist review may be made blocking by an accepted issue or
   ruleset. Green generic CI does not waive a recorded security, compatibility,
   provenance or conformance finding.

## Consequences

- Agents can complete ordinary delivery through `develop` without waiting for a
  routine push or merge confirmation.
- Issues become mandatory even for OpenSpec-exempt administrative pull requests.
- Local review evidence and required CI statuses become the objective merge
  boundary for `develop`; blanket human approval counts are removed from the
  desired integration ruleset.
- Release, publication and `main` workflows remain deliberately separate and
  human-controlled.
- The repository needs a stable PR-policy status and live ruleset reconciliation
  after that status has succeeded on `develop`.

## Operational merge checklist

Before merging a pull request into `develop`, the responsible human or agent
confirms:

- the PR targets `develop`, is ready and is mergeable;
- the linked issue matches the delivered scope;
- the local review result is recorded and no blocking finding remains;
- every required check is present and successful;
- every review thread required by repository rules is resolved;
- the operation uses the normal protected merge path without bypass.

If any item is false or unknown, the pull request remains open.
