# Constraint and limitation impact

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/197
Constraint blockers: none

## Existing entries affected

`SDK-SEC-003` requires an explicit resource ceiling before changing this
untrusted parser boundary. `SDK-COMPAT-001` requires the newly rejected input
class to be recorded. `SDK-LIM-007` remains effective because #197 covers two
codec parsers and their JWK consumer, not the repository-wide inherited audit.

## Introduced or changed constraints

No new machine constraint ID is needed. The effective behavior becomes:
`HexStr::from_str` and `Base64UrlStrNoPad::from_str` accept no more than 4,096
UTF-8 bytes and check that budget before decoding. Changing the value later
requires a new material compatibility and resource-budget decision.

Trusted `From<B: AsRef<[u8]>>` encoding remains infallible and caller-budgeted.
This is an explicit asymmetric API contract, not evidence that all instances
or source allocations are intrinsically bounded.

## Introduced or changed limitations

`SDK-LIM-007` will record the new codec evidence while remaining effective for
other inherited surfaces. The intrinsic check does not cap an allocation made
before `FromStr`, JSON/serde materialization, transport bodies, decompression,
nesting, extension values or trusted byte-to-text encoding.

The change does not replace the existing codec crates, add streaming decode,
or create a generic large-payload encoding API. JWK keeps its existing
canonical-encoding and decoded-width error contract.

## Consumer and product impact

Callers parsing encoded text above 4,096 bytes must reject it or use a
separately budgeted large-payload codec; they cannot rely on these primitive
wrappers. Existing key, signature, digest, mnemonic/derivation and JWK uses fit
well below the ceiling. No Oxid, Midnight Identity, NeoPRISM, Apollo or Lace
repository is mutated or claimed migrated.

## Activation and rollback

Issue #197 is the decision authority during unpublished active development.
Activation requires focused boundary/error/JWK tests, unchanged dependency and
feature evidence, complete local Nix validation, distinct review and green
hosted CI before merge to `develop`. Rollback restores the unbounded parser and
the prior `SDK-LIM-007` wording; changing the limit is not an implicit rollback.

## Evidence

Acceptance requires exact 4,096-byte success, 4,097-byte resource-first
failure, otherwise-valid oversized failure, preserved in-budget
canonicalization, JWK inherited rejection, redaction-safe stable bridging, no
dependency/feature/unsafe/native drift, canonical spec and constraint-index
atomicity, full Nix validation and hosted CI.
