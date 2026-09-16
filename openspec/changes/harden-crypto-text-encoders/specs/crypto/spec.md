# Crypto delta

## ADDED Requirements

### Requirement: Public byte encoding is bounded and fallible

The SDK SHALL make every public construction path that encodes caller-owned
bytes into `HexStr` or `Base64UrlStrNoPad` reject before the retained canonical
text would exceed `MAX_CRYPTO_TEXT_BYTES`. No blanket infallible conversion from
arbitrary `AsRef<[u8]>` SHALL remain public.

#### Scenario: Exact encoded limit

- **WHEN** input bytes encode to exactly 4,096 canonical text bytes
- **THEN** fallible byte construction SHALL accept them and preserve exact
  canonical round-trip behavior

#### Scenario: One byte-class over the raw boundary

- **WHEN** hex receives 2,049 bytes or Base64url-no-pad receives 3,073 bytes
- **THEN** construction SHALL fail before retained encoded allocation and SHALL
  expose only stable redacted size diagnostics

#### Scenario: Fixed-size internal encoding

- **WHEN** bounded parser output or a fixed 32-byte JWK coordinate is encoded
- **THEN** the crate-private trusted path MAY encode it without changing output
- **AND** that path SHALL NOT be reachable through the public API

## MODIFIED Requirements

### Requirement: Public crypto text parsing has one explicit byte boundary

`identus-crypto` SHALL export `MAX_CRYPTO_TEXT_BYTES` with the value 4,096.
`HexStr::from_str` and `Base64UrlStrNoPad::from_str` SHALL reject input whose
UTF-8 byte length exceeds this value through `Error::KeyParsing`. The byte check
SHALL occur before decoder or canonical re-encoding work.

Every public byte-encoding constructor SHALL enforce the same retained text
ceiling through a fallible result. This limit SHALL NOT be represented as
preventing allocation already performed by a caller, transport, decompressor,
JSON parser, or deserializer.

#### Scenario: Exact parser boundary preserves canonical codecs

- **WHEN** canonical hex or unpadded base64url input is exactly 4,096 UTF-8
  bytes
- **THEN** parsing SHALL succeed, decoding SHALL preserve the source bytes, and
  output SHALL use the existing lowercase-hex or canonical unpadded-base64url
  representation

#### Scenario: One byte above is rejected before syntax

- **WHEN** hex or unpadded base64url input is 4,097 UTF-8 bytes and is also
  malformed for its encoding
- **THEN** parsing SHALL fail for the byte budget before decoder syntax work

#### Scenario: Otherwise valid oversized text is rejected

- **WHEN** an otherwise canonical encoded value exceeds 4,096 UTF-8 bytes
- **THEN** parsing SHALL fail without allocating its decoded representation

#### Scenario: Oversized local and bridged errors are redaction-safe

- **WHEN** oversized text or byte input contains a sentinel and its error is
  inspected locally and through `to_identus_error()`
- **THEN** neither display SHALL contain the sentinel
- **AND** the bridge SHALL retain `crypto.key_parsing`, capability `crypto`,
  `ErrorKind::InvalidInput`, and the static public message `key parsing failed`

#### Scenario: Public byte encoding is caller-visible and bounded

- **WHEN** a caller encodes bytes whose canonical text would exceed the parser
  budget
- **THEN** fallible construction SHALL reject before retaining encoded text
- **AND** no blanket infallible encoding conversion SHALL bypass the limit

#### Scenario: JWK coordinates inherit the decoder ceiling

- **WHEN** a public JWK coordinate exceeds 4,096 encoded bytes
- **THEN** construction SHALL reject it as invalid coordinate encoding before
  base64url decode allocation
- **AND** existing in-budget canonical-encoding and decoded-32-byte-length error
  behavior SHALL remain unchanged
