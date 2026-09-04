# Pre-implementation semantic, API, and security review

- **Date:** 2026-09-05
- **Issue:** #71, child of #6 / `IDR-007` and #20
- **Develop base:** `02011c9ca61dc03502f16b89aca0347d79019ce5`
- **Result:** contract is implementable with no unresolved blocker

## Findings

1. Activating the existing `identus-credentials` member is lower coupling than
   adding a speculative `identus-vc-core` package. Because all packages remain
   unpublished, the choice is reversible and does not settle #3.
2. Oxid's closed Midnight format enum cannot be reused. A validated open token
   gives future SD-JWT VC, mdoc, VCDM, Compact, and test adapters an integration
   seam without a core edit.
3. The envelope must preserve opaque bytes rather than parse or verify them.
   Parsed/constructed state is explicitly not verified or trusted state.
4. Separate body, proof, and private-material wrappers improve type safety and
   permit the SDK to apply erasure only to genuinely private material. One
   internal size helper is sufficient; a public generic artifact abstraction
   would weaken semantics.
5. `CredentialPrivateMaterial` needs explicit and drop-time zeroization plus
   manual redacted debug. Payload and detached proof also require redacted
   debug because credential artifacts may contain PII even when not secret key
   material.
6. No serde shape should be invented. Format adapters own wire bytes; a wrapper
   wire model would be premature and add dependencies without enabling the
   first consumer.
7. The error bridge must use static codes/messages and never retain caller
   input. Each local variant can remain specific enough for tests and callers
   without leaking values.
8. Metadata, verification reports, status, disclosure, executable registries,
   storage, and trust policy remain coherent child slices. Omitting them is the
   intended 70–80% delivery boundary, not missing acceptance for #71.
9. Consumer repositories are provenance/conformance inputs only. No downstream
   build, branch switch, formatting, dependency repoint, or deletion is needed.

Verdict: READY to implement after strict structural validation.
