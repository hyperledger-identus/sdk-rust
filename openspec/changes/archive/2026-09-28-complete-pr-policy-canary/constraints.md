# Constraint readiness

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/432
Constraint blockers: none

## Existing entries affected

ADR 0166 and the base-owned pull-request policy contract remain authoritative.

## Introduced or changed constraints

The steady-state workflow has exactly one pull-request event:
`pull_request_target`. A natural exact-head run is mandatory evidence.

## Introduced or changed limitations

Policy monotonicity remains #339. No repository setting is changed.

## Consumer and product impact

None; this is repository governance only.

## Activation and rollback

Green natural trusted-event evidence activates the steady state. A revert is
the rollback, with an incident record if legacy execution is restored.

## Evidence

Focused source tests, actionlint, factory checks, signed/DCO commits, file
hygiene, fast CI, and the exact-head `pull_request_target` run are required.
