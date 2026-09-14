# ADR 0118: use crate-local multi-kind JOSE error contracts

- **Status:** Accepted for the JOSE slice
- **Date:** 2026-09-15
- **Decision authority:** umbrella issue
  [#271](https://github.com/hyperledger-identus/sdk-rust/issues/271) and
  delivery issue [#280](https://github.com/hyperledger-identus/sdk-rust/issues/280)
- **Applies to:** `identus-jose` only
- **Related decisions:** ADR 0110, ADR 0116, and ADR 0117
- **Assessed revision:** sdk-rust
  `c32c1c8cd0194466a8c7fbfaa8c050f4bf2971a1`

## Context

`JoseError` has 51 fieldless variants and 51 public stable error-code
constants. Its 214-line `to_identus_error()` repeats each code and message in
one match and assigns five `ErrorKind` values in a second match with a wildcard
fallback. `Display` delegates through that bridge. The behavior is correct,
but reviewers must reconcile two decision sites and scan unrelated JOSE and
proof-validation responsibilities together.

ADRs 0116 and 0117 establish crate ownership, exhaustive routing, and immutable
pre-refactor characterization. Their record shapes are not reusable here:
credentials needs five heterogeneous fields; presentations has uniform kind
and needs only two. JOSE requires a third private shape because kind varies
while capability and local/public text do not.

## Decision

1. `identus-jose` owns a private compile-time `ErrorContract` containing
   exactly `ErrorCode`, `ErrorKind`, and one `&'static str` message.
2. The uniform `jose` capability remains centralized as `CAPABILITY`; the
   single message feeds both local `Display` and the public projection.
3. Four private catalogue modules own compact/header (15),
   algorithm/key/registry/signing (11), proof/key/evidence (16), and
   proof/policy/time/replay (9) records.
4. One private wildcard-free macro/router maps every public variant to exactly
   one record and generates only a test inventory. It does not generate public
   declarations.
5. The enum, order, derives, `#[non_exhaustive]`, public constants and paths,
   `From`, `Display`, `Error`, and `pub const fn to_identus_error` remain
   explicit and unchanged. All contract access and conversion remains const.
6. A planning-only 51-row golden captured from the assessed revision pins
   constant identity/visibility, code, kind, capability, exact local/public/full
   display, and source behavior. Its stable test copy is byte-identical and
   bound to the preimplementation receipt's immutable Git blob.
7. The existing data-driven checker, mutation suite, factory fixture, and Nix
   source contract gain one bounded JOSE binding. They retain fail-closed
   active/archive, hash, provenance, root, regular-file, and symlink checks.
8. No dependency, feature, public/shared framework, protocol or algorithm,
   retryability, metadata, localization, serialization, FFI, unsafe/native
   code, OID4VCI behavior, consumer, #7, or #168 change is introduced.

## Consequences

- The two current mapping sites become one complete record per variant and the
  wildcard kind fallback is eliminated.
- All 51 behavioral rows remain; this is cohesion and review-locality work, not
  decision deduplication. Total production lines may increase and are reported.
- No catalogue owns more than 16 rows, while the public surface remains
  directly readable and stable.
- A new variant cannot silently inherit `InvalidInput`; compilation requires an
  explicit record and kind.
- Later crates still decide their own irreducible private shape. No workspace
  error framework is established.

## Alternatives

Keeping the two matches preserves behavior but retains duplicated mapping and
the wildcard fallback. Reusing either prior record repeats invariant fields or
cannot express kind. A shared trait/crate, public generator, procedural macro,
or build-time schema adds coupling and a release/tooling axis. Adding broader
JOSE domains or reclassifying kinds changes behavior. All are rejected here.

## Verification and rollback

Verification requires the exact 51-row regression (including the previously
unnamed `SizeOverflow` case), empty public API and manifest/lock diffs, const
and source-free checks, current conformance/vector/redaction tests, strict
quality gates, direct supported-target compilation, and factory/Nix mutation
evidence. Rollback restores the explicit matches and removes only private
catalogue/test infrastructure; no persisted data or wire migration exists.
