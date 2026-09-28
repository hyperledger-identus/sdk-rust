# Complete the trusted PR-policy event canary

## Why

PR #431 deliberately retained the legacy `pull_request` event for one merge so
the pre-transition required status could report. Protected `develop` now owns
the base-context event, so the immediate canary must remove the legacy trigger
before unrelated integration.

## What changes

- Remove `pull_request` and retain only `pull_request_target`.
- Update the regression test from bootstrap state to steady-state denial.
- Require a natural exact-head trusted-event run as activation evidence.

## What does not change

No policy invariant, permission, secret, repository setting, Rust code, SDK
API, or monotonicity rule changes.

## Capabilities

### Modified capabilities

- `ai-software-factory`: record and verify the steady-state trusted event.

## Authority

Issue #432 implements the canary required by #341 and ADR 0166.
