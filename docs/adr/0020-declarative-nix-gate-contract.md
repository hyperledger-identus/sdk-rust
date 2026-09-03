# ADR 0020: generate Rust gates from a declarative contract

- **Status:** Proposed
- **Date:** 2026-09-04
- **Related work:** sdk-rust issue #24 and OpenSpec change
  `declare-nix-support-gates`
- **Supersedes:** the transitional Nix-expression inference portions of ADR
  0002 and PR #23; their toolchain and compatibility decisions remain accepted

## Context

PR #23 made the SDK support matrix executable and fail-closed, but its offline
validator discovers Nix definitions and reconstructs Cargo selections from
regular-expression matches over Nix source. This caught meaningful bootstrap
drift, yet it makes harmless Nix syntax a policy language and burdens future
agents with coordinating duplicate argument strings.

## Decision

1. `sdk-support-policy.toml` remains the machine-readable compatibility claim.
2. `nix/checks/gates.toml` is the versioned execution contract for every named
   Rust gate. The support policy retains the independent expected-operation map
   used to detect semantic operation drift.
3. Each gate declares its Crane operation, etalon/MSRV toolchain, source and
   artifact class, and structured Cargo selection including workspace,
   packages, exclusions, feature mode and target.
4. Nix reads the TOML and generates the real check attributes and Crane calls.
5. The offline validator reads the same TOML, validates its closed schema and
   compares effective selections to policy claims without parsing Cargo
   semantics from Nix text.
6. The validator retains only narrow execution-wiring checks. A full Nix gate
   remains the proof that generated derivations evaluate and build.
7. Twenty-sample warm and process-cold p50/p95 measurements run on Linux and
   macOS as diagnostic evidence. They are not a support budget.

## Consequences

- Agents add or change a gate in one structured execution record and update a
  compatibility claim only when the support promise changes.
- Nix comments, quoting, interpolation and formatting no longer affect Cargo
  policy interpretation.
- Unknown fields and execution combinations fail closed instead of silently
  becoming custom Nix behavior.
- Gate-specific behavior outside the schema requires an explicit schema and
  generator extension.
- A coordinated malicious repository edit remains outside this drift-control
  threat model and is handled by signed review, exact-head CI and protection.

## Rollback

Revert the focused PR and restore the hand-written gate modules plus #23
validator. No public API, persisted data, consumer repository, release or
`main` migration is involved.
