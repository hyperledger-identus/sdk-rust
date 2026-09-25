# Constraint readiness

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/366
Constraint blockers: none

## Existing entries affected

- `SDK-SEC-003`: security-sensitive protocol state transition requires exact
  review, redaction and negative precedence tests.
- `SDK-LIM-007`: caller-owned HTTP allocation remains outside the SDK; typed
  body and Content-Type limits remain explicit.
- IDR-023 remains in progress and advances to successor #368.

## Introduced or changed constraints

- A request can enter the new response transition once by Rust ownership.
- Status is classified before media type and body.
- Immediate credential count cannot exceed the originating proof count.
- No replacement request authority is accepted.

## Introduced or changed limitations

- Only unencrypted JSON 200/202/400 Final envelopes are recognized.
- RFC 6750 authorization errors remain unsupported.
- HTTP execution/provenance, trust, retries, polling, validation and storage
  remain caller/downstream responsibilities.

## Consumer and product impact

Additive experimental Rust API only. No downstream repository, release,
certification, product behavior or compatibility promise is activated.

## Activation and rollback

Activation requires signed issue-linked PR, exact-diff review and green hosted
`fast` CI. Rollback is the additive removal described in research/design.

## Evidence

Focused branch tests cover every status branch, media/body precedence,
proof-count binding, one-shot ownership and redaction; full and portable gates
plus ADR/inventory/roadmap updates complete the slice.
