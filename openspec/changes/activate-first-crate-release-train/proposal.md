# Activate the first crates.io release train

## Why

The three-crate `0.1.0-rc.1` candidate is reproducible, its compiler and target
contract is selected, public documentation is deployed, an external consumer
canary is green, and the first natural weekly slow run passed on protected
`develop`. The remaining M3 work is to convert that reviewed candidate into a
repeatable, least-privilege crates.io release without making unrelated
workspace packages publishable.

The first upload has a special constraint: crates.io trusted publishing can be
configured only after a crate namespace exists. The release train therefore
needs one protected bootstrap-token path and a distinct OIDC path for every
later release.

## What changes

- Activate canonical `0.1.0-rc.1` metadata for exactly `identus-derive`,
  `identus-core`, and `identus-crypto`; retain `0.0.0` and `publish = false`
  defaults for every other workspace member.
- Preserve the deterministic isolated archive builder and make its receipt
  explicitly eligible for a separately authorized release operation.
- Add a fail-closed publisher and GitHub Actions workflow that require an exact
  protected-`develop` revision, signed immutable tag, protected `crates-io`
  environment, two-person review, and dependency-order publication.
- Support a one-time `CARGO_PUBLISH` bootstrap credential and a subsequent
  short-lived crates.io OIDC credential without allowing fallback between the
  two modes.
- Record release checksums, package ownership/recovery duties, migration and
  rollback guidance, and the transition to trusted publishing.

## Capabilities

### Added capabilities

- `crate-release-trains`: define package activation, immutable release identity,
  protected publication, authentication transition, and release receipts.

### Modified capabilities

- `unpublished-crypto-candidate`: promote the reviewed isolated candidate to a
  release-eligible candidate while keeping candidate preparation unable to
  publish.

## Non-goals

No unrelated workspace crate, final `0.1.0`, `main` promotion, downstream
dependency update, chain-specific behavior, FFI package, runtime certification,
or automatic yank enters this change. crates.io ownership and trusted-publisher
configuration remain explicit human operations after the namespace-creating
upload.

## Delivery

Issue #326 owns implementation and the exact release packet; issue #3 owns the
namespace, owner, token-revocation, and trusted-publisher receipts. The release
PR may merge through protected `develop` after normal CI and independent
maintainer review. Publication still requires the protected environment gate
and exact signed-tag approval.
