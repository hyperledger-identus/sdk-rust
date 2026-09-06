# OID4VCI proof JWT capability

## ADDED Requirements

### Requirement: Holder evidence construction is explicit and bounded

The holder builder SHALL accept an optional validated evidence value containing
one `key_attestation` compact JWT and/or one `trust_chain` of 1 through 8
compact entity-statement JWTs. It SHALL validate token shape and all configured
string, header and compact bounds before invoking a signer. A trust-chain proof
SHALL use `kid` as its only proof key reference. Existing evidence-free
construction SHALL retain its API and wire output.

#### Scenario: an attested holder proof is constructed without trust claims

- **WHEN** a holder supplies bounded evidence, a compatible proof key and valid
  claims
- **THEN** preparation includes the exact evidence in the protected signing
  input while the resulting proof makes no validation or trust claim

#### Scenario: invalid evidence never reaches the signer

- **WHEN** evidence is empty, malformed, too deep, too large, or a trust chain
  is combined with `jwk` or `x5c`
- **THEN** preparation fails through a static error and a recording signer
  observes zero calls

### Requirement: Parsed evidence remains explicitly untrusted

OID4VCI proof parsing SHALL retain bounded `key_attestation` and `trust_chain`
values while invoking no key, signature, attestation, federation, clock or
replay provider. A trust chain SHALL require an exact non-empty `kid` key
reference; other trust-chain key-reference combinations SHALL fail closed.

#### Scenario: parsing exposes evidence without elevating state

- **WHEN** a syntactically valid proof contains either or both evidence members
- **THEN** the parsed state exposes their bounded values and remains explicitly
  signature-unverified and trust-unvalidated

### Requirement: Trust-chain proof keys come from one injected capability

When `trust_chain` is present, signature verification SHALL invoke exactly one
caller-owned trust-chain provider with the protected algorithm, exact `kid` and
bounded chain. The provider SHALL return a trusted public JWK only after it has
validated entity statements, signatures, topology, time, metadata policy and a
caller-selected trust anchor. The SDK SHALL re-bind the returned key to the
outer algorithm and verify the exact proof signature. Missing, rejecting or
unavailable providers SHALL fail closed without DID fallback, ambient trust or
network access.

#### Scenario: a trusted federation key verifies the outer proof

- **WHEN** the provider accepts the chain and returns the exact public key
  selected by `kid`
- **THEN** the SDK binds that key to the protected algorithm and only a valid
  signature can produce cryptographically verified proof state

#### Scenario: algorithm or key confusion fails independently

- **WHEN** the provider returns a key incompatible with the protected algorithm
  or a different key whose signature does not verify
- **THEN** verification fails through the applicable static key or signature
  class without trying DID, X.509 or another trust chain

### Requirement: Key attestation is bound after proof of possession

When `key_attestation` is present, trust validation SHALL invoke exactly one
caller-owned attestation validator only after the outer proof signature is
valid. It SHALL receive the bounded nested token, exact verified proof public
key and optional proof nonce. Provider success SHALL mean the nested JWT type,
algorithm, signature, key source, trust, time, expiration, status and caller
policy are accepted; the exact proof key occurs in `attested_keys`; and a
supplied proof nonce matches the attestation nonce. Missing, rejecting or
unavailable providers SHALL fail closed.

#### Scenario: untrusted attestation cannot elevate proof state

- **WHEN** a parsed or signature-verified proof carries an attestation that has
  not been accepted by the injected validator
- **THEN** no trust-evaluated or issuer-authorized proof state can be produced

#### Scenario: invalid proof signatures avoid attestation work

- **WHEN** the outer proof signature is invalid for its selected key
- **THEN** verification fails and a recording attestation validator observes
  zero calls

### Requirement: Proof states separate syntax, cryptography, trust and policy

The capability SHALL expose distinct parsed, cryptographically verified,
trust-evaluated and issuer-authorized proof types. Trust evaluation SHALL be an
explicit callable transition. Existing authorization methods SHALL remain
source compatible and SHALL internally require that transition before replay
acceptance. Extension-free proofs MAY transition without an external trust
call. Debug, Display and errors SHALL omit nested tokens, chains, identifiers,
keys, claims, nonces, timestamps, provider details and compact bytes.

#### Scenario: every guarantee is observable in the type system

- **WHEN** a caller processes a proof in stages
- **THEN** it cannot obtain the next state until the exact preceding signature,
  trust or issuer-policy gate succeeds

#### Scenario: optional capabilities remain portable and bounded

- **WHEN** the crate compiles for Rust 1.85, native, Android, iOS and browser
  WASM targets
- **THEN** the capability adds no executor, HTTP, certificate, federation,
  storage, chain, product or ambient-platform dependency
