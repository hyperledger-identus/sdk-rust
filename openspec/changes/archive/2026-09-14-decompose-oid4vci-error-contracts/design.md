# Design

## Context and goals

OID4VCI behavior is correct, but 171 contracts occupy one 860-line public const
bridge spanning six protocol responsibilities. The goal is smaller
responsibility-owned review units and exact characterization while preserving
the already-single mapping site and wildcard-free behavior.

## Decisions

### OID4VCI-specific private record

Add a crate-private const record containing `ErrorCode`, `ErrorKind`, and one
static message. Its const conversion supplies centralized `CAPABILITY`. It is
neither public, serialized, bound, allocated, nor returned. The public
`to_identus_error()` remains const and delegates through this record.

### Six protocol-cohesive modules and exhaustive router

Add private offer transport/JSON, offer semantics/grants,
issuer/authorization-server metadata, token request/response/errors,
credential/nonce/HTTP, and deferred/immediate issuance catalogues with
12/21/39/34/36/29 records. Keep the public enum and constants explicit.

One private macro list adjacent to the enum maps all 171 variants to catalogue
records and produces only a cfg(test) inventory. Its match has no wildcard and
no code, kind, capability, or message literals. The list does not generate the
public enum, constants, wire types, or Serde implementations.

### Independent immutable golden

Capture 171 ordered rows from the exact base before implementation. Each row
contains error type, variant, constant name/visibility, code, kind, capability,
local display, public message, full public display, and source. Copy the bytes
to a stable OID4VCI test fixture only after the durable receipt.

The staged exact-base byte sequence has SHA-256
`2c9e03381744b11902eb8b8bbc08934374781fad99d6561573cfcd62490bcd39`.
The planning change must contain that exact file before its planning-only
commit.

Extend the existing checker as a fourth bounded binding rather than copying
its path-security logic. The binding independently requires exact repo, issue,
change, branch, base, timestamp, schema, hash, byte identity, one complete
active-or-archived planning copy, Git ancestry/blob binding, root confinement,
regular files, and no symlinked component. Extend the current 76-case mutation
suite to 101 cases, including strict scalar-type and coordinated-drift cases.

### Compatibility and measurement

The regression independently enumerates all 171 variant/constant pairs and
checks exact ordered rows, discriminant order, 167/4 kinds, capability, both
display surfaces, `From`, const conversion, `Debug`, and both source results.
It explicitly names the 21 currently unreferenced metadata variants.

Require empty public API, manifest, feature, dependency, lockfile, and
canonical protocol-spec diffs. Measure the current 860-line bridge, 857-arm
match, one mapping site, zero wildcard defaults, 171 decisions,
1,381/1,371-line error file, and 8,526/7,852-line production crate against the
result. Success keeps one mapping site, zero wildcards, and 171 decisions,
caps catalogues at 39 records, and makes the public bridge a small delegator.
Report every total-line increase; no line-count compression gate is imposed.

## Risks and mitigations

- **Golden and implementation drift together:** fixed exact-base hash plus
  immutable Git receipt/blob binding keeps the oracle independent.
- **Future variant omitted:** the wildcard-free router, shared test inventory,
  independent golden pair list, and discriminant-order assertion fail.
- **Kind silently changes:** every record names its kind; group totals and exact
  rows pin all four `Unsupported` cases.
- **Wire and SDK errors become coupled:** public wire models and Serde code are
  excluded; the catalogue is private to the SDK diagnostic enum.
- **Catalogue fragmentation:** only six observed protocol axes are used and no
  group exceeds 39 records.
- **Checker security regresses:** add configuration to the existing hardened
  validator; do not fork or weaken its path/provenance checks.
- **Target claims are overstated:** report compile-only evidence and exclude
  runtime/device/package/FFI/binding support.
- **Scope absorbs feature work:** manifest, canonical OID4VCI specs, protocol
  modules, issues #7/#168, and consumer repositories remain diff guards.

## Migration and rollback

1. Add ADR/OpenSpec and the exact planning golden, independently review them,
   then commit the planning-only evidence with signature and DCO.
2. Write, validate, and commit the durable exact-base/exact-head receipt.
3. Copy the golden byte-for-byte to the stable fixture before source edits.
4. Add the private record/catalogues/router without altering public or wire
   declarations.
5. Add exact compatibility tests and the fourth checker/factory/Nix binding.
6. Run compatibility, quality, target, factory/Nix, and mutation evidence.
7. Resolve distinct architecture/API/security and adversarial review, archive,
   open the PR, and merge only after hosted gates pass.

Rollback removes the private modules and stable fixture and restores the
single match. No external state changes.
