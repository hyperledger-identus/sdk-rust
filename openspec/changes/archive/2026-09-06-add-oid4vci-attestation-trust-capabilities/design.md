# Design: OID4VCI attestation and trust-chain capabilities

## Context and source audit

Issue #104 advances `IDR-004` from
`develop@25e388b4ed62a7dac98b140855f31d974c9cff11`. OpenID for Verifiable
Credential Issuance 1.0 Final Appendix D.1 defines the
`key-attestation+jwt` claims and Appendix F.1 defines `key_attestation` and
`trust_chain` protected parameters on `openid4vci-proof+jwt`. The proof must be
signed by an attested key. A trust chain used for proof signature verification
requires `kid`. OpenID Federation 1.0 Final section 4 defines trust-chain
construction and verification; trust anchors are configured out of band.

No donor code or fixture is copied. Read-only inspection found no reusable
implementation in the authorized Rust consumers:

- `MediaNoxLabs/oxid@bfe3b481568dc738f0732c2b27548fab8721fd95`
  rejects unsupported proof key headers and has no attestation/trust-chain port.
- `input-output-hk/lace-id-portal@804de0a9e58cf48ece3cc6c24b2245bb70bc80f1`
  rejects embedded key material in its current issuer proof boundary.
- `midnightntwrk/midnight-identity@427f8571950c42967a18726cbcbefecc19ef8d79`
  and `input-output-hk/neoprism@d6ad1ecade80757f08da4f9101d14c2fb1a4d02b`
  contain no matching OID4VCI attestation behavior.

The standards therefore define the contract. Consumer code is evidence of the
current gap only.

## Decisions

### D1 — retain opaque bounded evidence at the JOSE boundary

`ProtectedHeader` gains optional key-attestation compact text and a leaf-first
trust-chain array. The codec checks member uniqueness, JSON shape, visible
ASCII compact-token shape, per-string limits, chain depth and the existing
complete header limit. It neither parses nested JWT claims nor assigns trust.
Unknown members, `crit` and `b64` continue to fail closed.

### D2 — preserve existing construction while adding explicit evidence

`Oid4vciProofJwtEvidence` validates optional evidence once. Existing
`prepare` delegates to an evidence-free value; `prepare_with_evidence` emits
the additional protected members. A proof carrying `trust_chain` must select a
string `kid`; `jwk` and `x5c` combinations are rejected as ambiguous for this
SDK profile. `key_attestation` can accompany any existing proof key reference.

### D3 — federation trust selects exactly one proof key

When a parsed proof contains `trust_chain`, the verifier invokes one injected
`Oid4vciTrustChainKeyProvider` with the protected algorithm, exact `kid` and
bounded chain. The provider owns entity-statement parsing, signatures, chain
topology, time, metadata policy, trust marks, anchor selection and revocation
or freshness policy. It returns only the trusted public JWK selected by `kid`.
The SDK re-binds that key to the outer protected algorithm and verifies the
exact proof signing input. It performs no DID fallback, ambient trust lookup or
network access.

### D4 — key attestation is checked after proof of possession

After the outer signature verifies, one injected
`Oid4vciKeyAttestationValidator` receives the bounded attestation, exact
verified proof key and optional proof nonce. The provider owns nested JWT
parsing, algorithm allowlisting, signature/key-source validation, trust
anchors, time, expiration, status and assurance policy. Success asserts that
the exact proof key is present in `attested_keys` and that a supplied nonce
matches. Running this after proof verification avoids expensive trust work for
an attacker who cannot demonstrate possession of the selected key.

### D5 — distinguish syntax, cryptography, trust and authorization

Parsing yields `Oid4vciParsedProofJwt`; signature verification yields
`Oid4vciVerifiedProofJwt`; evidence validation yields
`Oid4vciTrustedProofJwt`; issuer claim/freshness/replay policy yields
`Oid4vciAuthorizedProofJwt`. Existing `authorize` remains source-compatible by
performing the trust transition internally before replay. Callers that need
observable stages can call `validate_trust` and `authorize_trusted` directly.

### D6 — keep providers bounded, static and portable

Both new ports are object-safe asynchronous traits with redaction-safe static
failure classes. All attacker-controlled bytes are bounded before either call;
one attempt invokes each applicable provider at most once. The crate adds no
executor, HTTP, certificate, federation, storage, chain or product dependency
and stays safe Rust under the existing native, mobile and browser-WASM gates.

## Risks and trade-offs

- Opaque nested JWTs cannot receive structural claim errors until the injected
  provider runs. This avoids coupling the generic SDK to platform trust stacks
  or federation algorithms while retaining strict outer allocation bounds.
- Requiring `kid` whenever an outer `trust_chain` is present is narrower than
  using the chain only as unrelated metadata. It prevents an ambiguous key
  source and matches the Final rule for signature verification.
- A successful provider is part of the caller's trusted computing base. The
  SDK independently checks returned-key algorithm binding and the outer proof
  signature, and requires key-attestation membership in the provider contract.
- Existing `authorize` becomes capable of an additional async provider call
  only for proofs that opt into key attestation; extension-free behavior is
  unchanged.

## Rollback and migration

All APIs and wire members are additive and unreleased. Reverting removes the
evidence type, provider ports and trusted state. No consumer, stored record,
release or migration is involved.
