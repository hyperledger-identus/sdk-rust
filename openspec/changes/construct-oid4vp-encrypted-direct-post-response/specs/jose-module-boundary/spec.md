# jose-module-boundary delta

## MODIFIED Requirements

### Requirement: JOSE owns protocol-neutral compact and signature mechanics

The `identus-jose` crate SHALL own bounded canonical JWS Compact parsing and
encoding, validated protected-header syntax, signature algorithms, public-key
binding, signer/signature-suite ports, and explicit unverified/verified compact
states. It SHALL NOT own OIDC/OID4VCI claims, DID resolution, certificate or
federation trust policy, key-attestation policy, clocks, freshness, replay,
network, storage, wallet, product, or chain behavior.

It SHALL additionally own bounded SDK-owned JWE encryption port types without
owning OID4VP claims or a concrete encryption engine.

#### Scenario: generic consumers reuse JOSE without identity protocols

- **WHEN** a consumer requires bounded compact JWS signing or verification
- **THEN** it can depend on `identus-jose` without bringing a DID or OIDC
  protocol dependency

#### Scenario: bounded header evidence remains untrusted syntax

- **WHEN** a protected header carries a supported bounded extension used by a
  higher-level profile
- **THEN** JOSE preserves only its validated syntax and makes no profile or
  trust claim

#### Scenario: generic consumers reuse the bounded JWE port

- **WHEN** a consumer requires one bounded JWE encryption capability
- **THEN** it can depend on `identus-jose` without bringing OID4VP claims or a
  concrete encryption engine
