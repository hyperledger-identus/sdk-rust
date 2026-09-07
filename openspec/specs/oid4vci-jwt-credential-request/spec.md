# oid4vci-jwt-credential-request Specification

## Purpose
TBD - created by archiving change add-oid4vci-jwt-credential-request. Update Purpose after archive.
## Requirements
### Requirement: Final JWT Credential Request construction is state-bound and bounded

The SDK SHALL expose positive `JwtCredentialRequestLimits` with independent
maximums for proof count, one compact proof byte length, the complete JSON body
byte length, and the complete Authorization field-value byte length. Every
maximum SHALL be non-zero; invalid limits SHALL fail with one fieldless static
invalid-limits error. Defaults SHALL be finite.

`CredentialOfferWithMetadata::try_create_jwt_credential_request` SHALL borrow
that matched state, one `TokenResponseCore`, a zero-based offered Credential
Configuration index, a non-empty ordered slice of
`identus_jose::Oid4vciProofJwt`, and the limits. It SHALL succeed only when the
index selects an ID from the matched offer, the Token Response has no
unvalidated Authorization Details, its token type is case-insensitively
`Bearer`, its exact access token matches RFC 6750 `b64token`, the proof list and
every compact proof are within bounds, and the resulting authorization/body
values are within bounds.

#### Scenario: validated states produce one request

- **WHEN** matched offer/metadata, a Bearer Token Response without Authorization
  Details, one valid offered index and bounded holder-produced JWT proofs are
  supplied
- **THEN** construction returns one owned request for the selected offered
  configuration while preserving proof order

#### Scenario: selection is not a raw caller claim

- **WHEN** the offered index is outside the validated offer list
- **THEN** construction fails before copying token or proof material and cannot
  emit a request for an unoffered configuration

#### Scenario: incompatible token route or syntax fails closed

- **WHEN** the Token Response contains Authorization Details, advertises a
  non-Bearer token type, or contains an access token outside the RFC 6750
  Bearer grammar
- **THEN** construction fails with the corresponding static state/type error
  and does not guess Credential identifiers or token presentation syntax

#### Scenario: proof and allocation limits are independent

- **WHEN** limits are zero, the proof list is empty or excessive, one proof is
  oversized, or the final authorization/body value is oversized
- **THEN** construction returns the corresponding fieldless limit error without
  returning a partial request

### Requirement: Final JWT Credential Request wire output is minimal and deterministic

`JwtCredentialRequest` SHALL own an independently duplicated validated HTTPS
Credential Endpoint, a zeroizing Authorization field value and a zeroizing JSON
body. It SHALL expose `POST`, `application/json`, endpoint, proof count and exact
authorization/body byte lengths through non-sensitive accessors. The exact
Authorization field and JSON body SHALL be available only through explicitly
sensitive accessors with storage/logging guidance.

The Authorization value SHALL be the canonical ASCII prefix `Bearer ` followed
by the exact access token. The body SHALL be deterministic compact JSON with
exactly `credential_configuration_id` followed by `proofs`, whose only member
is `jwt`, a non-empty array of exact compact proof strings in caller order.
Serde JSON escaping SHALL prevent member/value injection without normalizing
the selected configuration ID or compact proofs.

#### Scenario: consumer-shaped request has exact transport metadata

- **WHEN** one offered configuration and one or more proofs are constructed
- **THEN** endpoint, method, media type, Authorization value and compact JSON
  exactly match the Final configuration-ID/JWT-proof request shape

#### Scenario: proof order and JSON escaping are stable

- **WHEN** several proofs and a configuration ID requiring JSON escaping are
  encoded within limits
- **THEN** parsing the body yields the exact values in order and repeated
  construction yields byte-identical JSON

#### Scenario: request diagnostics remain redacted

- **WHEN** request `Debug` and every direct/bridged error are formatted with
  unique token, proof, configuration and endpoint canaries
- **THEN** no canary, body, Authorization value or parser/serializer cause is
  present in diagnostics

### Requirement: Credential Request construction remains policy-neutral and portable

Success SHALL prove only that already-validated local states were combined into
the bounded unencrypted Final configuration-ID/JWT-proof wire shape. It SHALL
NOT prove actual request execution, endpoint provenance/reachability/trust,
DNS/TLS/redirect/private-network safety, token validity/freshness/scope, DPoP,
proof audience/nonce/time/signature/trust/replay correctness, metadata proof
requirements, Credential identifier or Authorization Details semantics,
format/chain extensions, response validity, consent or retry policy.

The request SHALL expose no `Clone`, `Display`, Serde, FFI, generic header map,
HTTP executor or raw mutable secret bytes. The only new dependency SHALL be the
existing local `identus-jose` crate in the architecture-approved direction.
No feature, external dependency, unsafe code, consumer, chain or product
mutation SHALL occur, and Rust 1.85, browser-WASM, Android ARM64 and iOS ARM64
portability SHALL remain green.

#### Scenario: holder proof capability remains separate

- **WHEN** a caller supplies an `Oid4vciProofJwt`
- **THEN** the protocol request preserves its compact representation but does
  not create, verify, authorize, trust or freshness-check the proof

#### Scenario: headless adapter retains transport authority

- **WHEN** a native, mobile or browser adapter borrows the sensitive request
  values
- **THEN** the adapter still owns network, header placement, logging controls,
  token/proof lifecycle and all response processing

#### Scenario: portable dependency gates remain green

- **WHEN** focused, workspace, factory, target/MSRV, supply-chain and full Nix
  gates run
- **THEN** the request passes with only the declared local JOSE dependency edge
  and no feature, target or downstream drift
