# release-candidate-trains Specification

## Purpose
TBD - created by archiving change define-reusable-release-candidate-manifest. Update Purpose after archive.
## Requirements
### Requirement: Independent train identities are closed and collision-free

The SDK SHALL maintain one closed ordered train index. Every train SHALL have a
unique stable identifier, lifecycle, descriptor, primary exact Cargo package,
package ownership and immutable tag. Every new independent train after the
first historical release SHALL use tag `<primary-package>-v<version>` and SHALL
NOT reuse or move an existing tag.

#### Scenario: Second train shares the first version

- **WHEN** the DID train selects version `0.1.0-rc.1`
- **THEN** its identity is `identus-did-v0.1.0-rc.1` and the existing
  `v0.1.0-rc.1` crypto release remains unchanged

#### Scenario: Train identity collides

- **WHEN** IDs, descriptors, tags or package ownership are duplicated or a new
  tag omits the primary exact package prefix
- **THEN** validation fails before candidate assembly

### Requirement: Candidate staging does not activate publication

A candidate-only descriptor SHALL select exact ordered package names, paths,
version, metadata, dependency classes, profiles and resource bounds. Candidate
assembly MAY render release-shaped staged manifests, but canonical package
manifests SHALL retain workspace version `0.0.0` and `publish = false` until a
separate release decision.

#### Scenario: DID candidate is assembled

- **WHEN** the valid candidate-only DID descriptor is selected
- **THEN** exactly `identus-did` then `identus-did-resolver-http` enter isolated
  staging and no canonical publication state changes

#### Scenario: Canonical DID package is activated early

- **WHEN** either selected package overrides the workspace version or publish
  denial before release authorization
- **THEN** the candidate checker fails closed

### Requirement: Candidate archives are deterministic and bounded

The candidate assembler SHALL require an exact current clean Git revision,
build twice in separate scratch outside every Git worktree, compare archive
bytes, reject unsafe or unbounded archive members, validate normalized metadata
and exact dependency requirements, and atomically expose completed evidence.

#### Scenario: Two assemblies agree

- **WHEN** both isolated assemblies complete from one exact revision
- **THEN** every corresponding archive is byte-identical and its SHA-256,
  byte size and bounded file list enter the receipt

#### Scenario: Archive or staging input is unsafe

- **WHEN** a link, special file, path traversal, duplicate member, unknown file,
  oversize input, retained Git/path dependency or non-deterministic archive is
  observed
- **THEN** assembly fails without exposing a completed output

### Requirement: Candidate closure is verified without registry claims

The assembler SHALL extract the exact archives and verify their ordered local
closure through explicit crates.io patches, including declared feature
profiles. The receipt SHALL state that this is not registry resolution,
publication, certification or a stable support promise.

#### Scenario: Local candidate closure passes

- **WHEN** archive manifests carry exact candidate/internal and published
  foundation requirements and every declared profile builds/tests
- **THEN** the receipt reports local patched-closure evidence and no registry
  availability claim

### Requirement: Candidate tooling cannot mutate release state

Candidate tooling SHALL accept no credential and SHALL contain no upload,
publish, tag, release, repository-setting, protected-environment, branch or
consumer mutation operation.

#### Scenario: Candidate succeeds

- **WHEN** deterministic package and closure evidence completes
- **THEN** only local review artifacts exist and every remote/release state
  remains unchanged

### Requirement: First candidate establishes an explicit API origin

The candidate descriptor SHALL bind one committed public-API baseline per
ordered package and exact evidence-tool versions. An initial candidate SHALL
record SemVer comparison as `not-applicable` and SHALL NOT compare against the
unpublished workspace identity or itself.

#### Scenario: First DID candidate is qualified

- **WHEN** staged all-feature rustdoc JSON is rendered for both packages
- **THEN** exact public-API text matches committed baselines that become the
  comparison origin for later DID candidates

#### Scenario: A meaningless compatibility check is proposed

- **WHEN** the baseline is the same candidate or workspace `0.0.0`
- **THEN** qualification fails rather than reporting a misleading green check

### Requirement: Candidate supply-chain evidence is bounded and identified

The assembler SHALL emit exactly one validated CycloneDX JSON document per
candidate package. Each SHALL bind exact name/version/license, contain no Git
dependency source, remain within configured resource bounds and enter the
atomic receipt by SHA-256 and byte size.

#### Scenario: Evidence matches the candidate

- **WHEN** both SBOMs identify the ordered `0.1.0-rc.1` packages and pass
  structural/dependency/resource validation
- **THEN** the atomic candidate receipt includes both immutable artifact
  identities and the exact generating tools

#### Scenario: Supply-chain evidence drifts

