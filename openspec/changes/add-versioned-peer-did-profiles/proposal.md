# Add versioned peer DID profiles

## Why

SDK-Rust has a method-neutral DID core but no portable peer DID implementation.
Existing Identus products use numalgo 2, while the current DID Methods WG
proposal selects numalgo 4. Treating either donor as the unversioned default
would either strand existing consumers or make legacy behavior normative.

## What changes

- Add an independently selectable `identus-did-peer` crate.
- Expose separate `numalgo4` and `numalgo2_compat` modules; neither profile
  silently parses, creates, or emits the other.
- Implement bounded creation, parsing, resolution and DID-document projection
  from caller-owned public keys, services, or input-document bytes.
- Make numalgo-4 long/short form linkage and numalgo-2 purpose/service encoding
  explicit and independently evidenced.
- Use the pinned specification, SDK-TS, `did-peer` 0.7.5 and
  `equs-vcx-did-peer` 0.1.0 as normative/differential evidence without adding
  either oracle to the production dependency graph.

## Capabilities

### Modified capabilities

- `peer-did-method`: replace the architecture-only requirements with an
  executable, bounded and profile-specific contract.

## Non-goals

No numalgo 0, 1 or 3; key generation; private keys; wallet custody; DIDComm
engine; relationship storage; agent runtime; networking; language facade;
consumer mutation; or crates.io publication.

## Delivery

Issue #523 owns this additive SDK-only slice. SDK-TS adoption remains #492.
The open NeoPRISM adoption PR does not depend on peer DID and is refreshed only
after the SDK A2 method surface is stable.

Pending quality declaration `did.peer.profiles.quality.v1`, owned by issue
#523, binds official, donor, negative, property, fuzz, differential, benchmark
and target evidence before consumer-visible activation.
