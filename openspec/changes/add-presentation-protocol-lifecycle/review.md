# Pre-implementation semantic, API and lifecycle review

- **Date:** 2026-09-05
- **Issue:** #85 under `IDR-008` / #20
- **Develop base:** `8fb533562d5b006151372e214a38ef8a7e3fa5bd`
- **Result:** contract is implementable with no unresolved blocker

## Findings

1. IDR-008's request, disclosure, artifact and receipt values now exist; the
   only missing promised type family is protocol state.
2. Oxid proves that a reusable vocabulary needs cancellation-requested as a
   non-terminal phase and must permit completion to win a cancellation race.
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
11. Exhaustive 11-by-11 transition tests are cheap and stronger than selected
    examples. The implementation can stay allocation-free and constant-time.
12. Apollo and NeoPRISM provide no presentation lifecycle surface to port.
    midnight-identity and Lace remain independent consumer/boundary evidence.

Verdict: READY to implement after ADR 0030 and strict structural validation
pass.

# Post-implementation architecture, API and lifecycle review

- **Date:** 2026-09-05
- **Reviewed production head:** `4814a03fbe7b504bfeb3de9903f3a36eeced7d8b`
- **Exact diff:** `develop@8fb5335...4814a03`
- **Result:** no unresolved finding

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
5. The transition match implements exactly the 28 allowed edges frozen in the
   specification. A separate 11-by-11 table test checks all 121 pairs rather
   than reproducing only positive examples.
6. Terminal states reject every outgoing edge. Refusal is accepted only before
   generation; generation cannot skip `ready` and delivery; backward and self
   transitions fail.
7. `cancellation_requested` may become either `cancelled` or `completed`, so a
   late cancellation never fabricates rollback after irreversible delivery.
8. Phase, outcome and state spellings round-trip exactly. Unknown, padded,
   differently cased and legacy `succeeded` spellings fail through distinct
   zero-data errors with static SDK contracts.
9. The transition and parsing paths allocate no memory and perform no external
   access. The pinned release diagnostic observed about 505 million transition
   decisions per second without defining a portable threshold.
10. Focused, workspace, factory and all 26 compatible local Nix checks passed,
    including Rust 1.85 MSRV and 340 principal tests. Consumer/donor postflight
    revisions and pre-existing status entries match preflight.

Verdict: READY for specification synchronization and pull-request review.
