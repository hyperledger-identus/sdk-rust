# Pre-implementation semantic and security review

- **Date:** 2026-09-03
- **Issue:** #34 (child of #5 / `IDR-005`)
- **Develop base:** `49b2ca29b6d6a63a240f110fa89a2d500204152a`
- **Source revisions:** Apollo `ccee22b`; NeoPRISM `8becb22`;
  midnight-identity `427f857`; Lace ID Portal `804de0a`; Oxid `685f967`
- **Target maturity:** 75% for this lexical slice
- **Estimated implementation effort:** 2–4 agent-hours, excluding hosted CI
- **Started:** 2026-09-03T01:24:00Z
- **Result:** no unresolved pre-implementation blocker

## Findings

1. **Ownership:** DID and DID URL lexical values are generic SSI primitives
   and belong in sdk-rust. PRISM and Midnight semantics belong in their method
   layers; wallet policy and user experience remain in Oxid.
2. **Ordering:** this small lexical boundary unblocks document, resolver and
   method-port work without importing those concerns. It is therefore a lower
   risk next step than starting JOSE or a complete DID document model.
3. **Provenance:** NeoPRISM supplies useful test behavior but delegates to
   IOTA Identity. midnight-identity's current checks are too shallow. Apollo,
   Lace and Oxid are compatibility evidence. The implementation will be
   standards-derived and copy no donor source.
4. **Architecture:** a self-contained single-pass parser avoids a large
   dependency and makes resource, error and serialization behavior explicit.
   Cached byte offsets make the frequently read component views free of
   allocation.
5. **Security:** byte limits are applied before scanning or allocation;
   errors expose invariant categories but never rejected input. Lexical
   acceptance remains explicitly separate from trust, method and resolution
   policy.
6. **Compatibility:** the API is additive in an unpublished crate, adds no
   dependency and remains compatible with wasm/mobile targets. Package naming
   and publication remain owned by issue #3.

Post-implementation findings, corrections and the exact reviewed head will be
added before the change is archived.
