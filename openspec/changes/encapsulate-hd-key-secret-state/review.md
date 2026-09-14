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
