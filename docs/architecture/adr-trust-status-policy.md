# ADR: Trust And Status Policy Boundary

**Status**: Accepted

**Date**: 2026-06-13

## Context

`sdk-rust` must verify credentials, presentations, OpenID4VC flows, mediator
trust decisions, and future service ports without duplicating status and trust
logic across language wrappers. Existing Identus SDKs and services expose
revocation, status, DID trust, and ecosystem trust decisions through several
surfaces. The Rust core needs one policy boundary before credential,
presentation, OpenID4VC, and wrapper APIs stabilize.

The official source baseline checked for this ADR is:

| Specification | Status | Source |
|---|---|---|
| Verifiable Credentials Data Model v2.0 | W3C Recommendation, 2025-05-15 | https://www.w3.org/TR/vc-data-model-2.0/ |
| Bitstring Status List v1.0 | W3C Recommendation, 2025-05-15 | https://www.w3.org/TR/vc-bitstring-status-list/ |
| Token Status List | IETF Internet-Draft draft-ietf-oauth-status-list-20, Standards Track intent, expires 2026-10-22 | https://datatracker.ietf.org/doc/draft-ietf-oauth-status-list/ |
| OpenID Federation 1.0 | Final, published 2026-02-17 | https://openid.net/specs/openid-federation-1_0.html |
| X.509 | ITU-T Recommendation family | https://www.itu.int/rec/T-REC-X.509 |

## Decision

`identus-trust` owns trust and status policy. It exposes typed ports and policy
results consumed by `identus-credentials`, `identus-presentations`,
`identus-openid4vc`, `identus-wallet`, `identus-agent`, and future Rust
services.

Credential, presentation, OpenID4VC, DIDComm, wallet, and adapter crates must
not implement ad hoc revocation, suspension, trust-chain, or ecosystem policy
logic. They may request trust decisions through `identus-trust` ports and must
preserve typed errors.

## Policy Domains

The trust boundary must support:

- Credential status: active, suspended, revoked, unknown, unavailable, and
  unsupported.
- Status purpose: revocation, suspension, message, and extension-defined
  purposes.
- Status mechanisms: W3C Bitstring Status List, IETF Token Status List,
  AnonCreds revocation, OpenID Federation trust marks, X.509/IACA roots, and
  explicit local allow/deny policy.
- Trust anchors: DID roots, PRISM roots, OpenID Federation trust anchors,
  X.509 roots, IACA roots, local enterprise anchors, and test anchors.
- Trust chains: issuer-to-anchor, verifier-to-anchor, wallet-to-anchor,
  federation entity chains, and adapter-provided ecosystem chains.
- Verification policy: required status check, soft-fail status check,
  trust-required, trust-optional, offline-only, and freshness windows.

## Ports

The first stable trust boundary must expose typed ports equivalent to:

- `StatusResolver`: fetch or resolve status material by typed reference.
- `StatusVerifier`: evaluate a credential or presentation status entry against
  policy and verification time.
- `TrustAnchorResolver`: resolve configured trust anchors and local trust
  policy.
- `TrustChainVerifier`: validate chain structure, signatures, expiration,
  metadata policy, and allowed ecosystems.
- `TrustPolicyEngine`: combine credential verification, presentation
  verification, status, issuer trust, verifier trust, wallet trust, and
  ecosystem policy into one decision.
- `TrustEvidenceStore`: cache status lists, trust chains, trust marks, and
  validation metadata with freshness and privacy policy.

Every port must use typed identifiers, typed errors, verification time,
freshness policy, source evidence, owner crate, and redaction-safe diagnostics.

## Status Mechanism Requirements

### W3C Bitstring Status List

Conformance must cover:

- `BitstringStatusListEntry`.
- `BitstringStatusListCredential`.
- Status purpose handling.
- Bitstring expansion and index bounds.
- Revoked, suspended, active, and unknown states.
- Status list credential proof verification through credential ports.
- Privacy requirements for caching and verifier correlation.
- Negative cases for invalid index, invalid bitstring, wrong purpose, expired
  status list, issuer mismatch, tampered status credential, and unsupported
  entry size.

