# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/543
Constraint blockers: none

## Existing entries affected

SDK-ARCH-001 through SDK-ARCH-003 retain inward dependencies and effects at the
edge. SDK-SEC-003 and SDK-LIM-007 retain finite hostile-input work. ADR 0182 and
the `siopv2.did-authentication.draft13.v1` ledger entry remain authoritative.

## Introduced or changed constraints

- Lifecycle idempotency is current-state-local: the same event identity is
  idempotent only while its resulting state remains current.
- A different identity for the current transition is a conflict. An event for a
  no-longer-current transition is invalid and does not mutate state.
- Durable/cross-transition deduplication, retention, and audit history remain
  caller-owned; the core does not add a collection, store, clock, or runtime.
- `OutcomeUnknown` remains terminal and never authorizes automatic retry.
- Wallet-nonce correlation and response-encoding failures receive distinct
  stable, static, redacted categories.
- JOSE failures are classified at header construction, signing-input
  construction, and signing. Explicit size failures retain `ResponseTooLarge`;
  algorithm, signer, and encoding/invariant failures map to their accurate
  SIOPv2 categories.
- Required string values are validated before optional state and wallet nonce,
  with the existing byte/control/empty rules and limits unchanged.
- The compact request remains owned and zeroized; parsing twice is deliberate
  so non-zeroizing parsed compact, payload, and signature copies are not retained
  for the lifetime of the public unverified state.
- URI and form helpers remain private to the protocol and cannot be used to
  bypass typed request/response validation.
- No new dependency, unsafe code, native library, target, feature, wire field,
  transport, persistence, trust, custody, UI, or downstream mutation enters.

## Introduced or changed limitations

The core does not remember event identities after their state is superseded and
does not provide durable idempotency across processes. Stable diagnostic
categories do not expose dependency details or sensitive values. The crate
continues to implement only the pinned experimental draft-13 profile.

## Consumer and product impact

Consumers may distinguish wallet-nonce mismatch from client binding and
response encoding from size. Existing successful API and wire behavior is
unchanged. Product storage, audit, retry, telemetry, and migration adapters
remain consumer-owned.

## Activation and rollback

Activation requires issue-bound planning preflight, focused and portable
evidence, exact-head independent CLI review, signed/DCO commits, and protected
green CI. Before publication, rollback removes the error refinements and
clarifying contract together; no persisted or wire migration is needed.

## Evidence

Issue #543 authorizes the slice. Research sources and commands are recorded in
`research.md`; implementation receipts are recorded only after they succeed.
