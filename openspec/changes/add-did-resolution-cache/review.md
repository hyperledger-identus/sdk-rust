# Pre-implementation semantic and misuse-resistance review

- **Date:** 2026-09-03
- **Issue:** #45 (child of #5 / `IDR-006`)
- **Develop base:** `2cef96ca6c986440d6a340fe1b1f999a5c113633`
- **Reviewed contract:** OpenSpec `add-did-resolution-cache` and ADR 0013
- **Result:** no unresolved blocker

## Findings

1. **Standards correction:** current W3C caching text requires representing
   optional `noCache`; `nextUpdate` is historical-version metadata, not TTL.
2. **Ownership:** time values/ports are generic foundation; cache policy and
   resolver decoration are generic DID infrastructure; system clocks and
   stores remain adapters.
3. **Interface segregation:** wall and monotonic clocks must be separate. DID
   cache expiry uses only monotonic ticks, preventing civil-clock rollback from
   extending security-sensitive entries.
4. **Identity:** a DID-only key is unsafe. All result-affecting options must
   participate, nested JSON objects must be normalized, and `noCache` must not
   fragment identity.
5. **Bounds:** key size, TTL and backend capacity require explicit ceilings.
   Existing result types supply structural/node bounds; entries are
   process-epoch only and non-serializable.
6. **Failure model:** a cache is optional infrastructure. Explicit bypass versus
   fail-closed policy avoids silently imposing one availability posture.
7. **Negative caching:** only `notFound` is stable enough for opt-in negative
   caching. Internal and unsupported failures could otherwise become a denial
   of service.
8. **Poisoning:** cached result identity must be revalidated before use;
   expiration, clock regression and invalid data all invalidate and refresh.
9. **Concurrency:** portable correctness does not require single-flight.
   Runtime-neutral blocking synchronization would be worse than explicit,
   duplicate-safe fills; cancellation-aware coalescing stays outer/later.
10. **Compatibility:** the option constructor change and additive APIs are
    acceptable in unpublished `0.0.0` crates. Existing uncached ports remain
    source- and behaviorally independent except for callers of the positional
    `ResolutionOptions::new` constructor, which are updated in this workspace.
11. **Provenance:** downstream repositories are immutable evidence only. No
    cache, clock implementation or fixture is copied.