### IETF Token Status List

Token Status List support is roadmap-gated while the source is an
Internet-Draft. Conformance must still define:

- Token status list format parsing.
- Status value mapping to Identus typed status decisions.
- JWT and CWT carrying formats when supported by crypto and credential crates.
- Freshness and cache policy.
- Negative cases for unsupported draft version, invalid list encoding, invalid
  status value, issuer mismatch, expired list, and tampered signature.

### AnonCreds Revocation

Conformance must cover:

- Registry lookup by DID:PRISM and HTTP AnonCreds methods.
- Revocation registry definitions.
- Revocation registry entries.
- Non-revocation interval handling.
- Negative cases for missing registry, timestamp outside interval, revoked
  credential, and unsupported registry method.

### OpenID Federation And Trust Marks

Conformance must cover:

- Entity statements and entity configurations.
- Trust chain resolution and validation.
- Metadata policy application.
- Trust marks and trust mark status.
- Federation historical keys.
- Federation endpoint errors.
- Negative cases for expired chain, untrusted anchor, invalid metadata policy,
  invalid trust mark, and transient fetch failures.

### X.509 And IACA

Conformance must cover:

- Root and intermediate trust anchors.
- Certificate chain validation.
- Extended key usage or profile constraints when credential formats require
  them.
- IACA trust anchors for mdoc.
- Negative cases for expired certificate, unknown root, wrong usage, revoked
  certificate, and unsupported algorithm.

## Privacy And Safety Rules

- Default tests must not call issuer, verifier, wallet, federation, OCSP, CRL,
  ledger, device, or cloud endpoints.
- Status fixtures must use redacted or synthetic identifiers and must not embed
  production credentials, claims, tokens, private keys, or holder identifiers.
- Status fetching must support cache policy, privacy-preserving prefetch, and
  offline verification modes where a specification allows it.
- Errors must never leak credential claims, bearer tokens, raw status list
  payloads, private metadata, or trust-chain secrets.
- Draft-based mechanisms must be versioned and feature-gated until stable.

## Fixture Strategy

Fixtures are added in this order:

1. Static-model fixtures for status references, trust anchors, trust chains,
   policy inputs, policy outputs, and typed errors.
2. Vector fixtures for bitstring lists, token status lists, AnonCreds
   revocation data, OpenID Federation entity statements, trust marks, X.509
   chains, and IACA anchors.
3. Transcript fixtures for trust-chain resolution and status-check workflows
   that do not require external services.
4. Interop fixtures for official or partner conformance suites.
5. Infrastructure fixtures for ledgers, HTTP status endpoints, OCSP, CRL,
   OpenID Federation endpoints, HSM/KMS, cloud trust stores, and device trust
   stores.

Default `cargo test --workspace` must use static-model, vector, and transcript
fixtures only. Infrastructure fixtures remain opt-in.

## Conformance Gates

No credential, presentation, OpenID4VC, wrapper, or service path may claim trust
or status parity until:

- The relevant trust or status mechanism has a conformance catalog entry.
- Fixtures are checked in with source evidence and redaction policy.
- Positive and negative tests run without Docker for the default path.
- Opt-in infrastructure tests exist for any external endpoint or device trust
  dependency.
- Error rendering is redaction-safe.
- Credential, presentation, OpenID4VC, DID, crypto, wallet, and adapter crates
  consume `identus-trust` decisions instead of duplicating policy.

## Consequences

- Trust/status behavior becomes portable across Rust services and language
  wrappers.
- Credential and presentation verification can share typed policy decisions.
- OpenID4VC Federation and HAIP can consume the same trust boundary as status
  checks.
- Draft mechanisms remain useful for planning without destabilizing default
  conformance.
