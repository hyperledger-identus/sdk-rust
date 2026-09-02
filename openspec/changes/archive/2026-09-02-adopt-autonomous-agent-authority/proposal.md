## Why

The factory delegates branch publication and green-CI merge but still models
every component scope and contract as waiting for human acceptance. That formal
handoff prevents continuous AI-first delivery and adds no protection for
routine, reversible work already bounded by the roadmap, repository isolation,
local review and CI.

## What Changes

- Establish the project roadmap and sponsor boundaries as standing authority
  for routine agent work on `develop`.
- Permit agents to choose, prioritize and specify bounded work and make
  reversible product and technical decisions without per-artifact approval.
- Treat issues, OpenSpec, ADRs, reviews and CI as durable evidence gates rather
  than human approval queues.
- Replace the `contract-approved` state with an evidence-based
  `contract-ready` state.
- Define narrow, risk-based escalation conditions for strategy, governance,
  licensing, secrets, private disclosure, irreversible external actions,
  releases, `main`, repository settings and unresolved material risk.
- Preserve mandatory issue linkage, contract completeness, local review, green
  CI, branch protection, repository isolation and protected human authority.

## Capabilities

### New Capabilities

- None.

### Modified Capabilities

- `ai-software-factory`: Establish standing authority and evidence-based
  escalation for continuous routine delivery.
- `spec-driven-delivery`: Make an agent-authored, semantically reviewed contract
  operational without a separate human acceptance state.

## Impact

The change affects governance, agent instructions, factory documentation,
GitHub templates and OpenSpec contracts. It changes no Rust API, crate graph,
wire format, release artifact, live GitHub setting or downstream repository.
