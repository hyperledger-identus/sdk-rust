# sdk-documentation-site

## ADDED Requirements

### Requirement: The release handbook is reproducible and self-contained

The repository SHALL build the complete SDK handbook with tools resolved by
the locked Nix input. Required architecture diagrams SHALL be generated from
tracked textual sources, and the deployed site SHALL NOT require mutable CDN
scripts, analytics, authentication, cookies, or repository credentials.

#### Scenario: Maintainer builds the site from a clean checkout

- **WHEN** the documented Nix site target is built from an exact repository
  revision
- **THEN** it produces the complete static site, generates every architecture
  diagram, and passes offline internal-link validation

#### Scenario: A page, diagram source, or internal target is invalid

- **WHEN** the book references a missing chapter/asset or a diagram cannot be
  rendered
- **THEN** the site check fails before the change may merge

### Requirement: Candidate crate responsibilities are truthful and bounded

The handbook SHALL explain the purpose, dependency direction, public surface,
feature policy, security boundary, and non-goals of `identus-derive`,
`identus-core`, and `identus-crypto`. It SHALL identify them as an experimental
candidate train and SHALL NOT imply release or support for unrelated workspace
packages.

#### Scenario: Engineer reviews the three-crate architecture

- **WHEN** an engineer opens the crate and architecture sections
- **THEN** the dependency direction and ownership boundaries are visible in
  prose and static diagrams, with links to current source and governing ADRs

#### Scenario: Candidate approval is incomplete

- **WHEN** any namespace, compiler, consumer, slow-evidence, security,
  provenance, or human approval gate remains open
- **THEN** the readiness page presents it as an explicit blocker rather than a
  completed release claim

### Requirement: Pages deployment is exact-revision and least-privilege

The documentation workflow SHALL build from protected `develop`, upload one
validated Pages artifact, and deploy through the `github-pages` environment
using only `contents: read`, `pages: write`, and `id-token: write`. Third-party
actions SHALL be pinned to exact commits.

#### Scenario: Protected develop documentation changes

- **WHEN** an eligible documentation revision merges to `develop`
- **THEN** the workflow deploys the Nix-built artifact and records the exact
  source SHA, workflow run, environment, and public URL

#### Scenario: Documentation is published

- **WHEN** the Pages deployment succeeds
- **THEN** no crate, tag, GitHub release, `main` content, consumer repository,
  or registry object is created or changed by that workflow

