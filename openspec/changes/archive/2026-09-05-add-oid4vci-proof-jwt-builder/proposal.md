# Add the OID4VCI proof JWT holder builder

## Why

The bounded JWS codec and signature capabilities now preserve and sign exact
wire bytes, but a wallet must still hand-assemble the required
`openid4vci-proof+jwt` header and claims. That duplicates security-sensitive
profile rules across Oxid and future consumers and makes anonymous
pre-authorized issuance, nonce use and key-reference exclusivity easy to get
wrong.

## What changes

- Add a narrowly profiled holder builder in `identus-jose` for Appendix F.1 of
  OpenID4VCI 1.0 Final.
- Extend the closed protected-header model with exactly one optional bounded
  `kid`, public `jwk`, or `x5c` key reference while preserving the current
  `ProtectedHeader::new` API.
- Require the proof media type, one key reference, a required audience, an
  integer issuance time, explicit identified/anonymous client mode, and an
  optional server nonce.
- Return staged signing input before any signer call, then produce a typed
  proof only through the existing external signer capability.
- Add deterministic final-spec and consumer-shaped positive/negative
  conformance cases with static redaction-safe errors.

## Non-goals

This change does not verify proofs; resolve or authorize DID keys; validate
X.509 chains; support `key_attestation` or `trust_chain`; own clocks, nonce
storage, replay state, custody, HTTP, issuance workflow or trust policy;
activate the quarantined `identus-openid4vc` placeholder; modify a consumer;
publish a crate; or claim downstream adoption.

## Impact

- **Issue:** #99, first reversible delivery under #8 and #20 / `IDR-004`.
- **Owner:** `identus-jose`; the later verifier may add the already-accepted
  inward edge to `identus-did`.
- **Compatibility:** additive and unreleased. Existing three-argument header
  behavior and compact/signature APIs remain valid.
- **Dependencies:** no new external dependency and no chain, product, runtime,
  network, storage, clock or custody edge.
- **Rollback:** revert this focused change before publication; existing compact
  values and APIs remain unchanged.
