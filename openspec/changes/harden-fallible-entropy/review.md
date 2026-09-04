# Pre-implementation semantic, security, and API review

- **Date:** 2026-09-04
- **Issue:** #67, child of #9 / `IDR-004` and #20
- **Develop base:** `0b98a2e9fdd70e8e0792bfcb54a658cf60f22c8d`
- **Result:** contract is implementable with no unresolved blocker

## Findings

1. The current allocation-returning port permits work proportional to an
   arbitrary requested length. A caller-owned slice makes allocation policy
   explicit and keeps current crypto paths fixed at 32 bytes.
2. Entropy is intrinsically fallible. Encoding failure as panic makes library
   consumers and foreign-language bindings unable to recover.
3. The existing `SecureRandomFailure` variant and stable redacted code are the
   correct abstraction. Backend strings and random material must not cross the
   capability boundary.
4. Ed25519/X25519 exact-length panics disappear naturally because the caller
   supplies fixed arrays. EC rejection sampling still requires a bounded
   exhaustion error after sixteen invalid candidates.
5. A backend may partially write before failing. Constructors must return
   immediately without constructing, logging, or exposing output.
6. This is a deliberate breaking correction to unpublished `0.0.0` APIs. No
   compatibility shim is warranted because it would preserve unbounded
   allocation or infallibility.
7. The change belongs in generic sdk-rust crypto and entropy-adapter surfaces;
   it requires no Midnight policy, wallet custody, downstream mutation, new
   dependency, release, repository setting, or `main` change.

Verdict: READY to implement after strict OpenSpec validation.
