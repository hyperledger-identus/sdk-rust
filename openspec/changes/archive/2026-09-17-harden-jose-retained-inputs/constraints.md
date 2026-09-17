# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/299
Constraint blockers: none

## Existing entries affected

- `SDK-SEC-003` governs the changed retained JOSE input boundaries.
- `SDK-LIM-007` remains effective but loses only the direct JOSE retained-enum
  construction clause after complete evidence.
- Pre-entry allocation, native DID cleanup, caller-budgeted work, and the
  historical `identus_did::Multihash` exception remain unchanged.

## Introduced or changed constraints

Every public `JwsKeyReference::KeyId`, `JwsKeyReference::X5c`, and
`Oid4vciProofJwtClient::Identified` inhabitant SHALL carry an opaque value that
was validated under its applicable positive limit contract. Raw `String` and
`Vec<String>` payloads SHALL NOT be directly accepted by those public variants.
Using an accepted value under tighter builder limits SHALL revalidate it.

## Introduced or changed limitations

No runtime limitation is introduced. Caller, transport, generic Serde, FFI, or
JavaScript allocation before typed validation remains outside the guarantee.
The source migration is pre-release compatibility work, not a published support
or SemVer commitment.

## Consumer and product impact

Direct raw variant construction becomes a compile-guided migration to named
fallible constructors. Successful key references, client modes, protected
headers, claim JSON, compact tokens, and verifier behavior remain unchanged.
No inspected named consumer calls the affected sdk-rust constructors, and no
downstream repository is mutated.

## Activation and rollback

Activation requires constructor/parser equivalence, exact/one-over and API-
closure tests, workspace and portable-target evidence, fresh review, a signed
issue-linked PR, and green exact-head fast CI. Rollback restores the raw
variants and JOSE compatibility disclosure together.

## Evidence

Issue #299, ADR 0125, this OpenSpec research and compatibility decision, the
machine input-boundary inventory, API/SBOM and compile-fail evidence, tests,
review, and exact-head CI form the required chain.
