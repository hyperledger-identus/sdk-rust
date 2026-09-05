# Design: OID4VCI proof JWT issuer verifier

## Context and source audit

Issue #99 advances `IDR-004` from
`develop@17ae03c05bebb7dc2c728932781d2ad6c06e6413`. OpenID for Verifiable
Credential Issuance 1.0 Final Appendix F.1 requires exact proof typing,
asymmetric algorithm selection, one `kid`, public `jwk`, or `x5c` signing-key
reference, required `aud` and integer `iat`, optional flow-dependent `iss`, and
optional server-provided `nonce`. Appendix F.4 requires the issuer to verify
the referenced key, signature, private-key exclusion, algorithm policy, nonce
and acceptable creation-time window. Section 13.8 leaves nonce lifetime and
replay acceptance to issuer policy and permits bounded future clock skew.

The current `identus-jose` surface provides bounded compact parsing, a closed
exclusive key-reference header, exact unverified state, a caller-owned
signature-suite allowlist and cryptographically verified JWS state. The
current `identus-did` surface provides a runtime-neutral `DidUrlDereferencer`
and a generic adapter that can require exact verification-relationship
membership before returning a verification method. `identus-core` already
owns the object-safe `WallClock` port.

No donor code or fixture is copied. Read-only behavior was inspected at:

- `MediaNoxLabs/oxid@bfe3b481568dc738f0732c2b27548fab8721fd95`,
  Apache-2.0, especially `crates/adapters/openid4vci/src/lib.rs`; it proves the
  need for strict type/algorithm/key/nonce/time checks and DID authentication
  authorization but currently implements them product-locally.
- `input-output-hk/lace-id-portal@804de0a9e58cf48ece3cc6c24b2245bb70bc80f1`,
  especially `openspec/explorations/2026-07-28-holder-proof-of-possession-gap.md`;
  its deferred JWT verification is negative security evidence only.
- `midnightntwrk/midnight-identity@427f8571950c42967a18726cbcbefecc19ef8d79`
  and `input-output-hk/neoprism@d6ad1ecade80757f08da4f9101d14c2fb1a4d02b`;
  both demonstrate chain-owned DID resolution projected into generic
  authentication relationships and public JWK material.
- `hyperledger-identus/identus-apollo@ccee22bcd693e618b9b8ff3e15ed6f9c9156c27c`
  as legacy crypto provenance only; no verifier behavior or code is imported.

## Decisions

### D1 — expose three proof states

`Oid4vciParsedProofJwt` owns a bounded `UnverifiedCompactJws` and validated
profile claims but asserts neither signature validity nor policy acceptance.
`Oid4vciVerifiedProofJwt` is constructible only after the signature-suite
registry verifies the exact received bytes with the key selected and bound by
the protected header. `Oid4vciAuthorizedProofJwt` is constructible only after
client, audience, nonce, clock/freshness and replay policy all accept the
verified proof.

The convenience verifier composes these stages without erasing them. Callers
may inspect intermediate states, but their names, documentation and Debug
representations make the missing guarantees explicit.

### D2 — parse a narrow profile without rejecting unrelated JWT claims

The verifier reuses `UnverifiedCompactJws::parse`, then requires exact `typ`,
exactly one existing key reference and an accepted algorithm spelling. A
custom claims visitor recognizes `iss`, `aud`, `iat` and `nonce`, rejects
duplicates and wrong types, requires `aud` and `iat`, validates the same string
bounds as the holder builder, and skips unknown bounded payload members
without materializing them. This follows the Final profile's "contains"
language without turning arbitrary extension claims into public API.

Profile and caller-policy inputs are validated before any DID, certificate,
signature or replay provider is called. Errors carry only static classes.

### D3 — bind each key-reference form exactly once

An inline public JWK is bound directly to the protected algorithm and passed
to the existing registry.

A `kid` is accepted by this slice only when it is a syntactically valid DID URL
with a fragment and no path/query selector. The verifier calls the injected
`DidUrlDereferencer` once with `verificationRelationship=authentication`,
requires returned verification-method content with the exact requested ID and
a public `publicKeyJwk`, converts it to the SDK public JWK type, binds it to the
header algorithm, and verifies the signature. Non-DID identifiers and
multibase-only methods fail as unsupported inputs; generic non-DID key lookup
and multibase conversion require separate contracts.

For `x5c`, an object-safe asynchronous provider receives the already bounded
encoded chain and protected algorithm. Success returns an owned validated
public JWK for the leaf certificate. The SDK re-checks algorithm/key
compatibility and verifies the exact JWS signature. The provider contract owns
certificate parsing, leaf-key extraction, chain/path/time/revocation checks,
trust anchors and any I/O; the SDK selects no ambient platform trust store.

### D4 — make issuer policy explicit and deterministic

`Oid4vciProofJwtPolicy` owns a validated expected client mode, exact audience,
exact nonce mode, maximum proof age and allowed clock skew. Identified clients
require exact `iss`; anonymous pre-authorized clients require its absence.
Nonce policy is either an exact required server value or explicit absence, so
an unvalidated unexpected nonce is never silently accepted.

Authorization reads the injected `WallClock` once. Integer `iat` must be
non-negative, no later than `now + skew`, and no older than
`max_age + skew`; all conversions and arithmetic are checked. Clock errors are
static and do not expose time or claim values.

### D5 — keep replay acceptance atomic and caller-owned

An object-safe asynchronous replay guard receives a borrowed redaction-safe
input only after the proof is profile-valid, key-bound, signature-valid and
claim-policy-valid. Its `accept` operation is documented as the caller's
atomic check-and-record boundary and returns accepted, rejected or unavailable.
The input exposes the bounded proof and validated claim metadata through
accessors so a consumer can choose a nonce-, proof- or transaction-scoped key;
its Debug implementation exposes neither values nor cryptographic bytes.

No permissive SDK default is provided. The provider may deliberately permit
reuse according to issuer policy, but that decision remains explicit and
testable.

### D6 — keep async at injected boundaries only

The verifier uses native futures returned by existing or new object-safe
traits and introduces no executor or async framework. Inline JWK work is
synchronous internally. One verification attempt invokes at most one DID or
certificate provider, one signature suite and one replay guard. Parsing and
policy rejection invoke none.

## Risks and trade-offs

- Requiring a DID URL fragment and JWK material intentionally rejects generic
  `kid` and multibase DID methods. This is narrower than JOSE but matches the
  current Midnight/NeoPRISM/Oxid integration seam and fails safely.
- A successful `x5c` provider is part of the caller's trusted computing base.
  The SDK proves signature binding to the returned leaf key but cannot prove
  that the provider selected suitable trust anchors or revocation policy.
- Requiring explicit nonce absence is stricter than accepting an unvalidated
  optional nonce. Callers with another server-provided nonce channel can pass
  that exact value as required.
- Replay persistence can involve I/O, so the final transition is asynchronous.
  Manual boxed futures preserve MSRV and runtime neutrality at the cost of a
  slightly more verbose provider implementation.
- The verifier adds the accepted inward DID dependency to `identus-jose`.
  Holder construction still invokes no DID code, but consumers compile the DID
  domain crate with JOSE until a later feature-boundary decision has two real
  consumers.

## Rollback and migration

The API and dependency edge are additive and unreleased. Reverting removes the
issuer verifier, provider ports and new evidence without changing compact JWS,
signature capabilities or holder-generated wire values. Oxid, Lace,
midnight-identity and NeoPRISM adoption remain separate downstream work against
an immutable SDK candidate.
