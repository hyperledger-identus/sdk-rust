## Why

The DID document boundary recognizes `publicKeyMultibase` but currently checks
only that its JSON value is a non-empty, bounded printable string. Invalid
alphabets, unsupported prefixes, padding aliases and empty decoded key material
therefore cross a typed SDK boundary.

The earlier portfolio proposed `multibase 0.9.3` after an MSRV prerequisite.
Rust 1.98.1 satisfies that compiler prerequisite, but focused dependency
research finds a poor cohesion match: the crate unconditionally includes
unused Base45, Base256Emoji and build-time encoding machinery. The current
consumer needs only the `z` base58-btc and `u` unpadded-base64url encodings
recognized by Controlled Identifiers 1.0 and did:key v0.9.

## What changes

- Reject `multibase 0.9.3` for the current runtime boundary.
- Add exact `bs58 0.5.1` with default features disabled and `alloc` enabled,
  composed with the existing `base64 0.22` engine behind an Identus-private
  dispatcher.
- Accept only `z` and `u`, require a non-empty decoded payload, and require
  decode/re-encode equality for canonical spelling.
- Route `VerificationMethod` `publicKeyMultibase` validation through that
  dispatcher while preserving its public string accessor, JSON representation
  and stable redacted error, and tightening the recognized carrier ceiling
  from 16 KiB to 4 KiB after a worst-case resource probe.
- Add ADR 0085, exact artifact/cone/unsafe/maintenance evidence and official
  accepted/rejected vectors.

## Capabilities

### Modified capabilities

- `did-core`: make recognized `publicKeyMultibase` material structurally
  valid and canonical without interpreting multicodec or cryptographic keys.

## Non-goals

- No public generic multibase type, third-party error or blanket registry
  support.
- No multicodec, curve, key-length, cryptographic verification, multihash,
  did:key resolver or did:peer implementation.
- No private/secret multibase material, downstream repository edit or release.

## Delivery

Issue #156 owns this change under #151. This specification/research decision
lands before production code. Implementation requires research and constraint
readiness, distinct exact-diff review, full local evidence, a PR to `develop`
and green hosted gates before merge.
