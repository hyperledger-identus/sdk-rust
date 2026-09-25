# ADR 0153: use primary-package tags for independent release trains

- **Status:** Accepted under standing routine authority
- **Date:** 2026-09-25
- **Issue:** [#382](https://github.com/hyperledger-identus/sdk-rust/issues/382)
- **Milestone:** M5 — DID SDK consumable release candidate
- **Supersedes:** the unqualified-tag convention for trains created after the
  first immutable release; it does not change ADR 0134 or `v0.1.0-rc.1`
- **Review no later than:** before activating the DID publication workflow

## Context

The first SDK-Rust release contains `identus-derive`, `identus-core`, and
`identus-crypto` at one immutable tag, `v0.1.0-rc.1`. M5 prepares a separate
two-package DID train at the same prerelease version. Git and GitHub cannot
assign the existing tag to a second artifact, and moving or repurposing it
would break the signed first-train receipt.

SDK-Rust is a monorepo with independent release trains rather than one
workspace-wide version promise. A tag must therefore identify both the train
and version while remaining recognizable to Cargo users and maintainers.

## Decision

1. Preserve `v0.1.0-rc.1` as the immutable historical tag for the first crypto
   foundation train.
2. Every new independent train uses
   `<primary-exact-cargo-package>-v<version>`.
3. The M5 candidate tag is `identus-did-v0.1.0-rc.1`; it identifies the ordered
   `identus-did` then `identus-did-resolver-http` train.
4. One closed repository train index binds stable train ID, lifecycle, primary
   package, descriptor and tag. IDs, tags, descriptors and owned packages are
   unique.
5. A `candidate-only` lifecycle may render release-shaped manifests and local
   archives but cannot change canonical package versions/publish flags or
   configure credentials, workflows, releases or registry state.
6. Publication activation requires a separate ADR/issue, exact candidate
   receipt, complete slow evidence, independent approval and completion of the
   relevant trusted-publishing administration.
7. If packages later need independent versions, each becomes its own train and
   uses its own exact Cargo package tag; an existing tag is never moved.

## Consequences

Repeated versions across independent trains are unambiguous, the exact public
crate brand is visible in the tag, and the first release remains verifiable.
One train can still contain tightly ordered packages whose dependency and
approval identity is shared.

The repository carries a deliberate historical exception for the first tag.
Consumers must read the train receipt rather than assume every workspace crate
shares a version or release date.

## Alternatives rejected

- **Reuse or move `v0.1.0-rc.1`:** destroys immutable first-release identity.
- **`did-v0.1.0-rc.1`:** omits the exact Cargo brand and can collide with
  future naming.
- **One tag per package now:** fragments an intentionally atomic dependency
  train without independent package versioning.
- **Workspace-wide versioning:** imposes churn and compatibility claims on
  unrelated experimental crates.

## Verification and rollback

The train checker rejects duplicate or non-conforming identities and confirms
that the published first descriptor still binds its original tag. Before any
remote release activation, rollback removes the additive DID train record,
descriptor and candidate tooling. After a future tag is created, it is never
moved or reused; corrections receive a new version.
