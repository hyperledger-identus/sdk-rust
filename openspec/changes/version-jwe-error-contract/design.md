# Design

## Independent version axis

```text
JoseError ----------------> jose-error-contract-v1/v2 (unchanged)

JweEncryptionError -------> jwe-encryption-error-contract-v1
       |                                  |
       +-- wildcard-free private map      +-- immutable exact-base oracle
```

The two enums share the `jose` capability and CSV schema, not an ordering or
fixture lifecycle. The JWE fixture records eight rows and explicitly records
that code constants are absent.

## Private contract and exhaustive route

A crate-private `JweErrorContract` contains exactly code, kind, and static
message. `JweEncryptionError::contract` uses an explicit match with no wildcard.
Conversion and display consume that record. A test-only complete inventory
uses the public variants in order and compares every field to the golden.

The enum, derives, `#[non_exhaustive]`, order, `Display`, `Error`, `From`, and
public const conversion signature remain unchanged. No public constant or
shared error framework is introduced.

## Immutable evidence

The planning golden is committed before implementation and bound to the exact
base and planning head through `preimplementation.json`. Implementation copies
it byte-for-byte to `crates/jose/tests/fixtures/`. The existing checker receives
one bounded configuration entry and its mutation suite covers the fifth
binding without weakening the first four.

The stable Rust test validates provenance, exact row count and order, unique
variants/codes, enum discriminants, debug identity, kind/capability, local and
public displays, `From`, empty sources, and multiple private-data canaries.

## Append-only policy

The v1 fixture always protects its eight-row prefix. A future additive variant
is appended to the live inventory and receives feature-owned exact assertions;
the historical fixture is not regenerated. A breaking edit requires a new ADR
and separately versioned fixture.

## Failure and rollback

Missing, duplicate, reordered, mutated, escaped, symlinked, post-generated, or
receipt-unbound evidence fails closed. Any public API, dependency, runtime, or
wire delta is out of scope and blocks delivery. Rollback is source-local and
does not migrate persisted data.
