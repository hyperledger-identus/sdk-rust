# Add OID4VCI attestation and trust-chain capabilities

## Why

The bounded OID4VCI proof verifier rejects `key_attestation` and `trust_chain`
even though OpenID4VCI 1.0 Final defines both and OpenID Federation 1.0 is now
Final. Products otherwise have to widen the JOSE parser or silently ignore
trust-bearing evidence, either of which weakens interoperability and security.

## What changes

- Retain bounded opaque `key_attestation` and `trust_chain` values in the
  protected-header model without treating them as trusted.
- Let holders construct proofs carrying those extensions under the same bounds.
- Resolve a `kid` through an injected OpenID Federation trust-chain provider
  when `trust_chain` is present, then re-bind and verify the proof signature.
- Validate key-attestation signature, trust, time, status, nonce and membership
  through an injected provider after proof-of-possession verification.
- Add an explicit trust-evaluated proof state between cryptographic verification
  and issuer authorization.
- Preserve fail-closed behavior when a provider is absent, rejects input or is
  unavailable.

## Non-goals

This change does not implement X.509 or federation path validation, fetch
metadata, select trust anchors, define issuer allow/deny policy, interpret
assurance values, add algorithms, implement the OID4VCI issuance engine, modify
consumers, publish crates or promote `main`.

## Impact

- **Issue:** #104 under #8, #20 and `IDR-004`.
- **Owner:** `identus-jose`.
- **Compatibility:** additive and unreleased; existing proof construction and
  verification calls retain their signatures and extension-free behavior.
- **Dependencies:** no new crate, third-party, runtime, network or trust-store
  dependency.
- **Rollback:** revert this focused change before publication; existing compact
  JWS and extension-free OID4VCI behavior remain intact.
