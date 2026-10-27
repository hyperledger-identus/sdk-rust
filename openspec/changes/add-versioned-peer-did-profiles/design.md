# Design

## Crate and profile boundary

`identus-did-peer` exposes only shared profile classification, stable redacted
errors and two explicit modules:

```text
identus-did-peer
  ├── numalgo4          current short/long profile
  └── numalgo2_compat  opt-in Identus compatibility profile
          |
          v
      identus-did
```

There is no `PeerDid::create(algorithm)` convenience that could silently select
a legacy profile.

## Numalgo 4

`InputDocument` validates a bounded JSON object without root `id`. Local
resource identifiers and same-document references must be fragments. Creation
produces a long-form DID and its derived short form from one retained compact
JSON representation. The document bytes receive the JSON multicodec prefix,
base58btc multibase encoding, then SHA2-256 multihash/base58btc encoding over
the exact encoded-document string.

Parsing splits and classifies the method-specific identifier before decoding.
It bounds both text segments, verifies multibase/multicodec/multihash framing,
compares the fixed digest, then parses and validates JSON. Resolution inserts
the selected long or short DID as root `id`, contextualizes only typed local
identifiers/references, and records the counterpart through `alsoKnownAs`.
Short-form resolution takes a validated long form from the caller and verifies
the same short hash before projection.

## Numalgo 2 compatibility

Creation consumes ordered, caller-owned public key descriptors and optional
service descriptors. Each key has one explicit verification relationship and
a supported public multicodec value. Encoding emits dot-delimited purpose
elements; a service element uses unpadded base64url of the pinned compact JSON
shape, using one object for one service and one array for multiple services.
Omitted compact endpoint options remain omitted when re-encoded. Resolution
validates every delimiter/purpose/encoding before decode, assigns deterministic
fragment identifiers from ordered key positions, maps each purpose to only its
corresponding verification relationship, and projects relationships and services
through `identus-did` builders.

The compatibility implementation preserves pinned SDK-TS behavior only where
it agrees with the DIF draft. Deviations are fixtures and ledger entries, not
hidden special cases.

## Bounds and errors

Each profile has a limits value with conservative defaults not exceeding the
generic DID/document ceilings. Counters cover raw bytes, encoded/decoded bytes,
elements, keys, services, JSON nodes and projected items. Parsing checks cheap
syntax and size before base decoding, and decoded size before JSON traversal.

Errors are non-exhaustive stable categories: wrong profile, unsupported
algorithm, malformed encoding, unsupported codec/purpose, invalid input
document, hash mismatch, missing long form, resource limit and projection
failure. Diagnostics expose a bounded element index or limit kind only.

## Verification

Official and donor vectors are stored as data with source/provenance records.
Property tests cover create/parse/resolve round trips and short/long linkage;
fuzz targets exercise both parsers and input-document validation. Differential
tests invoke oracle behavior only in isolated evidence tooling or frozen
fixtures, never in the production graph. Benchmarks report create/parse/resolve
direction and boundary scaling without enforcing unstable wall-clock budgets.
