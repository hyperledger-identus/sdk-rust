# Design

## Public construction

Each codec adds `try_from_bytes(value: impl AsRef<[u8]>) -> Result<Self, Error>`.
Specific `TryFrom` implementations for byte slices, vectors, owned arrays, and
borrowed arrays delegate to that method. The blanket infallible `From` impl is
removed so arbitrary byte encoding cannot bypass validation.

Hex checks `input.len() <= 2048`. Base64url checks
`input.len() <= 3072`; these exact raw limits are derived from the unchanged
4,096-byte canonical text ceiling. Error reporting uses checked encoded-length
calculation and existing redacted `Error::encoded_text_too_large`.

## Trusted internal encoding

One private constructor per type performs canonical encoding without another
fallible branch. It is used only after `FromStr` has bounded canonical text and
decoded it, or for fixed 32-byte JWK inputs. No public unchecked hatch is added.

## Compatibility

This is a deliberate source migration while the crate is unpublished at
`0.0.0`. Successful encoded text and serde/JWK output remain byte-for-byte
identical. The migration is compile-guided: former `Type::from(bytes)` calls
become `Type::try_from_bytes(bytes)?` or `Type::try_from(bytes)?`.

## Verification

- exact and one-over raw limits for both codecs and each representative input
  ownership form;
- canonical output parity and parser round trips;
- stable redacted local/core errors;
- source/API guard that no unbounded public `From` implementation returns;
- crypto minimal/all-feature tests, workspace gates, SBOM/dependency evidence,
  supported-target/Nix checks, and fresh review.

## Rollback

Reintroduce the blanket `From`, reverse callers/tests/docs, and restore the
codec clause in `SDK-LIM-007` in one change.
