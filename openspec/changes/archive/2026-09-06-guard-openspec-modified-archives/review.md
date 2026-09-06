# Local review

- **Date:** 2026-09-06
- **Issue:** #102 (child of #20)
- **Develop base:** `f3e7219c97604fadc4478ad466c36749df6907dd`
- **Reviewed scope:** factory tooling, OpenSpec contract, tests and guidance
- **Consumers inspected or changed:** none
- **Result:** passed with no unresolved branch-owned blocker
- **Active effort:** approximately 75 minutes through archival and repeated
  full local gates

## Findings

### Resolved: canonical parent symlinks could bypass the file-only guard

The first implementation rejected symlinked active changes, capability inputs
and `spec.md` files, but a regular canonical file beneath a symlinked
capability directory could still be read. Bounded reads now reject any symlink
component beneath the resolved repository root. A focused fixture covers the
canonical-parent case in addition to the active capability case.

### Resolved: new scripts were absent from the first Nix source snapshot

The first full-flake invocation ran before the new checker and fixture suite
were staged. Nix's Git-aware clean source correctly omitted those untracked
files, causing the factory-contract derivation to stop before testing them.
The intended files were staged, generated Python bytecode remained untracked,
and the same full command then passed all 27 checks.

## Semantic and security review

The checker answers only whether a `MODIFIED` replacement preserves every
existing nonblank normalized line in order. It does not claim that new
semantics are correct. Destructive rewrites require one exact, reasoned intent
entry bound to the current canonical requirement hash; stale, duplicate,
malformed and unused entries fail closed. Rename-plus-modify binds intent to
the original canonical block and otherwise follows OpenSpec's rename-first
archive order.

All inputs are repository-local, size/count bounded and parsed with the Python
standard library. Symlink components and non-UTF-8 inputs fail closed.
Diagnostics expose paths, names and hashes but not requirement bodies. The
guarded facade runs readiness and the scoped preservation check before the
pinned OpenSpec command can mutate canonical or active state, then validates
the resulting repository.

The change modifies no Rust API, wire format, runtime behavior, dependency,
consumer repository, release setting or `main` state. Existing archives are
not rescanned. Rollback removes the checker, fixtures, facade operation and
guidance; archived intent remains ordinary historical evidence.

## Evidence

- focused preservation suite: 11/11 passed;
- factory mutation contract: passed, including no-mutation/no-archive proof;
- strict OpenSpec validation: 28/28 items passed;
- full `nix flake check --print-build-logs`: 27/27 pre-archive checks and 26/26
  post-archive checks passed on `aarch64-darwin` (the active-change derivation
  disappears after successful archive);
- workspace nextest: 411 passed, 21 skipped by existing feature conditions;
- diff, shell, Markdown, TOML, YAML and EditorConfig hygiene: passed;
- optional `ruff`: unavailable in the pinned devshell and not a repository
  gate.

The Nix run emitted existing platform warnings while successful: deprecated
`stdenv.isLinux`/`isDarwin` evaluation aliases, unavailable offline yanked-index
lookups inside the audit derivation, and Darwin fixup scanner diagnostics. No
warning was caused by or actionable within this tooling-only diff.
