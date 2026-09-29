# Tasks

## 1. Contract

- [x] 1.1 Record #482 current behavior, official Cargo evidence, candidates,
      constraints, design, threats, update/rollback boundary, and commands.
- [x] 1.2 Pass research, constraint, and strict OpenSpec readiness; commit the
      planning-only contract and persist immutable preimplementation evidence.

## 2. Frozen staged lock

- [x] 2.1 Generate and review one lock from exact staged `0.1.0-rc.1` manifests;
      bind its repository path and SHA-256 in the descriptor.
- [x] 2.2 Validate/copy the lock through one builder primitive before archive
      and matrix Cargo operations; prohibit lane-local resolution.
- [x] 2.3 Add a pinned complete DID-candidate Nix app for archive/API/SBOM
      verification without ambient tools.

## 3. Fail-closed policy

- [x] 3.1 Validate lock bounds, format, hash, package/source identity, and
      descriptor completeness offline.
- [x] 3.2 Add missing, modified, stale, malformed, and resolution-bypass
      mutations with bounded diagnostics.

## 4. Verification and delivery

- [x] 4.1 Prove candidate preparation plus local primary/MSRV lanes report the
      exact descriptor lock hash; run Nix, factory, OpenSpec, and lint gates.
- [x] 4.2 Complete architecture/security/compatibility review and prepare the
      issue-linked PR; exact-head CI, discovery review, merge, metrics, #388
      update, and worktree closeout remain delivery steps.
