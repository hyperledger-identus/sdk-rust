# oid4vci-proof-jwt Specification

## Purpose

Define portable holder construction of the OpenID4VCI 1.0 Final proof JWT with
exclusive bounded key references, explicit claim inputs, external signing and
typed states that make no issuer-verification or trust claim.
## Requirements
### Requirement: Final-profile holder proof construction

The JOSE capability SHALL provide a holder-side builder for the
`openid4vci-proof+jwt` proof defined by OpenID4VCI 1.0 Final Appendix F.1. It
SHALL require one accepted asymmetric algorithm, exactly one key reference, one
non-empty bounded Credential Issuer audience and an explicit integer issuance
time. The emitted protected `typ` SHALL equal `openid4vci-proof+jwt`.

#### Scenario: identified holder input is canonical

- **WHEN** an identified client prepares a proof with a DID URL `kid`, audience,
  issuance time and server nonce
- **THEN** the protected header and claims SHALL contain exactly the selected
  algorithm, proof type, `kid`, `iss`, `aud`, `iat` and `nonce` values and the
  builder SHALL expose the exact bytes supplied to the signer

#### Scenario: anonymous pre-authorized input omits issuer

- **WHEN** an anonymous pre-authorized client prepares an otherwise valid proof
- **THEN** the claims SHALL omit `iss` by construction rather than serialize a
  null, empty or inferred client identifier

### Requirement: Exclusive bounded proof key references

The protected-header model SHALL represent at most one `kid`, public `jwk` or
non-empty `x5c` chain. It SHALL preserve the existing `ProtectedHeader::new`
`kid` API, reject ambiguous wire input and unknown members, reject private JWK
material, and bound an X.509 chain to at most eight non-empty standard-base64
entries under the configured header and complete-token byte ceilings.

#### Scenario: each standard reference has one wire shape

- **WHEN** callers construct otherwise identical headers using `kid`, public
  JWK and X.509 chain references
- **THEN** each SHALL serialize and parse as its one matching JOSE member with
  no second key-reference member present

#### Scenario: ambiguous or unsafe references fail closed

- **WHEN** a header contains multiple reference members, an empty or excessive
  certificate chain, malformed certificate base64, a private JWK, or an unknown
  member
- **THEN** parsing or construction SHALL fail with a static error before any
  signer is invoked

### Requirement: Claims and resource bounds precede signing

Issuer, audience and nonce strings SHALL be non-empty, control-free and no
larger than a positive caller-selected claim-string ceiling. Header, payload,
signature and complete compact bounds SHALL reuse `JwsLimits`. Every locally
detectable invalid input, unsupported key/algorithm combination or impossible
fixed signature size SHALL fail before external signing.

#### Scenario: attacker-controlled claims cannot trigger a provider

- **WHEN** any claim or key reference violates its bound or the resulting
  compact value cannot contain the fixed signature
- **THEN** preparation or signing preflight SHALL fail and a recording external
  signer SHALL observe zero calls

### Requirement: Typed staged proof state

Preparation SHALL return a distinct OID4VCI signing-input type. Only successful
use of an algorithm-matched `JwsSigner` SHALL produce the signed OID4VCI proof
type. Neither type SHALL claim signature verification, DID authorization,
freshness, replay acceptance or trust, and Debug/errors SHALL not expose claim,
key, certificate, compact, payload or signature values.

#### Scenario: signed is not verified

- **WHEN** an accepted signer returns a signature over the exact prepared bytes
- **THEN** the result SHALL expose the compact proof for transport while its
  type and documentation SHALL make no issuer-verification or authorization
  claim

### Requirement: Portable profile boundary

The holder builder SHALL remain synchronous, safe Rust, runtime-neutral,
chain-neutral and product-neutral. It SHALL use only existing inward JOSE and
crypto capabilities, introduce no DID, HTTP, storage, clock, custody or
consumer dependency, and compile under the repository Rust 1.85 and portable
target matrix.

#### Scenario: consumer evidence remains isolated

- **WHEN** Oxid-shaped anonymous/DID-key behavior and Lace's deferred-proof gap
  inform conformance cases
- **THEN** the cases SHALL be independently reconstructed without modifying or
  importing either consumer repository

### Requirement: Bounded Final-profile issuer parsing

