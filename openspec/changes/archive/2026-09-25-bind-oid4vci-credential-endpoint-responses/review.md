# Exact-diff architecture and compatibility review

Review status: completed
Review date: 2026-09-25
Base: develop@97776294492057c7c601d03470b1eb9440a6f497
Implementation head: 0579e108cd7ef3bf91578369890483167be493b8
Reviewed head: 0579e108cd7ef3bf91578369890483167be493b8
Specification commit: eacb6d2f9fac2551fd197f8004398730ca6a9f44
Preimplementation receipt commit: 75e1432bcaaad65cf1060489290cb593a6d4e66f
Unresolved blockers: none

## Scope reviewed

The review inspected the complete base-to-head diff, issues #366 and #368,
ADR 0148, OpenID4VCI 1.0 Final sections 8.3 and 8.3.1.2, the immutable
preimplementation receipt, parser reuse, secret lifetime, status precedence,
resource ownership, redaction, compatibility, and focused/workspace/portable
verification.

## Findings

1. **One-shot request handling — accepted.** The new method consumes the exact
   `JwtCredentialRequest`, copies only its proof count, and drops the request
   before any remote Content-Type or body parsing. Its zeroizing bearer field
   and proof body therefore cannot be replayed through the typed state.
2. **Status-first classification — accepted.** Exact `200`, `202`, and `400`
   select only their immediate, deferred, and payload-error parsers. Every
   other status, including `401`, fails with one static error before untrusted
   fields are inspected, so RFC 6750 authorization errors are not mislabeled.
3. **Reuse and cohesion — accepted.** The immediate compatibility method and
   new `200` branch share one private binder. The other branches call their
   existing bounded response parsers; no JSON, media, count, or error behavior
   is forked.
4. **Proof-count continuity — accepted.** All accepted outcomes retain the
   originating count, and immediate credentials remain capped by it. The
   wrapper APIs expose only bounded response cores plus this local evidence.
5. **Resource and privacy boundary — accepted.** Composed limits contain three
   already validated branch policies. Status precedes media, media precedes
   body, and every Debug/error path excludes endpoint, bearer, proof,
   transaction, credential, description, and remote body values.
6. **Compatibility — accepted.** The public change is additive and unpublished.
   The old borrowed method remains behavior-compatible, the historical error
   golden remains unchanged, and the new fieldless error is appended after all
   prior live variants with an explicit wildcard-free mapping.
7. **Architecture — accepted.** The result proves structural one-shot response
   continuity and a proof-count bound only. It performs no HTTP and claims no
   provenance, authorization, trust, credential validation, recovery, retry,
   polling or storage policy. Issue #368 owns the distinct decision to retain
   authority for a deferred continuation.
8. **Delivery integrity — accepted.** All three branch commits verify with good
   OpenPGP signatures and DCO sign-offs. The roadmap points to open successor
   #368 and passes its live GitHub-state audit.

## Decomposition decision

The exact change spans 23 paths and 937 added lines, above the preferred
12-file guidance but below the 1,000-line guidance. Shipping code is one
155-line cohesive response-boundary module, a 38-line composed policy, one
small private extraction and a static error contract; 286 lines are focused
consumer-shaped tests. The other paths are mandatory OpenSpec research,
specification, ADR, review, and machine-readable inventory/roadmap evidence.
Splitting them would detach a secret-lifetime and status-classification change
from its governing contract. Deferred continuation is already decomposed to
#368, so no further functional split is warranted.

## Residual limitations

- HTTP origin, TLS, redirects, decompression, timeout and cancellation remain
  outer transport obligations.
- Tokens, proofs, transactions, datasets and credentials remain opaque and
  untrusted.
- Encrypted success and RFC 6750 authorization-error responses remain outside
  the accepted surface.
- This slice intentionally does not retain bearer/issuer/deferred-endpoint
  authority; issue #368 must decide that secret lifetime explicitly.
- Publication and downstream adoption remain separate release/consumer work.

## Review decision

The implementation is bounded, cohesive, one-shot, status-first,
redaction-safe, additive and reversible. No unresolved correctness, security,
privacy, compatibility, architecture, dependency or delivery finding remains.
