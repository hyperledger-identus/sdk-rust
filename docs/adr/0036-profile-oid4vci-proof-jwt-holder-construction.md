# ADR 0036: profile OID4VCI proof JWT holder construction in JOSE

- **Status:** Accepted for implementation
- **Date:** 2026-09-05
- **Decision authority:** standing roadmap and issue #99
- **Related work:** issues #8, #20 and #99; ADRs 0034 and 0035

## Context

The SDK has a bounded JWS compact codec and chain-neutral signer/verifier
capabilities, but no type-safe construction of the key proof required by
OpenID4VCI 1.0 Final Appendix F.1. Oxid currently reconstructs that profile in
product code, while Lace ID Portal exposes an older singular proof shape and
defers verification. Repeating header, anonymous-flow and claim rules in each
consumer would create incompatible and security-sensitive variants.

The inherited `identus-openid4vc` package is still a quarantined placeholder;
activating an umbrella protocol namespace is not required to add this narrow
JOSE profile.

## Decision

1. Add the holder-side `openid4vci-proof+jwt` builder to `identus-jose`, as
   assigned by the SDK blueprint. Keep the full OID4VCI wire/state component and
   namespace decision separate.
2. Extend the closed JOSE header with an exclusive bounded key-reference sum
   type for `kid`, public `jwk`, or `x5c`. Preserve the existing constructor as
   the compatible `kid` shorthand.
3. Represent identified and anonymous pre-authorized clients as different enum
   variants so anonymous proofs omit `iss` by construction. Require explicit
   audience and integer issuance time and accept only a bounded optional server
   nonce.
4. Preserve staged state: preparation returns exact public signing bytes and a
   typed signed proof is created only through the existing external signer
   capability. No ambient clock or key custody enters the crate.
5. Deliver issuer-side parsing, policy, replay input, signature verification and
   DID URL dereferencing under the `authentication` relationship in a second
   independently reversible #99 pull request.
6. Reject optional `key_attestation` and `trust_chain` until their nested
   validation and trust semantics receive a separate contract.

## Consequences

- Wallets receive one portable, allocation-bounded proof construction path and
  cannot accidentally emit ambiguous key references or anonymous `iss` claims.
- Existing JOSE callers and compact values remain compatible.
- Inline X.509 values are syntax and transport input only; chain validation and
  trust remain injected verifier concerns.
- The first delivery adds no DID, chain, product, runtime, network, storage,
  clock or custody dependency.
- Downstream adoption and publication remain separate work.
