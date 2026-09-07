# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-07
Source retrieval date: 2026-09-07
Research blockers: none

## Problem and existing implementation

The repository already classifies MSRV, targets, unsafe code, compatibility,
repository boundaries and releases in accepted ADRs and machine-readable
support policy. Those facts are distributed, and active OpenSpec changes have
no required constraint-impact artifact. PR policy currently checks only the
issue and local-review record.

## Normative sources

Repository-local governance is controlling: `GOVERNANCE.md`, ADRs 0003, 0004
and 0062, `docs/architecture/sdk-support-policy.toml`, the SDK blueprint and
the factory handbook. Issue https://github.com/hyperledger-identus/sdk-rust/issues/166
records the sponsor request. No external technical standard is changed.

## Candidate decisions

Use `retain-local` for a small TOML index and Python-standard-library checker,
matching the existing support-policy and research-readiness controls. Use
`not-adopt` for a general policy engine because this contract needs no service,
runtime, network, plugin language or new dependency cone.

## Compatibility and dependency evidence

The change adds documentation and offline repository tooling only. It changes
no Cargo version, feature, public API, wire representation, MSRV or supported
target. The checker reads existing TOML with Python `tomllib`; direct and
resolved runtime dependency cones remain unchanged.

## Security, privacy and maintenance evidence

The checker performs no network access, evaluates no untrusted code, uses no
unsafe or native code and handles no secrets or personal data. Maintenance is
bounded to a versioned schema, finite vocabularies, deterministic file reads
and focused tests. Release and security posture are unchanged.

## Rejected or deferred candidates

A remote governance service, OPA/Rego engine, CUE policy layer and GitHub-only
metadata were rejected for current use because they add coupling and a larger
supply-chain surface without improving this deterministic repository-local
contract. Reconsideration trigger: the local schema becomes unable to express
cross-repository or organization-wide policy shared by multiple projects.

## Open questions and blockers

None. The sponsor direction resolves the only material governance choice. The
effective Rust floor remains 1.85.0 and the 1.95.0 transition remains a target
owned by issue #154.

## Evidence commands

Run the focused checker tests, PR-policy tests, factory contract, strict
OpenSpec validation and full Nix flake check. Unrun checks must be reported
exactly; no passing result is inferred from prose.
