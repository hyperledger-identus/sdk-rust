use std::{
    collections::{BTreeMap, HashMap},
    future::Future,
    hint::black_box,
    pin::pin,
    sync::{
        Arc, Barrier, Mutex,
        atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering},
    },
    task::{Context, Poll, Wake, Waker},
    time::Instant,
};

use identus_core::{
    CapabilityId, ClockError, DurationMillis, ErrorCode, ErrorKind, IdentusError, MonotonicClock,
    MonotonicTimestampMillis,
};
use identus_did::{
    CacheError, CacheFailureMode, CachingDidResolver, Did, DidDocument, DidDocumentMetadata,
    DidResolutionCache, DidResolutionCacheEntry, DidResolutionCacheKey,
    DidResolutionCacheLookupFuture, DidResolutionCachePolicy, DidResolutionCacheStatus,
    DidResolutionCacheWriteFuture, DidResolutionDateTime, DidResolutionError,
    DidResolutionErrorKind, DidResolutionFuture, DidResolutionMetadata, DidResolutionResult,
    DidResolver, Error, MAX_DID_RESOLUTION_CACHE_ENTRIES, MAX_DID_RESOLUTION_CACHE_KEY_BYTES,
    MAX_DID_RESOLUTION_NOT_FOUND_TTL_MILLIS, MAX_DID_RESOLUTION_POSITIVE_TTL_MILLIS,
    ResolutionOptions, Uri, VersionId,
};
use serde_json::{Map, Value, json};

struct NoopWake;

impl Wake for NoopWake {
    fn wake(self: Arc<Self>) {}
}

fn block_on<F: Future>(future: F) -> F::Output {
    let waker = Waker::from(Arc::new(NoopWake));
    let mut context = Context::from_waker(&waker);
    let mut future = pin!(future);
    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(output) => return output,
            Poll::Pending => std::thread::yield_now(),
        }
    }
}

fn backend_error() -> IdentusError {
    IdentusError::public(
        ErrorCode::new("test.cache_unavailable"),
        ErrorKind::Storage,
        CapabilityId::new("did"),
        "cache unavailable",
    )
}

#[derive(Default)]
struct FakeClock {
    now: AtomicU64,
    reads: AtomicUsize,
    fail: AtomicBool,
}

impl FakeClock {
    fn at(now: u64) -> Self {
        Self {
            now: AtomicU64::new(now),
            reads: AtomicUsize::new(0),
            fail: AtomicBool::new(false),
        }
    }

    fn set(&self, now: u64) {
        self.now.store(now, Ordering::SeqCst);
    }
}

impl MonotonicClock for FakeClock {
    fn now(&self) -> Result<MonotonicTimestampMillis, ClockError> {
        self.reads.fetch_add(1, Ordering::SeqCst);
        if self.fail.load(Ordering::SeqCst) {
            Err(ClockError::Unavailable)
        } else {
            Ok(MonotonicTimestampMillis::new(
                self.now.load(Ordering::SeqCst),
            ))
        }
    }
}

struct MemoryCache {
    capacity: usize,
    entries: Mutex<HashMap<DidResolutionCacheKey, DidResolutionCacheEntry>>,
    lookups: AtomicUsize,
    stores: AtomicUsize,
    invalidations: AtomicUsize,
    fail_lookup: AtomicBool,
    fail_store: AtomicBool,
    fail_invalidation: AtomicBool,
}

impl MemoryCache {
    fn new(capacity: usize) -> Self {
        Self {
            capacity,
            entries: Mutex::new(HashMap::new()),
            lookups: AtomicUsize::new(0),
            stores: AtomicUsize::new(0),
            invalidations: AtomicUsize::new(0),
            fail_lookup: AtomicBool::new(false),
            fail_store: AtomicBool::new(false),
            fail_invalidation: AtomicBool::new(false),
        }
    }

    fn insert(&self, key: DidResolutionCacheKey, entry: DidResolutionCacheEntry) {
        self.entries.lock().unwrap().insert(key, entry);
    }

