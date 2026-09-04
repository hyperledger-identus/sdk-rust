# Pre-implementation semantic, API, privacy and performance review

- **Date:** 2026-09-05
- **Issue:** #83 under `IDR-008` / #20
- **Develop base:** `9074f7f7490759763a683a8fd879dba3272e4ebc`
- **Result:** contract is implementable with no unresolved blocker

## Findings

1. The generic layer needs opaque produced bytes, not a universal presentation
   codec or proof type. OID4VP and VCDM explicitly permit multiple formats.
2. One artifact per selected credential would reject valid aggregated
   presentations. One or more query/handle bindings per single-format artifact
   supports both aggregate and one-to-one adapters with one small extra value.
3. Artifact bindings must identify disclosure selections rather than claims.
   Claim coverage was already validated by the plan and repeating it would
   create divergent sources of truth.
4. A disclosure plan currently has no retained request identity. Exact private
   request binding is required before later objects can reject changed
   verifier, purpose, challenge, filter or query context.
5. Every plan selection must occur exactly once across artifacts. Missing,
   unknown and repeated bindings are distinct integration failures and need
   stable errors.
6. Artifact format must match each bound query. Multi-binding artifacts are
   therefore limited to selections whose queries use one format.
7. Per-artifact and aggregate byte ceilings are both required. Applying only
   Oxid's 4 MiB item bound would permit a 256 MiB aggregate at the existing
   64-artifact limit.
8. Receipt creation and receipt input are separate responsibilities. The SDK
   may summarize a validated generation result, but only a consumer knows
   whether consent, transport, acknowledgement and persistence succeeded.
9. Receipt input needs the verifier, optional purpose and value-free plan
   entries to be useful. It must omit the replay challenge and artifact bytes
   to reduce retained secrets and correlation material.
10. Receipt entries keep opaque handles because they let the owner identify
    which local credentials were disclosed. They are private data, never
    diagnostic fields or wire identifiers.
11. Plan order is the stable semantic order for receipt entries. Artifact order
    belongs to the format adapter and must not silently reorder the receipt.
12. All new constructors can use checked lengths and bounded slice scans. No
    map, set, digest, codec, serializer or external dependency is justified.
13. Oxid and Midnight provide Apache-2.0 conceptual evidence. Lace provides
    response-shape evidence only because repository-level license metadata is
    absent. Apollo and NeoPRISM remain negative boundary evidence.

Verdict: READY to implement after ADR 0029 review and strict structural
validation passed.

# Post-implementation architecture, API, privacy and performance review

- **Date:** 2026-09-05
- **Reviewed production head:** `74d487d428f7d64f54fd8750e7a7642c91991ad9`
- **Exact diff:** `develop@9074f7f...74d487d`
- **Result:** no unresolved finding

## Exact-diff findings

1. The implementation remains in the existing credential-semantics crate and
   adds no manifest, lockfile, feature, serializer, codec, runtime, network,
   storage, chain, product or donor dependency.
2. The public surface is limited to artifact bindings, bounded opaque artifact
   bytes, an exact-plan generated result, and a value-free receipt input.
   Format adapters still own proof generation and representation.
3. A disclosure plan privately retains its exact request. Generated
   construction rejects any changed verifier, purpose, challenge, query,
   filter, claim or format before artifact correlation.
4. Each query/handle pair must name an existing plan selection, match its
   query format, occur in exactly one artifact and collectively cover every
   plan selection. One same-format artifact may deliberately aggregate several
   selected credentials.
5. Item count, binding count, per-artifact bytes and aggregate bytes are
   checked before bounded correlation scans. Checked addition rejects length
   overflow, and transferred payload vectors retain their allocations.
6. Receipt input derives only from a validated generated result and follows
   plan order independently of artifact order. It retains owner-private local
   handles but excludes challenge, artifact bytes, claim values, timestamp,
   outcome, transport state and trust or verification conclusions.
7. Debug implementations reveal only safe format/intent, count, length and
   optional-field presence. Tests pass verifier, purpose, challenge, query,
   handle, path and payload canaries through every new aggregate without a
   diagnostic leak; all new errors are static and redacted.
8. Positive tests cover one-to-one OID4VP-shaped output, aggregated
   Midnight-shaped output and unrelated open format identifiers. Negative
   tests cover exact bounds, total budget, duplicate/unknown/missing bindings,
   format and request mismatch, ordering, redaction and every error bridge.
9. The release diagnostic observed 250,000 complete
   request/candidate/plan/artifact/receipt-input flows in `477.063ms`, about
   524,040 flows/second on the pinned Apple Silicon toolchain. No portable
   threshold is encoded.
10. Focused, workspace and all 26 compatible local Nix checks passed,
    including Rust 1.85 MSRV and 334 principal tests. Final donor and consumer
    receipts match preflight, and no downstream repository changed.

Verdict: READY for specification synchronization and pull-request review.
