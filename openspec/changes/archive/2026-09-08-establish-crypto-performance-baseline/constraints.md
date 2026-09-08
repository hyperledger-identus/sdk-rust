# Constraint and limitation impact

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/214
Constraint blockers: none

## Existing entries affected

No effective constraint changes. Rust 1.98.1 remains the single etalon,
`SDK-DEP-001` remains satisfied because no dependency is added, and the
fast/slow topology from ADR 0081 remains unchanged.

## Introduced or changed constraints

Published crypto performance artifacts require at least 20 post-warm-up
samples, exact revision/toolchain/platform/features, median and p95, and an
explicit measurement-only/no-Apollo-comparison marker. These are evidence
integrity rules, not runtime budgets.

## Introduced or changed limitations

Timing values are host- and load-dependent and provide no cross-machine,
cross-language, constant-time, certification, throughput, battery or product
latency promise. Apollo performance remains unavailable at the pinned source.

## Consumer and product impact

Maintainers can observe representative SDK crypto cost and later detect trends.
Oxid, Midnight Identity, NeoPRISM, Lace ID Portal and Apollo are unchanged.

## Activation and rollback

Activation is an issue-linked merge into `develop` plus a green slow-lane
artifact. Rollback removes the example, workflow job, manifest fields and
documentation together. Threshold activation requires a future ADR.

## Evidence

Acceptance requires schema/unit tests, release execution with 20 samples,
artifact inspection, factory/Nix gates, distinct security/performance review,
hosted slow evidence and a Discussion #178 receipt.
