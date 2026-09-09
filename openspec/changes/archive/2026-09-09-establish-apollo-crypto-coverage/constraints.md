# Constraint and limitation impact

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/212
Constraint blockers: none

## Existing entries affected

`SDK-COMPAT-002`, `SDK-COMPAT-004` and `SDK-COMPAT-005` remain unchanged at
Rust 1.98.1. `SDK-LIM-009` continues to keep exhaustive evidence weekly/manual;
the fast pull-request lane does not gain coverage work. `SDK-DEP-001` does not
apply because the selected tool is not a production dependency.

## Introduced or changed constraints

M2 crypto coverage artifacts must cover at least 74.82% of executable
first-party lines below `crates/crypto/src` across the declared four feature
profiles. A reduction of this accepted threshold requires a superseding ADR.
This is an evidence-quality ratchet, not a runtime or consumer budget.

## Introduced or changed limitations

The evidence is line coverage only. It does not claim branch, path, mutation,
side-channel, platform runtime, production input or certification coverage.
Doc tests remain outside the stable instrumentation path; dependencies,
generated code, tests and examples are outside the first-party denominator.

## Consumer and product impact

Consumers gain reviewable confidence in `identus-crypto` tests without a new
dependency or compatibility promise. Apollo, NeoPRISM, Midnight Identity,
Oxid and Lace ID Portal remain unchanged.

## Activation and rollback

Activation is the issue-linked `develop` merge plus a successful slow artifact
for the implementation revision. Rollback removes the Nix tool/component,
runner, workflow job, manifest coverage table and ADR together. Lowering the
threshold is not a routine rollback and requires an ADR.

## Evidence

Acceptance requires the four-profile runner, normalizer mutation tests,
74.82% gate, vector/file mapping validation, local Rust 1.98.1 evidence,
factory/Nix gates, distinct review, a slow Actions artifact and a Discussion
#178 receipt.
