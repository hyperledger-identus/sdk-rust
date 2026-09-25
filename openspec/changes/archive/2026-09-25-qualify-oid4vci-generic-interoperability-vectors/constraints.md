# Constraint readiness

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/376
Constraint blockers: none

## Existing entries affected

- `SDK-SEC-003`: the suite executes existing bounded untrusted-input entry
  points and cannot imply bounds on HTTP, decompression or pre-SDK allocation.
- `SDK-LIM-007`: transport, delegated proof/format work and external effects
  remain caller-owned.
- IDR-023 and M4: functional completion becomes permissible only after the
  machine matrix has no missing required row and the reviewed suite preserves
  every residual limitation.

## Introduced or changed constraints

- Generic conformance fixtures are repository-authored under Apache-2.0 and
  contain no copied consumer bytes.
- Every retained fixture has closed provenance, SHA-256 drift protection,
  normative mapping, expected result and a public SDK entry point.
- Consumer artifacts with ambiguous licensing or chain/product behavior remain
  immutable reference evidence only.
- An executable wallet-core vector may establish generic expressibility; it
  cannot establish transport provenance, live product interoperability or
  human consumer approval.

## Introduced or changed limitations

- The suite covers the bounded wallet-side profile, not issuer-side proof
  validation, HTTP execution, credential-format semantics, trust or storage.
- The `midnight_cbor_phase1` representation remains downstream and is not
  imported, named in executable payloads or interpreted by the SDK.
- Portal's missing license remains recorded; closing the generic SDK row does
  not legalize copying its files later.
- No official OpenID certification, current app-team sign-off or production
  Oxid/Portal end-to-end result is claimed.

## Consumer and product impact

No consumer code or behavior changes. The pinned consumer revisions become
durable design oracles only. A later downstream adoption or live
interoperability issue must build against an immutable SDK release/candidate
and produce its own result.

## Activation and rollback

Activation requires a signed issue-linked PR with manifest drift validation,
positive and negative public-API execution, matrix/backlog updates, exact-diff
review and green required fast CI. Rollback removes additive fixtures/tests
and restores the matrix/backlog row to missing/in-progress; no runtime, stored
data or consumer migration is involved.

## Evidence

Issue #376 and parent #7 explicitly direct provenance qualification and generic
vector delivery. The OpenID4VCI Final source, pinned consumer ADR/profile
artifacts, fixture hashes, public-API tests, conformance checker and consumer
read-only receipts provide the material provenance evidence.
