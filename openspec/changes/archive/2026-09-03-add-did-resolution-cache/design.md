# Design: injectable DID resolution cache and clocks

## Standards and evidence

The SDK's DID work pins the W3C DID Resolution v1 Candidate Recommendation
Draft dated 28 August 2026. The published 6 August Candidate Recommendation
Snapshot confirms that a resolver may keep generic or method-specific caches,
that `noCache` is optional, and that `true` requests a fresh VDR result. It also
warns that bypass can amplify denial of service.

The same draft says `created`, `updated`, `versionId`, `nextVersionId`,
`equivalentId` and `canonicalId` describe a document/version. `nextUpdate` may
appear when the resolved version is not latest and identifies the next Update
operation. None is a generic freshness lifetime. Configuration or
method-specific policy therefore supplies TTLs; metadata is retained in the
cached result but does not silently override policy.

Normative sources:

- <https://www.w3.org/TR/did-resolution/>, especially sections 4.1, 4.3,
  13.2, 13.4 and 13.7.2.
- <https://www.w3.org/TR/2026/CR-did-resolution-1.0-20260806/>.

Read-only donor evidence at fixed revisions:

| Repository | Revision | Relevant evidence |
| --- | --- | --- |
| NeoPRISM | `d6ad1ecade80757f08da4f9101d14c2fb1a4d02b` | object-safe resolver seam; no generic resolution cache |
| midnight-identity | `427f8571950c42967a18726cbcbefecc19ef8d79` | Midnight resolver and document metadata; no cache/clock port |
| Lace ID Portal | `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` | injectable wall clock and product TTL stores; no reusable DID cache |
| Oxid | `bfe3b481568dc738f0732c2b27548fab8721fd95` | fallible clock port and DID resolver injection; product-specific caches |
| Apollo | `ccee22bcd693e618b9b8ff3e15ed6f9c9156c27c` | no relevant DID cache; legacy evidence only |

Apache-2.0 provenance is present for NeoPRISM, midnight-identity, Oxid and
Apollo. Lace remains design evidence only because repository-level license
metadata was unresolved during the prior boundary review. No implementation or
fixture is copied.

## Foundation time boundary

`identus-core` owns `UnixTimestampMillis`, `MonotonicTimestampMillis` and
`DurationMillis`. Values use `u64`, checked addition/subtraction, and do not
read ambient time. `UnixTimestampMillis` may cross persistence/FFI boundaries;
monotonic timestamps deliberately do not serialize because their origin is a
process/boot epoch.

Two small synchronous object-safe ports satisfy interface segregation:

- `WallClock::now()` returns Unix epoch milliseconds.
- `MonotonicClock::now()` returns milliseconds from an opaque stable origin.

Both return a closed, redaction-safe `ClockError`. DID caching depends only on
`MonotonicClock`; credentials, sessions and protocol engines can opt into the
wall clock without coupling cache expiry to clock adjustments.

## W3C option and cache identity

`ResolutionOptions` gains `Option<bool> no_cache`, serialized as `noCache`.
Absent and `false` both permit caching, while `true` bypasses both reads and
writes. `noCache` is control input and never participates in cache identity.

`DidResolutionCacheKey` owns the validated DID and a bounded canonical byte
encoding of every other option. Extension object members are recursively
sorted before encoding, so semantically identical JSON object order cannot
split the cache or enable normalization bypass. The key implements equality
and hashing, but its `Debug` output exposes only encoded length and method;
callers receive no raw byte accessor. The original DID is available to cache
implementations solely for all-options invalidation.

The key is limited to 64 KiB. If an otherwise valid option map cannot produce
a bounded key, resolution continues uncached in bypass mode or fails closed by
the selected policy. No string concatenation or raw DID URL is used.

## Policy, entry and port

`DidResolutionCachePolicy` has optional positive and `notFound` TTLs plus
`CacheFailureMode::{Bypass, FailClosed}`. Positive TTL is limited to 24 hours;
negative TTL is limited to five minutes. Zero is rejected; `None` disables the
class. A fully disabled policy bypasses cache and clock.

