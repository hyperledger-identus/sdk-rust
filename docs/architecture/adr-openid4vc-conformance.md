# ADR: OpenID4VC Conformance Boundary

**Status**: Accepted

**Date**: 2026-06-13

## Context

`sdk-rust` must support OpenID4VC as a core SSI protocol family, not as
wrapper-specific HTTP glue. Existing Identus components and SDKs already expose
issuance, proof, verification, and integration-runner behavior that must be
portable across Rust, TypeScript, Swift, Kotlin, WASM, Node, and mobile
wrappers.

The official OpenID source baseline checked for this ADR is:

| Specification | Status | Source |
|---|---|---|
| OpenID for Verifiable Credential Issuance 1.0 | Final, published 2025-09-16 | https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0.html |
| OpenID for Verifiable Presentations 1.0 | Final, published 2025-07-09 | https://openid.net/specs/openid-4-verifiable-presentations-1_0-final.html |
| Self-Issued OpenID Provider v2 | Draft 13, published 2023-11-28 | https://openid.net/specs/openid-connect-self-issued-v2-1_0.html |
| OpenID4VC High Assurance Interoperability Profile 1.0 | Final, published 2025-12-24 | https://openid.net/specs/openid4vc-high-assurance-interoperability-profile-1_0.html |
| OpenID Federation 1.0 | Final, published 2026-02-17 | https://openid.net/specs/openid-federation-1_0.html |

## Decision

`identus-openid4vc` owns protocol state machines and typed DTOs for issuance,
presentation, SIOPv2, HAIP, and OpenID Federation-backed trust entry points.
It must depend on ports from `identus-credentials`, `identus-presentations`,
`identus-trust`, `identus-wallet`, `identus-crypto`, and `identus-did` instead
of duplicating credential, trust, key, DID, or storage behavior.

HTTP, browser invocation, deep-link, QR, Digital Credentials API, device, and
service adapters belong in `identus-adapters` or wrapper crates. They may not
own protocol semantics.

## Roles

The conformance model must represent these roles explicitly:

- Credential issuer.
- Wallet.
- Holder.
- Verifier.
- OAuth authorization server.
- Trust anchor.
- Federation entity.
- Status or revocation service.

## OID4VCI Coverage

The first conformance boundary for OpenID for Verifiable Credential Issuance
must cover:

- Credential issuer metadata and credential configuration metadata.
- Credential offer by value and by reference.
- Authorization code flow.
- Pre-authorized code flow.
- Pushed authorization request when configured.
- Nonce endpoint.
- Token endpoint.
- Credential endpoint and holder binding proof.
- Deferred credential endpoint.
- Notification endpoint.
- Encrypted credential requests and responses.
- Credential formats: JWT VC, SD-JWT VC, W3C VC JSON, AnonCreds bridge,
  OpenBadges profile, and mdoc where supported.

Negative conformance must cover expired metadata, wrong issuer, bad audience,
wrong nonce, wrong state, unsupported credential format, unsupported proof type,
invalid request object, invalid holder binding, missing authorization server,
invalid encryption parameters, deferred transaction failure, and trust-chain
rejection.

## OID4VP And SIOPv2 Coverage

The first conformance boundary for OpenID for Verifiable Presentations and
SIOPv2 must cover:

- Same-device flow.
- Cross-device flow.
- Request URI by value, by reference, and `post`.
- Response modes `direct_post` and `direct_post.jwt`.
- Request object validation.
- Client identifier prefixes and verifier metadata.
- Wallet metadata.
- Verifier attestation JWT.
- DCQL credential, credential set, claims, and trusted authorities queries.
- Presentation Exchange compatibility while DCQL adoption matures.
- VP token validation.
- Encrypted responses.
- SIOPv2 self-issued ID token flow.
- SIOPv2 subject syntax types, including DID subject syntax where applicable.

Negative conformance must cover bad request object signature, wrong audience,
wrong nonce, wrong state, verifier metadata mismatch, unsupported response
mode, missing holder binding, unsupported DCQL query, invalid claims path,
untrusted verifier, invalid encrypted response parameters, and trust-chain
rejection.

## HAIP Coverage

The High Assurance Interoperability Profile conformance boundary must cover:

