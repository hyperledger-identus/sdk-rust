# Harden DID staged-lock attestation and refresh

## Why

PR #483 froze one reviewed dependency lock across DID candidate and matrix
lanes. Post-green discovery review found bounded follow-up gaps: aggregation
only compares lane hashes to one another, the archive receipt echoes rather
than carries the installed digest, two staged Cargo evidence commands omit
`--locked`, and the static generation policy can miss alternate helpers or
variable-built commands. The intended refresh procedure is also prose rather
than an executable, review-oriented path.

## What changes

- Bind every matrix lane hash directly to the descriptor digest before
  aggregation.
- Carry the digest returned by staged-lock installation into candidate output.
- Require demonstrably locked resolution for Rustdoc and CycloneDX
  staged-workspace operations.
- Combine runtime command rejection with AST checks around two explicit lock
  generation purposes: extracted-package closure verification and local
  staged-lock refresh.
- Add a pinned, non-mutating refresh mode that writes proposed lock bytes and a
  dependency-cone drift report outside the repository without updating the
  descriptor.
- Retain exact-path, regular-file, symlink, size, format, source, and checksum
  validation without redundant traversal predicates.

## Capabilities

### Modified capabilities

- `release-candidate-trains`: candidate lock evidence is directly attested and
  refresh is an explicit review input rather than an ordinary evidence side
  effect.

## Non-goals

No dependency update, crate API, manifest requirement, candidate version,
compiler, target, publication, tag, release, workflow dispatch/rerun, consumer
repository, or support promise change.

## Delivery

Issue #484 owns this repository-local hardening and blocks final exact-SHA M5
evidence in #388. The PR targets protected `develop`; release and slow-run
authority remain unchanged.
