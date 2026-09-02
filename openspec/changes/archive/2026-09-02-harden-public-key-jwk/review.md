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

## Post-implementation misuse-resistance review

- **Reviewed head:** `d7e3af398c94512f465a1f044c2c4e8498a95bcf`
- **Result:** no blocker; suitable for exact-head CI and hosted review

The implementation was reviewed afresh from the complete `develop...HEAD`
diff after all code was committed.

1. **Construction bypass:** all fields are private. Both serde and native
   `from_parts` enter the same validation path. The byte constructors accept
   fixed `[u8; 32]` coordinates and still reject the wrong curve family.
2. **Wire ambiguity:** serde rejects duplicate structural members. Canonical
   base64url tests cover padding, invalid alphabet, non-zero trailing bits and
   both 31/33-byte widths for `x`; `y` runs the identical gate and has a
   dedicated width test.
3. **Private material:** top-level `d` cannot be stored by either construction
   path. Rejection errors do not echo its value. Structural extension names
   cannot override serialized fields.
4. **Algorithm confusion:** only four typed profiles deserialize, and every
   incompatible `kty`/`crv` pair fails. The API documentation does not imply
   on-curve, subgroup or key-use validation.
5. **Failure behavior:** four encoder `expect` calls depend only on compile-time
   correct curve-family constants and fixed-size public bytes, never on
   untrusted input. No new `unsafe`, network, filesystem, randomness or secret
   handling was introduced.
6. **Complexity/performance:** coordinate work is constant over 32-byte values;
   extension serialization is linear in member count. No hot cryptographic
   loop changed, so a benchmark gate would add no useful signal for this slice.
7. **Dependency/layering:** serde dependencies are optional, workspace-pinned,
   wasm-safe and confined to `jwk`; conformance confirms no outward SDK edge.

Residual risk is limited to caller-owned total JSON size limits and
curve-operation validation after structural parsing. Those boundaries are
documented and belong to later DID/JOSE integration, not this component.

The archive diff review caught one documentation defect before delivery:
OpenSpec `MODIFIED` requirements replace the complete prior requirement, so
the first generated canonical spec had dropped existing error and feature
scenarios. The archived delta and canonical crypto spec were corrected to
retain every prior code/feature guarantee while adding the JWK cases. Strict
whole-store validation then passed with 15/15 specifications.