    fn len(&self) -> usize {
        self.entries.lock().unwrap().len()
    }
}

impl DidResolutionCache for MemoryCache {
    fn capacity(&self) -> usize {
        self.capacity
    }

    fn lookup<'a>(&'a self, key: &'a DidResolutionCacheKey) -> DidResolutionCacheLookupFuture<'a> {
        Box::pin(async move {
            self.lookups.fetch_add(1, Ordering::SeqCst);
            if self.fail_lookup.load(Ordering::SeqCst) {
                Err(backend_error())
            } else {
                Ok(self.entries.lock().unwrap().get(key).cloned())
            }
        })
    }

    fn store<'a>(
        &'a self,
        key: DidResolutionCacheKey,
        entry: DidResolutionCacheEntry,
    ) -> DidResolutionCacheWriteFuture<'a> {
        Box::pin(async move {
            self.stores.fetch_add(1, Ordering::SeqCst);
            if self.fail_store.load(Ordering::SeqCst) {
                return Err(backend_error());
            }
            let mut entries = self.entries.lock().unwrap();
            if entries.len() >= self.capacity && !entries.contains_key(&key) {
                let evicted = entries.keys().next().cloned();
                if let Some(evicted) = evicted {
                    entries.remove(&evicted);
                }
            }
            entries.insert(key, entry);
            Ok(())
        })
    }

    fn invalidate<'a>(
        &'a self,
        key: &'a DidResolutionCacheKey,
    ) -> DidResolutionCacheWriteFuture<'a> {
        Box::pin(async move {
            self.invalidations.fetch_add(1, Ordering::SeqCst);
            if self.fail_invalidation.load(Ordering::SeqCst) {
                Err(backend_error())
            } else {
                self.entries.lock().unwrap().remove(key);
                Ok(())
            }
        })
    }

    fn invalidate_did<'a>(&'a self, did: &'a Did) -> DidResolutionCacheWriteFuture<'a> {
        Box::pin(async move {
            self.invalidations.fetch_add(1, Ordering::SeqCst);
            if self.fail_invalidation.load(Ordering::SeqCst) {
                Err(backend_error())
            } else {
                self.entries
                    .lock()
                    .unwrap()
                    .retain(|key, _| key.did() != did);
                Ok(())
            }
        })
    }
}

struct FixedResolver {
    result: DidResolutionResult,
    calls: AtomicUsize,
    barrier: Option<Arc<Barrier>>,
}

impl FixedResolver {
    fn new(result: DidResolutionResult) -> Self {
        Self {
            result,
            calls: AtomicUsize::new(0),
            barrier: None,
        }
    }

    fn with_barrier(result: DidResolutionResult, barrier: Arc<Barrier>) -> Self {
        Self {
            result,
            calls: AtomicUsize::new(0),
            barrier: Some(barrier),
        }
    }
}

impl DidResolver for FixedResolver {
    fn resolve<'a>(
        &'a self,
        _did: &'a Did,
        _options: &'a ResolutionOptions,
    ) -> DidResolutionFuture<'a> {
        Box::pin(async move {
            self.calls.fetch_add(1, Ordering::SeqCst);
            if let Some(barrier) = &self.barrier {
                barrier.wait();
            }
            self.result.clone()
        })
    }
}

fn success(did: &Did, source: &str, next_update: Option<&str>) -> DidResolutionResult {
    let metadata = DidResolutionMetadata::new(
        None,
        None,
        BTreeMap::from([("source".to_owned(), json!(source))]),
    )
    .unwrap();
    let mut document_metadata = DidDocumentMetadata::builder();
    if let Some(next_update) = next_update {
        document_metadata =
            document_metadata.next_update(DidResolutionDateTime::parse(next_update).unwrap());
    }
    DidResolutionResult::success(
        metadata,
        DidDocument::builder(did.clone()).build().unwrap(),
        document_metadata.build().unwrap(),
    )
    .unwrap()
}

