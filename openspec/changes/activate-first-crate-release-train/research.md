# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-09-22
Source retrieval date: 2026-09-22
Research blockers: none

## Problem and existing implementation

The current implementation under ADR 0113 and the `crypto-candidate` Nix
application already produce two
byte-identical archives for `identus-derive`, `identus-core`, and
`identus-crypto`, validate normalized metadata and exact internal requirements,
test the extracted closure, and emit public-API, SBOM, checksum, tool, feature,
limitation, and source receipts. ADR 0133 adds independent Rust 1.89.0 MSRV and
Rust 1.98.1 primary evidence. The first natural weekly slow run succeeded on
protected default branch `develop` at revision
`19d0362038c3f2af6898624ea04347e3cd4648f7`.

The repository deliberately has no publishing workflow. Its selected package
manifests still inherit workspace version `0.0.0` and `publish = false`, and the
candidate descriptor says publication is prohibited. The final release slice
must activate only the reviewed closure without making the rest of the
monorepo releasable.

## Normative sources

- Issues #3 and #326 define the first-publication outcome and human authority;
  `RELEASING.md` requires a release manager, independent maintainer, signed
  tag, protected environment, exact candidate, and immediate OIDC transition.
- Cargo's official publishing guide recommends inspecting package contents and
  running `cargo publish --dry-run`/`cargo package` before irreversible upload:
  https://doc.rust-lang.org/cargo/reference/publishing.html
- Cargo supports a local path plus registry version for workspace packages;
  the path is used locally and the version is retained for publication:
  https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#multiple-locations
- Cargo package metadata and `publish` restrictions are defined by the
  official manifest reference:
  https://doc.rust-lang.org/cargo/reference/manifest.html
- crates.io trusted publishing binds a GitHub organization, repository,
  workflow filename, and optional environment, and uses `id-token: write` plus
  the official authentication action:
  https://crates.io/docs/trusted-publishing and
  https://github.com/rust-lang/crates-io-auth-action
- Rust RFC 3691 records that a trusted publisher can be configured only after
  initial manual publication, justifying the one-time bootstrap token:
  https://rust-lang.github.io/rfcs/3691-trusted-publishing-cratesio.html

## Compatibility and dependency evidence

The crates.io API returned `404` for all three exact names on 2026-09-22, so no
namespace collision currently exists. The selected dependency order is fixed:
derive has no Identus runtime dependency; core depends exactly on derive;
crypto depends exactly on core and derive. The existing archive inspector
already rejects a normalized Git/path dependency and any internal requirement
other than `=0.1.0-rc.1`.

The direct and resolved dependency cone is the locked Cargo graph already
tested by the candidate. The release adopts no new runtime dependency, native
library, unsafe code,
wire format, cryptographic primitive, or third-party public type. The facade
boundary stays with the three Identus-owned APIs; no third-party type is newly
re-exported. The only new
third-party build component is the official crates.io OIDC authentication
action, pinned to an immutable revision and used only in the protected release
workflow. Existing Cargo-deny, advisory, license and provenance, source, API,
conformance,
feature, target, and archive gates remain authoritative.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Publish the entire workspace | `not-adopt` | Converts experimental/source-only packages into accidental API and namespace promises. | Each package has its own approved train and evidence. |
| Keep all canonical manifests unpublished and upload hand-built HTTP payloads | `not-adopt` | Bypasses normal Cargo publication semantics and makes package identity hard to inspect. | Never for ordinary crates.io releases. |
| Activate only three canonical manifests and publish the deterministic isolated workspace | `adopt` | Makes package intent reviewable while retaining a cohesive, byte-reproducible closure and denying every unrelated member. | Cargo gains a safer native monorepo release primitive that removes staging. |
| Store a permanent token in ordinary repository secrets | `not-adopt` | Broadens credential exposure and conflicts with the approved protected-environment/OIDC policy. | Emergency incident process only. |
| One protected bootstrap token, then crates.io OIDC | `adopt` | Satisfies the namespace bootstrap constraint and minimizes all later credential lifetime/scope. | crates.io supports pending trusted publishers before first publication. |
| Publish locally from a maintainer laptop | `not-adopt` | Loses immutable hosted evidence and consistent two-person environment control. | Governance-recorded emergency only. |
| Tag push automatically publishes | `not-adopt` | A pushed tag could enter the irreversible path before explicit release-mode and receipt review. | Protected tag rules and release approvals become sufficiently expressive. |
| Manual exact-tag dispatch behind a protected environment | `adopt` | Separates immutable release identity, verification, independent approval, and upload while retaining a durable run receipt. | A dedicated release orchestrator provides equivalent or stronger controls. |

