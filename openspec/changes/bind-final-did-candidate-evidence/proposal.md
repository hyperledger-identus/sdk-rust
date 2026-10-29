# Bind final DID candidate evidence

## Why

The DID candidate train has deterministic archives, API origins, SBOMs,
documentation, and a closed compiler/target matrix, but milestone M5 still
lacks one durable exact-revision readiness receipt. The latest natural weekly
slow run failed on a subsequently resolved, narrowly governed UniFFI generator
maintenance advisory, so an older green run cannot be presented as current
release evidence.

## What changes

- Add a durable final-readiness report for the unchanged `identus-did` and
  `identus-did-resolver-http` `0.1.0-rc.1` candidate.
- Bind candidate identity, API/SBOM/source evidence, documentation, native and
  portable target evidence, slow-run provenance, and explicit non-claims.
- Link immutable NeoPRISM source-pinned adoption and fixed-boundary Preprod
  parity/QoS receipts as downstream compatibility evidence.
- Reconcile the M5 issue hierarchy only after the evidence actually passes.

## Capabilities

### Modified capabilities

- `release-candidate-trains`: requires one closed exact-SHA approval packet
  before a candidate-only train can be proposed for activation.
- `sdk-documentation-site`: exposes the durable candidate-readiness packet and
  its limitations to engineers.

## Non-goals

No package version or publish-flag change, tag, GitHub release, crates.io
upload, trusted-publisher setting, branch-protection change, stable API claim,
new DID behavior, downstream mutation, or production promotion.

## Delivery

Issue #388 owns this final M5 evidence slice from protected
`develop@34420235103fe9dc599aa061047525c459d422cf`, after the additive-only
API-origin rebaseline merged through PR #584.
