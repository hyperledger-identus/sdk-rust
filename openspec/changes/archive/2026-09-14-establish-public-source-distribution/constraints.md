# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/255
Constraint blockers: none

## Existing entries affected

- `SDK-REPO-001`: `develop` remains the source of candidate commits, not a
  floating consumer reference or release branch.
- `SDK-REPO-002`: publication, release, and normal delivery from `main` remain
  prohibited.
- `SDK-COMPAT-002`: source consumers use the current Rust 1.98.1 floor.
- `SDK-COMPAT-003`: a published release-candidate compiler matrix remains
  deferred; this source channel does not activate one.
- `SDK-LIM-001`: `0.0.0` source artifacts retain no stable public API promise.
- `SDK-LIM-006`: NeoPRISM evidence proves the channel, while downstream
  adoption remains separately owned.

## Introduced or changed constraints

Before a registry candidate exists, a public Rust consumer may obtain the five
proven packages only from the public HTTPS repository at an exact full commit.
The consumer commits its Cargo lock, records features and target evidence, and
does not represent the source revision or workspace version as a release.

## Introduced or changed limitations

The channel is source-only, pre-release and Rust-consumer-facing. It provides
no crates.io checksum, SDK-owned binary/Nix package, SemVer compatibility,
support lifetime, registry availability or foreign-language distribution. Git
dependencies prevent a consuming crate from being published to crates.io until
it replaces them with registry dependencies.

## Consumer and product impact

NeoPRISM, Oxid, Midnight, Lace and other Rust consumers can evaluate exact SDK
commits without credentials. They retain their own feature selection, target
validation, compatibility facades and update decisions. No consumer repository
is changed by this SDK issue.

## Activation and rollback

The documentation and offline check activate after the issue #255 PR merges to
protected `develop`. Rollback removes the source-distribution capability and
restores the prior undocumented experimental state; existing exact downstream
pins remain reproducible but unsupported. Publication still requires separate
human release authority.

## Evidence

- NeoPRISM PR #324 and issue #321 provide the clean anonymous downstream
  canary.
- Repository tests validate package identities, manifest publication denial,
  exact example revision syntax, and absence of floating example references.
- The issue #255 PR exercises the newly protected `develop` ruleset.
