# Constraint readiness

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/368
Constraint blockers: none

## Existing entries affected

- `SDK-SEC-003`: security-sensitive protocol state transitions require exact
  review, redaction and negative precedence tests.
- `SDK-LIM-007`: caller-owned HTTP allocation remains outside the SDK; typed
  request and response bounds remain explicit.
- IDR-023 remains in progress under a focused successor.

## Introduced or changed constraints

- Only a successfully parsed response from the consuming HTTP 202 branch can
  produce the new authorized Deferred Credential Request.
- The exact validated issuer, advertised Deferred Credential Endpoint, bearer
  Authorization, transaction identifier and proof count cannot be replaced.
- The proof body and obsolete Credential Endpoint are erased before parsing the
  remote HTTP 202 fields.
- The bearer capability remains zeroizing and moves exactly once.

## Introduced or changed limitations

- The new path recognizes only the existing unencrypted JSON profile.
- Missing advertised Deferred Credential Endpoint fails at request construction
  with the existing static error.
- The API preserves authority but does not validate token audience, freshness,
  scope, issuer control, endpoint origin, TLS or replay safety.
- HTTP, timers, interval/retry policy, polling and credential processing remain
  caller/downstream responsibilities.

## Consumer and product impact

Additive experimental Rust API only. No downstream repository, release,
certification, product behavior or compatibility promise is activated.

## Activation and rollback

Activation requires signed issue-linked PR, exact-diff review and green hosted
`fast` CI. Rollback is the additive removal described in research/design.

## Evidence

Focused tests cover exact authority retention, no detached replacement inputs,
status/error precedence, body bounds, missing endpoint, one-shot ownership and
redaction. Full and portable gates plus ADR/inventory/roadmap updates complete
the slice.
