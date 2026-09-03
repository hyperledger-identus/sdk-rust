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

# Post-implementation semantic, security and API review

- **Reviewed head:** `b4ef090a3ffdd2b0832bc788fe825cad5c16cf22`
- **Review completed:** 2026-09-03T05:28:31Z
- **Result:** passed with no unresolved finding
- **Effort:** approximately 87 minutes from issue creation to reviewed,
  fully-gated implementation head

The exact diff from the recorded `develop` base was re-read after focused,
workspace, feature and all 26 Nix gates. The decorator remains opt-in and
preserves the existing resolver port. Disabled policy and `noCache: true`
avoid all generic cache and clock work; other requests use an exact DID plus
canonicalized result-affecting options, with `noCache` excluded from identity.

Positive/deactivated and standard `notFound` results are the only cacheable
classes. Extension, unsupported and internal errors remain uncached. Hits are
checked against the requested DID, the current policy lifetime, exclusive
expiry and non-regressed monotonic time before use. Invalid hits are
invalidated; infrastructure failures follow the explicit bypass/fail-closed
policy; no document or version metadata is treated as TTL.

The public ports are object-safe and runtime-neutral. Entries cannot serialize
across monotonic epochs; key, TTL and declared backend capacity are bounded;
custom `Debug`, statuses and stable errors omit full DIDs, options, documents
and adapter details. The positional options-constructor change is acceptable
for the unpublished `0.0.0` crate and every workspace caller is updated.

Duplicate-safe concurrent fills are intentional. Cancellation-aware
single-flight remains outside this slice and is tracked by #50, gated on two
independent consumers proving matching semantics. No downstream repository,
concrete cache, system clock, transport, chain or async executor enters the
diff.
