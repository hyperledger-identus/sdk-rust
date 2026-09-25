# sdk-documentation-site Specification

## Purpose
TBD - created by archiving change publish-sdk-documentation-site. Update Purpose after archive.
## Requirements
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

### Requirement: The DID candidate train has a truthful review surface

The handbook SHALL present `identus-did` and
`identus-did-resolver-http` as one independent candidate-only train. It SHALL
explain each package's responsibility, dependency direction, feature policy,
source-evaluation path, evidence status and exclusions without implying
registry availability, stable compatibility, portable support, signed
provenance, vulnerability freedom, chain behavior or publication.

#### Scenario: Engineer reviews the DID train

- **WHEN** an engineer follows the handbook navigation to the DID candidate
- **THEN** prose and a static diagram distinguish the generic DID domain/ports
  from the optional host-side HTTP adapter and link the governing evidence

#### Scenario: Consumer evaluates the candidate before publication

- **WHEN** a consumer follows the source-evaluation example
- **THEN** it uses both package names from one exact protected 40-hex revision
  without a branch, pull-request ref, nonexistent tag or `0.0.0` selector

#### Scenario: M5 promotion remains incomplete

- **WHEN** compiler/target, complete slow, approval, registry or publication
  evidence is not complete
- **THEN** the readiness and limitations pages show that state explicitly
  rather than inheriting a claim from the crypto train

