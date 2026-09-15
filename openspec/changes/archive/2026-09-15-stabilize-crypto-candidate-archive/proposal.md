# Stabilize unpublished crypto candidate archives

## Why

The first post-activation slow canary on protected `develop` failed while
preparing the unpublished crypto candidate. The double assembly creates both
temporary workspaces beneath the checked-out Git repository, so Cargo 1.98.1
embeds stage-specific `path_in_vcs` values and produces different package bytes
from identical SDK source.

## What changes

- Build both Cargo staging workspaces outside the source repository's VCS tree.
- Preserve atomic final-output publication through a separate temporary sibling
  of the requested output directory.
- Reject a build scratch path that unexpectedly resolves beneath the source
  repository.
- Add focused regression and policy evidence, then rerun the complete candidate
  command and the protected slow canary.

## Capabilities

### Modified capabilities

- `unpublished-crypto-candidate`: make the existing byte-determinism contract
  independent of the caller's output location and Cargo VCS metadata discovery.

## Non-goals

- No cryptographic, public API, wire, package-version, dependency, MSRV, release,
  publication, registry, or consumer change.
- No weakening of byte-identical archive comparison.
- No claim that the manual canary is natural scheduled evidence.

## Delivery

Issue #276 owns this activation defect from protected
`develop@7bc2030a45e59cfea3f994d13d411a32e79f9b7c`. The repair uses a focused PR
to `develop`; issue #276 remains open until a later successful natural schedule.
