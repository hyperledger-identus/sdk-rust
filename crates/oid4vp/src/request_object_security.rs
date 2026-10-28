//! Audience, freshness, and replay evidence for verified Request Objects.

use std::{fmt, future::Future, pin::Pin};

use identus_core::WallClock;
use identus_jose::{JwsAlgorithm, ProtectedHeader};
use serde_json::Value;
use zeroize::Zeroizing;

use crate::{Oid4vpError, RequestObjectValidationLimits, VerifiedRequestObject};

/// Static-discovery audience required by OpenID4VP 1.0 Final section 5.8.
pub const STATIC_DISCOVERY_AUDIENCE: &str = "https://self-issued.me/v2";

/// Explicit caller-owned audience and temporal policy for one Request Object.
#[derive(Clone, PartialEq, Eq)]
pub struct RequestObjectSecurityPolicy {
    expected_audience: Zeroizing<String>,
    max_age_seconds: u64,
    allowed_clock_skew_seconds: u64,
}

impl RequestObjectSecurityPolicy {
    /// Construct a finite policy from trusted discovery evidence.
    pub fn new(
        expected_audience: impl AsRef<str>,
        max_age_seconds: u64,
        allowed_clock_skew_seconds: u64,
        limits: RequestObjectValidationLimits,
    ) -> Result<Self, Oid4vpError> {
        let expected_audience = expected_audience.as_ref();
        if expected_audience.is_empty()
            || expected_audience.chars().any(char::is_control)
            || expected_audience.len() > limits.max_string_bytes()
            || max_age_seconds
                .checked_add(allowed_clock_skew_seconds)
                .is_none()
        {
            return Err(Oid4vpError::InvalidRequestObjectSecurityPolicy);
        }
        Ok(Self {
            expected_audience: Zeroizing::new(expected_audience.to_owned()),
            max_age_seconds,
            allowed_clock_skew_seconds,
        })
    }

    /// Construct the OpenID4VP static-discovery policy.
    pub fn static_discovery(
        max_age_seconds: u64,
        allowed_clock_skew_seconds: u64,
        limits: RequestObjectValidationLimits,
    ) -> Result<Self, Oid4vpError> {
        Self::new(
            STATIC_DISCOVERY_AUDIENCE,
            max_age_seconds,
            allowed_clock_skew_seconds,
            limits,
        )
    }

    /// Explicitly reveal the caller-supplied expected audience.
    pub fn expose_sensitive_expected_audience(&self) -> &str {
        &self.expected_audience
    }

    /// Maximum accepted age in seconds before allowed skew.
    pub const fn max_age_seconds(&self) -> u64 {
        self.max_age_seconds
    }

    /// Allowed clock skew in seconds.
    pub const fn allowed_clock_skew_seconds(&self) -> u64 {
        self.allowed_clock_skew_seconds
    }
}

impl fmt::Debug for RequestObjectSecurityPolicy {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RequestObjectSecurityPolicy")
            .field("expected_audience_len", &self.expected_audience.len())
            .field("max_age_seconds", &self.max_age_seconds)
            .field(
                "allowed_clock_skew_seconds",
                &self.allowed_clock_skew_seconds,
            )
            .finish()
    }
}

/// Static failure returned by an injected Request Object replay guard.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum RequestObjectReplayFailure {
    /// The object was replayed or rejected by caller policy.
    Rejected,
    /// The replay provider could not atomically decide.
    Unavailable,
}

/// Future returned by [`RequestObjectReplayGuard`].
pub type RequestObjectReplayFuture<'a> =
    Pin<Box<dyn Future<Output = Result<(), RequestObjectReplayFailure>> + Send + 'a>>;

/// Atomic caller-owned Request Object replay check-and-record capability.
pub trait RequestObjectReplayGuard: Send + Sync {
    /// Atomically accept and record one otherwise valid Request Object.
    fn accept<'a>(&'a self, input: RequestObjectReplayInput<'a>) -> RequestObjectReplayFuture<'a>;
}

/// Borrowed bounded evidence supplied to a replay guard.
#[derive(Clone, Copy)]
pub struct RequestObjectReplayInput<'a> {
    payload: &'a [u8],
    client_id: &'a str,
    audience: &'a [Zeroizing<String>],
    issued_at: u64,
    expires_at: Option<u64>,
    not_before: Option<u64>,
    jwt_id: Option<&'a str>,
}

