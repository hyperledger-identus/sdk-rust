# Post-implementation security, architecture and API review

- **Date:** 2026-09-16
- **Issue:** #168
- **Develop base:** `66ec2b9b3a7ec35cf21ecc52cdca5bebed0b4d0d`
- **Planning commits:** `b25c23d`, `c6c31eb`, and pre-JWK addendum `32dadd8`
- **Reviewed candidate:** staged implementation diff before archive
- **Result:** passed

## Findings and resolutions

1. **Coverage and ownership:** all thirteen packages classified `implemented`
   in the bootstrap inventory have at least one boundary-family row. The four
   dispositions distinguish typed SDK enforcement, fixed/no runtime input,
   caller-budgeted work, and allocation that occurs before a typed boundary.
   No row claims control the SDK does not own.
2. **Checker safety:** the checker is standard-library-only, offline, bounded
   to a 256 KiB inventory and 256 rows, rejects unknown schema, and validates
   repository-relative nonsymlink evidence without executing it. Its mutation
   suite covers missing package/evidence/limits, invalid classifications,
   duplicate IDs, unknown fields, missing paths, and oversized input.
3. **BIP-39 precedence:** entropy sizes are rejected before dependency entry;
   invalid word count and the English-list byte ceiling are rejected before
   joining; passphrase bytes are rejected before NFKD and PBKDF2. Standard and
   KMP paths share the same 4,096-byte policy and redacted error.
4. **JWK retained-resource safety:** native and serde construction converge on
   one validator before the map is retained or coordinates are decoded. Its
   borrowed iterative traversal bounds members, stack growth, depth, nodes,
   and aggregate key/string bytes without cloning attacker-controlled JSON.
5. **Outer-allocation honesty:** serde, Axum, UniFFI, JavaScript, and already
   owned Rust values can allocate before typed validation. The narrowed
   `SDK-LIM-007` preserves those obligations and keeps generic borrowed crypto
   work and injected async/storage adapters caller-budgeted.
6. **API compatibility:** public limit constants are additive. A proposed new
   JWK error variant was removed during review; resource failures reuse the
   existing redacted `ReservedExtension` variant, preserving the exhaustive
   enum shape and stable `IdentusError` contract. There is no wire or persisted
   representation change.
7. **Dependency and unsafe surface:** no Cargo dependency, feature default,
   native code, or authored unsafe Rust is added. The candidate API/SBOM gate
   completed with the pinned Rust 1.98.1 toolchain.
8. **Factory robustness:** Taplo formatting exposed spacing assumptions in the
   mutation test. The test now locates TOML assignments independent of
   alignment, and both its local and Nix-isolated forms pass.

## Residuals

- Supporting another BIP-39 language requires a deliberate word-byte policy
  review; the current eight-byte ceiling is explicitly English-list scoped.
- Consumers must continue applying preallocation, transport, timeout, retry,
  cancellation, and backend quotas at the earliest boundary they own.
- A newly discovered omitted family restores the broad limitation immediately
  until the inventory and executable evidence are corrected.

These are documented ownership constraints, not unresolved implementation
findings.

## Hosted review addendum

Review of PR #296 accepted three findings before further remediation: the
inventory omitted the DID method registry and resolution-cache families, and
native JWK rejection could recursively destroy a hostile deep owned JSON tree.
The inventory now has distinct method-registry, portable cache-policy, and
injected cache/clock adapter rows with exact source evidence. An internal guard
now owns the native extension map from constructor entry, dismantles nested
arrays and objects with an explicit work stack on every error path, and yields
the unchanged map only after successful validation. A depth-32,768 regression
exercises the profile-error path that precedes extension validation.

The correction adds no public type, dependency, authored unsafe code, wire
change, or accepted-resource expansion. Focused, workspace, candidate, factory,
and compatible Nix checks pass. The corrected head is published and all three
hosted review threads have evidence-backed replies and are resolved.

A later hosted review found the public `identus_did::Multihash` compatibility
placeholder. Its infallible `Vec<u8>` construction and transparent hex serde
retain unbounded caller input, so classifying all DID values as bounded was
incorrect. Imposing a ceiling would change the existing public contract without
a named consumer or migration policy and conflict with ADR 0082. The audit now
uses a fifth `known-unbounded-compatibility` disposition, adds a dedicated row,
and names the exception in `SDK-LIM-007`. Consumers must cap bytes or hex text
before entry and must not expose the placeholder directly to hostile input.

Another hosted review identified recursive destruction after borrowed resource
validation rejects an already-owned hostile-depth DID JSON tree. The pattern is
not limited to the three named constructors, so a partial cleanup would leave
the family misleadingly covered. The DID JSON inventory row and `SDK-LIM-007`
now preserve the full native owned-JSON rejection-cleanup limitation and require
bounded wire-slice parsing or an equivalent caller depth bound. Accepted typed
values remain bounded; no runtime behavior or public API changes in this PR.
Issue #297 owns comprehensive iterative cleanup and removal of only this clause.

The final hosted review found that package-level coverage still concealed a DID
option-map family and several credential families. A repository-wide comparison
of public `MAX_`/`MIN_` constants to inventory limits found the same structural
weakness in DID document/result/registration, presentation, and HTTP rows. The
inventory then had 36 cohesive families and named every public resource constant
declared by implemented packages. The bounded offline checker scans those
package sources and fails on any future omission; its mutation suite proves the
new failure mode. Verification-only packages remain outside runtime coverage.

A final matcher review found two `DEFAULT_MAX_` JOSE constants outside the
initial `MAX_`/`MIN_` prefix rule. Discovery now recognizes those tokens as name
segments, and the JOSE row includes header-string and proof-claim-string limits.
A broad repository comparison leaves only the verification-only conformance
crate outside the runtime inventory, as intended.

The last exact-head review identified two compatibility paths missed by
constant scanning. `HexStr::from` and `Base64UrlStrNoPad::from` retain encodings
of arbitrary byte slices, while directly constructible `JwsKeyReference` and
`Oid4vciProofJwtClient` variants retain arbitrary strings or collection
cardinality before later validation. The audit now records both as distinct
`known-unbounded-compatibility` families, expands `SDK-LIM-007`, and assigns
validated public migrations to issues #298 and #299. No runtime behavior or
public API changes in this audit correction.
