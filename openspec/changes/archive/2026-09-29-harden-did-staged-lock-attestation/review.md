# Distinct local review

## Scope

Reviewed `f44bda2cd2ee0aac404e4c38d4c51a36a6e437e6..3d0d3140537cba451677f55732cd8767f668dede`
independently from the implementation pass, including the release builder,
static policy checker, mutation suite, OpenSpec contract, and generated
candidate evidence.

## Findings

No blocking finding remains.

- The staged-lock generation capability is closed over two literal purposes,
  with one privileged runner and runtime rejection in the ordinary command
  paths.
- Per-lane, aggregate, archive-pass, and receipt digests derive from observed
  installed bytes rather than descriptor echoing.
- Rustdoc and CycloneDX use Cargo locked mode; the latter uses the verified
  global Cargo option required by the plugin.
- Refresh is opt-in, pinned, review-only, external-output-only, exact-HEAD, and
  clean-worktree guarded. It cannot be selected with candidate or matrix mode.
- The path simplification preserves symlink, regular-file, size, and digest
  checks while removing redundant resolution logic.
- Mutations exercise both static and runtime bypasses and the relevant digest
  and locked-mode failure boundaries.

## Architecture and maintainability

The changes remain within the existing DID release-candidate policy owner and
introduce no runtime crate dependency or public SDK API. The refresh helper
reuses the candidate's identity and source policy, so it does not create a
second acceptance model. The implementation is cohesive with the release
builder despite its security-sensitive size; splitting the policy across new
modules in this slice would weaken review locality.

## Residual evidence boundary

Linux exact-head CI, the independent discovery review, protected merge, and
the next natural weekly slow run are delivery evidence, not local
implementation prerequisites. No manual slow workflow dispatch is authorized
by issue #484.
