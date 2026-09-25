# Constraint readiness

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/391
Constraint blockers: none

## Existing entries affected

ADR 0156 and the `oid4vc-rust-reuse-assessment` specification remain
authoritative. SDK architecture, compatibility, security, delivery, and
limitation constraints continue unchanged.

## Introduced or changed constraints

- Every issue #391 capability and measurement request receives an explicit
  result or a truthful unmeasured limitation.
- Vectors remain clean-room, spec-derived, deterministic, and isolated.
- Measurements identify command, compiler, host, scope, and interpretation;
  proxies cannot be presented as guaranteed production savings.

## Introduced or changed limitations

- One cold local compile is diagnostic, not a CI SLO or cross-host benchmark.
- Source lines are maintenance surface, not guaranteed SDK deletion.
- Allocation behavior is assessed structurally because adding an unsafe global
  allocator solely for research conflicts with repository policy.

## Consumer and product impact

None. The follow-up completes decision evidence only.

## Activation and rollback

The report activates as repository evidence after merge. Rollback removes the
report and additive vectors; production behavior and data are unaffected.

## Evidence

The final report must cite exact revisions/releases and commands, distinguish
measured facts from inference, and preserve all unrun runtime limitations.
