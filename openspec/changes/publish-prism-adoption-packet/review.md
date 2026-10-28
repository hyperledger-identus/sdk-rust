# Exact-diff architecture and adoption review

- **Review date:** 2026-10-28
- **Issue:** #554
- **Develop base:** `c763c3bd0020f9c726eb08f390629a1b353e5e01`
- **Planning commit:** `800686cbd3e8f0d1f6de7305d68d35976d791233`
- **Implementation commit:** `bc329b0e309252c552b6fb1b194ae3d18d22c4c2`
- **Reviewed head:** `7dc3722464bb1432fc972a45f85d1274661406ce`
- **Reviewer:** local Codex CLI, exact `origin/develop..HEAD` diff
- **Result:** passed after findings were remediated; no unresolved actionable
  blocker, high, or medium finding

## Review focus

The review covered crate responsibility and dependency direction, effect and
secret ownership, the atomic indexing contract, replay semantics, roadmap
truthfulness, immutable downstream evidence, handbook navigation, and the
readability and boundary coverage of the deterministic E2E test.

## Resolved findings

1. The first documentation draft referred generally to NeoPRISM parity. It now
   pins the exact candidate revision, Preprod slot, manifest digest, summary,
   and both independent run receipts.
2. VDR lookup wording originally risked implying that the portable crate owns
   payload bytes. It now states that the VDR resolves authenticated references
   while the caller-owned content store retains payloads.
3. Private-key wording was scoped to the three PRISM crates instead of making
   an unsupported repository-wide claim.
4. The adjacent ownership-boundaries page and diagram still described every
   PRISM component as downstream-owned. They now distinguish portable
   method/indexing semantics in SDK-Rust from downstream ledger, source, store,
   submitter, runtime, and policy adapters.
5. Dependency-cone wording overstated the machine guard. The handbook now says
   the graph guard fixes internal SDK edges and denies prohibited dependency
   families, while a separate `cargo tree` receipt proves the observed cone.
6. IDR-040 initially used this closeout issue as its owner. Issue #555 now
   remains open as the durable research/specification owner for `did:key`,
   `did:jwk`, and `did:web` after #554 closes.

## Final decision

The final Codex CLI review reported that the change adds a focused integration
test and documentation without an apparent regression or actionable defect.
The E2E adapter remains test-only. No Cardano client, SQL store, HTTP service,
CLI, runtime, persistence policy, product policy, or downstream mutation enters
the SDK boundary.

