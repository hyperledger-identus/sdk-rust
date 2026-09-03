//! Opt-in bounded DID resolution caching over injected ports.
//!
//! This module defines portable policy and orchestration only. Concrete cache
//! storage, system clocks, eviction strategy, telemetry and runtime-specific
//! single-flight coordination belong in outer adapters.

use std::{collections::BTreeMap, fmt, future::Future, pin::Pin, sync::Arc};

use identus_core::{DurationMillis, IdentusResult, MonotonicClock, MonotonicTimestampMillis};
use identus_derive as identus;
use serde::Serialize;
use serde_json::Value;

use crate::{
    Did, DidResolutionError, DidResolutionErrorKind, DidResolutionFuture, DidResolutionMetadata,
    DidResolutionResult, DidResolver, Error, ResolutionOptions, error::CacheError,
};

/// Maximum normalized bytes in one resolution cache key.
pub const MAX_DID_RESOLUTION_CACHE_KEY_BYTES: usize = 64 * 1_024;
/// Maximum entries a cache behind the portable decorator may declare.
pub const MAX_DID_RESOLUTION_CACHE_ENTRIES: usize = 4_096;
/// Maximum positive-result cache lifetime: 24 hours.
pub const MAX_DID_RESOLUTION_POSITIVE_TTL_MILLIS: u64 = 24 * 60 * 60 * 1_000;
/// Maximum `notFound` cache lifetime: five minutes.
pub const MAX_DID_RESOLUTION_NOT_FOUND_TTL_MILLIS: u64 = 5 * 60 * 1_000;

/// Stable, normalized identity for a DID resolution request.
///
/// The encoded identity is intentionally opaque. `Debug` does not reveal the
/// DID or option values, while equality and hashing let adapters use this type
/// directly as a map key.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct DidResolutionCacheKey {
    did: Did,
    encoded: Arc<[u8]>,
}

impl DidResolutionCacheKey {
    /// Derive a bounded key from a validated DID and all result-affecting
    /// options except the `noCache` control.
    pub fn for_resolution(did: &Did, options: &ResolutionOptions) -> Result<Self, Error> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Wire<'a> {
            did: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            accept: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            expand_relative_urls: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            version_id: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            version_time: Option<&'a str>,
            #[serde(skip_serializing_if = "BTreeMap::is_empty")]
            extensions: BTreeMap<&'a str, Value>,
        }

        let extensions = options
            .extensions()
            .iter()
            .map(|(name, value)| (name.as_str(), canonical_json(value)))
            .collect();
        let wire = Wire {
            did: did.as_str(),
            accept: options.accept().map(crate::MediaType::as_str),
            expand_relative_urls: options.expand_relative_urls(),
            version_id: options.version_id().map(crate::VersionId::as_str),
            version_time: options
                .version_time()
                .map(crate::DidResolutionDateTime::as_str),
            extensions,
        };
        let encoded = serde_json::to_vec(&wire)
            .expect("validated DID resolution cache-key data is serializable");
        if encoded.len() > MAX_DID_RESOLUTION_CACHE_KEY_BYTES {
            return Err(invalid(CacheError::KeyTooLarge));
        }
        Ok(Self {
            did: did.clone(),
            encoded: encoded.into(),
        })
    }

    /// Borrow the exact DID solely for all-options invalidation by a cache
    /// adapter.
    #[must_use]
    pub const fn did(&self) -> &Did {
        &self.did
    }

    /// Return the bounded encoded key length without exposing its contents.
    #[must_use]
    pub fn encoded_len(&self) -> usize {
        self.encoded.len()
    }
}

impl fmt::Debug for DidResolutionCacheKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DidResolutionCacheKey")
            .field("method", &self.did.method())
            .field("encoded_len", &self.encoded_len())
            .finish_non_exhaustive()
    }
}

/// Cache behavior when an optional cache or clock dependency fails.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum CacheFailureMode {
    /// Continue through the upstream resolver without relying on the cache.
    Bypass,
    /// Return a standards-shaped `internalError` result.
    FailClosed,
}

/// Immutable bounded TTL and failure policy for DID resolution caching.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DidResolutionCachePolicy {
    positive_ttl: Option<DurationMillis>,
    not_found_ttl: Option<DurationMillis>,
    failure_mode: CacheFailureMode,
}

impl DidResolutionCachePolicy {
    /// Construct and validate an opt-in policy.
    pub fn new(
        positive_ttl: Option<DurationMillis>,
        not_found_ttl: Option<DurationMillis>,
        failure_mode: CacheFailureMode,
    ) -> Result<Self, Error> {
        validate_ttl(positive_ttl, MAX_DID_RESOLUTION_POSITIVE_TTL_MILLIS)?;
        validate_ttl(not_found_ttl, MAX_DID_RESOLUTION_NOT_FOUND_TTL_MILLIS)?;
        Ok(Self {
            positive_ttl,
            not_found_ttl,
            failure_mode,
        })
    }

