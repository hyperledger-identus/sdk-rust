# jose-module-boundary Specification

## Purpose
TBD - created by archiving change narrow-jose-boundary. Update Purpose after archive.
## Requirements
### Requirement: JOSE owns protocol-neutral compact and signature mechanics

The `identus-jose` crate SHALL own bounded canonical JWS Compact parsing and
encoding, validated protected-header syntax, signature algorithms, public-key
binding, signer/signature-suite ports, and explicit unverified/verified compact
states. It SHALL NOT own OIDC/OID4VCI claims, DID resolution, certificate or
federation trust policy, key-attestation policy, clocks, freshness, replay,
network, storage, wallet, product, or chain behavior.

#### Scenario: generic consumers reuse JOSE without identity protocols

- **WHEN** a consumer requires bounded compact JWS signing or verification
- **THEN** it can depend on `identus-jose` without bringing a DID or OIDC
  protocol dependency

#### Scenario: bounded header evidence remains untrusted syntax

- **WHEN** a protected header carries a supported bounded extension used by a
  higher-level profile
- **THEN** JOSE preserves only its validated syntax and makes no profile or
  trust claim

### Requirement: OID4VCI owns its proof profile and policy

The `identus-oid4vci` crate SHALL own OID4VCI proof claims, holder construction,
issuer parsing, key-reference resolution, attestation/federation provider
ports, verification/trust/authorization states, clock/freshness/replay policy,
and profile-specific static errors. It SHALL compose generic compact and
signature mechanics through `identus-jose` and SHALL own its direct DID
dependency.

#### Scenario: profile failures belong to the protocol capability

- **WHEN** a proof violates an OID4VCI claim, key-resolution, trust, time, or
  replay rule
- **THEN** the failure maps to a static redacted `oid4vci.*` error rather than
  expanding the generic JOSE error vocabulary

#### Scenario: ownership changes without wire drift

- **WHEN** the profile modules and tests move between crates
- **THEN** the same valid proof bytes, invalid-input matrix, provider call
  counts, and staged security claims remain in force

### Requirement: Third-party JOSE engines remain private and evidence-gated

The SDK SHALL keep every third-party JOSE engine private and evidence-gated. A
third-party JOSE crate MAY implement one private operation only after an
issue and ADR prove exact semantic, strictness, bounds, error, MSRV, target,
unsafe/native, license, maintenance, and dependency-cone fit. Its public types,
errors, algorithm defaults, and trust policy SHALL NOT escape an Identus-owned
facade.

#### Scenario: popular crate lacks exact profile parity

- **WHEN** a candidate exposes legacy `EdDSA`, permissive parsing, broad
  JWT/JWE scope, native dependencies, or coupled identity models
- **THEN** it remains an oracle or private-operation candidate and is not
  adopted as the SDK's public JOSE type system

#### Scenario: a narrow engine later proves exact parity

- **WHEN** a candidate deletes risky local mechanics while passing the entire
  existing facade contract and dependency policy
- **THEN** a separately authorized change may adopt it privately without
  changing consumer APIs or security-state meaning
