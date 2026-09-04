## Purpose

The SDK bootstrap inventory makes repository-local governance, release status
and package maturity explicit and executable. It prevents seed placeholders or
version metadata from becoming accidental capability or publication promises,
while keeping protected live settings and release authority with accountable
human maintainers.
## Requirements
### Requirement: One machine-readable bootstrap inventory defines local evidence

The repository SHALL contain one normative machine-readable inventory that
identifies the repository, active and reserved branch roles, unreleased and
non-publishable bootstrap state, required local governance records, immutable
canonical-policy references, protected follow-up issues and every explicit
Cargo workspace package exactly once. A workspace-root package and every
in-tree path dependency SHALL be explicit members so Cargo cannot silently
auto-enrol an uninventoried package. A human-readable inventory SHALL explain
the
implemented API families and SHALL NOT claim greater maturity than the machine
data.

#### Scenario: Contributor evaluates an SDK package

- **WHEN** a contributor or consumer reads a package inventory entry
- **THEN** it has one path, layer, maturity classification, public API status
  and owner issue without implying a release or namespace commitment

#### Scenario: Package coverage drifts

- **WHEN** an inventory entry is missing, duplicated, unknown or mapped to the
  wrong workspace path or layer
- **THEN** offline structural validation fails

#### Scenario: Cargo discovers an implicit in-tree member

- **WHEN** a manifest declares an in-tree path dependency outside the explicit
  workspace member set
- **THEN** offline structural validation fails before the package can bypass
  inventory and publication-denial checks

#### Scenario: Workspace manifest declares a root package

- **WHEN** the root manifest adds `[package]` beside `[workspace]`
- **THEN** that automatic Cargo member requires inventory coverage and explicit
  inheritance of publication denial

### Requirement: Bootstrap publication fails closed

The workspace SHALL declare `publish = false`, and every workspace package
SHALL explicitly inherit that value. Package existence, version `0.0.0` or an
implemented experimental API SHALL NOT authorize publication. Publication can
be enabled only by a later accepted package/release decision after issue #3
establishes approved namespace and protected ownership.

#### Scenario: A package stops inheriting publication denial

- **WHEN** a member omits or overrides the workspace `publish = false` value
- **THEN** structural validation fails before the bootstrap package can be
  published

#### Scenario: Implemented foundation is inventoried

- **WHEN** an implemented crate is listed as experimental
- **THEN** it remains non-publishable and no SemVer support promise is inferred

### Requirement: Placeholder packages are quarantined

Every package classified `placeholder` SHALL contain only crate documentation,
an `identus_core::Component` import and its public `COMPONENT` constant. Its
normal workspace dependency set SHALL be exactly
`identus-core.workspace = true`, with no package-level custom build script,
development, build, target, library, feature, binary, example, test or benchmark
surface. Its public API status SHALL be `none`.

#### Scenario: Placeholder gains speculative coupling

- **WHEN** a placeholder declares another dependency or feature before its
  component contract is accepted
- **THEN** structural validation fails

#### Scenario: Placeholder redirects its core dependency

- **WHEN** a placeholder replaces workspace inheritance with a registry,
  Git or alternate path declaration for `identus-core`
- **THEN** structural validation fails before Cargo can resolve the substitute

#### Scenario: Placeholder declares a custom build-script path

- **WHEN** a placeholder sets package-level `build` metadata, including a
  source path whose extension is not `.rs`
- **THEN** structural validation fails before Cargo can execute build-time code

#### Scenario: Placeholder declares a custom library target

- **WHEN** a placeholder sets `[lib]` metadata, including an extensionless
  source path omitted by the `.rs` file scan
- **THEN** structural validation fails before Cargo can expose that API

#### Scenario: Placeholder declares custom test or benchmark targets

- **WHEN** a placeholder sets `[[test]]` or `[[bench]]` metadata, including an
  extensionless source path omitted by the `.rs` file scan
- **THEN** structural validation fails before Cargo can compile or execute it

#### Scenario: Placeholder documentation contains a doctest

- **WHEN** crate or item documentation contains a fenced or indented executable
  code block, directly or nested in another Markdown container
