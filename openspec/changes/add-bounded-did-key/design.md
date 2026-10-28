# Design

## Ownership and API

`identus-did-key` exposes a typed `PublicKey` enum containing only SDK public
key types, a validated `DidKey`, deterministic `create`, `parse`, and `resolve`
operations, public limits, and a stable redacted error enum.

```text
caller public key -> identus-did-key -> identus-did document
                           |
                           +---------> identus-crypto validation
```

The crate does not expose private material, a generic byte constructor for
creation, or a process-global registry. An example composes the method into a
caller-owned generic registry explicitly.

## Canonical identifier pipeline

Creation selects one fixed unsigned-varint multicodec prefix and canonical
public encoding: 32-byte Ed25519/X25519, or compressed 33-byte P-256/
secp256k1. It concatenates codec and key, base58-btc encodes with `z`, and
builds `did:key:<fingerprint>` through `identus-did`.

Parsing checks the DID method, 128-byte method-specific ceiling, exact `z`
prefix, and a conservative encoded-length bound before base58 decode. It
rejects decoded data above 67 bytes, decodes the shortest canonical unsigned
varint, maps only the four fixed codecs, checks exact key length, and invokes
the corresponding SDK public-key constructor. Re-encoding must equal the input.

## Document projection

Resolution creates one verification method whose ID is
`<did>#<fingerprint>`, controller is the DID, type is `Multikey`, and
`publicKeyMultibase` is the original canonical fingerprint. Ed25519, P-256,
and secp256k1 reference that method from authentication, assertionMethod,
capabilityInvocation, and capabilityDelegation. X25519 references it only from
keyAgreement. No second derived key is synthesized.

Create and resolve form the primary deterministic E2E: construct each SDK key
from fixed public bytes, create its DID, parse it, resolve it through an opt-in
registry, serialize/reparse the document, and prove the relationships and
canonical fingerprint remain identical without I/O.

## Errors and bounds

Errors distinguish wrong method, malformed/non-canonical encoding,
unsupported codec, invalid key, resource limit, and document projection.
`Display` uses fixed text. Public limit constants are part of the profile.
Tests prove rejection occurs before avoidable allocation for wrong prefix,
oversized text, and impossible decoded lengths.

## Evidence layout

Attributed JSON fixtures hold normative/oracle inputs and expected normalized
outcomes. Unit/conformance tests cover all codecs and negatives; properties
cover create/parse/resolve round trips; fuzzing targets parser/resolver input;
directional benchmarks measure create, parse, resolve, and boundary rejection.
The handbook explains ownership, support, limits, registry composition, and
non-goals. The quality declaration and change ledger make support discoverable.
