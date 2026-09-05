# Add JWS signature capabilities

## Why

The bounded JWS Compact codec deliberately stops at an unverified wire value.
Oxid, Lace ID Portal, Midnight identity components and future OpenID/credential
crates need one chain-neutral way to sign the exact prepared bytes and turn a
parsed value into explicit cryptographic evidence without importing custody,
DID authorization or protocol-claim policy.

## What changes

- Add closed JOSE algorithm identifiers for fully specified `Ed25519`,
  `ES256`, and explicitly legacy `EdDSA` compatibility.
- Add a synchronous object-safe external signer port over public signing-input
  bytes, plus typed software-key adapters over `identus-crypto`.
- Add an allocation-bounded verifier-suite registry whose contents are the
  caller's algorithm allowlist.
- Bind each selected public JWK to exactly one algorithm before verification.
- Add strict Ed25519 and raw 64-byte `R || S` ES256 suites.
- Add `VerifiedCompactJws`, constructible only after a registered verifier
  accepts the exact received signing input.
- Add static redaction-safe signing, registry, key and verification errors.

## Non-goals

This change does not interpret JWT or OpenID claims; resolve, dereference or
authorize DID keys; match `kid`; decide trust; hold or export raw private key
bytes; define an async runtime or remote HSM transport; add symmetric, RSA,
Ed448 or secp256k1 algorithms; support JWE or JSON JWS serialization; modify a
consumer; publish a crate; or claim downstream adoption.

## Impact

- **Issue:** #98, child of #8 and #20 / `IDR-004`.
- **Owners:** `identus-jose` for capability/state policy and
  `identus-crypto` for primitive fixed-width P-256 operations.
- **Compatibility:** additive, experimental and unreleased; legacy `EdDSA`
  remains opt-in and is never in the recommended registry.
- **Dependencies:** `identus-jose` gains an inward dependency on
  `identus-crypto`; no chain, product, network, async, storage or custody edge.
- **Rollback:** revert the focused change before publication; consumers and
  stored wire values remain unchanged.