fn failure(kind: DidResolutionErrorKind) -> DidResolutionResult {
    DidResolutionResult::failure(
        DidResolutionMetadata::new(
            None,
            Some(DidResolutionError::standard(kind)),
            BTreeMap::new(),
        )
        .unwrap(),
    )
    .unwrap()
}

fn extension_failure() -> DidResolutionResult {
    DidResolutionResult::failure(
        DidResolutionMetadata::new(
            None,
            Some(
                DidResolutionError::new(
                    Uri::parse("https://example.com/problems/transient").unwrap(),
                    None,
                    None,
                    None,
                    BTreeMap::new(),
                )
                .unwrap(),
            ),
            BTreeMap::new(),
        )
        .unwrap(),
    )
    .unwrap()
}

fn policy(mode: CacheFailureMode) -> DidResolutionCachePolicy {
    DidResolutionCachePolicy::new(
        Some(DurationMillis::new(10)),
        Some(DurationMillis::new(5)),
        mode,
    )
    .unwrap()
}

fn compose(
    resolver: Arc<dyn DidResolver>,
    cache: Arc<MemoryCache>,
    clock: Arc<FakeClock>,
    policy: DidResolutionCachePolicy,
) -> CachingDidResolver {
    CachingDidResolver::new(resolver, cache, clock, policy).unwrap()
}

#[test]
fn cache_key_normalizes_json_and_excludes_no_cache_control() {
    let did = Did::parse("did:prism:private-subject").unwrap();
    let mut left_nested = Map::new();
    left_nested.insert("z".to_owned(), json!(1));
    left_nested.insert("a".to_owned(), json!({"right": 2, "left": 1}));
    let mut right_nested = Map::new();
    right_nested.insert("a".to_owned(), json!({"left": 1, "right": 2}));
    right_nested.insert("z".to_owned(), json!(1));

    let left = ResolutionOptions::builder()
        .no_cache(false)
        .extensions(BTreeMap::from([(
            "network".to_owned(),
            Value::Object(left_nested),
        )]))
        .build()
        .unwrap();
    let right = ResolutionOptions::builder()
        .no_cache(true)
        .extensions(BTreeMap::from([(
            "network".to_owned(),
            Value::Object(right_nested),
        )]))
        .build()
        .unwrap();
    let left_key = DidResolutionCacheKey::for_resolution(&did, &left).unwrap();
    let right_key = DidResolutionCacheKey::for_resolution(&did, &right).unwrap();
    assert_eq!(left_key, right_key);
    assert!(left_key.encoded_len() <= MAX_DID_RESOLUTION_CACHE_KEY_BYTES);

    let versioned = ResolutionOptions::builder()
        .version_id(VersionId::parse("2").unwrap())
        .extensions(right.extensions().clone())
        .build()
        .unwrap();
    assert_ne!(
        left_key,
        DidResolutionCacheKey::for_resolution(&did, &versioned).unwrap()
    );

    let debug = format!("{left_key:?}");
    assert!(debug.contains("prism"));
    assert!(!debug.contains("private-subject"));
    assert!(!debug.contains("network"));

    let large = ResolutionOptions::builder()
        .extensions(BTreeMap::from([(
            "large".to_owned(),
            json!("x".repeat(64 * 1_024)),
        )]))
        .build()
        .unwrap();
    assert!(matches!(
        DidResolutionCacheKey::for_resolution(&did, &large),
        Err(Error::InvalidCache(CacheError::KeyTooLarge))
    ));
}