- OID4VCI profile requirements.
- OID4VP redirect flow requirements.
- OID4VP Digital Credentials API requirements.
- SD-JWT VC profile requirements.
- ISO mdoc profile requirements.
- Wallet attestation.
- Key attestation.
- Digital signature requirements.
- Hash algorithm requirements.
- Browser and OS support requirements as adapter-gated capabilities.

HAIP tests must be opt-in until the necessary credential format, key
attestation, device, and browser adapters exist.

## Federation And Trust Coverage

OpenID Federation-backed trust must be modeled through `identus-trust` ports and
OpenID4VC state machines must consume those ports. Conformance must cover:

- Entity statements and entity configurations.
- Trust chain resolution and validation.
- Metadata policy application.
- Trust marks and trust mark status.
- Federation historical keys.
- Federation endpoint error responses.
- Trust-chain expiration and transient validation errors.

OpenID Federation support is required for roadmap trust scenarios, but it must
not block Docker-free OID4VCI/OID4VP transcript tests.

## Fixture Strategy

Fixtures are added in this order:

1. Static-model fixtures for typed request, response, metadata, and error DTOs.
2. Vector fixtures for signed request objects, metadata, DCQL, holder binding
   proofs, SD-JWT VC, mdoc, entity statements, and trust chains.
3. Transcript fixtures for OID4VCI authorization code, OID4VCI pre-authorized
   code, OID4VCI deferred credential, OID4VP same-device direct post, OID4VP
   cross-device direct post, SIOPv2, HAIP, and federation-backed trust.
4. Interop fixtures for official or partner conformance suites.
5. Infrastructure fixtures for real HTTP, browser, device, OIDC provider,
   federation, and trust anchor deployments.

Default `cargo test --workspace` must use static-model, vector, and transcript
fixtures only. Infrastructure fixtures remain opt-in.

## State Machines And Ports

`identus-openid4vc` must expose state machines behind ports:

- `CredentialIssuanceFlow`.
- `CredentialIssuanceFlowKind`, `CredentialIssuanceState`,
  `CredentialIssuanceEvent`, `CredentialIssuanceTransition`, and
  `CredentialIssuanceStateMachine` for authorization-code,
  pre-authorized-code, and deferred credential issuance.
- `PresentationFlow`.
- `PresentationFlowKind`, `PresentationState`, `PresentationEvent`,
  `PresentationTransition`, and `PresentationStateMachine` for same-device
  `direct_post`, cross-device `direct_post.jwt`, and wrong-nonce rejection.
- `SelfIssuedOpenIdProviderFlow`.
- `SelfIssuedOpenIdProviderState`, `SelfIssuedOpenIdProviderEvent`,
  `SelfIssuedOpenIdProviderTransition`, and
  `SelfIssuedOpenIdProviderStateMachine` for `SIOPv2` self-issued ID token
  request and response transitions.
- `HighAssuranceProfileFlow`.
- `FederationTrustFlow`.

The state machines must use:

- `CredentialFormatRegistry` from `identus-credentials`.
- `PresentationQueryEngine` from `identus-presentations`.
- `TrustPolicyResolver` from `identus-trust`.
- `SecureStore`, `KeyStore`, and `SecretResolver` from `identus-wallet`.
- `DidResolverPort` from `identus-did`.
- `SignerPort` and `KeyAgreementPort` from `identus-crypto`.
- HTTP, QR, deep-link, browser, and device adapters from `identus-adapters`.

## Conformance Gates

No wrapper may claim OpenID4VC replacement parity until:

- The relevant flow has metadata tests in `identus-conformance`.
- Fixtures are checked in with source evidence and redaction policy.
- Positive and negative tests run without Docker for the default path.
- Opt-in infrastructure tests exist for real HTTP and device/browser flows when
  the feature requires them.
- Error rendering is redaction-safe.
- Credential, presentation, trust, DID, crypto, wallet, and adapter ports are
  used instead of protocol-level duplication.

## Consequences

- OpenID4VC behavior becomes portable across server, mobile, browser, and
  wrapper targets.
- The Rust core can replace language SDK protocol semantics while wrappers own
  invocation and platform integration.
- SIOPv2 remains clearly marked as draft-based until a final source supersedes
  it.
- HAIP and Federation are first-class roadmap conformance targets without
  blocking the default Docker-free test path.
