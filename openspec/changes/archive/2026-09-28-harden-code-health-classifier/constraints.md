# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/301
Constraint blockers: none

## Existing entries affected

- `SDK-ARCH-004`: the touched code-health classifier must improve semantic
  cohesion, preserve separated populations, and retain explicit evidence.
- ADR 0126: production-wins reachability, unknown-as-production behavior,
  bounded protocol inputs, and linear projection remain effective.

## Introduced or changed constraints

Canonical evidence additionally binds all classifier Rust source bytes and the
resolved `syn`/`proc-macro2` package identities. A source or parser dependency
change therefore requires deliberate evidence regeneration rather than sharing
the previous logical classifier name.

## Introduced or changed limitations

Macro expansion, arbitrary Cargo build-script target generation, and a complete
cross-product of every Rust cfg key remain outside the classifier contract.
Differential fixtures cover only the syntax and target roles supported by ADR
0126. Timing and memory observations are regression evidence, not a public SDK
performance promise.

## Consumer and product impact

None. `identus-conformance` remains unpublished and no SDK API, feature,
dependency surface, target promise, wire shape, or downstream repository
changes.

## Activation and rollback

Activation occurs atomically when the decomposed classifier, corrected
semantics, tests, documentation, policy digests, and regenerated exact baseline
merge. Reverting that PR restores the prior private implementation and evidence
contract; no consumer migration is required.

## Evidence

Focused Rust and Python tests, deterministic differential fixtures, bounded
stress/complexity checks, exact baseline verification, OpenSpec/factory gates,
strict Clippy, Rustdoc, workspace tests, Nix checks, and hosted fast CI.
