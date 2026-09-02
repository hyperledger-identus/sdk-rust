# Pre-implementation semantic review

- **Date:** 2026-09-03
- **Issue:** #28
- **Base:** `0449d206ceb3437cd3b26a3437c8bc4a52945666`
- **Result:** ready for implementation; no uncleared blocker

## Findings

1. **Boundary:** the proposal changes only `identus-crypto`; DID, JOSE, custody,
   chain-specific curves, publication and downstream repositories are excluded.
2. **Standards:** the four profiles and coordinate shapes align with RFC 7518,
   RFC 8037 and RFC 8812. RFC 7517 extension tolerance is preserved without
   assigning protocol policy in crypto.
3. **Misuse resistance:** private fields plus one shared constructor/serde gate
   remove every invalid state admitted by the current public struct. The threat
   contract correctly states that structural validation is not on-curve proof.
4. **Compatibility:** the API break is acceptable only because the crate is
   version `0.0.0`, publication is disabled and there is no supported consumer.
   Existing encoder wire bytes are an explicit regression gate.
5. **Provenance:** NeoPRISM is adapted, midnight-identity and Oxid provide
   independently proven validation patterns, Apollo is conformance-only, and
   Lace remains evidence-only. No license blocker applies to the planned code.
6. **Dependencies:** optional workspace-pinned serde dependencies stay inside
   the existing `jwk` feature cone and do not add a native-only backend.

## Residual risks to verify after implementation

- serde duplicate structural fields must fail rather than let extensions
  override them;
- all constructor and deserialize paths must use identical validation;
- error and debug output must not contain rejected `d` or coordinate values;
- minimal `jwk`/curve and wasm builds must stay green.