The JOSE capability SHALL parse the OpenID4VCI 1.0 Final
`openid4vci-proof+jwt` issuer profile through the existing `JwsLimits` and
`Oid4vciProofJwtLimits`. It SHALL require exact protected `typ`, one accepted
asymmetric algorithm, exactly one `kid`, public `jwk`, or `x5c` reference, one
non-empty bounded string `aud`, one integer `iat`, and optional bounded string
`iss` and `nonce`. Known claim duplicates, wrong types, missing required
claims, private JWK material and invalid references SHALL fail through static
errors. Unknown claims MAY be skipped but SHALL remain bounded by the complete
payload ceiling and SHALL NOT become trusted public state.

Parsing SHALL produce an explicit profile-parsed but signature-unverified
state and SHALL invoke no resolver, certificate, signature, clock, or replay
provider.

#### Scenario: Final proof shape becomes parsed evidence only

- **WHEN** a bounded compact JWT contains the exact proof type, one key
  reference, required audience and integer issuance time plus optional issuer,
  nonce and unrelated extension claims
- **THEN** parsing SHALL retain the exact compact signing input and recognized
  claims in a parsed state without asserting signature or policy acceptance

#### Scenario: malformed profile fails before providers

- **WHEN** the JWT has a wrong or absent type, no key reference, duplicate or
  malformed recognized claims, missing audience or issuance time, or an
  unsupported algorithm spelling
- **THEN** parsing SHALL fail and recording DID, certificate, signature, clock
  and replay providers SHALL observe zero calls

### Requirement: Header-selected keys are exactly bound to signatures

An inline `jwk` SHALL be bound to the exact protected algorithm and verified
through the caller-owned `SignatureSuiteRegistry`. A `kid` supported by this
slice SHALL be a DID URL containing an exact verification-method fragment and
no path or query selector. The verifier SHALL call the injected
`DidUrlDereferencer` once with the exact `authentication` verification
relationship, require returned verification-method content whose identifier
equals the `kid`, require a public JWK, bind it to the protected algorithm and
verify the exact received JWS signing input. A non-DID `kid`, multibase-only
method, missing material, mismatched resource, failed dereference or method not
authorized for `authentication` SHALL fail closed.

An `x5c` reference SHALL call at most one injected certificate-key provider
with the already bounded chain and protected algorithm. The provider SHALL
return an owned validated public JWK only after applying its caller-defined
certificate, path, time, revocation and trust-anchor policy. The verifier SHALL
re-bind that key to the protected algorithm and verify the exact signature.
Absence, rejection or unavailability of the provider SHALL fail closed; no
ambient trust store or network SHALL be selected by the SDK.

#### Scenario: DID authentication key verifies exact proof bytes

- **WHEN** a DID URL `kid` dereferences to an exact public-JWK method authorized
  by the subject document's `authentication` relationship and the signature is
  valid for that key and algorithm
- **THEN** verification SHALL produce a cryptographically verified and
  key-reference-bound proof state while making no claim-policy or replay claim

#### Scenario: wrong relationship cannot authorize a valid signature

- **WHEN** a signature is cryptographically valid for a DID method that is
  absent from the exact `authentication` relationship
- **THEN** verification SHALL fail without trying another method, relationship
  or key source and SHALL not construct verified proof state

#### Scenario: certificate provider owns trust while SDK owns binding

- **WHEN** the injected `x5c` provider accepts the bounded chain and returns a
  leaf public JWK compatible with the header algorithm
- **THEN** the SDK SHALL verify the proof signature with that exact key, while
  certificate trust remains an explicit provider result rather than an SDK
  ambient-policy claim

### Requirement: Verification and issuer authorization are separate states

Only successful suite-registry verification with the header-selected key SHALL
construct the cryptographically verified proof type. Only successful client,
audience, nonce, clock/freshness and replay checks over that verified value
SHALL construct the issuer-authorized proof type. Parsed, verified and
authorized types SHALL be distinct and SHALL retain the bounded original proof
without permitting callers to construct a later state directly.

Debug, Display and all errors for these states and their providers SHALL omit
claim strings, DID URLs, certificate values, compact text, payloads,
signatures, replay material and timestamps. Accessors MAY deliberately expose
validated public inputs to the caller that already owns the proof.