#[test]
fn policy_entry_and_backend_capacity_are_bounded_and_redacted() {
    for (positive, negative) in [
        (Some(DurationMillis::new(0)), None),
        (
            Some(DurationMillis::new(
                MAX_DID_RESOLUTION_POSITIVE_TTL_MILLIS + 1,
            )),
            None,
        ),
        (
            None,
            Some(DurationMillis::new(
                MAX_DID_RESOLUTION_NOT_FOUND_TTL_MILLIS + 1,
            )),
        ),
    ] {
        assert!(matches!(
            DidResolutionCachePolicy::new(positive, negative, CacheFailureMode::Bypass),
            Err(Error::InvalidCache(CacheError::InvalidTtl))
        ));
    }

    let did = Did::parse("did:example:123").unwrap();
    let result = success(&did, "origin", None);
    assert!(matches!(
        DidResolutionCacheEntry::new(
            result.clone(),
            MonotonicTimestampMillis::new(5),
            MonotonicTimestampMillis::new(5),
        ),
        Err(Error::InvalidCache(CacheError::InvalidEntryLifetime))
    ));

    for capacity in [0, MAX_DID_RESOLUTION_CACHE_ENTRIES + 1] {
        let error = CachingDidResolver::new(
            Arc::new(FixedResolver::new(result.clone())),
            Arc::new(MemoryCache::new(capacity)),
            Arc::new(FakeClock::at(1)),
            policy(CacheFailureMode::Bypass),
        )
        .unwrap_err();
        assert!(matches!(
            error,
            Error::InvalidCache(CacheError::InvalidCapacity)
        ));
        let public = error.to_identus_error();
        assert_eq!(public.code().as_str(), "did.invalid_resolution_cache");
        assert!(!public.to_string().contains("4097"));
    }
}

#[test]
fn positive_cache_uses_monotonic_ttl_not_document_metadata() {
    let did = Did::parse("did:midnight:undeployed:contract").unwrap();
    let result = success(&did, "origin", Some("2020-01-01T00:00:00Z"));
    let resolver = Arc::new(FixedResolver::new(result.clone()));
    let cache = Arc::new(MemoryCache::new(16));
    let clock = Arc::new(FakeClock::at(100));
    let caching = compose(
        resolver.clone(),
        cache.clone(),
        clock.clone(),
        policy(CacheFailureMode::Bypass),
    );
    let options = ResolutionOptions::empty();

    let first = block_on(caching.resolve_with_cache_status(&did, &options));
    assert_eq!(first.status(), DidResolutionCacheStatus::MissStored);
    assert_eq!(first.result(), &result);
    assert_eq!(resolver.calls.load(Ordering::SeqCst), 1);

    clock.set(109);
    let hit = block_on(caching.resolve_with_cache_status(&did, &options));
    assert_eq!(hit.status(), DidResolutionCacheStatus::Hit);
    assert_eq!(resolver.calls.load(Ordering::SeqCst), 1);

    clock.set(110);
    let refreshed = block_on(caching.resolve_with_cache_status(&did, &options));
    assert_eq!(refreshed.status(), DidResolutionCacheStatus::RefreshStored);
    assert_eq!(resolver.calls.load(Ordering::SeqCst), 2);
    assert_eq!(cache.invalidations.load(Ordering::SeqCst), 1);
}

#[test]
fn no_cache_true_bypasses_reads_writes_and_clock() {
    let did = Did::parse("did:prism:123").unwrap();
    let resolver = Arc::new(FixedResolver::new(success(&did, "origin", None)));
    let cache = Arc::new(MemoryCache::new(16));
    let clock = Arc::new(FakeClock::at(100));
    let caching = compose(
        resolver.clone(),
        cache.clone(),
        clock.clone(),
        policy(CacheFailureMode::Bypass),
    );
    let options = ResolutionOptions::builder().no_cache(true).build().unwrap();

    for _ in 0..2 {
        assert_eq!(
            block_on(caching.resolve_with_cache_status(&did, &options)).status(),
            DidResolutionCacheStatus::RequestedBypass
        );
    }
    assert_eq!(resolver.calls.load(Ordering::SeqCst), 2);
    assert_eq!(cache.lookups.load(Ordering::SeqCst), 0);
    assert_eq!(cache.stores.load(Ordering::SeqCst), 0);
    assert_eq!(clock.reads.load(Ordering::SeqCst), 0);
}