    /// A policy that never consults cache or clock dependencies.
    #[must_use]
    pub const fn disabled() -> Self {
        Self {
            positive_ttl: None,
            not_found_ttl: None,
            failure_mode: CacheFailureMode::Bypass,
        }
    }

    /// Positive/deactivated result lifetime, when enabled.
    #[must_use]
    pub const fn positive_ttl(self) -> Option<DurationMillis> {
        self.positive_ttl
    }

    /// `notFound` failure lifetime, when enabled.
    #[must_use]
    pub const fn not_found_ttl(self) -> Option<DurationMillis> {
        self.not_found_ttl
    }

    /// Cache infrastructure failure behavior.
    #[must_use]
    pub const fn failure_mode(self) -> CacheFailureMode {
        self.failure_mode
    }

    /// Whether at least one cache class is enabled.
    #[must_use]
    pub const fn is_enabled(self) -> bool {
        self.positive_ttl.is_some() || self.not_found_ttl.is_some()
    }
}

impl Default for DidResolutionCachePolicy {
    fn default() -> Self {
        Self::disabled()
    }
}

/// One process-epoch cache entry.
#[derive(Clone, PartialEq, Eq)]
pub struct DidResolutionCacheEntry {
    result: DidResolutionResult,
    inserted_at: MonotonicTimestampMillis,
    expires_at: MonotonicTimestampMillis,
}

impl DidResolutionCacheEntry {
    /// Construct an entry whose expiry is strictly after insertion.
    pub fn new(
        result: DidResolutionResult,
        inserted_at: MonotonicTimestampMillis,
        expires_at: MonotonicTimestampMillis,
    ) -> Result<Self, Error> {
        if expires_at <= inserted_at {
            return Err(invalid(CacheError::InvalidEntryLifetime));
        }
        Ok(Self {
            result,
            inserted_at,
            expires_at,
        })
    }

    /// Borrow the cached typed result.
    #[must_use]
    pub const fn result(&self) -> &DidResolutionResult {
        &self.result
    }

    /// Process-epoch insertion tick.
    #[must_use]
    pub const fn inserted_at(&self) -> MonotonicTimestampMillis {
        self.inserted_at
    }

    /// Exclusive process-epoch expiry tick.
    #[must_use]
    pub const fn expires_at(&self) -> MonotonicTimestampMillis {
        self.expires_at
    }

    fn is_allowed_at(
        &self,
        now: MonotonicTimestampMillis,
        policy: DidResolutionCachePolicy,
    ) -> bool {
        let Some(ttl) = ttl_for(&self.result, policy) else {
            return false;
        };
        let Ok(maximum_expiry) = self.inserted_at.checked_add(ttl) else {
            return false;
        };
        self.inserted_at <= now && now < self.expires_at && self.expires_at <= maximum_expiry
    }
}

impl fmt::Debug for DidResolutionCacheEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DidResolutionCacheEntry")
            .field("inserted_at", &self.inserted_at)
            .field("expires_at", &self.expires_at)
            .finish_non_exhaustive()
    }
}

/// Type-erased asynchronous cache lookup.
pub type DidResolutionCacheLookupFuture<'a> =
    Pin<Box<dyn Future<Output = IdentusResult<Option<DidResolutionCacheEntry>>> + Send + 'a>>;
/// Type-erased asynchronous cache mutation.
pub type DidResolutionCacheWriteFuture<'a> =
    Pin<Box<dyn Future<Output = IdentusResult<()>> + Send + 'a>>;

/// Storage-neutral cache port for typed DID resolution entries.
#[identus::port]
pub trait DidResolutionCache: Send + Sync {
    /// Maximum entries enforced by this backend instance.
    fn capacity(&self) -> usize;

    /// Look up an exact normalized request key.
    fn lookup<'a>(&'a self, key: &'a DidResolutionCacheKey) -> DidResolutionCacheLookupFuture<'a>;

    /// Store or replace one entry. Concurrent fills may be last-write-wins.
    fn store<'a>(
        &'a self,
        key: DidResolutionCacheKey,
        entry: DidResolutionCacheEntry,
    ) -> DidResolutionCacheWriteFuture<'a>;

    /// Invalidate one exact request key.
    fn invalidate<'a>(
        &'a self,
        key: &'a DidResolutionCacheKey,
    ) -> DidResolutionCacheWriteFuture<'a>;

    /// Invalidate every option variant for one exact DID.
    fn invalidate_did<'a>(&'a self, did: &'a Did) -> DidResolutionCacheWriteFuture<'a>;
}