- **THEN** structural validation fails before Cargo can compile or execute it

#### Scenario: Placeholder gains an apparent API

- **WHEN** a placeholder source adds another public item or source target
- **THEN** structural validation fails rather than presenting the package as a
  supported capability

### Requirement: Implemented API inventory remains honest

The human-readable inventory SHALL identify the current public API families,
feature surfaces and stabilization issue for every `implemented` package. It
SHALL identify `identus-conformance` as verification-only and every placeholder
as unsupported. It SHALL label all implemented surfaces experimental,
unreleased and subject to focused component contracts. The
`identus-credentials` entry SHALL identify its bounded envelope and staged
verification evidence while explicitly excluding trust, verifier execution,
metadata/schema, status bindings, concrete formats, protocols, and storage.

#### Scenario: Consumer inspects current foundations

- **WHEN** a consumer considers integrating core, derive, crypto, DID or the
  entropy adapter
- **THEN** the inventory identifies the real surface and owner issue while
  warning that no immutable release or compatibility commitment exists

#### Scenario: Placeholder name resembles a planned component

- **WHEN** a placeholder name overlaps a roadmap concept
- **THEN** the inventory states that the name and dependency graph remain
  unaccepted until a focused issue replaces the placeholder

#### Scenario: Consumer inspects current credential foundations

- **WHEN** a consumer considers integrating `identus-credentials`
- **THEN** the inventory identifies #71 and #73 as delivered experimental
  slices without implying the deferred capabilities are supported

### Requirement: Governance evidence is local and release authority stays protected

The inventory SHALL require the repository's license, maintainer inheritance,
governance, contribution, DCO, security, release, conduct, ownership and
repository-settings records. It SHALL identify human maintainer release
authority and SHALL record live GitHub activation as
`external-action-required` under #26, not as a locally satisfied control.

#### Scenario: Required governance record disappears

- **WHEN** a required local governance path is missing or empty
- **THEN** offline structural validation fails

#### Scenario: Local evidence exists while live controls remain incomplete

- **WHEN** repository-local records pass but protected GitHub activation has
  not been completed
- **THEN** IDR-001 remains `in_progress` and the inventory points to #26

### Requirement: Inventory drift validation is deterministic and offline

The repository SHALL provide a standard-library-only offline validator for the
inventory and SHALL run it from the structural factory contract. Focused
regressions SHALL cover every demonstrated fail-closed class without invoking
GitHub, Cargo, Nix, donors or consumers.

#### Scenario: Factory validates the canonical repository

- **WHEN** the factory structural contract runs
- **THEN** it invokes the inventory validator and fails if the canonical
  governance/package contract is invalid

#### Scenario: External services are unavailable

- **WHEN** the inventory validator runs without network or GitHub credentials
- **THEN** local validation produces the same result

### Requirement: Credential envelope activation is inventoried

The machine-readable and human-readable bootstrap inventories SHALL classify
`identus-credentials` as an implemented, experimental credential-semantics
package after its bounded format-neutral envelope contract is accepted. The
inventories SHALL identify issue #71 as the focused owner and SHALL continue to
describe unsupported credential capabilities as future work.

#### Scenario: Consumer inspects the credential package

- **WHEN** a consumer inspects the bootstrap inventory after issue #71
- **THEN** `identus-credentials` is accepted as implemented and experimental,
  is not listed as a placeholder, and does not imply support for credential
  metadata, verifier execution, status bindings, concrete formats, protocols,
  or storage

### Requirement: Credential verification activation is inventoried

The human-readable bootstrap inventory SHALL identify the canonical staged
verification evidence added under issue #73 as part of the experimental
`identus-credentials` surface. It SHALL continue to exclude trust decisions,
verifier execution, metadata/schema, status bindings, concrete formats,
protocols, and storage.

#### Scenario: Consumer inspects verification support

- **WHEN** a consumer inspects `identus-credentials` in the bootstrap inventory
- **THEN** the entry identifies #73 without implying that the SDK executes any
  verification stage or makes a relying-party trust decision