- **WHEN** an SBOM is missing, duplicated, oversized, malformed, names another
  component/version/license, or contains a Git source
- **THEN** qualification fails without exposing completed output

### Requirement: Evidence qualification preserves candidate archives

API and supply-chain evidence SHALL NOT alter staged package inputs or archive
assembly. Normal qualification SHALL reproduce the candidate archive SHA-256
values accepted by the preceding train contract.

#### Scenario: Evidence tools are added

- **WHEN** the qualified candidate is assembled from the same source
- **THEN** both package archive hashes equal the retained #382 values

### Requirement: Local supply-chain evidence is not release attestation

The receipt SHALL distinguish repository license/advisory policy, local SBOM,
API origin, platform evidence, signed provenance and registry availability.
Only the first three are in scope for this slice.

#### Scenario: Qualification succeeds

- **WHEN** local API/SBOM evidence passes
- **THEN** no stable compatibility, vulnerability-free, platform, signed
  provenance, publication or certification claim is made

### Requirement: A candidate train has package-specific compiler and target evidence

Before final approval, a candidate train SHALL declare a closed package,
profile, compiler, native-host, portable-target, operation, tier, and limitation
matrix. Execution SHALL use the same staged release-shaped sources and exact
internal requirements as archive evidence, not canonical unpublished manifest
identity. Candidate policy MAY narrow but SHALL NOT broaden the effective
global support policy.

Native lane receipts SHALL bind exact clean source SHA, staged version,
generated lock identity, detected host, exact Rust/Cargo version, compiler
class, commands, entries, outcomes, and limitations. One aggregate receipt
SHALL reject missing/duplicate lanes, source/lock drift, failed supported
entries, extra packages/targets, and an unsupported package represented as
portable. Receipts SHALL be closed, bounded, atomic, credential-free, and
contain no raw command output or environment dump.

#### Scenario: Staged DID candidate qualifies

- **WHEN** Linux x86_64 and macOS ARM64 each produce primary and MSRV lane
  receipts for the DID candidate
- **THEN** the aggregate receipt proves both packages on both hosts and proves
  only `identus-did` for WASM, Android ARM64, and iOS ARM64 compilation

#### Scenario: Canonical manifest substitutes for candidate source

- **WHEN** a lane reports `0.0.0`, path-only internal identity, a dirty source,
  or another revision
- **THEN** qualification fails even if the canonical workspace gates are green

#### Scenario: HTTP adapter is overclaimed

- **WHEN** a receipt reports `identus-did-resolver-http` as compile-checked or
  supported for WASM, Android ARM64, or iOS ARM64
- **THEN** aggregation fails rather than inferring support from Cargo success

### Requirement: Candidate lanes consume one reviewed staged dependency lock

A release-candidate train SHALL bind one bounded repository-owned lockfile and
exact digest to its staged manifest identity before archive or compiler/target
qualification. Every independent lane SHALL verify and consume those exact
bytes with locked Cargo operations and SHALL NOT generate, update, or select a
different dependency resolution. An intentional manifest or dependency change
SHALL refresh the staged lock through a separately reviewable diff.

#### Scenario: Four DID lanes qualify one candidate

- **WHEN** Linux/macOS primary/MSRV lanes render the same candidate manifests
- **THEN** every lane copies the descriptor-bound lock and reports its exact
  digest without consulting registry state for a newer resolution

#### Scenario: Lock bytes or manifest identity drift

- **WHEN** the lock is missing, modified, malformed, stale for the staged
  manifests, or differs from the descriptor digest
- **THEN** candidate preparation fails before a lane can create passing evidence

#### Scenario: A compatible dependency is published during the matrix

- **WHEN** registry state changes after the first lane starts
- **THEN** later lanes retain the reviewed lock versions and the aggregate
  result is determined by the repository candidate rather than execution timing

### Requirement: Candidate lock attestation and refresh are closed

A release-candidate train SHALL attest the digest computed from lock bytes
installed into each staged workspace directly against descriptor-owned source
identity. Every Cargo operation that resolves or inspects staged workspace
sources SHALL use locked resolution. Ordinary candidate and matrix evidence
SHALL reject dependency lock generation after command construction. A distinct
package-extraction closure MAY generate one closure-local lock, and an explicit
review-only refresh mode MAY generate one proposed staged lock outside the
repository. Refresh SHALL run under the pinned primary compiler, report exact
dependency-cone drift, and SHALL NOT edit tracked source identity.

#### Scenario: Four lane receipts agree on a wrong digest

- **WHEN** all submitted lane receipts contain the same validly shaped digest
  that differs from the candidate descriptor
- **THEN** aggregation fails before reporting passing matrix evidence

