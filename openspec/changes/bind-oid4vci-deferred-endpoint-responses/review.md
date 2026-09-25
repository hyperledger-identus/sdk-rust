# Exact-diff architecture and compatibility review

Review status: completed
Review date: 2026-09-25
Base: develop@f42b2a5862f541ff68d6a5c8b9b270fbc600924e
Implementation head: 5aec47807b15dbf90f46f632d7f260d7e3d07bf1
Reviewed head: 5aec47807b15dbf90f46f632d7f260d7e3d07bf1
Specification commit: 957f3b17fc563dd368df9777d591cbcd3733a387
Preimplementation receipt commit: b8cd06fa672a87c22e1a88a4f63872654083f977
Unresolved blockers: none

## Scope reviewed

The review inspected the complete base-to-head diff, issues #370 and #372,
ADR 0150, OpenID4VCI 1.0 Final sections 8.3, 9.2 and 9.3, the immutable
preimplementation receipt, status/error precedence, authority ownership,
secret lifetime, transaction correlation, proof-count cardinality, parser
reuse, public compatibility, and focused/workspace/portable verification.

## Findings

1. **Closed classification — accepted.** The public consuming method exposes
   only issued, pending and deferred-error outcomes for exact 200, 202 and 400
   statuses. Unsupported statuses return the established static Deferred
   Credential HTTP status error before media/body parsing.
2. **Terminal erasure — accepted.** The 200 and 400 branches copy only the
   nonsensitive proof count and drop the complete request before invoking any
   remote parser. Unsupported statuses likewise drop it first. No terminal
   result owns issuer, endpoint, Authorization, transaction or request bytes.
3. **Continuing authority — accepted.** The 202 branch destructures the request
   and explicitly drops its serialized body before parsing. It retains only
   the zeroizing request transaction for equality and moves the exact issuer,
   endpoint, Authorization and proof count. Any media, body or correlation
   error drops that authority; exact success reuses the existing bound response
   and next-request transition without replacement inputs.
4. **Proof cardinality — accepted.** The issued branch reuses a single private
   request-bound immediate constructor shared with the original Credential
   Endpoint parser. It rejects more credentials than the exact proof count
   without changing borrowed Deferred Credential response behavior.
5. **Parser and limit cohesion — accepted.** The borrowed and consuming APIs
   share private successful/deferred-error parsing helpers. The new composite
   limits value contains only already validated child policies. There is no
   duplicate wire model or new error, dependency, feature, manifest or lockfile.
6. **Privacy — accepted.** All secret-bearing states remain non-Clone and
   non-Serde; Authorization, request bodies and transactions stay zeroizing.
   Debug and static/bridged errors were exercised with issuer, endpoint,
   bearer, transaction, credential and description canaries.
7. **Compatibility — accepted.** Existing borrowed validators retain their
   exact tests and semantics, including the intentionally unbound issued count
   on the legacy structural path. The public change is additive and the crate
   remains unpublished at version 0.0.0.
8. **Architecture — accepted.** The transition proves bounded syntax and owned
   request lineage only. It performs no HTTP, origin/TLS proof, token
   validation, interval/retry/polling behavior, credential verification/storage,
   trust or product policy. Issue #372 owns M4 conformance reconciliation rather
   than speculative new protocol behavior.
9. **Delivery integrity — accepted.** The planning, receipt and implementation
   commits have valid OpenPGP signatures and DCO sign-offs. The canonical
   backlog points to open successor #372 and passes its live GitHub-state audit.

## Decomposition decision

The exact change spans 24 paths and 1,181 changed text lines, exceeding both
review guidance thresholds. The functional center remains cohesive: 281 added
shipping-source lines across eight closely related files and one 393-line
consumer-shaped integration test. The remaining volume is the planning
contract, ADR, inventories, roadmap and immutable receipt. Splitting the
status classifier, authority decomposition, shared parser helpers or
proof-count binding would detach one branch-lifetime decision from its tests.
M4 conformance/fixture reconciliation is already separated into #372, so no
additional functional split is warranted.

## Residual limitations

- HTTP origin, TLS, redirects, decompression, timeout and cancellation remain
  outer transport obligations.
- Access tokens and transactions remain opaque and structurally correlated,
  not validated for audience, scope, freshness or replay.
- Interval scheduling, retries, polling loops and transaction invalidation
  effects remain outside the SDK transition.
- Encrypted responses and RFC 6750 authorization-error responses remain outside
  the current unencrypted profile.
- Credential verification/storage, publication and downstream adoption remain
  separate work.

## Review decision

The implementation is status-first, least-authority, one-shot, bounded,
redaction-safe, additive and reversible. No unresolved correctness, security,
privacy, compatibility, architecture, dependency or delivery finding remains.
