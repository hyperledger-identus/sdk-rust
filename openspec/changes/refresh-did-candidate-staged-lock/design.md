# Design

Copy the review-only proposed lock bytes without manual normalization. Update
the descriptor and readiness-manifest digests in the same commit. Extend
existing candidate mutation coverage only if it does not already detect stale
or mismatched lock identity.

Run preparation from a clean exact revision into scratch outside every Git
worktree. The assembler must package twice, byte-compare archives, verify local
closure and profiles, and emit bounded API/SBOM receipts. The canonical
workspace manifests stay unchanged.

Failure remains fail-closed: unexpected package drift, source/checksum change,
advisory/license/source failure, MSRV/target regression, or non-deterministic
archives blocks #388 rather than weakening policy.