#[test]
fn negative_cache_is_opt_in_and_only_not_found_is_cacheable() {
    let did = Did::parse("did:example:missing").unwrap();
    let clock = Arc::new(FakeClock::at(20));
    let cache = Arc::new(MemoryCache::new(16));
    let not_found = Arc::new(FixedResolver::new(failure(
        DidResolutionErrorKind::NotFound,
    )));
    let caching = compose(
        not_found.clone(),
        cache,
        clock.clone(),
        policy(CacheFailureMode::Bypass),
    );
    let options = ResolutionOptions::empty();
    assert_eq!(
        block_on(caching.resolve_with_cache_status(&did, &options)).status(),
        DidResolutionCacheStatus::MissStored
    );
    clock.set(24);
    assert_eq!(
        block_on(caching.resolve_with_cache_status(&did, &options)).status(),
        DidResolutionCacheStatus::Hit
    );
    assert_eq!(not_found.calls.load(Ordering::SeqCst), 1);

    let internal = Arc::new(FixedResolver::new(failure(
        DidResolutionErrorKind::InternalError,
    )));
    let caching = compose(
        internal.clone(),
        Arc::new(MemoryCache::new(16)),
        clock,
        policy(CacheFailureMode::Bypass),
    );
    for _ in 0..2 {
        assert_eq!(
            block_on(caching.resolve_with_cache_status(&did, &options)).status(),
            DidResolutionCacheStatus::MissNotStored
        );
    }
    assert_eq!(internal.calls.load(Ordering::SeqCst), 2);

    let extension = Arc::new(FixedResolver::new(extension_failure()));
    let caching = compose(
        extension.clone(),
        Arc::new(MemoryCache::new(16)),
        Arc::new(FakeClock::at(24)),
        policy(CacheFailureMode::Bypass),
    );
    for _ in 0..2 {
        assert_eq!(
            block_on(caching.resolve_with_cache_status(&did, &options)).status(),
            DidResolutionCacheStatus::MissNotStored
        );
    }
    assert_eq!(extension.calls.load(Ordering::SeqCst), 2);
}

#[test]
fn backend_and_clock_failures_follow_explicit_policy() {
    let did = Did::parse("did:prism:123").unwrap();
    let options = ResolutionOptions::empty();
    let result = success(&did, "origin", None);

    let bypass_resolver = Arc::new(FixedResolver::new(result.clone()));
    let bypass_cache = Arc::new(MemoryCache::new(16));
    bypass_cache.fail_lookup.store(true, Ordering::SeqCst);
    let bypass = compose(
        bypass_resolver.clone(),
        bypass_cache,
        Arc::new(FakeClock::at(1)),
        policy(CacheFailureMode::Bypass),
    );
    let observed = block_on(bypass.resolve_with_cache_status(&did, &options));
    assert_eq!(observed.status(), DidResolutionCacheStatus::BackendBypass);
    assert_eq!(observed.result(), &result);
    assert_eq!(bypass_resolver.calls.load(Ordering::SeqCst), 1);

    let closed_resolver = Arc::new(FixedResolver::new(result));
    let closed_cache = Arc::new(MemoryCache::new(16));
    closed_cache.fail_lookup.store(true, Ordering::SeqCst);
    let closed = compose(
        closed_resolver.clone(),
        closed_cache,
        Arc::new(FakeClock::at(1)),
        policy(CacheFailureMode::FailClosed),
    );
    let observed = block_on(closed.resolve_with_cache_status(&did, &options));
    assert_eq!(observed.status(), DidResolutionCacheStatus::FailedClosed);
    assert_eq!(
        observed.result().metadata().error().unwrap().kind(),
        Some(DidResolutionErrorKind::InternalError)
    );
    assert_eq!(closed_resolver.calls.load(Ordering::SeqCst), 0);

    let failed_clock = Arc::new(FakeClock::at(1));
    failed_clock.fail.store(true, Ordering::SeqCst);
    let resolver = Arc::new(FixedResolver::new(success(&did, "origin", None)));
    let caching = compose(
        resolver.clone(),
        Arc::new(MemoryCache::new(16)),
        failed_clock,
        policy(CacheFailureMode::Bypass),
    );
    assert_eq!(
        block_on(caching.resolve_with_cache_status(&did, &options)).status(),
        DidResolutionCacheStatus::BackendBypass
    );
    assert_eq!(resolver.calls.load(Ordering::SeqCst), 1);
}

