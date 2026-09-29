# Design

## Descriptor-owned lock identity

Add `staged_lock` and `staged_lock_sha256` to the closed DID descriptor. The
path names a regular repository file outside crate sources; the hash is exact
lowercase SHA-256. Offline policy verifies both fields, file bounds, format,
checksum, candidate package identities, and absence of local/Git sources.

## One staging primitive

Introduce one builder helper that validates the descriptor-bound lock and
copies it to `<stage>/Cargo.lock`. Archive assembly and every matrix lane call
that helper, then use only Cargo commands carrying `--locked`. They never call
`generate-lockfile` for the staged workspace. The extracted archive verification
workspace retains its own internal lock because its topology is distinct and
is not the cross-lane source identity.

## Drift detection

`cargo metadata --locked` or the first existing `--locked` build fails if the
staged manifests and lock disagree. Focused tests additionally reject a missing
lock, wrong descriptor digest, modified bytes, invalid lock shape/source, and
reintroduction of matrix-lane generation. Lane/aggregate receipt schemas do not
change: their existing lock hash becomes the descriptor-bound hash.

## Pinned complete-candidate app

Expose one `did-candidate` Nix app with the same pinned compiler,
`cargo-public-api`, `cargo-semver-checks`, and `cargo-cyclonedx` inputs as the
established crypto candidate app. This makes full archive/API/SBOM validation a
repository-owned command; the existing primary/MSRV matrix apps remain narrow.

## Update procedure

An intentional dependency change regenerates the lock once under the pinned
primary compiler from the rendered staged manifests, reviews the cone, updates
the descriptor hash, and proves both primary/MSRV consumption. No hosted lane
may refresh it implicitly.

## Verification and rollback

Run focused policy/mutation tests, candidate preparation, local primary/MSRV
matrix apps, Nix/factory/OpenSpec gates, and exact-diff review. Hosted required
CI supplies Linux repository integration; later M5 slow evidence supplies the
two-host receipt. Rollback removes the binding atomically and restores the
documented time-dependent failure mode.
