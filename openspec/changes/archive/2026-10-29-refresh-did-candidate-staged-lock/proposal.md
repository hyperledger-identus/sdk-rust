# Refresh the DID candidate staged lock

## Why

Clean candidate preparation at
`7189d256c22b049849882320506962f7ea7fe888` fails because the frozen staged
lock no longer satisfies the current release-shaped manifests. Final #388
evidence cannot proceed with a lock that `cargo package --locked` rejects.

## What changes

- Adopt the review-only proposed lock containing six patch-level updates.
- Update the descriptor-bound SHA and readiness knowledge atomically.
- Reproduce the unchanged two-package candidate and its bounded evidence.

## Capability

### Modified capability

- `release-candidate-trains`: an explicit refresh rebinds the same candidate
  manifests to one newly reviewed deterministic lock.

## Non-goals

No dependency requirement, package, API, feature, candidate version, canonical
publish flag, tag, release, registry write, downstream mutation, or support
claim change.

## Delivery

Issue #581 owns this prerequisite to #388 from exact branch revision
`7189d256c22b049849882320506962f7ea7fe888`.
