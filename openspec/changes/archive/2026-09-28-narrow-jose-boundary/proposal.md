# Narrow the JOSE boundary

## Why

`identus-jose` is intended to be the reusable, protocol-neutral owner of
bounded JWS Compact parsing, protected headers, signature suites, and explicit
unverified/verified states. It currently also owns OID4VCI proof construction,
issuer verification, DID dereferencing, attestation/federation trust, clock,
freshness, and replay policy. That creates an outward `identus-jose ->
identus-did` dependency and makes the primitive crate less reusable.

The available Rust JOSE crates remain useful implementation references, but no
evaluated package simultaneously provides RFC 9864 `Ed25519`, the SDK's strict
bounded/duplicate-rejecting wire behavior, its validated-state API, and its
portable dependency boundary. Replacing the SDK facade wholesale would trade
local code for semantic and coupling risk.

## What changes

- Keep `identus-jose` as an Identus-owned facade over bounded compact/header
  mechanics and private RustCrypto-backed signature operations.
- Move the complete OID4VCI proof JWT profile, verifier ports, trust evidence,
  DID resolution, freshness, and replay policy into `identus-oid4vci`.
- Move profile-specific static errors and error contracts to the OID4VCI
  capability while retaining generic JOSE errors in `identus-jose`.
- Remove the `identus-jose -> identus-did` dependency and add the already
  architecturally correct `identus-oid4vci -> identus-did` edge.
- Preserve generic protected-header evidence carriers as bounded untrusted
  syntax; JOSE does not validate their protocol meaning or trust.
- Record exact library candidates and keep them as private-engine/oracle
  candidates until an operation-level adoption proves exact parity.

## Capability

### Added capability

- `jose-module-boundary`: define protocol ownership, dependency direction,
  private-engine isolation, and compatibility expectations.

## Non-goals

No JWE, general JWT claims model, OIDC transport, new algorithm, public
third-party type, network behavior, trust framework, or crate publication is
added. The change does not loosen limits, duplicate rejection, redaction, or
cryptographic binding.

## Delivery

Issue #398 owns the change. A planning-only commit and factory preflight precede
implementation. The implementation then moves behavior and tests without
semantic growth, regenerates dependency/code-health evidence, and opens a
signed/DCO PR to protected `develop`.
