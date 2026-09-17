# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-17
Source retrieval date: 2026-09-17
Research blockers: none

## Problem and existing implementation

The repository has extensive Markdown, Cargo Rustdoc, an unpublished
three-package candidate descriptor, and architecture evidence, but no site
generator, Pages configuration, or public project URL. The GitHub Pages API
returns `404`, confirming that no site is configured. The README links raw
files and the broader Identus site, so release reviewers must reconstruct crate
roles and gate status from many sources.

## Normative sources

- Issue #324 and M3 issue #326 define the requested review outcome.
- `RELEASING.md`, ADR 0113, the candidate descriptor, manifests, architecture
  boundary documents, and current constraints define factual release content.
- mdBook's official configuration and build documentation defines book
  structure, `site-url`, deterministic missing-page failure, and static HTML
  output.
- GitHub's official custom Pages workflow documentation requires a Pages
  artifact, the `github-pages` environment, and `pages: write` plus
  `id-token: write` for deployment.
- Graphviz `dot` is the established deterministic source-to-SVG renderer;
  lychee supports offline local-link validation.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Raw repository Markdown only | `not-adopt` | No stable navigation, search, deployment receipt, or coherent approval surface. | Never for the release handbook. |
| Docusaurus or another Node SPA | `not-adopt` | Adds a large dependency/build/runtime surface that is unnecessary for a Rust SDK handbook. | Interactive application documentation becomes required. |
| MkDocs Material | `not-adopt` | Capable, but adds Python/plugin theming where the Rust-native tool is sufficient. | mdBook cannot meet accessibility/navigation needs. |
| mdBook from locked Nixpkgs | `adopt` | Rust-native, static, searchable, and reproducible within the current Nix supply chain. | Security or accessibility evidence rejects the selected version. |
| Mermaid loaded from a CDN | `not-adopt` | Mutable client-side code weakens reproducibility, privacy, and offline review. | Never for required diagrams. |
| Tracked DOT rendered to SVG during the build | `adopt` | Human-reviewable sources, static output, no browser execution, and deterministic generation. | A repository-wide diagram standard supersedes it. |
| Publish generated files to a branch | `not-adopt` | Creates mutable generated history and conflicts with protected source ownership. | GitHub Actions Pages becomes unavailable. |
| GitHub Pages artifact deployment from `develop` | `adopt` | Exact workflow/run evidence, OIDC deployment, no generated branch, and least privilege. | Hosting ownership or policy changes. |

## Compatibility and dependency evidence

The site does not enter any Cargo package, feature graph, runtime, wire format,
or consumer build. mdBook, Graphviz, and lychee are build-only tools resolved
by the existing locked Nix input. The new Nix output is independent of Rust
package artifacts. Rollback removes the site module/workflow/content and may
disable Pages without changing crates or consumers.

## Security, privacy and maintenance evidence

The site is static, contains no analytics, cookies, forms, authentication,
secrets, mutable CDN assets, or custom executable JavaScript. The Pages deploy
job receives only `contents: read`, `pages: write`, and `id-token: write`, runs
in the `github-pages` environment, and consumes the build job's artifact. All
third-party actions are pinned to exact commits.

## Rejected or deferred candidates

API hosting on docs.rs remains a post-publication surface. Building Rustdoc
into this handbook is deferred to avoid duplicating the API contract or
expanding the first site. Custom domains, version switching, analytics,
translations, and full-workspace crate promises are deferred.

## Open questions and blockers

None for repository implementation. Pages activation is a protected setting,
but the explicit user request and issue #324 direct that publication after the
merged workflow exists. Crate publication remains blocked and separate.

## Evidence sources

- https://rust-lang.github.io/mdBook/
- https://rust-lang.github.io/mdBook/format/configuration/renderers.html
- https://rust-lang.github.io/mdBook/continuous-integration.html
- https://docs.github.com/en/pages/getting-started-with-github-pages/using-custom-workflows-with-github-pages
- https://graphviz.org/documentation/
- https://github.com/lycheeverse/lychee
- GitHub Pages API for `hyperledger-identus/sdk-rust` observed `404` on
  2026-09-17.

## Evidence commands

Before implementation: GitHub issue/workflow/Pages inspection, manifest and
release-policy review, official documentation review, and locked tool
evaluation. After implementation: Nix site build, offline link validation,
workflow lint, repository factory checks, exact-head CI/review, Pages deploy,
and public URL inspection.
