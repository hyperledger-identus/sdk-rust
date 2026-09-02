# ADR 0004: establish standing autonomous agent authority

- **Status:** Accepted
- **Date:** 2026-09-03
- **Decision authority:** explicit project-sponsor direction
- **Supersedes:** the routine scope-acceptance boundary in ADR 0003 and the
  initial AI Software Factory bootstrap
- **Related work:** sdk-rust issue #18

## Context

ADR 0003 removed separate push and merge approvals, but the factory still
required a human to accept each scope and contract before implementation. That
handoff prevents an AI-first project from operating continuously even when work
is bounded by an existing roadmap, reversible, fully specified, locally
reviewed and protected by CI.

The issue, OpenSpec contract, ADR, evidence receipt and pull request remain
valuable because they make intent, decisions and evidence durable. Their value
does not depend on turning each artifact into a human approval queue. Risk and
external consequence, rather than document format or authorship, should decide
when an agent must escalate.

## Decision

1. The project sponsor sets product objectives, roadmap and protected
   boundaries. That recorded mandate is standing authority for routine work on
   `develop`.
2. Within the mandate, agents may select and prioritize backlog work; create or
   refine issues, OpenSpec contracts and ADRs; make reversible product and
   technical decisions; implement, test and locally review changes; publish
   focused branches; triage and repair branch-owned CI; and merge eligible pull
   requests into `develop`.
3. No separate human acceptance is required for task wording, artifact format,
   naming, routine scope decomposition, implementation details, refactoring,
   tests, documentation, CI repair or reversible tooling and dependency work.
4. Issues, contracts, ADRs, reviews and CI remain mandatory where the repository
   policy requires them. They are evidence and coordination gates, not approval
   ceremonies. A distinct review may be performed by an agent in a fresh
   context.
5. Agents escalate when a decision would materially change product strategy or
   public commitments; change governance or licensing; use secrets; handle a
   private vulnerability; perform an irreversible external action; publish or
   release artifacts; promote to `main`; change protected repository settings;
   or accept unresolved security, privacy, cryptographic, compatibility, legal,
   provenance or data-loss risk.
6. Material ambiguity between different product outcomes also requires
   direction after safe investigation cannot resolve it. Ordinary uncertainty
   is handled with explicit assumptions, reversible choices and recorded
   evidence.
7. Direct pushes, branch-protection bypass, false evidence and silent waiver of
   findings remain prohibited. Downstream repositories remain read-only unless
   a separate authorization names them.

## Consequences

- An agent can move from backlog selection through a green merge to `develop`
  and continue to the next eligible slice without waiting for ceremonial
  approval.
- Product and technical decisions remain auditable because agents must record
  scope, assumptions, trade-offs, evidence and review findings.
- Escalation becomes exceptional and risk-based instead of being triggered by
  the author or format of a planning artifact.
- Protected human authority over releases, external commitments, secrets,
  disclosures, repository administration and `main` is unchanged.
- The repository policy enables continuous operation but does not itself
  provide a scheduler or hosted agent runtime.

## Operational stop/go rule

The default is **go** when the work is issue-linked, within the roadmap,
reversible, repository-local and covered by a complete contract plus applicable
evidence gates. The default is **stop and escalate** only when a protected
boundary or unresolved material risk above is reached. Missing or failing CI is
normally a repair condition, not a request for human approval.