#[test]
fn store_invalidation_key_and_disabled_paths_follow_policy() {
    let did = Did::parse("did:prism:failure-paths").unwrap();
    let options = ResolutionOptions::empty();

    let store_cache = Arc::new(MemoryCache::new(16));
    store_cache.fail_store.store(true, Ordering::SeqCst);
    let store_resolver = Arc::new(FixedResolver::new(success(&did, "origin", None)));
    let bypass = compose(
        store_resolver.clone(),
        store_cache.clone(),
        Arc::new(FakeClock::at(1)),
        policy(CacheFailureMode::Bypass),
    );
    assert_eq!(
        block_on(bypass.resolve_with_cache_status(&did, &options)).status(),
        DidResolutionCacheStatus::BackendBypass
    );
    assert_eq!(store_resolver.calls.load(Ordering::SeqCst), 1);

    let closed = compose(
        Arc::new(FixedResolver::new(success(&did, "origin", None))),
        store_cache,
        Arc::new(FakeClock::at(1)),
        policy(CacheFailureMode::FailClosed),
    );
    assert_eq!(
        block_on(closed.resolve_with_cache_status(&did, &options)).status(),
        DidResolutionCacheStatus::FailedClosed
    );

    let invalidation_cache = Arc::new(MemoryCache::new(16));
    let key = DidResolutionCacheKey::for_resolution(&did, &options).unwrap();
    invalidation_cache.insert(
        key,
        DidResolutionCacheEntry::new(
            success(&did, "expired", None),
            MonotonicTimestampMillis::new(1),
            MonotonicTimestampMillis::new(2),
        )
        .unwrap(),
    );
    invalidation_cache
        .fail_invalidation
        .store(true, Ordering::SeqCst);
    let refresh = compose(
        Arc::new(FixedResolver::new(success(&did, "origin", None))),
        invalidation_cache,
        Arc::new(FakeClock::at(3)),
        policy(CacheFailureMode::Bypass),
    );
    assert_eq!(
        block_on(refresh.resolve_with_cache_status(&did, &options)).status(),
        DidResolutionCacheStatus::BackendBypass
    );

    let oversized = ResolutionOptions::builder()
        .extensions(BTreeMap::from([(
            "large".to_owned(),
            json!("x".repeat(64 * 1_024)),
        )]))
        .build()
        .unwrap();
    let key_bypass_resolver = Arc::new(FixedResolver::new(success(&did, "origin", None)));
    let key_bypass = compose(
        key_bypass_resolver.clone(),
        Arc::new(MemoryCache::new(16)),
        Arc::new(FakeClock::at(1)),
        policy(CacheFailureMode::Bypass),
    );
    assert_eq!(
        block_on(key_bypass.resolve_with_cache_status(&did, &oversized)).status(),
        DidResolutionCacheStatus::BackendBypass
    );
    assert_eq!(key_bypass_resolver.calls.load(Ordering::SeqCst), 1);
    let key_closed = compose(
        Arc::new(FixedResolver::new(success(&did, "origin", None))),
        Arc::new(MemoryCache::new(16)),
        Arc::new(FakeClock::at(1)),
        policy(CacheFailureMode::FailClosed),
    );
    assert_eq!(
        block_on(key_closed.resolve_with_cache_status(&did, &oversized)).status(),
        DidResolutionCacheStatus::FailedClosed
    );

    let disabled_cache = Arc::new(MemoryCache::new(16));
    disabled_cache.fail_lookup.store(true, Ordering::SeqCst);
    let disabled_clock = Arc::new(FakeClock::at(1));
    disabled_clock.fail.store(true, Ordering::SeqCst);
    let disabled_resolver = Arc::new(FixedResolver::new(success(&did, "origin", None)));
    let disabled = compose(
        disabled_resolver.clone(),
        disabled_cache.clone(),
        disabled_clock.clone(),
        DidResolutionCachePolicy::disabled(),
    );
    assert_eq!(
        block_on(disabled.resolve_with_cache_status(&did, &options)).status(),
        DidResolutionCacheStatus::Disabled
    );
    assert_eq!(disabled_resolver.calls.load(Ordering::SeqCst), 1);
    assert_eq!(disabled_cache.lookups.load(Ordering::SeqCst), 0);
    assert_eq!(disabled_clock.reads.load(Ordering::SeqCst), 0);
}

