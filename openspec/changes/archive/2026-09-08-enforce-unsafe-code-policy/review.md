# Exact-diff security and architecture review

- **Reviewer:** Codex distinct contract and implementation pass
- **Date:** 2026-09-08
- **Base:** `7f1ab5771bbacfe400d3fa1d15117ea27f1f728b`
- **Reviewed implementation:** `175a5504bbc61210a8dbb8d546ac20a4d9aa7758`
- **Scope:** issue #169, ADR 0087, workspace/member lint configuration,
  conformance guard and fixtures, constraint index, factory fixture and OpenSpec
- **Result:** ready; zero unresolved blockers

## Findings

1. **Verified — the compiler policy is uniform for authored source.** The
   workspace declares exact `unsafe_code = "forbid"`; all 17 package manifests
   retain exact `[lints] workspace = true`. Existing crate-local forbids remain
   defense in depth and no package exception exists.
2. **Verified — configuration drift fails closed.** The conformance guard uses
   the existing manifest walker and `LAYER_RULES`, not a second package list.
   Synthetic inputs reject a missing or weaker root level and missing or false
   member inheritance with bounded path/package diagnostics.
3. **Verified — behavior is independently exercised.** Dependency-free,
   offline disposable Cargo workspaces prove rejection for authored library,
   binary, integration-test, example, benchmark, build-script and proc-macro
   implementation targets, plus an attempted source-level `allow` override.
   Each probe requires both failure and the `unsafe_code` lint identifier.
4. **Verified — no unsafe SDK fixture is compiled.** Unsafe snippets exist only
   as Rust string data. Temporary directories are process-scoped, removed by an
   RAII guard and diagnostic output is capped at 8 KiB.
5. **Verified — the change has no consumer surface.** There is no dependency,
   lockfile, public API, wire, runtime, MSRV, feature or target-support change.
   The guard remains test-only inside `identus-conformance`.
6. **Accepted residual — external dependency source is outside this lint.**
   Cargo cap-lints behavior is unchanged. Dependency unsafe/native posture
   remains governed by per-dependency research, ADRs and supply-chain gates.
7. **Accepted residual — proc-macro expansions can bypass the lint.** A Rust
   1.98.1 negative probe and compiler source show that `span.allows_unsafe()`
   intentionally suppresses this lint for eligible expansion output. The claim
   was narrowed before implementation, `SDK-LIM-008` remains effective and
   issue #189 owns proportionate remediation research.
8. **Verified — exceptions cannot appear silently.** The exception set is
   empty. ADR 0087 requires a separate issue, safety ADR, indexed record,
   explicit scope/invariants/owner/reviewer and atomic guard update before any
   package may leave workspace inheritance.

## Corrections made during review

1. Rejected the initial assumption that workspace `forbid` uniformly covers
   external procedural-macro output after the Rust 1.98.1 probe disproved it.
2. Narrowed the specification, ADR and `SDK-LIM-008` before implementation and
   opened #189 instead of overstating assurance.
3. Removed a Markdown anchor from the constraint canonical path after the exact
   Nix source check rejected it.
4. Synchronized the factory's isolated repository fixture with ADR 0087 after
   Nix proved that the fixture copied the constraint index without its source.

## Decision

The implementation satisfies issue #169 and ADR 0087 for authored first-party
source. The procedural-macro expansion exception is a disclosed, separately
tracked compiler limitation rather than hidden residual risk. The change is
approved for guarded archive and an issue-linked PR to `develop`.