#### Scenario: signature validity is insufficient for acceptance

- **WHEN** a proof has a valid signature but its issuer policy has not run or
  rejects one claim
- **THEN** callers can hold only the verified state and cannot obtain an
  issuer-authorized state

#### Scenario: diagnostics do not copy proof canaries

- **WHEN** every claim, key reference, compact segment and replay input carries
  a distinct canary and success or failure values are formatted
- **THEN** none of those canaries or the issuance timestamp SHALL appear in
  Debug, Display or the core error bridge

### Requirement: Issuer claim and freshness policy is explicit

The verifier SHALL require a validated caller-owned policy containing one
expected client mode, exact bounded Credential Issuer audience, exact nonce
mode, maximum proof age and allowed clock skew. Identified mode SHALL require
`iss` to equal the expected OAuth client identifier. Anonymous
pre-authorized mode SHALL require `iss` to be absent. Required-nonce mode SHALL
require exact equality with the expected server value; absent-nonce mode SHALL
reject a supplied nonce.

Authorization SHALL read an injected `WallClock` exactly once. The `iat`
NumericDate SHALL be non-negative, SHALL NOT exceed current whole Unix seconds
plus allowed skew, and SHALL NOT be older than maximum age plus allowed skew.
All conversions and arithmetic SHALL fail closed without saturation or wrap.

#### Scenario: each claim policy fails independently

- **WHEN** client mode, issuer, audience, nonce presence/value, stale issuance
  time or future issuance time differs from the explicit policy
- **THEN** that condition SHALL fail through its static policy class before
  replay acceptance and without substituting a default value

#### Scenario: clock is an injected deterministic input

- **WHEN** the supplied wall clock returns one valid time or a static failure
- **THEN** authorization SHALL use that one observation for both freshness
  bounds or fail without consulting an ambient operating-system clock

### Requirement: Replay acceptance is an atomic final gate

The capability SHALL define an object-safe, runtime-neutral asynchronous replay
guard. After all profile, key, signature and claim-policy checks succeed, the
verifier SHALL invoke its atomic `accept` operation exactly once with a borrowed
bounded input exposing the proof and validated claim/key-reference metadata
through deliberate accessors. Replay rejection or provider unavailability
SHALL fail closed and SHALL NOT construct authorized proof state.

The SDK SHALL provide no implicit permissive guard, storage, retention period,
hash choice or nonce-consumption policy. A caller MAY implement deliberate
reuse, but it SHALL do so through the same explicit acceptance result.

#### Scenario: invalid proofs never consume replay state

- **WHEN** parsing, key selection, signature verification, client, audience,
  nonce, clock or freshness validation fails
- **THEN** a recording replay guard SHALL observe zero calls

#### Scenario: concurrent replay policy has one atomic boundary

- **WHEN** otherwise acceptable proof evidence reaches the replay guard
- **THEN** only a successful caller-owned atomic check-and-record result SHALL
  produce issuer-authorized proof state

### Requirement: Verifier boundary stays portable and observable

The verifier SHALL remain safe Rust, chain-neutral, product-neutral and
executor-neutral. It MAY depend inward on `identus-did`, `identus-crypto` and
`identus-core`, but SHALL add no chain, product, HTTP, storage, certificate,
trust-store or async-runtime dependency. One attempt SHALL be linear in bounded
input and invoke at most one key-source provider, one signature suite, one
clock read and one replay call.

Tests SHALL independently reconstruct the applicable Oxid positive/negative
behavior and Lace deferred-verification gap without importing fixtures or
modifying either consumer. An ignored release diagnostic SHALL report
representative inline-JWK verification throughput without a machine-specific
threshold.

#### Scenario: consumer evidence remains isolated

- **WHEN** Oxid behavior, Lace's deferred verifier gap and Midnight/NeoPRISM
  DID shapes inform the conformance matrix
- **THEN** repository-local tests SHALL reconstruct the behavior from standards
  and public contracts while every downstream working tree remains unchanged

#### Scenario: maintainer observes verifier cost

- **WHEN** the ignored diagnostic repeatedly parses, verifies and authorizes a
  representative proof with deterministic in-memory providers
- **THEN** it SHALL validate every result and print elapsed time plus operations
  per second without a timing assertion

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
