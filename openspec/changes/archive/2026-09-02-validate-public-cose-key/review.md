# Semantic and misuse-resistance review

- **Date:** 2026-09-03
- **Issue:** #30 (child of #9 / `IDR-004`)
- **Develop base:** `82819ac622cf601cc1f5c9cb40a71754c53992c0`
- **Reviewed implementation head:** `aba9b1e491054e940c1cc9631826d59673787c90`
- **Result:** no unresolved blocker; suitable for exact-head hosted review

## Pre-implementation findings

1. **Ownership:** a public COSE Key representation is generic SSI/crypto
   infrastructure and belongs in `identus-crypto`. Product CBOR, DID policy,
   chain rules, custody, messages and adoption remain outside this slice.
2. **Provenance:** the audited donor revisions contain no reusable COSE key
   codec. Apache-2.0 `coset` is therefore wrapped behind SDK-owned types; no
   donor code or fixture is copied, and Lace remains evidence-only.
3. **Standards:** RFC 9052/9053 define OKP and EC2 key maps, RFC 8812 assigns
   secp256k1 curve value 8, and RFC 8949 defines length-first deterministic map
   ordering. The four selected profiles exactly match implemented SDK curves.
4. **Trust boundary:** the type validates representation only. It does not
   claim curve membership, subgroup validity, algorithm authorization,
   `key_ops` authorization or trust.
5. **Compatibility:** this is additive work in an unpublished `0.0.0` crate.
   `coset` stays optional and hidden, the feature builds independently, and no
   downstream repository changes.

## Post-implementation misuse review

The complete `develop...aba9b1e` diff was reviewed afresh after implementation
and after resolving the first hosted review finding.

1. **Construction bypass:** all wire and structural fields are private. Fixed
   array constructors and `from_cbor` converge on `from_wire`, which enforces
   one supported key family, exact coordinate widths and EC2/OKP shape.
2. **Private material:** label `-4` is scanned before key-type dispatch, so it
   is rejected even when an attacker supplies an unsupported `kty`. Error and
   debug surfaces contain invariant names and lengths, never raw CBOR or key
   bytes.
3. **Wire ambiguity:** exact-end parsing rejects tags and trailing items.
   Recursive normalization detects duplicate top-level and nested keys before
   `coset` interpretation. Supported text `kty`/`crv` spellings normalize to
   assigned integers.
4. **Resource behavior:** the byte cap is checked before decode; CBOR depth is
   limited to 16; retained top-level extension parameters are capped at 32.
   Recursive work is bounded by the 4096-byte input envelope.
5. **Determinism:** maps use RFC 8949 length-first ordering (encoded-key length,
   then bytewise lexical order); integers and collection lengths use preferred
   forms. Floating values are rejected because this codec cannot guarantee
   preferred half-precision serialization for them.
6. **Conversion safety:** full-coordinate JWK conversion preserves the key
   family, curve and bytes. Compressed EC2 input fails explicitly rather than
   inventing point decompression. Format-specific policy metadata is not
   translated.
7. **Dependency and target posture:** `coset` is optional,
   default-feature-disabled, workspace-declared, Apache-2.0, and absent from
   public signatures. Minimal, MSRV, WASM, Android, iOS, audit and license
   gates pass.

## Corrections made during review

- Replaced plain bytewise map sorting with RFC 8949 length-first ordering and
  updated the issue, spec, ADR, implementation and fixture together.
- Moved private-label rejection ahead of supported-profile dispatch and added
  an unsupported-`kty` regression fixture.
- Narrowed identifier-normalization wording to the supported structural
  `kty`/`crv` fields; common policy metadata is retained but not interpreted.
- Expanded every `MODIFIED` OpenSpec requirement to preserve all prior error,
  feature and dependency scenarios during archive synchronization.
- Resolved hosted review finding P2 by retaining the normalized source CBOR map
  as the round-trip value. Explicitly present empty RFC-valid `kid` and Base IV
  byte strings now survive encoding; `coset::CoseKey` is only a temporary
  typed interpretation. A dedicated fixture covers this boundary.

Residual risks are explicit: consumers still validate curve points and
operation-specific algorithm/key-use policy, and enclosing protocols may set
smaller message limits. Those are not blockers for this structural boundary.
