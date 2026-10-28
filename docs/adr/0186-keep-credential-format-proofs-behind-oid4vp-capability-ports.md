# ADR 0186: Keep credential-format proofs behind OID4VP capability ports

- Status: Accepted
- Date: 2026-10-28
- Decision owners: SDK-Rust OID4VP maintainers
- Issue: https://github.com/hyperledger-identus/sdk-rust/issues/547

## Context

OpenID4VP Final is credential-format agnostic but makes DCQL metadata and
transaction binding format-specific. Accepting arbitrary JSON would lose
mandatory semantics; implementing SD-JWT VC, mdoc, or W3C VC proof behavior in
`identus-oid4vp` would make the protocol crate a coupled proof-engine hub.

## Decision

The protocol crate owns bounded exact transaction envelopes, known/open format
identifiers, DCQL references, selected-input linkage, and static errors. It
accepts finite caller-supplied capabilities for format metadata and transaction
semantics. Credential-format crates own proof creation, verification, hashing,
embedding, and their native types. There is no global registry or foreign DTO
compatibility contract.

Empty metadata without transaction data remains the baseline profile. Every
non-empty metadata object and transaction type requires exactly one capability.
The exact encoded transaction string is retained behind an explicit sensitive
accessor because some profiles bind its encoded representation.

## Consequences

This preserves low coupling, cohesive ownership, portable effects, and a small
dependency cone. Adopters must provide capabilities for advanced formats, and
the SDK cannot claim format conformance until the owning crate and E2E evidence
exist. Unknown mandatory semantics fail closed.

## Rejected alternatives

- Arbitrary opaque JSON: insufficient semantic rejection and linkage evidence.
- Bundled format engines: wrong ownership and dependency growth.
- Global plugin registry: ambient state and target/runtime complexity.
- Foreign SDK DTO/error parity: migration belongs in thin adapters.
