## ADDED Requirements

### Requirement: Core URL values have one explicit byte boundary

`identus-core` SHALL export `MAX_URL_BYTES` with the value 8,192. Every
validated construction path for `Url`, including `parse`/`FromStr`, `try_new`,
`TryFrom<String>` and serde deserialization, SHALL reject input whose UTF-8 byte
length exceeds this value with `UrlError::TooLong`. The limit check SHALL occur
before input-proportional URL syntax validation.

This bound limits accepted/retained `Url` values and their syntax-validation
work. It SHALL NOT be represented as bounding allocation already performed by
the caller, transport, decompressor or generic deserializer; those layers
require outer resource limits.

#### Scenario: Exact ASCII boundary is accepted

- **WHEN** a syntactically valid URL is exactly 8,192 UTF-8 bytes
- **THEN** every validated construction path SHALL accept it

#### Scenario: One byte above the boundary is rejected

- **WHEN** a syntactically valid URL is 8,193 UTF-8 bytes
- **THEN** every validated construction path SHALL return or surface
  `UrlError::TooLong` and SHALL NOT construct a `Url`

#### Scenario: The boundary measures UTF-8 bytes

- **WHEN** a syntactically valid URL contains multibyte Unicode scalars and its
  character count is below the limit but its UTF-8 byte length exceeds it
- **THEN** validated construction SHALL return `UrlError::TooLong`

#### Scenario: Oversized error bridging remains stable and redaction-safe

- **WHEN** `UrlError::TooLong` is bridged with `to_identus_error()`
- **THEN** the result SHALL use `core.invalid_url`, capability `core`, and
  `ErrorKind::InvalidInput`
- **AND** its display SHALL contain neither the rejected input nor a dynamic
  measured length

#### Scenario: Outer allocation limits remain a caller obligation

- **WHEN** an oversized source string has already been allocated before
  validated `Url` construction
- **THEN** the SDK SHALL reject the value but SHALL NOT claim to have prevented
  that earlier allocation
