# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/266
Constraint blockers: none

## Existing entries affected

- `SDK-REPO-001`: the candidate is prepared through signed, reviewed, green
  work targeting protected `develop`.
- `SDK-REPO-002`: `main`, tags, and release publication remain prohibited.
- `SDK-COMPAT-002`: Rust 1.98.1 is evidence for this unpublished candidate only.
- `SDK-COMPAT-003`: the release-candidate compiler matrix remains unelected and
  blocks publication.
- `SDK-LIM-001`: the canonical workspace remains unreleased; the staged SemVer
  candidate is experimental.
- `SDK-LIM-006`: no downstream adoption is inferred.

## Introduced or changed constraints

An unpublished package candidate must be generated from an exact protected
source revision by the repository script, include only the three approved
packages, normalize internal dependencies to exact candidate versions, pass
archive-closure verification, and emit deterministic checksums, SBOM, API, and
provenance evidence.

## Introduced or changed limitations

The candidate is not a registry artifact, signed attestation, support promise,
runtime target qualification, ABI, or certification. Local archive patches used
to verify the unpublished closure are not evidence that crates.io can resolve
the packages. Rust 1.98.1 does not become the eventual published MSRV by this
decision.

## Consumer and product impact

No consumer changes. Maintainers gain a reviewable package shape and exact
technical evidence before deciding release ownership and compatibility.

## Activation and rollback

The candidate tooling and contract activate after issue #266 merges to
`develop`. Rollback removes the descriptor, tooling, package docs, and evidence
baseline. It cannot yank or invalidate a public artifact because none is
published.

## Evidence

Candidate-specific tests, exact package archives/checksums, clean extracted
consumer checks, API/SBOM output, and existing protected CI form the evidence.
