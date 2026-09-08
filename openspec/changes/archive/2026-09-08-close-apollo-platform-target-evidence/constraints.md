# Constraint and limitation impact

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/213
Constraint blockers: none

## Existing entries affected

No effective constraint changes. The evidence implements the existing
Rust/target policy under `SDK-COMPAT-002`, `SDK-COMPAT-004`,
`SDK-LIM-003` and `SDK-LIM-009`.

## Introduced or changed constraints

The Apollo parity ledger must bind its portable-target closing receipt to the
exact target gate, package list, feature list, Rust toolchain and common green
slow-run revision from the support policy. This is an evidence-integrity rule,
not a new compatibility promise.

## Introduced or changed limitations

None. WASM, iOS and Android remain compile-checked only. Browser/device runtime,
linking, bundling, FFI, native packages, platform stores, storage/key services,
performance and certification remain downstream or future work.

## Consumer and product impact

Consumers gain an auditable answer to which SDK packages compiled for each
portable target at the M2 closing SHA. Oxid, Midnight Identity, NeoPRISM, Lace
ID Portal and Apollo remain unchanged.

## Activation and rollback

Activation is the issue-linked merge of the validator and receipt to `develop`
after the exact slow run is green. Rollback removes the receipt fields and
validator extension together; it does not affect crate behavior.

## Evidence

Acceptance requires mutation tests for stale revisions, host-gate substitution,
non-Actions links and support-policy drift; deterministic report rendering;
factory checks; and the exact successful manual slow run.
