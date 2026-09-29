# Distinct local review

## Scope

Reviewed the complete PR from base
`f44bda2cd2ee0aac404e4c38d4c51a36a6e437e6`, including the release builder,
static policy checker, mutation suite, OpenSpec contract, and generated
candidate evidence. A separate Claude discovery review then challenged the
first locally reviewed implementation and every concrete claim was reproduced
or rejected against the pinned tools before remediation.

## Findings

No blocking finding remains.

- The staged-lock generation capability is closed over two literal purposes,
  with one privileged runner and runtime rejection in the ordinary command
  paths.
- Per-lane, aggregate, archive-pass, and receipt digests derive from observed
  installed bytes rather than descriptor echoing.
- Rustdoc places Cargo `--locked` before the rustdoc separator. The pinned
  cargo-cyclonedx runs directly and its internal Cargo metadata call is forced
  through a metadata-only wrapper that injects `--locked`; a postcondition
  independently rejects any staged-lock byte change.
- Refresh is opt-in, pinned, review-only, external-output-only, exact-HEAD, and
  clean-worktree guarded. It cannot be selected with candidate or matrix mode.
- The path simplification preserves symlink, regular-file, size, and digest
  checks while removing redundant resolution logic.
- Mutations exercise both static and runtime bypasses and the relevant digest
  and locked-mode failure boundaries.

## Discovery review disposition

The external review produced five actionable findings:

1. **Accepted:** `cargo --locked cyclonedx` consumed the option before the
   external plugin, while cargo-cyclonedx 0.5.9 invoked unlocked metadata. The
   direct-plugin metadata wrapper and digest postcondition above replace it.
2. **Accepted as defense-in-depth:** per-pass digest validation made the
   cross-pass mismatch branch logically redundant. Validation now occurs after
   both observed digests are compared, so a one-pass copy fault and a repeated
   wrong copy have distinct fail-closed checks while the receipt still derives
   from observed installed bytes.
3. **Accepted:** the AST gate was position-blind. It now requires Rustdoc's
   `--locked` before `--` and verifies the CycloneDX call's closed environment.
4. **Accepted:** the generation guard compared its last argument to itself and
   split a literal to satisfy its checker. It now uses an explicit command
   shape and requires the manifest to equal the purpose-owned workspace path.
5. **Accepted:** the refresh output name could be occupied during resolution.
   The absent directory is now atomically reserved before the long operation,
   and unexpected contents fail closed rather than being replaced.

The exact-head follow-up review produced four additional findings, all
accepted and remediated before merge:

1. Public-API rendering now brackets the staged lock with observed digests,
   even though the pinned tool receives an existing Rustdoc JSON document.
2. The unused prefix-form Cargo allowlist was removed so the runtime command
   boundary accepts only command shapes that are actually used.
3. A leftover unused AST string helper was removed.
4. Failed refreshes recursively remove the output directory reserved by that
   invocation, while surfacing any cleanup failure on the original exception;
   a retry with the same output path is therefore not blocked by partial
   evidence owned by the failed invocation.

The final exact-head review found two assertion-coverage gaps. Both were
accepted: release evidence now anchors the staged lock to the descriptor before
tool execution and checks it immediately after Rustdoc, while the archive and
matrix installed-lock rejection branches are named in offline policy and
covered by mutations. No blocking finding remains.

## Architecture and maintainability

The changes remain within the existing DID release-candidate policy owner and
introduce no runtime crate dependency or public SDK API. The refresh helper
reuses the candidate's identity and source policy, so it does not create a
second acceptance model. The implementation is cohesive with the release
builder despite its security-sensitive size; splitting the policy across new
modules in this slice would weaken review locality.

## Residual evidence boundary

Linux exact-head CI, protected merge, and the next natural weekly slow run are
delivery evidence, not local implementation prerequisites. No manual slow
workflow dispatch is authorized by issue #484.
