# Constraint and limitation impact

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/170
Constraint blockers: none

## Existing entries affected

`SDK-COMPAT-002` remains effective at Rust 1.85.0. `SDK-COMPAT-003`
changes from a Rust 1.95 release-distance target to an evidence-gated Rust 1.89
candidate. A new effective entry records the Rust 1.98.1 primary validation
compiler, and another records the independent NeoPRISM etalon nightly.

## Introduced or changed constraints

Primary stable, public MSRV, edition and etalon are independent axes. Rust
1.98.1 becomes the reproducible primary development and full-validation
compiler. Rust 1.85 remains the consumer floor. MSRV changes require measured
dependency or consumer value and exact target/downstream evidence rather than
calendar or release arithmetic.

## Introduced or changed limitations

No platform or runtime support claim changes. The Rust 1.89 target does not
promise compilation. NeoPRISM alignment now applies to its etalon lane and the
shared Nix baseline, not to the SDK's primary stable compiler.

## Consumer and product impact

Existing Rust 1.85 consumers remain supported. Contributors receive current
stable diagnostics and code generation. Future Rust 1.89 activation remains a
separate compatibility event. Dioxus and product UI dependencies cannot lift
the generic SDK floor.

## Activation and rollback

Merge activates the Rust 1.98.1 primary lane and evidence-driven MSRV policy.
Rollback restores the previous primary etalon gates and removes the second
overlay input. Rust 1.85 remains continuously enforced, so rollback does not
lower or raise the public compiler promise.

## Evidence

Machine policy, Cargo, Nix providers and generated gate mappings must agree.
Negative tests must reject cross-wired primary, etalon and MSRV providers.
Required CI must pass before integration.
