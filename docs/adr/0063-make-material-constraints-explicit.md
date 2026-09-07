# ADR 0063: make material constraints and limitations explicit

- **Status:** Accepted
- **Date:** 2026-09-07
- **Issue:** [#166](https://github.com/hyperledger-identus/sdk-rust/issues/166)
- **Decision authority:** explicit project-sponsor direction in issue #166
- **Clarifies:** ADR 0004 standing agent authority and ADR 0062 rolling MSRV

## Context

The SDK already treats MSRV, targets, unsafe code, public compatibility and
release policy as architecture decisions. That classification was not enough
to prevent surprise. ADR 0062 introduced a technically defensible
stable-minus-three MSRV target, but the product consequence was not presented
through one explicit constraint-impact contract before the decision was
integrated.

Constraints and limitations are currently spread across ADRs, the blueprint,
support policy, governance, issues and implementation specs. Readers must infer
which statements are effective promises, future targets, deferred work or
prohibitions. Agents also lack a uniform point at which to distinguish an
ordinary reversible design choice from a material product or consumer choice.

The project must retain autonomous delivery without allowing plausible
generated prose to become a surprising compatibility or product commitment.

## Decision

1. Maintain `docs/governance/sdk-constraints.toml` as the machine-readable
   index of material cross-cutting constraints and known limitations. It
   references controlling sources instead of replacing their detailed policy.
2. Classify every entry independently by kind and lifecycle state:
   - kind: `hard`, `guardrail`, `budget` or `limitation`;
   - state: `effective`, `target`, `deferred` or `prohibited`.
3. Every entry records a stable ID, category, scope, source, authority,
   rationale, consumer impact, enforcement evidence, owner, review triggers,
   activation path and rollback path.
4. Every active OpenSpec change includes `constraints.md`. It declares impact
   as `none`, `routine` or `material`, names changed constraints and
   limitations, distinguishes effective state from targets, and records
   consumer impact, activation, rollback, evidence and blockers.
5. A change is material when it changes an effective consumer-visible
   compatibility floor, supported target or feature, public/wire/data promise,
   product or chain boundary, security/privacy/cryptographic posture, license
   obligation, certification claim, resource/performance budget, release
   promise or irreversible migration.
6. A new or changed material outcome is implementation-ready only when an
   exact durable sponsor or responsible-maintainer decision resolves it, or an
   existing effective index entry or roadmap decision already authorizes that
   exact outcome. The change record uses `directed` and cites that source.
7. A material proposal without that authority remains valid for research and
   reversible non-activating preparation, but its constraint-readiness gate
   fails. Agents do not turn it into an effective promise or merge its
   activation.
8. Routine naming, formatting, decomposition, testing, implementation detail,
   refactoring and reversible repository-local tooling remain under standing
   agent authority and require no human format approval.
9. Targets never activate by date, dependency update or quarterly review
   alone. Activation changes the controlling source and enforcement evidence
   through a focused issue, decision record, specification and PR.
10. Exceptions are separate bounded records. They identify the base
    constraint, affected scope, owner, rationale, exit trigger and
    security/maintenance cost instead of silently weakening the constraint.

## MSRV interpretation

ADR 0062 remains a researched policy direction, not an automatic compiler-floor
change. Rust 1.85.0 is the effective SDK promise because that is the value in
`sdk-support-policy.toml` and the value compiled by the MSRV gates. Rust 1.95.0
is a target only. Issue #154 must present its consumer impact and obtain an
exact activation decision before changing the effective source. A quarterly
review does not supply that decision.

## Consequences

- Current promises, future intent and unsupported surfaces become visible in
  one inventory.
- Product-impacting choices are surfaced before activation rather than after
  implementation.
- Agents can continue routine work continuously; escalation is based on
  consequence, not authorship or document format.
- The registry and checker add maintenance work when cross-cutting constraints
  change, but ordinary component-local invariants do not belong in the index.
- Machine checks prove declared structure and selected projections. They do not
  prove the author classified impact honestly; semantic review remains
  accountable for that judgment.
- When current implementation does not yet satisfy a forward guardrail, the
  gap is indexed as an effective limitation rather than weakening the forward
  rule or claiming unearned coverage.

## Alternatives rejected

### Put every implementation invariant in one registry

This would duplicate capability specs, create constant churn and obscure the
few constraints that materially affect consumers. The index is deliberately
cross-cutting.

### Treat every ADR as sufficient product awareness

An ADR explains a decision but does not guarantee its consumer consequence was
made visible or that a future target was kept separate from an effective
promise.

### Require human approval for every change

This would reintroduce the format and handoff friction removed by ADR 0004.
Routine reversible choices remain autonomous; only unresolved material outcomes
need direction.

### Keep the rules in GitHub metadata only

That would make fresh-clone and offline validation incomplete and would couple
the engineering contract to one hosting service.

## Rollout and rollback

Seed the index only from already accepted sources, add the active-change
artifact and checker, integrate it into factory and PR gates, and update agent
and contributor guidance. This rollout changes no effective SDK compatibility
value.

Rollback reverts the governance/factory PR. Any later constraint activation is
separately reversible according to its own issue and migration contract.
