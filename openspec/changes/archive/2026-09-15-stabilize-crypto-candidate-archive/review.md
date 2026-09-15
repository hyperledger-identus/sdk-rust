# Local review

- **Review date:** 2026-09-15
- **Review angle:** Cargo VCS discovery, native archive integrity, temporary
  path containment, atomic output visibility, cleanup, regression strength,
  and scope control
- **Scope:** implementation diff after planning head `ddc3df06`
- **Result:** passed after resolving the exact-head P2 finding below

## Resolved findings

The first guard rejected scratch only beneath the canonical SDK checkout. The
exact-head Codex review correctly identified that a caller-controlled `TMPDIR`
could place the system temporary directory beneath an unrelated Git worktree,
reintroducing stage-specific Cargo VCS metadata. The guard now walks every
resolved ancestor, fails closed on an unreadable marker, rejects any `.git`
directory/file/symlink, and has a foreign-worktree behavioral regression.

The refreshed review then identified that the test suite's positive fixture
also inherits caller-controlled `TMPDIR`. The test now derives the expected
result from the fixture's actual ancestors: it requires acceptance under a
VCS-free temporary root and requires rejection under an ambient worktree. A
separate nested foreign-worktree fixture remains an unconditional rejection.

## Root-cause review

The prior implementation placed both generated Cargo workspaces below the
requested output parent. For the normal `artifacts/` destination, Cargo found
the canonical checkout and wrote distinct stage-relative `path_in_vcs` values
into `.cargo_vcs_info.json`. Dependent generated lockfiles then inherited the
different internal package checksums. The repair removes that input by placing
both build stages under a system temporary root outside the canonical checkout.

## Contract review

- The canonical checkout remains the only source-content and revision
  authority; no synthetic Git repository or archive rewriting is introduced.
- Resolved build scratch is rejected when equal to or below the checkout or any
  other Git worktree before Cargo is invoked.
- Byte-for-byte comparison of all three native Cargo archives is unchanged and
  still precedes closure/API/SBOM evidence.
- Completed evidence is staged below the destination parent and renamed on the
  same filesystem, retaining atomic visibility and failure cleanup.
- Focused behavioral coverage rejects canonical and unrelated worktree scratch;
  static mutation tests reject output-rooted build scratch and guard removal.

## Security and compatibility review

No Rust source, crate dependency, package metadata, public API, serialization,
feature, target, compiler, release, registry, credential, or consumer change.
Temporary paths are library-created, automatically cleaned, and excluded from
the receipt. The candidate remains unpublished and no byte comparison is
weakened.

## Separate canary finding

The same pre-repair canary independently found a missing ambient `sdkmanager`
command on the macOS hosted image. That is not concealed in this result: its
specification is preserved on `scratch/issue-276-macos-plan` and will proceed
from the post-merge `develop` head so the factory retains one preflighted
implementation per branch.

No unresolved blocking finding remains in the candidate repair.