/// Non-sensitive result of one caching-resolver invocation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum DidResolutionCacheStatus {
    /// Policy had no enabled cache class.
    Disabled,
    /// `noCache: true` requested direct resolution.
    RequestedBypass,
    /// A valid fresh entry was served.
    Hit,
    /// An ordinary miss was resolved and stored.
    MissStored,
    /// An ordinary miss produced a result class that was not stored.
    MissNotStored,
    /// An expired, regressed or invalid entry was replaced.
    RefreshStored,
    /// An expired, regressed or invalid entry refreshed without a stored result.
    RefreshNotStored,
    /// Optional cache infrastructure failed and bypass policy continued.
    BackendBypass,
    /// Cache infrastructure failed and fail-closed policy returned an error.
    FailedClosed,
}

/// A resolution result paired with redaction-safe cache observability.
#[derive(Clone, PartialEq, Eq)]
pub struct CachedDidResolution {
    result: DidResolutionResult,
    status: DidResolutionCacheStatus,
}

impl fmt::Debug for CachedDidResolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CachedDidResolution")
            .field("status", &self.status)
            .finish_non_exhaustive()
    }
}

impl CachedDidResolution {
    fn new(result: DidResolutionResult, status: DidResolutionCacheStatus) -> Self {
        Self { result, status }
    }

    /// Borrow the unchanged W3C result envelope.
    #[must_use]
    pub const fn result(&self) -> &DidResolutionResult {
        &self.result
    }

    /// Return the non-sensitive cache outcome.
    #[must_use]
    pub const fn status(&self) -> DidResolutionCacheStatus {
        self.status
    }

    /// Consume the observation and return the W3C result.
    #[must_use]
    pub fn into_result(self) -> DidResolutionResult {
        self.result
    }
}

/// Future returned by [`CachingDidResolver::resolve_with_cache_status`].
pub type CachedDidResolutionFuture<'a> =
    Pin<Box<dyn Future<Output = CachedDidResolution> + Send + 'a>>;

/// Opt-in cache decorator for any object-safe DID resolver.
#[derive(Clone)]
pub struct CachingDidResolver {
    upstream: Arc<dyn DidResolver>,
    cache: Arc<dyn DidResolutionCache>,
    clock: Arc<dyn MonotonicClock>,
    policy: DidResolutionCachePolicy,
}

impl CachingDidResolver {
    /// Compose an upstream resolver with validated cache infrastructure.
    pub fn new(
        upstream: Arc<dyn DidResolver>,
        cache: Arc<dyn DidResolutionCache>,
        clock: Arc<dyn MonotonicClock>,
        policy: DidResolutionCachePolicy,
    ) -> Result<Self, Error> {
        if !(1..=MAX_DID_RESOLUTION_CACHE_ENTRIES).contains(&cache.capacity()) {
            return Err(invalid(CacheError::InvalidCapacity));
        }
        Ok(Self {
            upstream,
            cache,
            clock,
            policy,
        })
    }

