# Constraint readiness

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/370
Constraint blockers: none

## Existing entries affected

- `SDK-SEC-003`: the consuming protocol transition requires exact review,
  redaction and negative precedence tests.
- `SDK-LIM-007`: caller-owned HTTP allocation remains outside the SDK; typed
  media and body bounds remain explicit.
- IDR-023 remains in progress under a focused successor.

## Introduced or changed constraints

- Status selects exactly one existing bounded parser before media/body input is
  inspected.
- HTTP 200 credentials cannot exceed the exact originating proof count.
- Only a parsed HTTP 202 carrying the exact request transaction may retain the
  exact issuer, endpoint, Authorization and proof count.
- Terminal, unsupported, malformed and mismatched paths erase all request
  authority and accept no replacement lineage inputs.

## Introduced or changed limitations

- The classifier recognizes only the existing unencrypted JSON profile.
- It proves structural lineage, correlation and bounded parsing, not response
  origin, token validity, issuer control, TLS, replay or trust.
- HTTP, decompression, timers, interval/retry policy, polling and credential
  processing remain caller/downstream responsibilities.

## Consumer and product impact

Additive experimental Rust API only. No downstream repository, release,
certification, product behavior or compatibility promise is activated.

## Activation and rollback

Activation requires a signed issue-linked PR, exact-diff review and green
hosted `fast` CI. Rollback removes the additive classifier, bound error wrapper
and composite limits type while preserving all borrowed APIs.

## Evidence

Focused tests cover every status, exact transaction retention, over-issuance,
media/body bounds, error classification, no replacement inputs, one-shot
ownership and redaction. Full and portable gates plus ADR/inventory/roadmap
updates complete the slice.
