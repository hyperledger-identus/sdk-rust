# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/510
Constraint blockers: none

## Existing entries affected

`SDK-ARCH-001` and `SDK-ARCH-002` keep canonical SDK contracts product-neutral
and dependency direction inward. `SDK-SEC-003` requires non-weakenable bounds
and safe failure at changed input boundaries. `SDK-DELIVERY-001` requires this
issue-first, spec-driven correction after a blocking review. `SDK-LIM-005` and
`SDK-LIM-006` keep wallet policy and consumer adoption outside the slice. ADR
0170 remains the exact architecture authority. No indexed entry is weakened.

## Introduced or changed constraints

Every repository source path named by a language mapping must be relative,
resolve inside the repository, contain no symlink component below the resolved
root, name a bounded regular file, and resolve the declared public constant
exactly once. A validation error is bounded diagnostic data, never a traceback.

Cargo package identity, Rust API path, and public stable error code are distinct
concepts. Human rendering cannot concatenate them into a fabricated Rust path.

Mapping direction constrains field direction through a closed matrix. Every
lossy value or error record names each lost distinction, consequence, and
mitigation. Unsupported-value records remain separate.

A single pinned language revision supports only its exact patch interval in
schema v1. Every vector reference resolves exactly once in the canonical
catalog and agrees on capability and language target. The same vector may be
referenced by multiple conforming language mappings.

## Introduced or changed limitations

The corrected seed remains limited to SDK-TS 8.1.4 at revision
`4bf86ebf69d5e96616a148e4c973f831f95fa38e`. It does not claim other 8.x or 9.x
releases. Expanding a window requires newly pinned differential evidence.

The validator proves repository metadata integrity, not Rust compiler semantic
resolution, consumer compilation, binding generation, or runtime conformance.
It recognizes literal public `usize` constants only. Computed or private bounds
need a later schema decision rather than permissive evaluation.

## Consumer and product impact

No consumer behavior changes. SDK-TS, SDK-Swift, SDK-KMP, Oxid,
midnight-identity, and NeoPRISM remain untouched. The change improves the
contract later consumed by #492 without claiming that any adapter exists.

## Activation and rollback

Activation requires the issue #510 planning-only commit and exact preflight
receipt, focused implementation, mutation evidence for every confirmed gap,
full factory and Nix validation, one fresh independent review, and a green
exact-head replacement PR targeting `develop`. PR #509 remains draft and is
closed as superseded only after the replacement candidate is ready.

Before integration, rollback drops the #510 branch and leaves #509 blocked.
After the replacement PR merges, rollback follows a new mapping schema/version
and explicit replacement; it does not restore the known containment or
semantic defects.

## Evidence

Issue #510, PR #509 comment `5907933432`, ADR 0170, the canonical
`language-adapter-mappings` spec, `scripts/check-cross-language-vectors.py`, and
the landed cross-language vector catalog resolve the material decisions.
