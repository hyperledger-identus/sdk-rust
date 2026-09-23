# Design

## Package activation boundary

The workspace continues to default to version `0.0.0` and `publish = false`.
Only the three selected manifests carry explicit `0.1.0-rc.1`, crates.io
publication permission, repository/documentation metadata, and crate-specific
descriptions. Workspace path dependencies for the train also carry exact
`=0.1.0-rc.1` registry requirements, which Cargo replaces for local builds and
retains in normalized published manifests.

The deterministic candidate builder remains the source of reviewed archives.
It copies only allow-listed source files into a VCS-independent three-member
workspace, packages twice, compares bytes, verifies extracted closure, and
emits checksums, SBOMs, public API, tool identities, limitations, and a clean
publication workspace. The publication workspace is first generated and
lockfile-normalized in the same guarded VCS-independent scratch, then copied
without build outputs into the evidence directory. This remains true when the
requested evidence directory is inside the source checkout. Candidate
preparation contains no credential, upload, tag, release, or repository
mutation.

## Release identity and controls

The protected workflow is manual and accepts an exact source SHA, exact
`v0.1.0-rc.1` tag, and explicit authentication mode. Before access to
the protected environment it checks out the requested tag, verifies that the
tag and checkout resolve to the same full SHA, verifies the tag signature,
proves the commit is contained in protected `origin/develop`, re-runs the full
candidate builder, and uploads the immutable evidence artifact.

The publication job uses GitHub environment `crates-io`, downloads only the
artifact produced by its own verification job, revalidates its receipt and
checksums, and publishes in the fixed order `identus-derive`, `identus-core`,
then `identus-crypto`. There is no workspace-wide publish. A partially
completed train stops; immutable successful uploads are recorded and the same
version is never overwritten.

Because environment approval may delay the credential-bearing job, that job
repeats the tag/SHA/develop ancestry binding after checkout and before acquiring
or using a registry credential. The candidate artifact name is stable for the
workflow run rather than the attempt, so GitHub's failed-job-only rerun can
reuse the successful verification artifact. GitHub release creation is also
retry-safe: an absent release is created, while an existing release is accepted
only when it resolves to the exact immutable tag; conflicting identity fails.

The environment is configured for protected branches/tags, prevents self
review, disables administrator bypass, and requires approval from the Identus
maintainer team. This is the second-person control in addition to the release
manager who creates the tag and dispatches the run.

## Authentication transition

`bootstrap-token` mode reads only environment secret `CARGO_PUBLISH`, maps it
to Cargo's standard `CARGO_REGISTRY_TOKEN`, and fails if it is absent. It is
valid only while at least one selected namespace is absent. It must be revoked
after all first publications and after crates.io ownership is verified.

`trusted-publishing` mode requests `id-token: write` and uses the official
`rust-lang/crates-io-auth-action` pinned to an immutable commit. It cannot be
used until each crate owner configures this repository, workflow filename, and
`crates-io` environment in crates.io. The workflow never silently falls back
from OIDC to the bootstrap token.

This follows the Cargo Book guidance to inspect package contents and run
publication verification before upload, and the crates.io trusted-publishing
model of short-lived workflow-bound credentials. The initial-token exception
exists only because a trusted publisher cannot own a namespace before its
first release.

## Verification and receipts

Static mutation tests bind package scope, explicit versions, exact internal
requirements, release workflow triggers, environment, permissions, action
pins, tag/SHA checks, authentication separation, and publication order. The
publisher has a no-network verification mode used by required CI.

Regression tests additionally exercise an evidence destination inside a Git
checkout, malformed descriptor shapes, post-approval identity rebinding,
attempt-independent artifacts, exact existing-release handling, and failure
evidence retention. Policy checkers report bounded diagnostics for malformed
input rather than leaking interpreter tracebacks.

The release artifact includes the candidate receipt and three `.crate`
archives. After upload the workflow records crates.io version URLs and registry
checksums, uploads a publication receipt, and makes that receipt available for
issue #3, issue #326, and the GitHub release. A later human step configures the
three trusted publishers and records token revocation; automation does not
claim those web-console actions happened.

After publication closeout, the downstream midnight-identity canary replaces
its source revision with exact registry requirement `=0.1.0-rc.1`. That
consumer-owned CI becomes adoption evidence; it cannot retroactively authorize
or block the already approved namespace bootstrap.

## Failure and rollback

Before any upload, rollback is a normal revert of package activation and
release automation. After an upload, crates.io immutability applies: stop the
train, preserve the receipt, yank only when maintainers decide it is necessary,
and publish a corrected version through the same gates. Never retag, overwrite,
delete evidence, or publish a replacement under the same version.
