## Context

Material constraints currently live in multiple valid sources. The new index
must make them discoverable without becoming a second authority for detailed
support-policy values. Active changes also need a uniform impact record that
can distinguish exploratory targets from effective commitments.

## Goals and non-goals

Goals are a stable taxonomy, a concise cross-cutting index, deterministic
offline validation, explicit change-level impact and an authority rule that
prevents product surprises. Non-goals are changing an effective constraint,
inventing a human formatting ceremony, replacing specialist review or
building an organization-wide policy engine.

## Decisions

### Use a referenced TOML index

`docs/governance/sdk-constraints.toml` indexes only material cross-cutting
constraints and limitations. Each entry points to its canonical source and
records kind, category, state, scope, impact, enforcement, ownership, review
triggers, activation and rollback. Detailed target matrices remain in
`sdk-support-policy.toml`.

### Separate state from kind

Kind describes what a rule is: `hard`, `guardrail`, `budget` or `limitation`.
State describes lifecycle: `effective`, `target`, `deferred` or `prohibited`.
A target is never an effective promise. An exception is a separate bounded
record, not an undocumented weakening of the original entry.

### Require one change-level artifact

Every active OpenSpec change carries `constraints.md`. `none` and `routine`
impact may proceed under standing authority. `material` impact is ready only
when an exact sponsor/maintainer decision reference resolves the product
outcome. A proposed material choice remains structurally valid for research
but fails the pre-implementation readiness gate.

### Keep checks deterministic and proportional

The checker uses Python standard-library TOML and Markdown parsing, validates
referenced repository paths, and cross-checks the effective MSRV projection
against support policy. It does not use wall-clock expiry, network access or
prose-generation heuristics. Semantic review remains responsible for whether
the declared impact is truthful.

## Risks and mitigations

- A registry can drift: references are path-validated and high-risk
  projections such as MSRV are cross-checked.
- Authors can misclassify impact: PR disclosure and semantic review inspect
  the classification; machine validation is not represented as approval.
- Governance can block routine automation: only material consumer/product
  outcomes need an exact direction record; format and reversible local choices
  stay autonomous.
- Duplicate sources can conflict: the index explicitly references canonical
  sources and does not replace them.

## Rollout

Add the spec and ADR, seed the index, implement checker/tests, connect it to
factory readiness, then update contributor, agent, review and PR guidance.
Archive only after focused and repository-wide checks pass.
