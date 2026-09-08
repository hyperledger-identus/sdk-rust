# Bound identus-core URL input

## Why

`identus-core::Url` is a validated public value but currently accepts an
unbounded `String`. Issue #168 records this as the concrete example of the
repository's incomplete inherited-input resource audit. Issue #193 isolates
that example into one reviewable compatibility and security change.

RFC 3986 defines generic URI syntax without an implementation-wide size
ceiling. RFC 9110 section 4.1 recommends support for at least 8,000 octets in
HTTP URI protocol elements. A maximum of 8,192 UTF-8 bytes meets that floor
while matching the SDK's explicit byte-limit convention.

## What Changes

- Export `MAX_URL_BYTES` with the value 8,192 from `identus-core`.
- Make every validated `Url` construction path reject longer input with a
  stable `UrlError::TooLong` outcome.
- Prove exact-boundary and multibyte UTF-8 semantics, including serde-backed
  validated construction and redaction-safe core-error bridging.
- Add the accepted-size behavior to the canonical core specification.
- Narrow the named `Url` clause in `SDK-LIM-007` while retaining the broader
  inherited resource-audit limitation.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `core-error-conventions`: define the public URL byte boundary, error mapping,
  and outer-allocation limitation.

## Impact

- **Issue:** #193, child of security audit #168.
- **Public API:** adds `MAX_URL_BYTES` and `UrlError::TooLong`.
- **Accepted behavior:** previously accepted syntactically valid values above
  8,192 bytes become invalid.
- **Wire behavior:** serde rejects over-limit strings after the deserializer has
  materialized them; serialization of valid values is unchanged.
- **Dependencies:** none.
- **Rollback:** revert the limit and restore the named `Url` limitation; do not
  silently raise the bound without a new compatibility/security decision.
