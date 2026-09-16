# Design

## Validated payload types

`JwsKeyId` privately owns one non-empty, control-free string within
`JwsLimits::max_header_string_bytes`. `JwsX5c` privately owns one through eight
non-empty canonical standard-base64 certificate strings, each within both the
header-string and protected-header ceilings. `Oid4vciProofJwtClientId` privately
owns one non-empty, control-free string within
`Oid4vciProofJwtLimits::max_claim_string_bytes`.

Each type exposes only a fallible constructor, a borrowed accessor, and
redacted `Debug`. `JwsKeyReference` and `Oid4vciProofJwtClient` retain their
existing alternatives but carry these opaque types. Named enum constructors
provide the ordinary ergonomic path; raw payload construction no longer
compiles.

## Parser and builder convergence

Protected-header parsing first reads bounded raw wire values, then constructs
the same validated payload types with the parser's limits. OID4VCI proof parsing
uses the existing `identified` constructor. Builder validation calls wrapper
validation again so a value created under roomy limits cannot enter a tighter
builder. Serialization and verifier dispatch use borrowed accessors, preserving
wire bytes and provider inputs.

## Compatibility

This is an intentional source migration while `identus-jose` remains
unpublished at `0.0.0`. Existing high-level constructors and proof state
transitions remain. Direct callers replace raw variants with
`JwsKeyReference::key_id`, `JwsKeyReference::x5c`, or
`Oid4vciProofJwtClient::identified`. No aliases, unchecked constructors, or
deprecated unbounded escape hatches are retained.

## Verification

- exact/one-over bytes for `kid` and identified-client strings;
- one/eight accepted X.509 entries plus empty/ninth, malformed, and oversized
  rejection;
- accepted native values serialize and parse identically;
- roomy construction fails when reused under tighter builder limits;
- errors and diagnostics do not reveal caller-controlled data;
- compile-fail/source evidence proves raw payload variants are unavailable;
- focused, minimal/all-feature, workspace, API/SBOM, supported-target, factory,
  and exact-head CI evidence plus fresh security/API/architecture review.

## Rollback

Restore raw variant payloads and their call sites, then restore the inventory
exception and `SDK-LIM-007` wording in the same change.
