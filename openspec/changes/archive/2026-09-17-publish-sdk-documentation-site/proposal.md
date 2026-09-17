# Publish the SDK-Rust documentation site

## Why

The first isolated release train has reproducible package evidence but no
public, navigable explanation of the three crate responsibilities, dependency
direction, downstream ownership boundary, or remaining approval gates.
Engineers need one review surface before approving `0.1.0-rc.1`; repository
Markdown and generated API listings alone do not provide that product and
architecture view.

## What changes

- Add a repository-pinned mdBook site covering SDK direction, the three
  candidate crates, architecture, ownership boundaries, adoption, release
  readiness, governance, and limitations.
- Generate architecture SVGs deterministically from tracked Graphviz DOT
  sources without runtime CDN dependencies.
- Add a Nix package/check that builds the site and validates offline links.
- Add a least-privilege GitHub Pages workflow that publishes only protected
  `develop` output through the `github-pages` environment.
- Link the public handbook from the repository README and the M3 approval
  record.

## Capability

### Added capability

- `sdk-documentation-site`: reproducible, truthful, release-gated public SDK
  architecture documentation and deployment.

## Non-goals

No crate version, API, dependency, release, tag, crates.io upload, `main`
promotion, analytics, authentication, or downstream code mutation.

## Delivery

Issue #324 owns the site implementation and publication. The repository-local
change may merge under normal `develop` authority. User direction captured by
#324 separately authorizes enabling and publishing the GitHub Pages site after
the exact merged workflow succeeds; it does not authorize a crate release.

