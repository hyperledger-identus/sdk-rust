# Constraints and limitations

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/539
Constraint blockers: none

## Existing entries affected

ADRs 0118 and 0137 govern the existing `JoseError` contract. ADRs 0181 and
0192 govern the engine-neutral JWE port and production-adapter qualification.
This slice changes none of those supported or deferred surfaces.

## Introduced or changed constraints

The eight base JWE error rows are immutable and ordered. Their stable codes,
kinds, capability, messages, displays, source behavior, and absence of public
code constants cannot drift without a later compatibility decision. Future
additive JWE variants append after the v1 prefix and own complete tests.

The planning and stable fixtures must remain byte-identical, fixed-hash,
regular files inside the repository, and resolvable from the exact
preimplementation Git blob. Golden generation from production code is
prohibited.

## Introduced or changed limitations

The contract does not provide a production JWE engine, broader algorithms,
engine diagnostics, localization, serialization, retry metadata, FFI, or
downstream language mappings. It does not make an interoperability or
cryptographic correctness claim.

## Consumer and product impact

None. Existing callers observe identical enum variants, conversions, displays,
codes, and wire behavior. No downstream repository changes.

## Activation and rollback

Activation requires exact #539 preflight, ADR 0195, immutable fixture evidence,
focused and workspace gates, independent CLI review, and green protected CI.
Rollback removes behavior-neutral evidence and the private record helper only.

## Evidence

Issue #539, ADRs 0118, 0137, 0181, 0192, and 0195, the exact-base planning
golden, preimplementation receipt, stable fixture tests, factory mutation suite,
workspace gates, independent CLI review, and protected CI are the evidence
authorities.