#[test]
fn poisoned_or_regressed_entries_are_invalidated_before_refresh() {
    let requested = Did::parse("did:prism:requested").unwrap();
    let poisoned = Did::parse("did:prism:poisoned").unwrap();
    let options = ResolutionOptions::empty();
    let key = DidResolutionCacheKey::for_resolution(&requested, &options).unwrap();
    let entry = DidResolutionCacheEntry::new(
        success(&poisoned, "poison", None),
        MonotonicTimestampMillis::new(200),
        MonotonicTimestampMillis::new(300),
    )
    .unwrap();
    let entry_debug = format!("{entry:?}");
    assert!(!entry_debug.contains("poisoned"));
    assert!(!entry_debug.contains("poison"));
    let cache = Arc::new(MemoryCache::new(16));
    cache.insert(key, entry);
    let resolver = Arc::new(FixedResolver::new(success(&requested, "origin", None)));
    let caching = compose(
        resolver.clone(),
        cache.clone(),
        Arc::new(FakeClock::at(250)),
        policy(CacheFailureMode::Bypass),
    );

    let observed = block_on(caching.resolve_with_cache_status(&requested, &options));
    assert_eq!(observed.status(), DidResolutionCacheStatus::RefreshStored);
    assert_eq!(
        observed.result().metadata().extensions()["source"],
        "origin"
    );
    assert_eq!(cache.invalidations.load(Ordering::SeqCst), 1);
    assert!(!format!("{observed:?}").contains("requested"));

    let key = DidResolutionCacheKey::for_resolution(&requested, &options).unwrap();
    cache.insert(
        key,
        DidResolutionCacheEntry::new(
            success(&requested, "old-epoch", None),
            MonotonicTimestampMillis::new(500),
            MonotonicTimestampMillis::new(600),
        )
        .unwrap(),
    );
    let regressed = compose(
        resolver,
        cache.clone(),
        Arc::new(FakeClock::at(100)),
        policy(CacheFailureMode::Bypass),
    );
    assert_eq!(
        block_on(regressed.resolve_with_cache_status(&requested, &options)).status(),
        DidResolutionCacheStatus::RefreshStored
    );
    assert_eq!(cache.invalidations.load(Ordering::SeqCst), 2);

    let key = DidResolutionCacheKey::for_resolution(&requested, &options).unwrap();
    cache.insert(
        key,
        DidResolutionCacheEntry::new(
            success(&requested, "overlong", None),
            MonotonicTimestampMillis::new(200),
            MonotonicTimestampMillis::new(300),
        )
        .unwrap(),
    );
    let overlong = compose(
        Arc::new(FixedResolver::new(success(&requested, "origin", None))),
        cache.clone(),
        Arc::new(FakeClock::at(205)),
        policy(CacheFailureMode::Bypass),
    );
    assert_eq!(
        block_on(overlong.resolve_with_cache_status(&requested, &options)).status(),
        DidResolutionCacheStatus::RefreshStored
    );
    assert_eq!(cache.invalidations.load(Ordering::SeqCst), 3);
}

