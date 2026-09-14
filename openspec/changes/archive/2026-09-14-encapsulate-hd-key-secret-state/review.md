# Pre-implementation semantic, security and API review

- **Date:** 2026-09-14
- **Issue:** #269
- **Develop base:** `21cdbde6b9c5650d073a4a61f443640363585b38`
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
   and separately prove absence of `Clone` and `Display`. Exact source and
   public-API review should prove absence of Serde and implicit raw-access
   traits. Runtime tests should cover wrapper redaction and explicit erasure
   without reading freed memory.
7. Existing published BIP-32/SLIP-0010 and Apollo-overlap vectors must migrate
   through the new boundary without changing expected values. No arithmetic,
   feature, target, dependency, FFI, custody, consumer, release, #7 or #168
   behavior is authorized.
8. Exposure methods cannot replace struct-literal construction. The contract
   must identify seed-plus-path reconstruction as the available path and raw
   extended-state rehydration as unsupported/deferred, without opportunistically
   adding an import constructor.

Verdict: READY for a planning-only commit and durable preflight receipt before
implementation.

# Post-implementation exact-diff review

- **Date:** 2026-09-14
- **Review head:** `85660b0628a351a77cec83a1c459deac477bb539`
- **Diff base:** `21cdbde6b9c5650d073a4a61f443640363585b38`
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
4. External trybuild coverage rejects all four raw field accesses and rejects
   `Clone` and `Display` on the exposure value. Exact source and public-API
   inventory review confirm that Serde, `Deref`, `AsRef`, a public constructor,
   and binding annotations are absent.
5. Every existing BIP-32, SLIP-0010 and Apollo-overlap vector was migrated
   through the explicit boundary and remains byte-identical in default,
   minimal, all-feature and KMP profiles.
6. No arithmetic, dependency, feature, target, wire, custody, FFI, consumer,
   publication, release, #7 or #168 behavior changed. The source break remains
   intentional and confined to the unpublished `0.1.0-rc.1` candidate.
7. A compiler-output portability failure in an additional Serde trybuild case
   was found during Nix verification: Cargo registry and Nix vendor paths were
   rendered differently. The wording-dependent case was removed; absence of
   Serde is instead evidenced by the exact source and public-API inventory,
   while the required portable trybuild regressions remain.
8. The API inventory confirms that no public `from_parts`, raw-state import, or
   struct-literal construction surface exists. Field readers can migrate to
   exposure methods; raw-state holders must retain the original seed and path
   for reconstruction or treat rehydration as unsupported pending a separate
   security/API decision.

Verdict: implementation matches ADR 0114 and the OpenSpec delta; archive and
ready pull-request delivery are authorized.
