# Freeze the DID candidate staged lock

## Why

The DID candidate aggregator requires Linux/macOS and primary/MSRV lane receipts
to report one lock hash, but each lane currently creates a fresh Cargo home and
runs `cargo generate-lockfile` independently. A compatible dependency published
between lanes can therefore make correct builds disagree and invalidate final
M5 evidence for a reason unrelated to the reviewed SDK revision.

## What changes

- Commit one release-candidate-specific lockfile generated from the staged
  `0.1.0-rc.1` manifests.
- Bind its path and SHA-256 to the DID candidate descriptor.
- Verify and copy that lock into every staged build before invoking Cargo with
  `--locked`; matrix lanes may no longer resolve independently.
- Extend offline structure and mutation tests for missing, modified, stale, or
  bypassed staged locks.
- Record the frozen lock identity in lane and aggregate evidence unchanged.

## Capabilities

### Modified capabilities

- `release-candidate-trains`: staged archive and compiler/target evidence use
  one repository-reviewed dependency resolution.

## Non-goals

No dependency upgrade, manifest requirement, feature, compiler, target,
candidate version, crate API, publication, registry credential, workflow
dispatch/rerun, consumer repository, or support promise changes.

## Delivery

Issue #482 owns implementation and blocks final M5 evidence in #388. The PR
targets protected `develop` and uses normal exact-head required CI. A later
natural or explicitly authorized slow run, not this change, supplies cross-host
execution evidence.