## Security, privacy and maintenance evidence

The publication job receives read-only contents plus OIDC identity permission;
the bootstrap token is scoped to the protected environment and never enters
candidate or PR jobs. Authentication modes are explicit and fail closed. The
publisher accepts only a signed tag and full SHA contained in protected
`develop`, consumes only same-run artifacts, validates checksums again, and
stops on the first dependency failure. It does not yank, overwrite, retag,
promote `main`, or mutate consumers.

Published crates are immutable and the `0.1.x` MSRV is fixed at Rust 1.89.0.
An error therefore requires preserved evidence, a maintainer yank decision
when appropriate, and a new corrected version. Ownership, recovery, yank,
token revocation, and trusted-publisher duties are mandatory closeout evidence.
This is the rollback boundary after an upload; before upload, rollback is a
normal repository revert. The maintenance, release, and security posture is
reviewed again before the next train.

Supply-chain evidence remains the locked dependency audit, deterministic crate
checksums, SBOM, exact tool/action revisions, signed tag, and hosted workflow
receipt. Public and wire compatibility is unchanged from the reviewed
candidate. Protocol or draft currency is not applicable because this slice
changes package distribution rather than an SSI protocol profile.

No private user data, wallet secret, credential, identity claim, telemetry, or
protocol exchange enters this build/release-only slice.

## Rejected or deferred candidates

`cargo-release`, `release-plz`, and workspace-wide version automation are
deferred: the first train has only three packages, an existing deterministic
builder, and stricter human/receipt controls than those tools provide by
default. Automated changelog generation, final `0.1.0`, additional crates,
main-branch promotion, signing services, and downstream upgrades remain later
slices.

## Open questions and blockers

None for repository implementation. This GitHub identity cannot enumerate an
organization-scoped secret, so the protected bootstrap job must fail closed if
`CARGO_PUBLISH` is not actually inherited. crates.io trusted-publisher setup
and bootstrap-token revocation remain explicit human web-console steps after
the namespace-creating upload.

## Evidence commands

Planning evidence: GitHub issue/milestone/ruleset/environment/secret metadata,
crates.io API name lookups, Cargo and crates.io primary documentation, existing
candidate receipts, `scripts/factory doctor`, strict OpenSpec validation,
research readiness, and constraint readiness.

Exact planning commands include `cargo metadata --locked --format-version 1`,
crates.io `GET /api/v1/crates/<name>`, `gh api` repository-policy inspection,
`scripts/factory validate activate-first-crate-release-train`,
`scripts/factory research-ready activate-first-crate-release-train`, and
`scripts/factory constraints-ready activate-first-crate-release-train`.

Implementation evidence: package metadata and list/dry-run checks, candidate
double assembly, release-policy mutation tests, actionlint, factory gates,
primary/MSRV support gates, clean focused-branch review, protected PR CI,
signed tag verification, protected release run, crates.io version/checksum and
owner inspection, docs.rs, and GitHub release evidence.

Unrun checks at planning time are archive regeneration, Cargo publication dry
run, hosted PR CI, tag signature verification, protected-environment approval,
registry upload, ownership inspection, docs.rs, and OIDC authentication; those
belong to implementation or external release activation.
