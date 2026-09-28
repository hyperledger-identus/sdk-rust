## ADDED Requirements

### Requirement: Trusted workflow event transitions are canaried

The factory MUST stage a transition from a pull-request-controlled workflow
event to a protected-base event with only the minimum bounded bootstrap needed
to preserve the existing required status. The immediate next integration MUST
remove the legacy event, and no unrelated integration may land between
bootstrap and canary.

#### Scenario: Protected base owns the trusted event

- **WHEN** the bootstrap revision has merged and emits the trusted required
  status for a canary pull request
- **THEN** the canary removes the legacy event before any unrelated merge

#### Scenario: Canary lacks trusted evidence

- **WHEN** the exact-head protected-base event is missing, pending, cancelled,
  or failing
- **THEN** the canary SHALL NOT merge or claim the transition complete
