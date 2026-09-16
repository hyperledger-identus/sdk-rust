# Harden public crypto text encoders

## Why

Issue #298 owns the final crypto-codec retention exception identified by the
repository input-resource audit. `HexStr::from` and
`Base64UrlStrNoPad::from` accept any `AsRef<[u8]>` and infallibly retain an
encoded `String`, while their text parsers already enforce the 4,096-byte
canonical text budget. A hostile native caller can therefore bypass the
retained-value limit through the byte-encoding path.

## What changes

- Remove the blanket public infallible `From<B: AsRef<[u8]>>` implementations.
- Add named bounded fallible byte constructors and conventional fallible
  conversions for slices, vectors, and arrays.
- Retain one crate-private trusted encoder for already bounded parser output and
  fixed-width internal JWK coordinates.
- Migrate every workspace caller and document the source migration for future
  consumers.
- Remove only the codec clause from `SDK-LIM-007` after public reachability,
  tests, API evidence, and supported-target gates prove the new boundary.

## Impact

- Affected spec: `crypto`, `sdk-input-resource-governance`.
- Affected crate: `identus-crypto`.
- Public source compatibility: intentional pre-release migration from
  infallible `From` to fallible construction; value and wire representations
  are unchanged.
- Dependencies/features/MSRV: unchanged.
- Consumers: no direct sdk-rust codec calls were found in the current local
  neoprism, midnight-identity, lace-id-portal, or oxid worktrees. Neoprism uses
  its own donor `identus-apollo` codec and is reference evidence, not a current
  sdk-rust caller.

## Non-goals

- No change to supported codec profiles or the 4,096-byte encoded-text budget.
- No downstream repository mutation.
- No claim that generic serializer or caller allocation before SDK entry is
  bounded.
