# Document the DID candidate train

## Why

#382 and #384 established a deterministic, candidate-only `0.1.0-rc.1` train
for `identus-did` and `identus-did-resolver-http`, including API origins and
reproducible CycloneDX evidence. The public handbook still presents only the
crypto train and says DID packages are deferred, so its release-review story is
now materially stale.

## What changes

- Add a dedicated DID candidate page with responsibilities, dependency
  direction, feature policy, source-evaluation examples and explicit non-goals.
- Add a tracked architecture diagram that separates the generic DID domain and
  ports from the optional host-side Axum adapter.
- Reconcile the handbook landing, release-train, crate index, adoption,
  readiness and limitations pages with the two independent train states.
- Link the exact #382/#384 decisions and retain clear API, SBOM, advisory,
  provenance, platform, registry and publication claim boundaries.

## Capability

### Modified capability

- `sdk-documentation-site`: adds a truthful, navigable engineer-review surface
  for the candidate-only DID train.

## Non-goals

No canonical manifest/version change, publication, tag/release, workflow or
repository-setting change, platform qualification, chain/DID-method behavior,
binding publication, downstream mutation, stable API promise, certification or
vulnerability-free claim.

## Delivery

Issue #386 owns this bounded M5 documentation slice from protected
`develop@cda086f3e7fe72d251c1f896bccdcf5dd1bc8c16`.
