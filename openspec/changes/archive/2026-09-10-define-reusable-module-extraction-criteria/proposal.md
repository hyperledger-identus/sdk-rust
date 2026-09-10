# Change: define reusable-module extraction criteria

## Why

The next SDK milestone will consume sdk-rust from NeoPRISM and move reusable
behavior in the opposite direction, from NeoPRISM into sdk-rust. The current
source matrix identifies likely surfaces, but it does not define a hard,
repeatable test for the word "reusable". Without that contract, extraction can
turn the SDK monorepo into a collection of donor-specific helpers, leak
PRISM/Cardano policy into generic APIs, or delete downstream code before an
immutable replacement is proven.

Issue #253 records the sponsor-directed architecture decision. NeoPRISM issue
#321 and branch `codex/sdk-rust-beta` provide an existing, isolated adoption
experiment; they are evidence, not a released dependency or completed
compatibility proof.

## What Changes

- Add ADR 0110 with mandatory reusable-module gates for ownership, dependency
  direction, cohesion, orthogonality, portability, security, provenance,
  consumer evidence, and release independence.
- Retain the finite source dispositions `extract`, `adapt`,
  `conformance-only`, `remain-downstream`, and `reject`.
- Require a per-candidate assessment against an immutable donor revision.
- Require SDK-first delivery and a separately authorized NeoPRISM adoption
  slice pinned to an immutable SDK revision before duplicate downstream code is
  removed.
- Record the current NeoPRISM module triage and characterize the existing beta
  branch without changing either repository's implementation.

## Capabilities

### New capabilities

- `reusable-module-extraction`: architecture gates and lifecycle for deciding,
  extracting, adopting, and retiring duplicated generic modules.

### Modified capabilities

None. The new capability refines the existing SSI upstream program without
replacing any canonical requirement.

## Impact

- Affected documentation: ADR index, NeoPRISM source evidence, and reusable
  module assessment contract.
- Affected future work: all NeoPRISM-to-sdk-rust extraction and sdk-rust-to-
  NeoPRISM adoption slices.
- No Cargo dependency, public API, runtime behavior, publication promise,
  release, `main` promotion, or downstream source changes are included.
