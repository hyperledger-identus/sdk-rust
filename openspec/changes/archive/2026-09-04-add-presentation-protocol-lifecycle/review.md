# Pre-implementation semantic, API and lifecycle review

- **Date:** 2026-09-05
- **Issue:** #85 under `IDR-008` / #20
- **Develop base:** `8fb533562d5b006151372e214a38ef8a7e3fa5bd`
- **Result:** contract is implementable with no unresolved blocker

## Findings

1. IDR-008's request, disclosure, artifact and receipt values now exist; the
   only missing promised type family is protocol state.
2. Oxid proves that a reusable vocabulary needs cancellation-requested as a
   non-terminal phase. The generic contract must also retain whether the
   request occurred before or during potentially irreversible delivery.
3. Oxid's `AwaitingConsent` is product language. `awaiting_authorization`
   preserves the external prerequisite without making the SDK a consent or
   authorization authority.
4. Oxid's `Presenting` combines proof generation, ready output and potentially
   irreversible delivery. Splitting those phases lets adapters cancel and
   recover truthfully.
5. `succeeded` would imply verifier/proof/trust success. `completed` is safer
   when documented as protocol-adapter terminality only.
6. Refusal is coherent only before generation. Later aborts are cancellation,
   expiration or failure.
7. Terminal states must be immutable. Allowing terminal rewrites would erase
   audit truth and invite optimistic reconciliation.
8. Self-transitions should be rejected; idempotent persistence replay can
   compare the copyable state before applying a transition.
9. A session identifier, timestamp, error detail or transport receipt would
   couple this vocabulary to IDR-010 storage and protocol/product policy.
10. Strict lowercase parsing is useful for adapters but is not a serialized
    format commitment. No serde dependency is warranted.
11. Exhaustive 12-by-12 transition tests are cheap and stronger than selected
    examples. The implementation can stay allocation-free and constant-time.
12. Apollo and NeoPRISM provide no presentation lifecycle surface to port.
    midnight-identity and Lace remain independent consumer/boundary evidence.

Verdict: READY to implement after ADR 0030 and strict structural validation
pass.

# Post-implementation architecture, API and lifecycle review

- **Date:** 2026-09-05
- **Reviewed production head:** `5b67401` after hosted-review correction
- **Exact diff:** `develop@8fb5335...5b67401`
- **Result:** initial hosted P1 resolved; no unresolved finding

## Exact-diff findings

1. The implementation is isolated in a new cohesive `lifecycle` module within
   the existing credential-semantics crate. It changes no manifest, lockfile,
   feature, dependency, unsafe-code, serializer, wire, runtime, transport,
   storage, chain or product surface.
2. Active phases and terminal outcomes are separate copyable enums. The
   composed state contains no identifier, request, verifier, credential,
   artifact, timestamp, transport result, error detail or audit evidence.
3. `awaiting_authorization` remains authority-neutral. No API records or
   validates user consent, custody approval, enterprise policy or agent
   authority.
4. `completed` is documented as adapter-reported terminality only. It cannot
   carry or imply proof validity, verifier acceptance, credential trust,
   acknowledgement or receipt persistence.
5. The transition match implements exactly the 31 allowed edges frozen in the
   corrected specification. A separate 12-by-12 table test checks all 144
   pairs rather than reproducing only positive examples.
6. Terminal states reject every outgoing edge. Refusal is accepted only before
   generation; generation cannot skip `ready` and delivery; generation-time
   cancellation cannot become completion; backward and self transitions fail.
7. `generation_cancellation_requested` and
   `delivery_cancellation_requested` preserve the minimum cancellation origin.
   Only the latter may become `completed`, so a late delivery cancellation
   never fabricates rollback and a pre-delivery cancellation cannot fabricate
   handoff.
8. Phase, outcome and state spellings round-trip exactly. Unknown, padded,
   differently cased and legacy `succeeded` spellings fail through distinct
   zero-data errors with static SDK contracts.
9. The transition and parsing paths allocate no memory and perform no external
   access. The pinned release diagnostic observed about 563 million transition
   decisions per second without defining a portable threshold.
10. Focused, workspace, factory and all 27 compatible local Nix checks passed,
    including Rust 1.85 MSRV and 340 principal tests. Consumer/donor postflight
    revisions and pre-existing status entries match preflight.

Verdict: READY for specification synchronization and pull-request review.

## Hosted review correction

The first hosted Codex review of PR #86 at `99475e0` found one P1: the shared
origin-free cancellation phase admitted `generating -> cancellation_requested
-> completed`. That path could report completion without reaching `ready` or
`delivering`.

Issue #85 received a specification-amendment receipt before corrective code.
Commit `4345156` split the normative vocabulary and transition table; commit
`5b67401` implemented the split and expanded the independent matrix. Focused,
workspace, factory, release-performance and complete Nix gates then passed.
The correction adds no payload or dependency: two unit variants carry only the
one bit of semantic origin required to distinguish reversible generation from
potentially irreversible delivery.
