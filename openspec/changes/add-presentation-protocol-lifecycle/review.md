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
