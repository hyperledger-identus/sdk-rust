## 1. Decision contract

- [x] 1.1 Add ADR 0064 superseding ADR 0062 and record the three-axis policy.
- [x] 1.2 Update the support-policy and dependency-research specifications.
- [x] 1.3 Update the constraint registry, blueprint and contributor guidance.

## 2. Toolchain and gates

- [x] 2.1 Add and pin the stable rust-overlay input and Rust 1.98.1 provider.
- [x] 2.2 Move full quality and compile-target gates to primary stable.
- [x] 2.3 Retain an independent NeoPRISM-etalon workspace gate and Rust 1.85 MSRV matrix.

## 3. Enforcement

- [x] 3.1 Extend support-policy validation for three providers and exact revisions.
- [x] 3.2 Add negative tests for primary, etalon, MSRV and artifact cross-wiring.
- [x] 3.3 Validate 1.89 as a candidate without changing the effective Cargo floor.

## 4. Verification and review

- [x] 4.1 Run focused factory, policy and Rust checks.
- [x] 4.2 Run the complete Nix flake gate and record host limitations.
- [x] 4.3 Complete a distinct local review, archive the change and prepare the issue-linked PR.
