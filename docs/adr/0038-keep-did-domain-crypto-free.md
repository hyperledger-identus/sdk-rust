# ADR 0038: keep the DID domain dependency cone crypto-free

- **Status:** Accepted for experimental implementation
- **Date:** 2026-09-05
- **Decision authority:** standing roadmap and issue #101
- **Related work:** issues #5, #9, #98 and #101; ADRs 0009, 0035 and 0037;
  OpenSpec change `narrow-did-crypto-feature-cone`

## Context

The workspace disables `identus-crypto` defaults so each consumer selects only
the capabilities it performs. `identus-did` nevertheless re-enabled the full
crypto default set after issue #98, making a DID-only build include four curve
families, hashing, encodings, JWK/thumbprints, COSE and derivation.

No production or test source in the DID crate imports `identus-crypto`. Its DID
document boundary treats `publicKeyJwk` as bounded, extensible JSON, rejects
private members and conflicting recognized material, and deliberately does not
validate curve points or signatures. Algorithm-aware conversion already lives
at the operation-owning boundary, such as the JOSE proof verifier.

## Decision

1. Remove `identus-crypto` from the `identus-did` manifest. Retain only
   `identus-core` and the used `identus-derive` proc macro as internal normal
   dependencies. Request the directly used `serde/derive` capability on DID's
   existing Serde dependency instead of receiving it through crypto feature
   unification.
2. Do not add DID passthrough features for JWKs or curves. The crate has no
   optional algorithm behavior, so default and no-default builds remain the
   same surface.
3. Preserve structural public-JWK validation and its existing API exactly.
   Consumers that perform crypto select, convert and algorithm-bind public key
   material through their own explicit dependency.
4. Add a manifest conformance assertion for the exact current internal DID
   dependency set. A later real dependency requires a focused contract and an
   intentional assertion update.
5. Use existing host, MSRV and compile-only target gates plus focused
   default/no-default Cargo tree evidence. Do not add duplicate permanent Nix
   gates for a crate with no feature variants.
6. Declare the exact complete feature prerequisites of each crypto integration
   target exposed by the honest no-default workspace graph. Add an isolated
   minimal-crypto test gate so future feature-unification regressions fail.

## Consequences

- DID-only consumers no longer compile unused cryptographic algorithms or
  supporting dependencies.
- The boundary between a structurally acceptable DID document and a
  cryptographically usable key remains explicit.
- Consumers that previously relied on accidental Cargo feature unification
  must request their crypto capabilities directly. The SDK is unpublished and
  already documents explicit feature ownership, so no supported API breaks.
- A future cryptographic DID feature remains possible when two real consumers
  justify its ownership; this decision does not prohibit that evidence-backed
  change.

## Rejected alternatives

- **Keep all crypto defaults:** preserves unnecessary compile cost and hides
  which component owns algorithm policy.
- **Select only the crypto `jwk` feature:** remains unused coupling and would
  conflate open DID document material with the SDK's closed typed key families.
- **Add empty DID feature flags:** increases configuration and testing surface
  without changing behavior.
- **Rely on review alone:** a same-layer crypto edge passes the broad ring guard
  and could return unnoticed.

## Rollback

Before publication, restore the manifest edge and remove its exact conformance
assertion and documentation. No data, wire, API or downstream migration is
required.