#[test]
fn did_wide_invalidation_removes_every_option_variant() {
    let did = Did::parse("did:midnight:undeployed:contract").unwrap();
    let other = Did::parse("did:prism:other").unwrap();
    let result = success(&did, "origin", None);
    let other_result = success(&other, "origin", None);
    let cache = MemoryCache::new(16);
    for (target, options, result) in [
        (&did, ResolutionOptions::empty(), result.clone()),
        (
            &did,
            ResolutionOptions::builder()
                .version_id(VersionId::parse("2").unwrap())
                .build()
                .unwrap(),
            result,
        ),
        (&other, ResolutionOptions::empty(), other_result),
    ] {
        let key = DidResolutionCacheKey::for_resolution(target, &options).unwrap();
        block_on(
            cache.store(
                key,
                DidResolutionCacheEntry::new(
                    result,
                    MonotonicTimestampMillis::new(1),
                    MonotonicTimestampMillis::new(2),
                )
                .unwrap(),
            ),
        )
        .unwrap();
    }
    assert_eq!(cache.len(), 3);
    block_on(cache.invalidate_did(&did)).unwrap();
    assert_eq!(cache.len(), 1);
}

#[test]
fn concurrent_misses_are_duplicate_safe_and_subsequent_calls_hit() {
    const CALLERS: usize = 8;
    let did = Did::parse("did:prism:concurrent").unwrap();
    let barrier = Arc::new(Barrier::new(CALLERS));
    let resolver = Arc::new(FixedResolver::with_barrier(
        success(&did, "origin", None),
        barrier,
    ));
    let caching = compose(
        resolver.clone(),
        Arc::new(MemoryCache::new(16)),
        Arc::new(FakeClock::at(1)),
        policy(CacheFailureMode::Bypass),
    );

    let handles = (0..CALLERS)
        .map(|_| {
            let caching = caching.clone();
            let did = did.clone();
            std::thread::spawn(move || {
                block_on(caching.resolve_with_cache_status(&did, &ResolutionOptions::empty()))
                    .status()
            })
        })
        .collect::<Vec<_>>();
    for handle in handles {
        assert_eq!(handle.join().unwrap(), DidResolutionCacheStatus::MissStored);
    }
    assert_eq!(resolver.calls.load(Ordering::SeqCst), CALLERS);
    assert_eq!(
        block_on(caching.resolve_with_cache_status(&did, &ResolutionOptions::empty())).status(),
        DidResolutionCacheStatus::Hit
    );
    assert_eq!(resolver.calls.load(Ordering::SeqCst), CALLERS);
}

#[test]
#[ignore = "manual release-mode performance diagnostic"]
fn cache_hit_throughput_diagnostic() {
    const ITERATIONS: usize = 100_000;
    let did = Did::parse("did:midnight:undeployed:contract").unwrap();
    let options = ResolutionOptions::empty();
    let caching = compose(
        Arc::new(FixedResolver::new(success(&did, "origin", None))),
        Arc::new(MemoryCache::new(16)),
        Arc::new(FakeClock::at(1)),
        policy(CacheFailureMode::Bypass),
    );
    assert_eq!(
        block_on(caching.resolve_with_cache_status(&did, &options)).status(),
        DidResolutionCacheStatus::MissStored
    );

    let started = Instant::now();
    for _ in 0..ITERATIONS {
        let result =
            block_on(caching.resolve_with_cache_status(black_box(&did), black_box(&options)));
        assert_eq!(result.status(), DidResolutionCacheStatus::Hit);
        black_box(result);
    }
    let elapsed = started.elapsed();
    println!(
        "served {ITERATIONS} DID resolution cache hits in {elapsed:?} ({:.0} hits/s)",
        ITERATIONS as f64 / elapsed.as_secs_f64()
    );
}
