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

# Post-implementation semantic, security and API review

- **Reviewed head:** `f991be6a24d57b94517586ae4b3c6e2656280fcb`
- **Review completed:** 2026-09-03T06:38:11Z
- **Result:** passed with no unresolved finding

The exact diff from `c002fc52d3ee1864c3499a3ca548232d774fbf9c`
was re-read after focused tests, both workspace feature modes, strict Clippy,
rustdoc, 94.31% line coverage of the new algorithm and all 26 compatible Nix
checks. The public surface remains one additive opt-in adapter and one validated
document projection helper. Existing resolver, dereferencer and registry
implementations are unchanged.

Parameter names and values are decoded once without plus substitution;
duplicates, malformed/control-bearing values and option collisions fail before
resolution. Every accepted DID parameter and dereferencing input reaches the
single injected resolver through bounded `ResolutionOptions`. Custom path/query
resources still resolve for method context but are never guessed by the generic
algorithm.

Exact absolute identifiers govern fragments, services and relationship
membership. The relationship option cannot be applied to bare documents or
service routing. CID error identities remain open URIs, endpoint maps are never
treated as locations, multiple endpoint fragments fail closed, and document
metadata survives both document and URI-list projections.

The relative-reference code was checked independently against RFC 3986 merge
cases and the stricter wallet scope. Review found and fixed the empty-path
authority case (`https://host` plus `child`), then added a regression test.
Direct, encoded, double-encoded and platform traversal, authority changes,
unsafe base paths and route escape are rejected. Returned URIs are values only;
there is no DNS, HTTP, redirect, filesystem, recursion, clock, cache or chain
access.

The 2026 W3C algorithm remains at risk, so method/extension strategies, remote
retrieval/SSRF (#10), recursive cycles and arbitrary media fragments remain
explicitly outside this independently removable adapter. No downstream tree
was modified, switched, staged, copied from or built.
