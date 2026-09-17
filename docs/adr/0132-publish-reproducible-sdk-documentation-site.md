# ADR 0132: publish a reproducible SDK documentation site

- **Status:** Accepted under sponsor direction
- **Date:** 2026-09-17
- **Issue:** [#324](https://github.com/hyperledger-identus/sdk-rust/issues/324)
- **Milestone:** [M3 — Crypto SDK 0.1.0-rc.1 approval](https://github.com/hyperledger-identus/sdk-rust/milestone/2)
- **Review no later than:** before any release-candidate publication

## Context

The repository contains detailed architecture, release, factory, and crate
evidence, but no public navigation or review surface. The first isolated
three-crate candidate needs an engineer-readable explanation of responsibilities,
dependency direction, boundaries, limitations, and remaining gates before a
human publication decision.

Documentation publication is an external repository setting and public
artifact. It therefore requires explicit sponsor direction even though the
repository implementation is routine. Issue #324 records that direction while
keeping crate release authority separate.

## Decision

1. Publish a static mdBook handbook from protected `develop` at the GitHub
   Pages project URL.
2. Resolve mdBook, Graphviz, and lychee through the locked Nixpkgs input and
   expose the same derivation as `packages.docs-site` and `checks.docs-site`.
3. Store diagrams as reviewed DOT sources and render static SVG artifacts
   during the build. Do not load mutable diagram or analytics code in browsers.
4. Make the fast integration line build the site so missing pages, broken
   internal links, or invalid diagrams block merge.
5. Deploy one validated Pages artifact through a separate least-privilege job
   with `pages: write`, `id-token: write`, and the `github-pages` environment.
   Pin every action to an exact commit.
6. Document only `identus-derive`, `identus-core`, and `identus-crypto` as the
   proposed release train. Other workspace packages remain source-only and do
   not inherit a release promise.
7. Treat the public site and deployment receipt as M3 evidence, not as a tag,
   registry release, MSRV commitment, support promise, or certification.

## Consequences

Engineers gain one searchable, reviewable architecture and release-readiness
surface. The site remains buildable offline after the pinned tools are present,
and its diagrams cannot drift independently from textual sources.

The fast lane performs a small additional Nix build. Documentation authors
must update tracked sources rather than a hosted CMS. API details remain in
Rustdoc/source until real artifacts can be published to docs.rs.

## Alternatives rejected

- **Raw Markdown only:** insufficient navigation and no deployment receipt.
- **Docusaurus:** unnecessary Node dependency/runtime surface.
- **MkDocs Material:** capable but less aligned than the Rust-native tool for
  this bounded handbook.
- **Client-rendered Mermaid/CDN:** mutable executable dependency and weaker
  offline/privacy evidence.
- **Generated Pages branch:** generated history and a second mutable source.
- **Rustdoc-only site:** explains items, not product boundaries and gates.

## Verification and rollback

Nix builds the site, Graphviz renders every diagram, mdBook rejects missing
chapters, and lychee checks generated local links offline. Actionlint and
repository workflow policy inspect the exact-action deployment. After merge,
the Pages run and public URL must bind the protected source revision.

Rollback disables Pages and reverts the site workflow, Nix module, build
script, content, README link, capability spec, and this ADR. No crate,
consumer, tag, registry object, or persisted application data changes.
