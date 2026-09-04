# credential-core Specification

## Purpose

`credential-core` defines the chain-neutral, format-neutral holder boundary
for encoded credential artifacts in `identus-credentials`. It preserves
bounded bytes without implying parsing, verification, trust, storage, or a
wire format. Concrete credential formats and protocols layer on this small
experimental core.

## Requirements

### Requirement: Open bounded credential format identifier

The SDK SHALL provide an owned `CredentialFormat` identifier that is open to
format adapters rather than defined by a closed format enum. It SHALL accept
1–128 ASCII bytes, require an alphanumeric first byte, allow only alphanumeric
bytes plus `.`, `_`, `+`, `-`, and `:` thereafter, validate before allocating,
preserve accepted spelling exactly, and reject all other input with a
redaction-safe error.

#### Scenario: unrelated formats share the same identifier type

- **WHEN** `vc+sd-jwt`, `mso_mdoc`, `midnight_cbor_phase1`, and a dummy format
  satisfying the grammar are parsed
- **THEN** every value SHALL succeed and round-trip its exact spelling without
  adding an SDK enum variant

#### Scenario: unsafe and oversized identifiers are rejected

- **WHEN** an identifier is empty, longer than 128 bytes, begins with
  punctuation, contains whitespace/control/non-ASCII bytes, or contains an
  unapproved punctuation byte
- **THEN** parsing SHALL fail without allocating an owned format string or
  including the rejected value in debug, display, or the public error bridge

### Requirement: Bounded opaque credential artifacts

The SDK SHALL provide distinct owned `CredentialPayload`,
`CredentialDetachedProof`, and `CredentialPrivateMaterial` types. Payload and
detached proof SHALL each accept 1–1,048,576 bytes. Private material SHALL
accept 1–262,144 bytes. Accepted bytes SHALL be preserved exactly and exposed
only through explicit borrowed accessors. Empty or oversized input SHALL fail
with a typed, redaction-safe error.

#### Scenario: boundary-sized artifacts preserve exact bytes

- **WHEN** each artifact type receives its one-byte and maximum-size values
- **THEN** construction SHALL succeed and the borrowed accessor SHALL return
  the exact input bytes

#### Scenario: invalid artifact sizes fail without disclosure

- **WHEN** an artifact receives empty or limit-plus-one bytes
- **THEN** construction SHALL return the matching typed size error and safe
  formatting SHALL NOT contain any caller byte sequence

### Requirement: Format-private material has an owned erasure contract

`CredentialPrivateMaterial` SHALL implement `Zeroize` and `ZeroizeOnDrop`.
Explicit erasure SHALL clear its owned vector, and construction failure SHALL
zeroize the transferred input allocation before releasing it. The type SHALL
not provide `Clone` or ordinary equality. Its debug representation SHALL show
length only. The contract SHALL state that borrowed/caller/compiler copies,
allocator state, swap, dumps, hardware, custody, and encrypted storage are
outside best-effort owned-buffer erasure.

#### Scenario: private material is explicitly erasable and redacted

- **WHEN** compile-time assertions inspect its traits and explicit zeroization
  is invoked on a known byte sequence
- **THEN** the type SHALL satisfy both erasure traits, its owned bytes SHALL be
  empty, and neither its own nor its envelope's debug output SHALL contain the
  known sequence

### Requirement: Format-neutral credential envelope

The SDK SHALL provide `CredentialEnvelope` containing exactly one validated
format, one bounded payload, and optional bounded detached proof and private
material. Construction SHALL preserve all values without interpreting,
normalizing, verifying, serializing, or applying trust/product policy. Safe
debug output SHALL expose format and artifact lengths only.

#### Scenario: two independent adapters use the same envelope

- **WHEN** an Oxid-shaped Midnight CBOR fixture and an unrelated dummy-format
  fixture construct envelopes with different optional artifact combinations
- **THEN** both SHALL use the same API, retain exact format/body/proof/private
  material, and require no dependency on a consumer or chain crate

#### Scenario: envelope construction does not imply verification

- **WHEN** arbitrary bounded bytes are placed in an envelope
- **THEN** construction SHALL succeed without parsing or verification and the
  API/documentation SHALL make no validity or trust claim

### Requirement: Stable credential construction errors

Every local construction error SHALL map to an `IdentusError` with capability
`credential`, `InvalidInput` kind, a stable `credential.*` code, and static
public text. The bridge SHALL carry no format string or artifact bytes.

#### Scenario: all error variants bridge without caller data

- **WHEN** every credential construction error is converted and formatted
- **THEN** its capability, kind, code, and static message SHALL match the
  contract and no rejected identifier or artifact bytes SHALL appear
