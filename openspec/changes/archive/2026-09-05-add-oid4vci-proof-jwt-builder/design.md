# Design: OID4VCI proof JWT holder builder

## Context and source audit

Issue #99 advances `IDR-004` from
`develop@0b2d5139f4ec626305dc3cfb7113b555506f1624`. OpenID for Verifiable
Credential Issuance 1.0 Final, published 2025-09-16, Appendix F.1 requires an
asymmetric `alg`, exact `typ` value `openid4vci-proof+jwt`, exactly one signing
key identified through `kid`, `jwk` or `x5c`, required `aud` and integer `iat`,
optional `iss`, and optional server-provided `nonce`. Anonymous access through
the pre-authorized-code flow requires `iss` to be absent.

The existing `identus-jose` surface provides a closed `alg`/`typ`/`kid`
protected header, bounded compact encoding, exact signing bytes, a fixed-width
external signer port, strict Ed25519/ES256 capabilities and static errors. It
does not interpret claims or accept inline JWK/certificate references.

No donor code or fixture is copied. Oxid is a conformance-only observation at
`MediaNoxLabs/oxid@bfe3b481568dc738f0732c2b27548fab8721fd95`, Apache-2.0,
particularly `crates/adapters/openid4vci/src/lib.rs` and its contract tests. It
demonstrates anonymous `iss` omission, DID URL `kid`, audience, `iat`, nonce and
external signing, but uses legacy `EdDSA`. Lace ID Portal is observed at
`input-output-hk/lace-id-portal@804de0a9e58cf48ece3cc6c24b2245bb70bc80f1`;
it currently accepts a legacy singular proof object without verifying the JWT,
so its behavior is negative/deferred evidence only. No Lace material is copied
or adapted and no repository license is inferred.

## Decisions

### D1 — keep the profile in the accepted JOSE crate

The blueprint assigns narrowly profiled proof-JWT builders/verifiers to
`identus-jose`. This change therefore adds an `oid4vci` module there and leaves
the quarantined umbrella `identus-openid4vc` crate untouched. The later full
OID4VCI wire/state component remains B09/#7 work.

### D2 — add a closed key-reference sum type without breaking callers

`JwsKeyReference` owns exactly one `KeyId(String)`, `Jwk(PublicKeyJwk)` or
`X5c(Vec<String>)`. `ProtectedHeader::new` remains the compatible `kid`
constructor. A new constructor accepts the sum type, and accessors expose the
selected shape without ambiguous combinations.

Public JWK parsing reuses `PublicKeyJwk`, which rejects private `d` material
and validates the bounded structural key shape. X.509 entries are non-empty
standard-base64 strings; the list is limited to eight entries and each encoded
entry uses the existing header-string bound. Certificate parsing, path
validation and trust are verifier/provider concerns. Unknown JOSE members,
including not-yet-supported `key_attestation` and `trust_chain`, continue to
fail closed.

### D3 — make flow semantics explicit in the claims API

`Oid4vciProofJwtClient` is either `Identified(String)` or
`AnonymousPreAuthorized`. Serialization emits `iss` only for the first case,
so anonymous callers cannot accidentally send an issuer claim. `aud` and
optional `nonce` are bounded, non-empty, control-free strings; `iat` is an
explicit signed integer supplied by the caller. The builder has no ambient
clock and cannot claim nonce freshness or replay protection.

`Oid4vciProofJwtLimits` combines existing `JwsLimits` with a positive claim
string ceiling. JSON serialization uses a private closed struct in stable field
order and the compact layer enforces the final payload and token byte limits.

### D4 — preserve staged state and external custody

`Oid4vciProofJwtBuilder::prepare` performs all locally knowable checks and
returns `Oid4vciProofSigningInput`, which exposes only the exact public bytes
that must be signed. `sign_with` delegates to `JwsSigner`; algorithm mismatch
and final size preflight occur before provider invocation. Success returns an
`Oid4vciProofJwt` wrapper around the signed compact value. Neither type asserts
issuer acceptance, DID authorization, nonce freshness or trust.

For an inline JWK, preparation also constructs `JwsVerificationKey` for the
selected algorithm, rejecting curve/type or JWK `alg` confusion before signing.
It cannot prove the external signer owns the corresponding private key; the
issuer-side verification delivery establishes that evidence.

### D5 — separate the verifier delivery

The second #99 PR will parse claims, apply caller-provided audience/client/
nonce/time/replay policy, select an inline or injected certificate key, or
dereference a DID URL under the `authentication` relationship before invoking
the suite registry. Keeping that work separate avoids introducing the DID edge
or pretending builder conformance proves verification.

## Risks and trade-offs

- Strictly rejecting `key_attestation` and `trust_chain` covers the common
  baseline rather than every optional Final feature. Their validation and trust
  semantics require a dedicated follow-up contract.
- X.509 strings are structurally bounded and decoded but not parsed as
  certificates. A later verifier must use an injected chain-validation
  capability and bind the leaf public key to the proof signature.
- Owned key references clone public data. The maximum list and byte bounds keep
  allocations deterministic; borrowing would make the prepared signing input
  self-referential or expose lifetime-heavy public APIs.
- The existing algorithm set is intentionally smaller than the JOSE registry.
  Unsupported algorithms fail before signing and can be added only by a later
  explicit cryptographic capability decision.

## Rollback and migration

The API is additive and unreleased. Reverting removes the profiled builder and
new header variants while leaving every prior constructor and wire value
unchanged. Oxid and Lace adoption are separate downstream changes against a
future immutable SDK candidate.
