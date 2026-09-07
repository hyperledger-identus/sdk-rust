# Constraint and limitation impact

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/discussions/172
Constraint blockers: none

## Existing entries affected

`SDK-COMPAT-002` changes the effective compiler floor from Rust 1.85.0 to
1.98.1. `SDK-COMPAT-003` stops naming Rust 1.89 as an active candidate and
becomes a deferred release-candidate compatibility decision. `SDK-COMPAT-004`
remains Rust 1.98.1 primary. `SDK-COMPAT-005` changes from the NeoPRISM nightly
to Rust 1.98.1 as compatibility etalon; the pinned nightly remains only a
tooling exception for sanitizer fuzzing. `SDK-DELIVERY-001` gains one intended
stable `fast` merge signal while the exhaustive `slow` matrix becomes weekly
and non-blocking during active development.

## Introduced or changed constraints

The exact effective Rust version is 1.98.1 across Cargo, ordinary Nix builds,
primary validation and compatibility etalon. Pull requests and `develop`
pushes receive one Linux fast line. The complete Linux/macOS matrix runs weekly
and manually. Review is mandatory by 2026-12-08 and before any release
candidate.

## Introduced or changed limitations

No compatibility is promised below Rust 1.98.1. Slow and sanitizer evidence is
not produced for every merge. A slow failure is non-blocking pre-release debt,
but a release candidate is prohibited until a separate decision restores an
appropriate consumer/target matrix and resolves all slow failures. Repository
settings are not changed by this PR, so maintainers must activate the exact
`fast` status before it can technically enforce merge blocking.

## Consumer and product impact

There is no published SDK consumer promise to migrate. Source consumers pinned
to `develop` must use Rust 1.98.1. Oxid, midnight-identity, Lace ID Portal,
Apollo and NeoPRISM are not edited or certified. The benefit is materially
shorter agent feedback and one unambiguous integration signal.

## Activation and rollback

Merge activates repository policy and workflow cadence. A maintainer then
configures `fast` as the required `develop` check and observes a test PR proving
it blocks until success. Rollback reverts the focused PR, restoring Rust 1.85,
the nightly etalon and per-PR full matrix. No artifact may be published under
the temporary policy, so rollback has no released-data migration.

## Evidence

Machine policy, Cargo, Nix providers, gate manifest, workflows, documentation,
constraints and structural tests must agree. Baseline timing uses 20 successful
PR runs. Hosted CI must prove the new fast status before merge; slow is manually
dispatchable for a post-merge full-matrix receipt.
