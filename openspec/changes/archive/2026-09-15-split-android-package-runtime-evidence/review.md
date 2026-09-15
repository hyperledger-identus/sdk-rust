# Exact-diff architecture, security and operations review

- **Review status:** completed
- **Review date:** 2026-09-16
- **Base:** `f2221d444e85e4b5e4bffdc0c0fd2d7e43ec83da`
- **Reviewed implementation head:** `59dd76abeed6a3ab72d8626e25c322a6065ea0c2`
- **Unresolved blockers:** none

## Scope reviewed

The review inspected the complete base-to-head diff, ADR 0124, exact-merge
canary `34992633358`, verifier failure and cleanup behavior, GitHub runner and
Android acceleration constraints, workflow authority, package/runtime role
separation, NDK/linker selection, ELF/AAR inspection, emulator admission,
diagnostic retention, final receipt binding, support claims and all policy
mutations.

## Resolved findings

1. **Resolved — factory fixture omitted newly read inputs.** The support checker
   began reading `nix/devshells/bindings.nix`, and the factory began requiring
   ADR 0124, but its isolated fixture copied neither. The complete Nix gate
   failed closed. Both inputs are now explicit fixture members and the isolated
   factory derivation passes.
2. **Resolved — living support claims described the removed topology.** The
   first implementation draft changed execution without updating all current
   architecture, support-policy, constraint and parity records. They now state
   ARM64 package proof, test-only x86_64 runtime proof and the missing ARM64
   execution evidence explicitly.
3. **Resolved — lint defects in new governance evidence.** The ADR had a wrapped
   issue reference parsed as a Markdown heading and the archive intent was not
   canonical Taplo format. Both are corrected and the full lint matrix passes.

## Accepted findings

1. **Architecture and cohesion.** One verifier owns common deterministic
   library/generator/AAR/consumer behavior while two explicit modes own their
   architecture-specific facts. The test ABI is never combined with the ARM64
   package or represented as a supported target.
2. **Correctness.** Both modes use the same crate, lockfiles, generator,
   templates and behavior marker. Each mode binds its Rust target, ABI, linker,
   ELF machine, evidence path and receipt role. The final slow receipt includes
   the dedicated runtime job result.
3. **Security and least authority.** The Linux job admits only the runner user
   to `/dev/kvm` with mode `0600`; the emulator requires acceleration. There is
   no blanket license acceptance, Google service image, secret, production
   permission or first-party unsafe code. Failure diagnostics are retained and
   the emulator is terminated by a trap.
4. **Reproducibility and supply chain.** ARM64 and x86_64 roles retain two-build
   library/AAR comparisons, exact NDK metadata, JNA checksum, Gradle locks and
   path-leak checks. The hosted emulator/tool archive revisions are not fully
   hermetic and remain an explicit evidence limitation.
5. **Compatibility and support.** Public Rust/Kotlin APIs, ABI version, wire
   behavior, minimum Android API and distributable ARM64 content do not change.
   `SDK-LIM-002` and `SDK-LIM-003` remain effective; x86_64 runtime execution is
   test evidence, not a release or support promise.
6. **Operations and rollback.** Role-specific artifacts upload on failure with
   exact SHA/attempt names and bounded retention. A missing KVM/image, boot
   failure or behavior failure is red. Reversion restores the previous known-red
   topology without changing any released artifact.

## Decision

The implementation is focused, reversible and suitable for protected review.
No unresolved correctness, architecture, security, privacy, licensing,
compatibility or operational finding remains. The exact hosted slow canary is
mandatory after merge because local verification cannot exercise Android.