`DidResolutionCacheEntry` owns a cloned bounded result, insertion tick and
exclusive expiry tick. It is fresh only when `inserted_at <= now < expires_at`.
Clock regression or another epoch therefore invalidates rather than extending
freshness. Entries do not serialize and are not portable across process epochs.

`DidResolutionCache` is an object-safe `Send + Sync` port with:

- a declared capacity between 1 and 4,096 entries;
- asynchronous lookup and store operations;
- per-key invalidation;
- all-options-for-DID invalidation.

Operations return the shared redaction-safe SDK error surface. A concrete
memory, encrypted, disk or distributed backend remains an outer adapter and
must enforce its declared capacity.

## Resolver decorator

`CachingDidResolver` owns `Arc` handles to an upstream resolver, cache and
monotonic clock plus immutable policy. It exposes the ordinary `DidResolver`
port and an opt-in `resolve_with_cache_status` method whose status contains no
DID or adapter detail.

Resolution proceeds as follows:

1. Bypass immediately if policy is disabled or `noCache` is `true`.
2. Read a monotonic tick and construct the normalized key.
3. On a hit, require a fresh same-epoch entry and `validate_for(requested_did)`.
   Otherwise invalidate it and continue to the upstream resolver.
4. Resolve upstream without changing inputs or result envelope.
5. Cache success/deactivation under the positive TTL or a `notFound` failure
   under the negative TTL. Never cache unsupported, invalid, transport-shaped
   or internal failures.
6. Read the clock again after upstream work so TTL starts when the result is
   stored, using checked expiry arithmetic.

Cache/clock/key/invalidation/store failures either fall back to uncached
resolution/return the already obtained result (`Bypass`) or produce a valid W3C
`internalError` result (`FailClosed`). Invalid cached content is never served
under either mode.

Statuses distinguish disabled/request bypass, hit, fresh miss stored, fresh
miss not stored, expired/invalid refresh, and backend bypass without carrying
request data. The standard `DidResolver` method discards status only.

## Concurrency and cancellation

All contracts are `Send + Sync`; immutable wrapper clones share adapters with
`Arc`. Concurrent hits are safe. Concurrent misses may invoke upstream more
than once and stores must tolerate last-write-wins entries ordered by monotonic
insertion time. This is explicit rather than an accidental promise.

A portable single-flight lease protocol would need cancellation recovery,
wakeups, timeouts and executor integration. Blocking mutex/condition-variable
coordination in an async resolver is unacceptable. A runtime-specific outer
coordinator may wrap the upstream or cache; a focused follow-up can standardize
one once two consumers prove the same requirement.

## Threat and misuse contract

- Canonical option encoding prevents JSON member-order cache splitting.
- Exact validated DID identity prevents prefix/fallback collisions.
- Key/entry/capacity/TTL bounds limit memory and stale-key exposure.
- `noCache` is explicit and observable but can amplify origin load; products
  may wrap the resolver with rate limits or reject the feature per W3C.
- Only `notFound` supports negative caching, and only when explicitly enabled.
- Cache poisoning is contained by typed entries plus request validation.
- Clock rollback/restart never extends an entry.
- Debug/status/errors do not expose full DIDs, option values, documents or
  adapter errors.
- Explicit DID invalidation supports successful update/deactivation flows; the
  future registrar does not gain an implicit cache dependency.

## Alternatives rejected

- **Use `nextUpdate` as TTL:** contradicts current W3C historical-version
  semantics.
- **Read `SystemTime`/`Instant` inside the decorator:** harms deterministic
  tests, FFI targets and dependency inversion.
- **Cache only by DID:** aliases distinct representations, versions and method
  options.
- **Cache every failure:** turns transient/internal failures into durable
  denial of service.
- **Embed an in-memory cache:** selects product capacity/eviction/runtime and
  leaves encrypted/mobile/server consumers unable to substitute storage.
- **Hide single-flight behind blocking synchronization:** risks executor stalls
  and cancellation deadlocks.

## Rollback

Revert issue #45's pull request. The new APIs are opt-in, unpublished, contain
no concrete persistence and leave existing resolver behavior unchanged.
