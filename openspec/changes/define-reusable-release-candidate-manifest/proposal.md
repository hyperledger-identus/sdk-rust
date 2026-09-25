# Define reusable release-candidate manifest

## Why

The first published train is deliberately hard-coded to three crypto
foundation crates and immutable tag `v0.1.0-rc.1`. That tag cannot identify a
second independent `0.1.0-rc.1` train. M5 needs reviewable candidate evidence
for `identus-did` and `identus-did-resolver-http` without changing canonical
package versions/publish flags, the protected publisher, or the first release
receipt.

## What changes

- Add one closed train index that records immutable first-train identity and a
  candidate-only DID train through separate descriptors.
- Adopt primary-package-prefixed tags for every new independent train, starting
  with `identus-did-v0.1.0-rc.1`; retain the first train's historical tag.
- Add a deterministic, credential-free DID candidate assembler/checker that
  stages exact package metadata, order and internal requirements outside Git,
  packages twice, inspects bounded archives, verifies a local patched closure,
  and emits checksums/file lists/tool identities/limitations.
- Preserve the existing crypto candidate and publication checks unchanged.

## Capability

### Added capability

- `release-candidate-trains`: closed identity, manifest and non-publishing
  evidence for multiple independently versioned monorepo trains.

## Non-goals

- No canonical DID package version or `publish` activation.
- No tag, GitHub release, crates.io, credential, workflow, environment,
  repository-setting, `main`, consumer, DID-method, chain or binding mutation.
- No replacement of the first train's advanced API/SBOM/release evidence.

## Delivery

Issue #382 owns this first M5 slice. It targets protected `develop` and remains
fully reversible before any later release activation.
