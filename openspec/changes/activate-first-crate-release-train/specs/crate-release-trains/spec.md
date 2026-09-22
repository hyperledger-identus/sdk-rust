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
SHALL prevent self-review, disable administrator bypass, and require an
independent Identus maintainer.

#### Scenario: Tag or approval identity differs

- **WHEN** the tag is unsigned, resolves to another SHA, is not contained in
  protected `develop`, or lacks protected-environment approval
- **THEN** no registry credential is acquired and no package is uploaded

### Requirement: Credential-bearing execution rebinds immutable identity

After the protected-environment wait, the publish job SHALL independently
verify that its checkout, signed tag, expected full SHA, and protected
`develop` ancestry still agree before acquiring or using either registry
credential.

#### Scenario: Tag moves during the approval wait

- **WHEN** the publish job checkout no longer resolves to the approved SHA
- **THEN** the job fails before authentication and no package is uploaded

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

### Requirement: Failed release attempts are recoverable without identity drift

The verify artifact identity SHALL remain stable across attempts of one
workflow run. A failed-job-only rerun SHALL reuse only that run's verified
candidate. GitHub release creation SHALL be idempotent only for an existing
release bound to the exact expected tag; conflicting release identity SHALL
fail closed. Verify and publication receipts SHALL be retained on failure when
their files exist.

#### Scenario: Publish succeeds but release finalization is interrupted

- **WHEN** the publish job is rerun without rerunning the successful verify job
- **THEN** it consumes the same run-scoped candidate, verifies any existing
  package checksums and release tag, and completes without creating a second
  release identity

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
