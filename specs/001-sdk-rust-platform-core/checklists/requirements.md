# Requirements Checklist: Identus Rust SDK Platform Core

**Purpose**: Validate the seed specification before implementation planning.
**Created**: 2026-06-13
**Feature**: `specs/001-sdk-rust-platform-core/spec.md`

## Scope

- [x] CHK001 Existing SDK module families are represented in requirements.
- [x] CHK002 Backlog themes are represented as requirements or explicit future decisions.
- [x] CHK003 Out-of-scope items prevent accidental production implementation in the seed.
- [x] CHK004 Cross-language binding expectations are stated as Rust-core wrappers.

## Standards

- [x] CHK005 OID4VCI, OID4VP, and SIOPv2 are explicitly required.
- [x] CHK006 SD-JWT VC, W3C VC, AnonCreds, OpenBadges, and ISO mdoc are captured.
- [x] CHK007 DIDComm v2, PRISM DID, did:peer, and DID rotation are captured.
- [x] CHK008 Revocation/status-list requirements include negative verification behavior.
- [x] CHK009 OpenID Federation, X.509/IACA, trusted lists, and wallet attestations are captured.

## Architecture

- [x] CHK010 Domain crates are separated from adapters and bindings.
- [x] CHK011 Storage, transport, resolver, signer, and binding extension points are required.
- [x] CHK012 neoprism convergence is treated as an explicit design decision.
- [x] CHK013 Security and conformance gates are required before production protocol code.