impl<'a> RequestObjectReplayInput<'a> {
    /// Explicitly reveal the exact verified payload for caller-owned keying.
    pub const fn expose_sensitive_payload(&self) -> &'a [u8] {
        self.payload
    }

    /// Explicitly reveal the correlated client identifier.
    pub const fn expose_sensitive_client_id(&self) -> &'a str {
        self.client_id
    }

    /// Explicitly reveal the accepted signed audiences.
    pub fn expose_sensitive_audiences(&self) -> impl ExactSizeIterator<Item = &'a str> {
        self.audience.iter().map(|value| value.as_str())
    }

    /// Exact issued-at Unix second.
    pub const fn issued_at(&self) -> u64 {
        self.issued_at
    }

    /// Optional exact expiry Unix second.
    pub const fn expires_at(&self) -> Option<u64> {
        self.expires_at
    }

    /// Optional exact not-before Unix second.
    pub const fn not_before(&self) -> Option<u64> {
        self.not_before
    }

    /// Explicitly reveal the optional bounded JWT identifier.
    pub const fn expose_sensitive_jwt_id(&self) -> Option<&'a str> {
        self.jwt_id
    }
}

impl fmt::Debug for RequestObjectReplayInput<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RequestObjectReplayInput")
            .field("payload_len", &self.payload.len())
            .field("client_id_len", &self.client_id.len())
            .field("audience_count", &self.audience.len())
            .field("has_expiry", &self.expires_at.is_some())
            .field("has_not_before", &self.not_before.is_some())
            .field("has_jwt_id", &self.jwt_id.is_some())
            .finish_non_exhaustive()
    }
}

struct SecurityClaims {
    audience: Vec<Zeroizing<String>>,
    issued_at: u64,
    expires_at: Option<u64>,
    not_before: Option<u64>,
    jwt_id: Option<Zeroizing<String>>,
}

/// A verified Request Object accepted by audience, time, and replay policy.
///
/// This evidence does not prove verifier-key authorization or product trust.
pub struct RequestObjectSecurityEvidence {
    request: VerifiedRequestObject,
    claims: SecurityClaims,
}

impl VerifiedRequestObject {
    /// Consume signature evidence through audience, freshness, and atomic replay policy.
    pub async fn validate_security(
        self,
        policy: &RequestObjectSecurityPolicy,
        clock: &dyn WallClock,
        replay_guard: &dyn RequestObjectReplayGuard,
    ) -> Result<RequestObjectSecurityEvidence, Oid4vpError> {
        let claims = parse_security_claims(self.expose_sensitive_payload())?;
        if claims.audience.len() != 1
            || claims.audience[0].as_str() != policy.expected_audience.as_str()
        {
            return Err(Oid4vpError::RequestObjectAudienceMismatch);
        }

        let now = clock
            .now()
            .map_err(|_| Oid4vpError::RequestObjectClockUnavailable)?
            .whole_seconds();
        let skew = policy.allowed_clock_skew_seconds;
        let now_with_skew = now
            .checked_add(skew)
            .ok_or(Oid4vpError::InvalidRequestObjectSecurityClaims)?;
        if claims.issued_at > now_with_skew {
            return Err(Oid4vpError::RequestObjectIssuedInFuture);
        }
        if claims
            .not_before
            .is_some_and(|not_before| not_before > now_with_skew)
        {
            return Err(Oid4vpError::RequestObjectNotYetValid);
        }
        if let Some(expires_at) = claims.expires_at {
            let latest = expires_at
                .checked_add(skew)
                .ok_or(Oid4vpError::InvalidRequestObjectSecurityClaims)?;
            if now >= latest {
                return Err(Oid4vpError::RequestObjectExpired);
            }
        }
        let latest_fresh = claims
            .issued_at
            .checked_add(policy.max_age_seconds)
            .and_then(|value| value.checked_add(skew))
            .ok_or(Oid4vpError::InvalidRequestObjectSecurityClaims)?;
        if now > latest_fresh {
            return Err(Oid4vpError::RequestObjectStale);
        }

        replay_guard
            .accept(RequestObjectReplayInput {
                payload: self.expose_sensitive_payload(),
                client_id: self.expose_sensitive_client_id(),
                audience: &claims.audience,
                issued_at: claims.issued_at,
                expires_at: claims.expires_at,
                not_before: claims.not_before,
                jwt_id: claims.jwt_id.as_ref().map(|value| value.as_str()),
            })
            .await
            .map_err(|failure| match failure {
                RequestObjectReplayFailure::Rejected => Oid4vpError::RequestObjectReplayRejected,
                RequestObjectReplayFailure::Unavailable => {
                    Oid4vpError::RequestObjectReplayUnavailable
                }
            })?;
        Ok(RequestObjectSecurityEvidence {
            request: self,
            claims,
        })
    }
}