#### Scenario: Archive receipt is assembled

- **WHEN** staged lock bytes are copied and verified for the archive build
- **THEN** the receipt records the digest returned from those installed bytes
  rather than independently echoing descriptor text

#### Scenario: An alternate helper constructs lock generation

- **WHEN** an ordinary candidate or matrix path constructs an equivalent Cargo
  lock-generation command through a variable or alternate execution helper
- **THEN** runtime command policy and offline structure policy reject it

#### Scenario: A reviewer proposes a lock refresh

- **WHEN** the explicit refresh mode runs under pinned Rust 1.98.1 from a clean
  exact repository revision into a new external output directory
- **THEN** it emits proposed lock bytes and a deterministic closed dependency-
  cone drift report without changing the repository lock or descriptor

#### Scenario: The extracted package closure is verified

- **WHEN** packaged DID crates are patched into the distinct verification
  workspace
- **THEN** that workspace may generate exactly one local lock and all following
  checks and tests consume it with locked resolution

### Requirement: Staged lock refreshes are closed and reviewable

The SDK SHALL refresh a candidate staged lock only through an explicit issue,
review-only resolver output, complete added/removed package report, exact lock
digest update, and rerun of deterministic archive, closure, API, supply-chain,
compiler, and target evidence. Direct dependency requirements, candidate
identity, and publication state SHALL remain unchanged unless separately
specified.

#### Scenario: a frozen lock becomes stale

- **WHEN** clean `--locked` candidate preparation rejects the reviewed lock
- **THEN** the SDK generates a proposal outside the repository and reviews the
  complete dependency drift before changing tracked lock bytes

#### Scenario: undeclared drift appears

- **WHEN** the adopted lock differs from the closed proposal or contains an
  undeclared package, version, source, or checksum change
- **THEN** qualification fails before the lock can become release evidence

### Requirement: Prepublication API-origin rebaselines are exact and additive

Before the first candidate is published, the SDK SHALL permit rebaselining its
committed API origin only through an issue-linked exact-source receipt that binds old and
new generated API hashes, proves an empty removal set and a closed expected
addition set, retains unchanged package/version/feature/dependency/publication
state, and passes normal candidate qualification without initialization mode.
The rebaseline SHALL remain a first-candidate origin rather than a stable
SemVer or released-predecessor claim.

#### Scenario: an already-delivered additive API is missing from the origin

- **WHEN** exact generated API comparison contains no removal or changed item
  and additions equal the reviewed allowlist
- **THEN** the SDK records the immutable classification and may replace the
  stale unpublished origin before rerunning normal qualification

#### Scenario: the diff is breaking, ambiguous, or broader than reviewed

- **WHEN** any API item is removed or changed, an unexpected addition appears,
  the source revision differs, or publication has already occurred
- **THEN** the rebaseline fails and requires a separate compatibility decision

### Requirement: A candidate train has one closed final readiness packet

The SDK SHALL bind one exact protected revision to its train descriptor, staged
lock, deterministic archives, API origins, SBOMs, documentation, package-
specific compiler/target matrix, security-policy result, complete required
slow lanes, downstream compatibility evidence where available, explicit
limitations, and independent approval before a candidate-only train can be
proposed for publication activation. Every receipt SHALL use immutable links
and SHALL distinguish passing, pending, failed, and not-applicable evidence.

#### Scenario: a required slow lane fails

- **WHEN** any required lane fails, is cancelled, is stale, runs from another
  branch or revision, or only a rerun is available
- **THEN** final readiness remains pending even if an aggregate artifact exists

#### Scenario: the exact candidate is approved

- **WHEN** every closed evidence class passes for one protected revision and an
  independent maintainer approves that exact receipt
- **THEN** the candidate may proceed to a separate publication-activation
  decision without implying that publication already occurred

#### Scenario: downstream interoperability is available

- **WHEN** a consumer has immutable source-pinned and network parity evidence
- **THEN** the packet links it as downstream compatibility evidence while
  preserving consumer ownership and its stated scope and limitations

### Requirement: Published API origins remain immutable compatibility inputs

The SDK SHALL preserve a published train's generated public-API origin and
compare later source to it without baseline initialization. Qualification MAY
accept additive current API, but SHALL fail when any published API occurrence
is removed or changed and SHALL emit bounded deterministic diff evidence.

#### Scenario: Current development is additive

- **WHEN** every published API occurrence remains and current source adds items
- **THEN** qualification retains the published origin and records the additions

#### Scenario: Published API history would be rewritten

- **WHEN** tooling requests initialization or baseline replacement for a train
  already recorded as published
- **THEN** qualification fails before modifying the origin
