# Exact-diff architecture, security, and compatibility review

Review status: completed
Review date: 2026-09-29
Base: develop@`7c7c8e9a02bebd65d63c00631208283778f12fe4`
Implementation head: `6acbe141705eba95ade1b09e1c9870f4fef08402`
Specification commit: `c39b0c8e2c597094d7a97b07064f5bc4d849d627`
Unresolved blockers: none

## Findings

1. **Root cause — confirmed.** Each candidate lane previously resolved current
   registry state independently. A compatible upstream publication during the
   run could therefore produce different locks without any source revision
   change.
2. **Ownership and cohesion — accepted.** The closed DID descriptor owns one
   repository lock path and digest. One builder primitive validates and copies
   it into archive and matrix staging; packaging policy remains outside the DID
   crates and their runtime APIs.
3. **Fail-closed behavior — accepted.** Path traversal, symlinks, empty or
   oversized files, digest drift, malformed lock shape, duplicate identities,
   non-crates.io registry sources, missing checksums, stale Identus versions,
   and lane-local resolution are rejected before evidence is accepted.
4. **Race resistance — accepted.** The source lock is hashed before copying and
   the destination is hashed again against the descriptor. A change between
   checks therefore fails rather than silently entering a receipt.
5. **Reproducibility — accepted.** Archive and matrix paths consume identical
   bytes under `--locked`. The independent extracted-package verification lock
   remains appropriate because that workspace has a different topology and is
   not compared across candidate lanes.
6. **Toolchain closure — accepted.** The complete candidate command now uses a
   pinned Nix app symmetric with the crypto train; it carries Rust, public API,
   semver, and CycloneDX tools without relying on an ambient shell.
7. **Compatibility — accepted.** No crate source, public API, feature, protocol,
   compiler floor, target claim, permission, publication authority, or tag
   changes. Primary 1.98.1 and MSRV 1.89.0 consumed the same lock successfully.
8. **Factory integration — accepted.** The lock and Nix app are mandatory
   factory inputs. Mutation tests cover material weakening, and the Nix sandbox
   independently proves the fixture includes both files.

## Residual limitations

- An intentional dependency update is a reviewed source change: regenerate the
  lock once with the pinned primary compiler, inspect its dependency cone,
  update the descriptor hash and checker expectation, then rerun both lanes.
- Local Darwin evidence cannot substitute for Linux or for the protected
  weekly two-host run. Issue #388 owns that milestone evidence and approval.
- The descriptor uses Cargo lock format v4. This is supported by the declared
  Rust 1.89 MSRV; lowering the compiler floor would require a separate decision.

## Decision

The change removes time-dependent registry resolution from the DID candidate
matrix with a narrow release-tooling boundary. No unresolved correctness,
security, compatibility, architecture, or maintainability finding remains.
