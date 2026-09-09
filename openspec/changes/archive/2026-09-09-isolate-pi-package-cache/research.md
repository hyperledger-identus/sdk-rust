# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-10
Source retrieval date: 2026-09-10
Research blockers: none

## Problem and existing implementation

Issue #246 launched the repository-pinned Pi through `./bootstrap.sh --pi`.
Pi created `<worktree>/.pi/npm`, containing 19,386 files, 1,797 directories
and 168,177,664 bytes. The same behavior reproduced from the `develop`
checkout. The cache violates clean-status and bounded-disk expectations when
duplicated across managed worktrees.

The current bootstrap audits Pi and then executes it in the worktree. The
tracked `.pi/settings.json` declares three exact npm package sources. There is
no repository cache-preparation layer.

## Normative sources

This is local factory infrastructure. The controlling sources are issue #247,
the accepted `factory-operations` specification, ADR 0108, the repository's
factory policy and Pi 0.84.2 as pinned by `flake.lock`.

The pinned Pi implementation is the primary behavior oracle. Its
`DefaultPackageManager.getNpmInstallRoot()` and
`getManagedNpmInstallPath()` functions hard-code project package storage to
`join(cwd, ".pi", "npm")`. `PI_PACKAGE_DIR` controls Pi's own application
asset directory, not project package storage, and is therefore rejected for
this use. Pi installs npm packages with exact project scope and
`--legacy-peer-deps` when npm is selected.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Ignore `.pi/npm/` only | `not-adopt` | Keeps Git clean but duplicates about 160 MiB per worktree | Pi gains a negligible package closure |
| Set `PI_PACKAGE_DIR` | `not-adopt` | Changes Pi application assets and does not relocate project package storage | Pi documents a dedicated project-store override |
| Move packages to user Pi settings | `not-adopt` | Couples unrelated repositories and mutates user configuration | Explicit user-owned global package policy |
| Symlink every worktree to one mutable store | `not-adopt` | Different branches/package sets can contaminate one another | Pi supplies transactional project package reconciliation |
| Content-addressed external store with worktree symlink | `adopt` | Preserves Pi's path semantics, deduplicates identical exact sets and isolates changed sets | Pi adds a supported project-store override or packages become Nix-native |
| Bake extensions into the Nix Pi derivation | `spike` | Broadens the flake closure and upgrade process beyond measured friction | Offline or supply-chain evidence requires a Nix-built extension closure |

## Compatibility and dependency evidence

The runtime versions remain Pi 0.84.2, Node 24.19.0, npm 11.17.0,
`dev-loops@0.9.0`, `pi-subagents@0.66.0` and `typebox@1.3.9`. The cache key
binds all of them, the tracked npm lock digest and a cache schema version. The
lock fixes the resolved transitive package cone and `npm ci` verifies published
integrities without executing lifecycle scripts. No Cargo manifest, Rust
artifact, feature, MSRV, target or public API changes.

The cache lives in a hidden repository-specific sibling directory derived
from the primary checkout identity, not under any registered Git working tree.
Only Pi package files and a closed metadata marker may enter it; authentication,
sessions, prompts, transcripts and provider data remain in their existing
locations and are neither inspected nor copied.

## Security, privacy and maintenance evidence

Paths are derived from Git and validated through canonical containment and
non-symlink checks. Bootstrap refuses an existing non-symlink `.pi/npm` or a
symlink to any unexpected target. Initial population uses the tracked exact
manifest and lock through `npm ci`, does not approve package lifecycle scripts,
verifies top-level identities and
versions, and publishes a complete store atomically. Concurrent initializers
use separate staging directories and converge on the same immutable target.

No general cache pruning is installed. Incomplete or operator-owned data is
reported with an explicit recovery path. Removal of a complete cache is a
manual, exact-path maintenance action after no Pi process uses it.

## Rejected or deferred candidates

Automatic version updates, shared user package state, arbitrary cache-root
environment overrides, session relocation, install-script approval and
destructive retention automation are rejected for this slice.

## Open questions and blockers

No implementation blocker remains. Silent progress during a long Pi print run
and the canary's one corrected preflight invocation are retained as observations
but do not justify another tuning change without repetition.

## Evidence commands

Planned evidence is strict OpenSpec validation, focused Node tests, a two-
worktree cache identity fixture, clean Git-status probes, Pi audit, bootstrap
factory checks, exact target planning and hosted `fast` CI. A live Pi smoke
will reuse the already populated exact cache without starting a product slice.
