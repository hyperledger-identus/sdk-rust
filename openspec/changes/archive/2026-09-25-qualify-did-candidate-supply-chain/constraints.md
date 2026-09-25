# Constraints and limitations

Impact class: routine
Decision status: not-required
Decision reference: not-required
Constraint blockers: none

## Existing entries affected

- `SDK-RELEASE-001`: canonical publication denial remains effective.
- `SDK-LIM-004`: Rust 1.89.0 MSRV and 1.98.1 etalon remain unchanged.
- `SDK-DEPENDENCY-001`: no runtime dependency is added; evidence tools are
  release-only and exact.
- ADRs 0134 and 0153 remain authoritative for protected publication and train
  identity.

## Introduced or changed constraints

Candidate API/SBOM evidence uses exact repository-owned tools and paths. A
first candidate cannot report SemVer comparison as passed; its state is
`not-applicable` until a real predecessor exists.

## Introduced or changed limitations

The API snapshots are a first-candidate origin, not a stable SemVer guarantee.
CycloneDX output is a local SBOM, not a signature, registry attestation,
vulnerability result or proof that every transitive crate contains no unsafe or
native code. Platform and slow promotion evidence remain separate.

## Consumer and product impact

None. Consumers continue using exact Git revisions; no product repository,
public API, wire behavior, runtime dependency or remote artifact changes.

## Activation and rollback

Activation requires issue-first OpenSpec, exact tool policy, mutation tests,
archive-hash preservation, local review, Taplo/factory checks and exact-head
hosted CI. Rollback removes only additive evidence configuration/artifacts.

## Evidence

Issues #381/#384, the #382 candidate receipt, repository Cargo-deny policy,
generated baseline/SBOM digests and protected PR CI provide evidence.
