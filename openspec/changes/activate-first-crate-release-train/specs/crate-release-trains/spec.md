# crate-release-trains

## ADDED Requirements

### Requirement: Release activation is explicitly scoped

The SDK SHALL activate canonical version `0.1.0-rc.1`, crates.io publication
permission, and complete package metadata for exactly `identus-derive`,
`identus-core`, and `identus-crypto`. Every other workspace package SHALL
retain the workspace `0.0.0`, `publish = false` default. Internal train
dependencies SHALL use local paths plus exact registry requirement
`=0.1.0-rc.1`.

#### Scenario: An unrelated crate is inspected

- **WHEN** release policy enumerates canonical workspace manifests
- **THEN** only the selected three are publishable and every other member is
  still explicitly denied by the inherited workspace policy

### Requirement: Publication binds reviewed immutable identity

The release workflow SHALL accept only the exact approved full source SHA and
signed release tag, SHALL verify they resolve to the same commit contained in
protected `develop`, and SHALL rebuild deterministic candidate evidence before
requesting access to the protected `crates-io` environment. The environment
SHALL prevent self-review and require an independent Identus maintainer.

#### Scenario: Tag or approval identity differs

- **WHEN** the tag is unsigned, resolves to another SHA, is not contained in
  protected `develop`, or lacks protected-environment approval
- **THEN** no registry credential is acquired and no package is uploaded

### Requirement: Bootstrap and trusted credentials are separated

The first namespace-creating publication SHALL use only protected environment
secret `CARGO_PUBLISH`. Later releases SHALL use only a short-lived token from
the official crates.io trusted-publishing OIDC action, bound to this repository,
workflow, and environment. The two modes SHALL be explicit and SHALL NOT fall
back to each other.

#### Scenario: Selected credential is unavailable

- **WHEN** the bootstrap secret is absent or the OIDC trust is not configured
- **THEN** the selected mode fails closed without trying the other credential

### Requirement: The release train is ordered, bounded, and receipted

The publisher SHALL revalidate candidate scope and checksums and SHALL publish
only `identus-derive`, then `identus-core`, then `identus-crypto`. It SHALL stop
on the first failure, never overwrite an existing version, and emit a receipt
with exact source, tag, workflow run, authentication class, archive and registry
checksums, version URLs, and result for every package.

#### Scenario: A dependency publication fails

- **WHEN** derive or core does not become available from crates.io with the
  expected checksum
- **THEN** the publisher stops before uploading any dependent package and
  preserves partial-train evidence

### Requirement: First publication transitions immediately to trusted publishing

After successful namespace creation, maintainers SHALL verify organization
ownership and recovery/yank duties, configure the workflow/environment as a
trusted publisher for all three crates, revoke the bootstrap token, and attach
the resulting receipt before the release milestone closes.

#### Scenario: Token remains usable after bootstrap

- **WHEN** publication succeeds but trusted publishers or token revocation are
  not evidenced
- **THEN** issue #3 and the release milestone remain open and no later train is
  authorized
