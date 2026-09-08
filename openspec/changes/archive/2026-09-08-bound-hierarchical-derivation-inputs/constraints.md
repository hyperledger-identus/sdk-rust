# Constraint and limitation impact

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/199
Constraint blockers: none

## Existing entries affected

`SDK-SEC-003` requires explicit resource ceilings before changing these
untrusted parser and cryptographic-work boundaries. `SDK-COMPAT-001` requires
newly rejected path, seed and depth classes to be recorded. `SDK-LIM-007`
remains effective because #199 covers hierarchical derivation, not the full
repository-wide inherited-bound audit.

## Introduced or changed constraints

No new machine constraint ID is needed. Hierarchical derivation text accepts
at most 4,096 UTF-8 bytes; parsed and consumed derivation paths contain at most
255 axes; BIP-32 and SLIP-0010 master seeds contain 16 through 64 bytes; and
stateful HD keys derive no child beyond depth 255. Resource checks execute
before input-proportional parse, HMAC or curve work.

Changing a value later requires a new material compatibility and resource-
budget decision. Chain- or wallet-specific policies must use a separate layer.

## Introduced or changed limitations

`SDK-LIM-007` will record this derivation evidence while staying effective for
other inherited surfaces. A caller can still allocate an oversized input or
construct an oversized typed path using the source-compatible infallible
append method; all cryptographic consumers reject that path before child work.
The API does not bound caller memory already spent, transport bodies or batch
request counts.

## Consumer and product impact

Callers must keep generic paths at or below 255 axes and must use 16–64-byte
seeds for BIP-32/SLIP-0010. Existing Apollo/BIP/SLIP/Cardano vectors are far
inside the limits. No Oxid, Midnight Identity, Apollo, NeoPRISM or Lace
repository is mutated or claimed migrated.

## Activation and rollback

Issue #199 is the decision authority. Activation requires focused boundary
tests, unchanged dependency/feature evidence, complete local Nix validation,
distinct review and green hosted CI before merge to `develop`. Rollback
restores prior unbounded behavior and the old `SDK-LIM-007` wording atomically.

## Evidence

Acceptance requires exact and one-over text/axis/seed/depth cases, resource-
first error precedence, no child cryptography for rejected Cardano paths,
preserved vectors and redacted errors, unchanged dependency/feature/unsafe/
native surfaces, canonical spec and constraint-index atomicity, full Nix
validation and hosted CI.
