# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/547
Constraint blockers: none

## Existing entries affected

SDK-ARCH-001 through SDK-ARCH-003 keep effects and foreign formats at adapters.
SDK-SEC-003 and SDK-LIM-007 require finite hostile-input and callback work.
ADRs 0168, 0179, 0180, 0181, 0183, 0184, and 0185 control the existing request,
DCQL, response, security, authorization, and effect-neutral lineage.

## Introduced or changed constraints

- The core validates and retains exact encoded transaction data, common Final
  fields, DCQL references, format identity, and selected-input linkage.
- Every non-empty DCQL `meta` object and every transaction type requires one
  exact caller-supplied capability; absence or ambiguity fails closed.
- Format capabilities return SDK-owned static outcomes and cannot expose
  foreign DTOs, errors, transports, persistence, or proof-engine types.
- Candidate adapters fail closed for non-empty metadata and affirm the
  accepted constraint against each credential during DCQL evaluation.
- Proof construction and verification remain in owning credential-format crates.
- The implementation has no ambient/global registry and performs no effects.
- All attacker-controlled structures and callback work have explicit finite bounds.
- Diagnostics, metrics, and formatting reveal only static categories and counts.

## Introduced or changed limitations

The core does not prove the business meaning of transaction data, create or
verify a presentation, establish credential authenticity, select consent, or
certify HAIP/format conformance. A successful port decision proves only that a
caller-owned capability accepted bounded metadata/binding input.

## Consumer and product impact

Consumers using non-empty metadata or transaction data must supply matching
capabilities. Rust-native types remain canonical; SDK-TS/Swift/KMP migration
belongs in thin adapters. Existing empty metadata without transaction data
continues through the built-in baseline capability.

## Activation and rollback

Activation requires planning readiness, issue preflight, vectors, local and
portable evidence, factory/Nix gates, CLI review, signed+DCO commits, green
protected CI, and guarded merge. Before publication, rollback removes the new
API/state and restores strict empty metadata.

## Evidence

Issue #547 and parent #541 direct this work. The normative and dependency
decisions are recorded in `research.md`; successful implementation evidence is
recorded only during closeout.
