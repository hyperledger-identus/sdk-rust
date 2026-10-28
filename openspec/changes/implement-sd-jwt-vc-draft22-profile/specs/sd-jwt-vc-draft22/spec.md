# sd-jwt-vc-draft22 Specification

## ADDED Requirements

### Requirement: draft-22 identity is exact and independently versioned

The SDK SHALL implement `draft-ietf-oauth-sd-jwt-vc-22` as a separately
versioned profile over RFC 9901. Issuance and verification SHALL require
protected `typ=dc+sd-jwt`; any media type emitted by the profile SHALL be
`application/dc+sd-jwt`. A later draft or RFC SHALL NOT change this capability
without a new reviewed migration.

#### Scenario: canonical profile credential is issued

- **WHEN** valid bounded claims are issued through the draft-22 profile
- **THEN** the issuer JWS contains exact protected type `dc+sd-jwt` and the envelope declares exact format `dc+sd-jwt`

#### Scenario: legacy protected type is received

- **WHEN** the profile receives an otherwise valid credential with protected type `vc+sd-jwt`
- **THEN** it fails as a profile-type mismatch and does not invoke an automatic compatibility path

### Requirement: profile claims and disclosure placement are validated

The SDK SHALL require a bounded collision-resistant `vct`; validate optional
`aka_vcts`, registered claims and numeric dates; and reject duplicate,
malformed or contradictory profile values. `iss`, `nbf`, `exp`, `cnf`, `vct`,
`vct#integrity`, `aka_vcts`, and `status` SHALL NOT be selectively disclosed.
`sub` and `iat` MAY be selectively disclosed. Caller-required cryptographic
key binding SHALL require structurally valid `cnf` evidence.

#### Scenario: protected profile claim was disclosed

- **WHEN** disclosure processing shows that `vct` or another non-selective profile claim came from a Disclosure
- **THEN** profile validation fails without exposing its name or value in diagnostics

#### Scenario: additional credential types are valid

- **WHEN** `aka_vcts` is non-empty, within limits, exact-unique, excludes `vct`, and every value is a valid bounded type identifier
- **THEN** the profile exposes redaction-safe type-match queries for `vct` and every additional type

### Requirement: issuer discovery remains structural evidence, not trust

The SDK SHALL accept bounded caller-supplied evidence for exactly the
draft-22 inline X.509 or JWT VC Issuer Metadata mechanisms. Metadata SHALL
correlate exact `issuer` with the verified `iss` claim and contain exactly one
of `jwks` or `jwks_uri`. The profile SHALL NOT perform HTTP, certificate path
validation, JWK cryptography, trust-anchor selection, key authorization or
issuer policy.

#### Scenario: issuer metadata names a different issuer

- **WHEN** caller-supplied JWT VC Issuer Metadata evidence has an `issuer` different from the verified credential `iss`
- **THEN** the evidence is rejected and cannot advance profile state

#### Scenario: caller supplies validated inline certificate evidence

- **WHEN** the exact protected header carried `x5c` and the caller supplies validated-chain evidence whose chain and extracted public key match the protected header and issuer-signature verification key
- **THEN** the receipt records only the discovery mechanism and not certificate, subject, key or trust-anchor material

### Requirement: Type Metadata inputs are bounded and policy-neutral

The SDK SHALL validate caller-supplied Type Metadata for exact type
correlation, finite acyclic extension chains, unique bounded claim paths,
mandatory rules and `always`/`allowed`/`never` disclosure rules. It SHALL NOT
retrieve metadata, execute rendering content, authorize an issuer, or treat
metadata publication as trust.

#### Scenario: metadata extension chain is cyclic

- **WHEN** bounded Type Metadata inputs revisit a previously processed type
- **THEN** validation fails before further traversal and exposes no type identifier in diagnostics

#### Scenario: undisclosed mandatory claim is absent from a presentation

- **WHEN** presentation-time metadata validation cannot observe a selectively disclosable mandatory claim
- **THEN** the SDK does not infer that the issuer omitted it and leaves acceptance to caller policy

### Requirement: status inputs produce evidence without acceptance policy

The SDK SHALL validate the draft-22 status object and map a bounded
`status_list` mechanism to generic credential-status bindings, requiring JWT
token format. Caller-supplied status evidence SHALL match the exact binding.
The profile SHALL NOT retrieve a list, decompress it, choose freshness, or
decide whether the credential is acceptable.

#### Scenario: status evidence is correlated

- **WHEN** caller-supplied evidence matches the credential's validated status-list binding
- **THEN** the profile records a redaction-safe checked-evidence class without declaring valid, revoked, suspended or accepted status

#### Scenario: status mechanism is unsupported

- **WHEN** a status object names a syntactically valid but unsupported mechanism
- **THEN** profile validation returns a stable unsupported-status category rather than treating it as checked

### Requirement: every profile path has finite resource and privacy bounds

The SDK SHALL apply public byte, element, depth, extension-chain,
claim-path, retained-output and aggregate-work limits before proportional
decode, parse, traversal, comparison or allocation. Public errors, formatted
values, receipts and metrics SHALL NOT contain credential bytes, claims,
identifiers, URLs, metadata content, key material, status handles, audience,
nonce or comparison values.

#### Scenario: oversized metadata is supplied

- **WHEN** raw Type Metadata exceeds its configured byte limit
- **THEN** it is rejected before JSON allocation or extension traversal

#### Scenario: private profile input fails

- **WHEN** a credential, metadata document, issuer evidence or status input is malformed
- **THEN** every returned and formatted diagnostic is independent of its private bytes and values

### Requirement: offline protocol integration uses generic capability seams

The SDK SHALL provide executable offline evidence that a draft-22 credential
can be issued, retained in a generic credential envelope, carried as an opaque
OID4VCI credential response, matched through OID4VP `dc+sd-jwt` format
capabilities, selectively presented, base-verified, profile-validated and
reported through redacted evidence without network, persistence, trust policy
or product runtime.

#### Scenario: complete offline journey succeeds

- **WHEN** deterministic injected effects and caller-supplied issuer, metadata and status evidence drive public SDK APIs
- **THEN** the result proves exact profile/base/protocol stages while retaining all trust, acceptance and transport decisions outside the profile

### Requirement: conformance evidence is source-bound and migration-aware

The implementation SHALL bind draft-22 Appendix A, negative and boundary
vectors to the exact source hash, add property/fuzz/benchmark/differential
evidence, and preserve a deterministic trigger for a later draft or RFC.
SDK-TS legacy evidence SHALL remain migration-only and canonical output SHALL
remain `dc+sd-jwt`.

#### Scenario: the IETF profile changes

- **WHEN** the current IETF draft number, publication state, normative source hash, protected type, media type or claim rules differ from the pinned manifest
- **THEN** the existing capability remains unchanged and a new issue/decision/profile version is required

#### Scenario: an oracle disagrees with draft-22

- **WHEN** an immutable oracle accepts or emits behavior contrary to the pinned draft or SDK resource contract
- **THEN** the divergence is recorded and does not weaken the SDK requirement

