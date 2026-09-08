## ADDED Requirements

### Requirement: Public crypto text parsing has one explicit byte boundary

`identus-crypto` SHALL export `MAX_CRYPTO_TEXT_BYTES` with the value 4,096.
`HexStr::from_str` and `Base64UrlStrNoPad::from_str` SHALL reject input whose
UTF-8 byte length exceeds this value through `Error::KeyParsing`. The byte
check SHALL occur before decoder or canonical re-encoding work.

The parser limit SHALL NOT make trusted `From<B: AsRef<[u8]>>` encoding
fallible or represent all constructed wrapper instances as intrinsically
bounded. It SHALL NOT be represented as preventing allocation already
performed by a caller, transport, decompressor, JSON parser or deserializer.

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

- **WHEN** oversized text contains a sentinel and its parse error is inspected
  locally and through `to_identus_error()`
- **THEN** neither display SHALL contain the sentinel
- **AND** the bridge SHALL retain `crypto.key_parsing`, capability `crypto`,
  `ErrorKind::InvalidInput`, and the static public message `key parsing failed`

#### Scenario: Trusted byte encoding remains caller-budgeted

- **WHEN** a caller explicitly encodes owned bytes whose text exceeds the
  parser budget
- **THEN** infallible construction and decoding SHALL still succeed
- **AND** the SDK SHALL document that reparsing that text is outside the
  bounded `FromStr` contract

#### Scenario: JWK coordinates inherit the decoder ceiling

- **WHEN** a public JWK coordinate exceeds 4,096 encoded bytes
- **THEN** construction SHALL reject it as invalid coordinate encoding before
  base64url decode allocation
- **AND** existing in-budget canonical-encoding and decoded-32-byte-length error
  behavior SHALL remain unchanged
