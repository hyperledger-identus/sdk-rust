# Pre-implementation architecture, API, standards, security and performance review

- **Date:** 2026-09-05
- **Issue:** #95 under #8 / #20 / `IDR-004`
- **Develop base:** `8bde038306c0a56850ba5972c48e2553015a2b03`
- **Result:** contract is implementable with no unresolved blocker

## Findings

1. The repeated donor seam is compact splitting, bounded base64url decoding
   and exact signing-input construction. Proof claims, DID authorization,
   clocks, suite selection and custody differ and must not move in this slice.
2. Credential semantics is the narrowest valid ring. Protocol crates may
   depend inward on it, while future credential formats avoid an OpenID edge.
   JOSE wire syntax does not belong among crypto key/curve primitives.
3. The public result must say `Unverified`; parse success proves syntax and
   bounds only. No convenience method may imply a valid proof or JWT.
4. RFC 7515 requires verification over the received encoded segments.
   Reserializing header JSON, even semantically equivalent JSON, can change the
   signature input and is prohibited.
5. An empty payload is valid JWS Compact. Only the protected-header and
   signature segments must be non-empty; the issue wording was corrected
   before implementation to preserve this standard behavior.
6. The complete compact limit must be checked before delimiter scanning and
   allocation. Decoded-length estimates must be checked before base64
   allocation, with actual byte limits checked after decoding.
7. Decode/re-encode equality is a small deterministic canonicality rule that
   rejects padding, whitespace, alphabet aliases and invalid tail bits.
8. A closed `alg`/`typ`/`kid` header is intentionally narrower than general
   JOSE. Silently accepting `crit`, `b64`, embedded keys or certificate chains
   would claim security semantics the SDK does not implement.
9. Top-level duplicate header members must be detected during deserialization;
   parsing first into `serde_json::Value` would discard the evidence.
10. `alg: none` is structurally rejected. Positive algorithm selection cannot
    be trusted to the header and remains an explicit verifier/profile policy,
    consistent with RFC 8725.
11. Algorithm text needs a small fixed visible-ASCII cap. `typ` and `kid` can
    be general non-control UTF-8 strings but remain caller-bounded.
12. A staged `JwsSigningInput` state lets a later signing capability receive
    exact public message bytes without this crate seeing raw private material
    or choosing a crypto backend.
13. Owning encoded and decoded forms has bounded memory cost and avoids a
    lifetime-heavy API. The aggregate 64 KiB default limits the duplication.
14. ProtectedHeader, signing-input and parsed-value Debug implementations must
    be custom. Deriving Debug would leak `kid`, payload, signature or compact
    text despite static errors.
15. Error variants can distinguish failure classes but must contain no parser
    offsets, Serde/Base64 causes or rejected values. The core bridge uses only
    static `jose.*` codes and messages.
16. Production dependencies need no crypto, random, async, HTTP, platform,
    donor or protocol crate. `base64`, `serde`, `serde_json` and the core error
    bridge are sufficient.
17. The exact RFC example proves received-input preservation; independently
    reconstructed EdDSA/ES256 proof-shaped values prove shared expressibility
    without copying donor fixtures or source.
18. A deterministic varied-length matrix provides immediate property-style
    evidence. Coverage-guided fuzzing remains a real follow-up rather than a
    falsely claimed placeholder.
19. Throughput should be observed in an ignored release test. No portable
    latency threshold is justified for a parser this early in an unreleased
    SDK.
20. Lace has no repository license file at the pinned revision, so it is
    behavior observation only. No donor code or fixture is copied from either
    repository.

Verdict: READY to implement after ADR 0034 and strict OpenSpec validation pass.