    /// Resolve while retaining a redaction-safe cache outcome.
    pub fn resolve_with_cache_status<'a>(
        &'a self,
        did: &'a Did,
        options: &'a ResolutionOptions,
    ) -> CachedDidResolutionFuture<'a> {
        Box::pin(async move { self.resolve_inner(did, options).await })
    }

    async fn resolve_inner(&self, did: &Did, options: &ResolutionOptions) -> CachedDidResolution {
        if !self.policy.is_enabled() {
            return self
                .resolve_upstream(did, options, DidResolutionCacheStatus::Disabled)
                .await;
        }
        if options.no_cache() == Some(true) {
            return self
                .resolve_upstream(did, options, DidResolutionCacheStatus::RequestedBypass)
                .await;
        }

        let now = match self.clock.now() {
            Ok(now) => now,
            Err(_) => return self.infrastructure_failure(did, options).await,
        };
        let key = match DidResolutionCacheKey::for_resolution(did, options) {
            Ok(key) => key,
            Err(_) => return self.infrastructure_failure(did, options).await,
        };

        let mut refreshing = false;
        let mut backend_bypassed = false;
        match self.cache.lookup(&key).await {
            Ok(Some(entry)) => {
                if entry.is_allowed_at(now, self.policy) && entry.result().validate_for(did).is_ok()
                {
                    return CachedDidResolution::new(
                        entry.result().clone(),
                        DidResolutionCacheStatus::Hit,
                    );
                }
                refreshing = true;
                if self.cache.invalidate(&key).await.is_err() {
                    if self.policy.failure_mode() == CacheFailureMode::FailClosed {
                        return failed_closed();
                    }
                    backend_bypassed = true;
                }
            }
            Ok(None) => {}
            Err(_) => return self.infrastructure_failure(did, options).await,
        }

        let result = self.upstream.resolve(did, options).await;
        let Some(ttl) = ttl_for(&result, self.policy) else {
            return CachedDidResolution::new(
                result,
                if backend_bypassed {
                    DidResolutionCacheStatus::BackendBypass
                } else if refreshing {
                    DidResolutionCacheStatus::RefreshNotStored
                } else {
                    DidResolutionCacheStatus::MissNotStored
                },
            );
        };
        if result.validate_for(did).is_err() {
            return CachedDidResolution::new(
                result,
                if backend_bypassed {
                    DidResolutionCacheStatus::BackendBypass
                } else if refreshing {
                    DidResolutionCacheStatus::RefreshNotStored
                } else {
                    DidResolutionCacheStatus::MissNotStored
                },
            );
        }

        let inserted_at = match self.clock.now() {
            Ok(now) => now,
            Err(_) => return self.after_resolution_failure(result),
        };
        let expires_at = match inserted_at.checked_add(ttl) {
            Ok(expires_at) => expires_at,
            Err(_) => return self.after_resolution_failure(result),
        };
        let entry = DidResolutionCacheEntry::new(result.clone(), inserted_at, expires_at)
            .expect("positive bounded TTL creates a valid cache lifetime");
        if self.cache.store(key, entry).await.is_err() {
            return self.after_resolution_failure(result);
        }
        CachedDidResolution::new(
            result,
            if backend_bypassed {
                DidResolutionCacheStatus::BackendBypass
            } else if refreshing {
                DidResolutionCacheStatus::RefreshStored
            } else {
                DidResolutionCacheStatus::MissStored
            },
        )
    }

    async fn infrastructure_failure(
        &self,
        did: &Did,
        options: &ResolutionOptions,
    ) -> CachedDidResolution {
        match self.policy.failure_mode() {
            CacheFailureMode::Bypass => {
                self.resolve_upstream(did, options, DidResolutionCacheStatus::BackendBypass)
                    .await
            }
            CacheFailureMode::FailClosed => failed_closed(),
        }
    }

    fn after_resolution_failure(&self, result: DidResolutionResult) -> CachedDidResolution {
        match self.policy.failure_mode() {
            CacheFailureMode::Bypass => {
                CachedDidResolution::new(result, DidResolutionCacheStatus::BackendBypass)
            }
            CacheFailureMode::FailClosed => failed_closed(),
        }
    }

    async fn resolve_upstream(
        &self,
        did: &Did,
        options: &ResolutionOptions,
        status: DidResolutionCacheStatus,
    ) -> CachedDidResolution {
        CachedDidResolution::new(self.upstream.resolve(did, options).await, status)
    }
}

impl fmt::Debug for CachingDidResolver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CachingDidResolver")
            .field("capacity", &self.cache.capacity())
            .field("policy", &self.policy)
            .finish_non_exhaustive()
    }
}

impl DidResolver for CachingDidResolver {
    fn resolve<'a>(
        &'a self,
        did: &'a Did,
        options: &'a ResolutionOptions,
    ) -> DidResolutionFuture<'a> {
        Box::pin(async move {
            self.resolve_with_cache_status(did, options)
                .await
                .into_result()
        })
    }
}

fn validate_ttl(value: Option<DurationMillis>, maximum: u64) -> Result<(), Error> {
    if value.is_some_and(|ttl| ttl.get() == 0 || ttl.get() > maximum) {
        return Err(invalid(CacheError::InvalidTtl));
    }
    Ok(())
}

fn ttl_for(
    result: &DidResolutionResult,
    policy: DidResolutionCachePolicy,
) -> Option<DurationMillis> {
    match result.metadata().error() {
        None => policy.positive_ttl(),
        Some(error) if error.kind() == Some(DidResolutionErrorKind::NotFound) => {
            policy.not_found_ttl()
        }
        Some(_) => None,
    }
}

fn failed_closed() -> CachedDidResolution {
    CachedDidResolution::new(
        resolution_failure(DidResolutionErrorKind::InternalError),
        DidResolutionCacheStatus::FailedClosed,
    )
}

fn resolution_failure(kind: DidResolutionErrorKind) -> DidResolutionResult {
    let metadata = DidResolutionMetadata::new(
        None,
        Some(DidResolutionError::standard(kind)),
        BTreeMap::new(),
    )
    .expect("standard error metadata is valid");
    DidResolutionResult::failure(metadata).expect("standard resolution failure is valid")
}

fn canonical_json(value: &Value) -> Value {
    match value {
        Value::Array(values) => Value::Array(values.iter().map(canonical_json).collect()),
        Value::Object(map) => {
            let mut members = map.iter().collect::<Vec<_>>();
            members.sort_unstable_by_key(|(name, _)| *name);
            let mut canonical = serde_json::Map::new();
            for (name, value) in members {
                canonical.insert(name.clone(), canonical_json(value));
            }
            Value::Object(canonical)
        }
        scalar => scalar.clone(),
    }
}

const fn invalid(reason: CacheError) -> Error {
    Error::InvalidCache(reason)
}