impl RequestObjectSecurityEvidence {
    pub(crate) fn into_request(self) -> VerifiedRequestObject {
        self.request
    }

    /// Exact algorithm accepted during Request Object verification.
    pub const fn algorithm(&self) -> JwsAlgorithm {
        self.request.algorithm()
    }

    /// Borrow the signature-protected Request Object header.
    pub const fn protected_header(&self) -> &ProtectedHeader {
        self.request.protected_header()
    }

    /// Explicitly reveal the correlated signed client identifier.
    pub fn expose_sensitive_client_id(&self) -> &str {
        self.request.expose_sensitive_client_id()
    }

    /// Explicitly reveal the signed wallet nonce, when present.
    pub fn expose_sensitive_wallet_nonce(&self) -> Option<&str> {
        self.request.expose_sensitive_wallet_nonce()
    }

    /// Exact verified payload byte length.
    pub fn payload_len(&self) -> usize {
        self.request.payload_len()
    }

    /// Explicitly reveal the accepted signed audiences.
    pub fn expose_sensitive_audiences(&self) -> impl ExactSizeIterator<Item = &str> {
        self.claims.audience.iter().map(|value| value.as_str())
    }

    /// Exact issued-at Unix second accepted by policy.
    pub const fn issued_at(&self) -> u64 {
        self.claims.issued_at
    }

    /// Optional exact expiry Unix second accepted by policy.
    pub const fn expires_at(&self) -> Option<u64> {
        self.claims.expires_at
    }

    /// Optional exact not-before Unix second accepted by policy.
    pub const fn not_before(&self) -> Option<u64> {
        self.claims.not_before
    }

    /// Explicitly reveal the optional bounded JWT identifier.
    pub fn expose_sensitive_jwt_id(&self) -> Option<&str> {
        self.claims.jwt_id.as_ref().map(|value| value.as_str())
    }
}

impl fmt::Debug for RequestObjectSecurityEvidence {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RequestObjectSecurityEvidence")
            .field("request", &self.request)
            .field("audience_count", &self.claims.audience.len())
            .field("has_expiry", &self.claims.expires_at.is_some())
            .field("has_not_before", &self.claims.not_before.is_some())
            .field("has_jwt_id", &self.claims.jwt_id.is_some())
            .finish()
    }
}

fn parse_security_claims(payload: &[u8]) -> Result<SecurityClaims, Oid4vpError> {
    let Value::Object(object) = serde_json::from_slice(payload)
        .map_err(|_| Oid4vpError::InvalidRequestObjectSecurityClaims)?
    else {
        return Err(Oid4vpError::InvalidRequestObjectSecurityClaims);
    };
    let audience = parse_audience(object.get("aud"))?;
    let issued_at =
        numeric_date(object.get("iat"))?.ok_or(Oid4vpError::InvalidRequestObjectSecurityClaims)?;
    let expires_at = numeric_date(object.get("exp"))?;
    let not_before = numeric_date(object.get("nbf"))?;
    let jwt_id = match object.get("jti") {
        None => None,
        Some(Value::String(value)) if valid_string(value) => Some(Zeroizing::new(value.to_owned())),
        Some(_) => return Err(Oid4vpError::InvalidRequestObjectSecurityClaims),
    };
    Ok(SecurityClaims {
        audience,
        issued_at,
        expires_at,
        not_before,
        jwt_id,
    })
}

fn parse_audience(value: Option<&Value>) -> Result<Vec<Zeroizing<String>>, Oid4vpError> {
    match value {
        Some(Value::String(value)) if valid_string(value) => {
            Ok(vec![Zeroizing::new(value.clone())])
        }
        Some(Value::Array(values)) if !values.is_empty() => values
            .iter()
            .map(|value| match value {
                Value::String(value) if valid_string(value) => Ok(Zeroizing::new(value.clone())),
                _ => Err(Oid4vpError::InvalidRequestObjectSecurityClaims),
            })
            .collect(),
        _ => Err(Oid4vpError::InvalidRequestObjectSecurityClaims),
    }
}

fn numeric_date(value: Option<&Value>) -> Result<Option<u64>, Oid4vpError> {
    match value {
        None => Ok(None),
        Some(Value::Number(value)) => value
            .as_u64()
            .map(Some)
            .ok_or(Oid4vpError::InvalidRequestObjectSecurityClaims),
        Some(_) => Err(Oid4vpError::InvalidRequestObjectSecurityClaims),
    }
}

fn valid_string(value: &str) -> bool {
    !value.is_empty() && !value.chars().any(char::is_control)
}
