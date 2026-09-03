# Pre-implementation semantic and misuse-resistance review

- **Date:** 2026-09-03
- **Issue:** #46 (child of #5 / `IDR-006`)
- **Develop base:** `c002fc52d3ee1864c3499a3ca548232d774fbf9c`
- **Reviewed contract:** OpenSpec `add-generic-did-url-dereferencer` and ADR 0014
- **Result:** no unresolved blocker

## Findings

1. **Standards volatility:** the W3C generic algorithm is at risk. The adapter
   must be opt-in and isolated from the existing port and method registry.
2. **Ownership:** parameter preparation, exact document-resource matching,
   relationship authorization and string service endpoint projection are
   reusable DID-domain concerns. Method paths, network retrieval and product
   routing are not.
3. **Resolution boundary:** one injected resolver call keeps dependency
   inversion intact and permits PRISM/Midnight implementations without either
   entering this crate.
4. **Parameter ambiguity:** form decoding and last-value-wins duplicates create
   selector aliases. Literal plus preservation, single decoding and duplicate
   rejection are required.
5. **Resource identity:** matching must use exact absolute identifiers based on
   the resolved document DID. Prefix, suffix and decoded lookalike matching is
   unsafe.
6. **Authorization:** finding a verification method does not prove its allowed
   relationship. The requested core relationship must contain that exact method
   reference/object and failures retain CID identity.
7. **Service typing:** endpoint maps are method/product data, not generic URLs.
   Only string endpoints can enter URI-list and relative-reference processing.
8. **Traversal:** RFC 3986 alone deliberately permits parent navigation. Wallet
   routing requires a stricter base-directory scope and detection across nested
   encoding and backslash variants.
9. **Hidden I/O:** returning an endpoint is not authorization to fetch it.
   Transport, redirect, SSRF, recursion and cycle policy remain issue #10.
10. **Cardinality:** inheriting an input fragment into multiple endpoints is
    ambiguous and must fail rather than choose by order.
11. **Compatibility:** the new adapter and document filtering helper are
    additive in unpublished `0.0.0` crates. Existing resolver/dereferencer
    implementations remain unchanged.
12. **Provenance:** all donor repositories are immutable evidence. Apollo is
    legacy-only and no source or fixture is copied.
