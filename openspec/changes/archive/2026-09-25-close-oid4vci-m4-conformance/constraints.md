# Constraint readiness

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/372
Constraint blockers: none

## Existing entries affected

- `SDK-SEC-003`: the report must distinguish bounded typed entry from HTTP,
  decompression and outer allocation work the caller still owns.
- `SDK-LIM-007`: the matrix must preserve all applicable outer-allocation and
  delegated-work limitations.
- IDR-023 and M4 remain in progress unless every required closeout condition is
  evidenced and every residual gap has an explicit owner.

## Introduced or changed constraints

- Every matrix row uses one closed status and repository-local evidence paths.
- An implemented row requires public surface, canonical spec and test evidence;
  a partial or unsupported row requires an explicit limitation.
- A missing required wallet-core row requires one open focused issue.
- Reference-only consumer fixtures cannot be represented as imported or
  executed SDK conformance evidence.

## Introduced or changed limitations

- The matrix is traceability evidence, not official OpenID certification or a
  claim that every optional Final capability is supported.
- Consumer vector import remains unavailable while the exact Portal source has
  no explicit license and its Midnight profile has not been separated from
  generic protocol evidence.
- File/path existence cannot by itself prove semantic correctness; review and
  executable tests remain required.

## Consumer and product impact

No consumer code or product behavior changes. The report may recommend
downstream adoption or consumer review, but it activates neither. Existing
Oxid and Lace ID Portal checkouts remain read-only.

## Activation and rollback

Activation requires a signed issue-linked PR, passing matrix/checker tests,
exact-diff review and green hosted `fast`. Rollback removes the matrix, report
and factory checker hook; it does not alter runtime behavior or wire data.

## Evidence

The checker validates schema, required Final sections, closed vocabulary,
existing bounded repository paths, implemented/partial evidence rules,
reference-only provenance and focused issue ownership. Focused crate tests and
full factory gates show that the evidence change does not regress behavior.
