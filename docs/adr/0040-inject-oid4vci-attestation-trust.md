# ADR 0040: inject OID4VCI attestation and federation trust

- **Status:** Accepted for experimental implementation
- **Date:** 2026-09-06
- **Decision authority:** standing roadmap and issue #104
- **Related work:** issues #8, #20, #99 and #104; ADRs 0034–0037
- **Supersedes:** ADR 0037 decision 10

## Context

OpenID4VCI 1.0 Final permits a key-attestation JWT and an OpenID Federation
trust chain in a proof JWT's protected header. OpenID Federation 1.0 is also
Final. The SDK currently rejects both, forcing products either to fork the
closed parser or to ignore trust-bearing semantics.

These mechanisms combine stable wire shapes with deployment-specific trust
anchors, certificate/federation algorithms, time, revocation, metadata and
assurance policy. Putting a concrete trust stack in `identus-jose` would couple
the portable codec to platform and product policy.

## Decision

1. Extend the closed protected-header model with bounded opaque
   `key_attestation` and `trust_chain` evidence; retain unknown-member rejection.
2. Add explicit holder construction for these values while keeping existing
   evidence-free construction source and wire compatible.
3. Require `kid` as the sole proof key reference whenever an outer trust chain
   is present.
4. Inject a trust-chain key provider that owns OpenID Federation validation and
   returns the trusted key selected by `kid`; re-bind the key and verify the
   outer proof in the SDK.
5. Inject a key-attestation validator that runs after proof-of-possession and
   owns nested JWT signature/trust/time/status/assurance checks plus exact
   `attested_keys` and nonce binding.
6. Expose parsed, cryptographically verified, trust-evaluated and
   issuer-authorized proof states. Existing composed APIs perform the added
   transition internally.
7. Bound evidence before providers, call each provider at most once, use static
   redacted failures and provide no fallback, ambient trust store or network.
8. Add no dependency; remain safe Rust, runtime-neutral, chain-neutral and
   product-neutral.

## Consequences

- Wallets and issuers can exchange the Final wire members without the SDK
  claiming a universal trust policy.
- Trust adapters can wrap platform X.509, federation or certification stacks
  outside this crate while the SDK independently enforces proof-key and
  algorithm binding.
- The stricter outer trust-chain/key rule rejects metadata-only combinations;
  a later issue can broaden this only with an unambiguous consumer contract.
- Consumers that do not opt into these headers retain existing behavior and no
  new provider requirement.

## Rollback

Before publication, revert issue #104's implementation. No downstream branch,
stored state, release or migration is involved.
