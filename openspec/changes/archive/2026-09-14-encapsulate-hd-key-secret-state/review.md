# Pre-implementation semantic, security and API review

- **Date:** 2026-09-14
- **Issue:** #269
- **Develop base:** `dbef9923e65d1a8332c4ba38f42532c8a12811f7`
- **Decision record:** ADR 0114
- **Result:** contract is implementable with no unresolved blocker

## Findings

1. The four public arrays are a real copy-boundary defect. A field read needs
   no semantically visible export operation and the copied `[u8; 32]` has no
   automatic erasure owner.
2. Returning `zeroize::Zeroizing<[u8; 32]>` would erase the copy but its
   derived `Debug` prints the bytes. The proposed SDK-owned wrapper closes
   that formatting surface and avoids a third-party concrete facade.
3. The wrapper must not implement `Deref` or `AsRef`: either would create an
   ambient raw-access spelling that defeats the explicitly named audit
   boundary. Its one raw borrow must be lifetime-bound to the wrapper.
4. A caller can deliberately copy the borrowed array. The specification
   states that residual risk and does not overclaim memory protection.
5. Field removal is source-breaking. The Cargo SemVer guidance supports that
   classification. Because the package and review candidate are unpublished,
   replacing the candidate baseline is safer than a compatibility shim and
   remains reversible.
6. Compile-fail tests should cover both key types in an external-crate context
   and separately prove absence of `Display` and Serde. Runtime tests should
   cover wrapper redaction and explicit erasure without reading freed memory.
7. Existing published BIP-32/SLIP-0010 and Apollo-overlap vectors must migrate
   through the new boundary without changing expected values. No arithmetic,
   feature, target, dependency, FFI, custody, consumer, release, #7 or #168
   behavior is authorized.

Verdict: READY for a planning-only commit and durable preflight receipt before
implementation.

# Post-implementation exact-diff review

- **Date:** 2026-09-14
- **Review head:** `b60ec9ef5d9fa06b22614eb216a1655f519002e0`
- **Diff base:** `dbef9923e65d1a8332c4ba38f42532c8a12811f7`
- **Result:** PASS with no unresolved finding

## Review evidence

1. Both HD owners retain private fixed arrays, drop erasure, explicit erasure,
   redacted `Debug`, derivation metadata and algorithms. No public raw field
   remains in source or the updated candidate inventory.
2. `HdKeySecretBytes` has one private fixed array, manual redacted `Debug`,
   `Zeroize` and `ZeroizeOnDrop`. It has no `Clone`, `Copy`, `Display`, Serde,
   `Deref`, `AsRef`, constructor or FFI annotation surface.
3. The two named owner methods return independently owned zeroizing copies.
   The one raw borrow is lifetime-bound to that owner. Deliberate copies made
   by callers remain the documented residual boundary.
4. External compile-fail coverage rejects all four raw field accesses and
   rejects ambient cloning and formatting of the exposure value. Exact public
   API and source review confirm that serialization and implicit raw-access
   traits are absent.
5. Every existing BIP-32, SLIP-0010 and Apollo-overlap vector was migrated
   through the explicit boundary and remains byte-identical in default,
   minimal, all-feature and KMP profiles.
6. No arithmetic, dependency, feature, target, wire, custody, FFI, consumer,
   publication, release, #7 or #168 behavior changed. The source break remains
   intentional and confined to the unpublished `0.1.0-rc.1` candidate.
7. A compiler-output portability failure in an additional Serde trybuild case
   was found during Nix verification: Cargo registry and Nix vendor paths were
   rendered differently. The wording-dependent case was removed; the
   security property and required private-field compile regression remain.

Verdict: implementation matches ADR 0114 and the OpenSpec delta; archive and
ready pull-request delivery are authorized.
