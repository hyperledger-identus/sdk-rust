# Establish public immutable-source distribution

## Why

Public downstreams can now read sdk-rust without credentials, and NeoPRISM PR
#324 has already compiled the five first consumer crates from exact revision
`04b45b7fceb094ae601e3cf7a3291de0a0248b57`. The repository does not yet state
which source references are acceptable, how Cargo and Nix consumers preserve
integrity, or how the existing package names relate to the temporary `0.0.0`
metadata and downstream compatibility facades.

Issue #255 owns the remaining SDK-side contract. Without it, successful public
resolution is an experiment rather than a repeatable alpha distribution path.

## What changes

- Select public Git source at an exact 40-hex commit as the only supported alpha
  distribution channel before registry publication.
- Document reproducible Cargo and Nix consumption, lockfile and provenance
  requirements, supported package identities and update procedure.
- Add an offline drift check for the distribution contract and its five proven
  source-consumable packages.
- Record the decision in an ADR and the repository README while retaining all
  publication and release prohibitions.

## Capabilities

### Added capabilities

- `source-distribution`: public, immutable, unauthenticated source consumption
  for pre-release Rust consumers.

## Non-goals

- No crate publication, crates.io reservation, tag or release.
- No package rename or claim that `0.0.0` is a useful SemVer release.
- No supported compiler matrix beyond the current Rust 1.98.1 source-consumer
  floor.
- No downstream repository mutation or claim that every downstream has
  migrated.
- No floating `develop`, branch, tag or pull-request reference as an integrity
  boundary.

## Delivery

Issue #255 owns this material but reversible distribution-policy change from
`develop@e197810b7d94cfa2e9182b7413110c5edac944db`. The PR targets `develop`.
The protected setting activation in issue #26 remains separately evidenced.
