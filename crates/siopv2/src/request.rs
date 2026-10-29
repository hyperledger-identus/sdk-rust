use std::fmt;

use fluent_uri::Uri as ParsedUri;
use identus_did::{JsonWireLimits, validate_unique_object_names};
use identus_jose::{
    JwsAlgorithm, JwsVerificationKey, ProtectedHeader, SignatureSuiteRegistry, UnverifiedCompactJws,
};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use zeroize::Zeroizing;

use crate::{SIOPV2_AUDIENCE, Siopv2Error, Siopv2Limits};

/// Required protected-header type for the signed request object.
pub const REQUEST_OBJECT_JWT_TYPE: &str = "oauth-authz-req+jwt";

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Claims {
    iss: String,
    aud: String,
    client_id: String,
    redirect_uri: String,
    response_type: String,
    response_mode: String,
    scope: String,
    id_token_type: String,
    nonce: String,
    wallet_nonce: Option<String>,
    state: Option<String>,
    iat: i64,
    exp: i64,
}

/// A bounded request object awaiting caller-authorized signature verification.
pub struct UnverifiedSiopRequest {
    compact: Zeroizing<String>,
    header: ProtectedHeader,
    expected_client_id: Option<Zeroizing<String>>,
    expected_wallet_nonce: Option<Zeroizing<String>>,
    limits: Siopv2Limits,
}

impl UnverifiedSiopRequest {
    /// Parse a retrieved request and retain its client and optional nonce binding.
    pub fn from_retrieved(
        retrieved: &identus_openid4vc::RetrievedRequestObject,
        limits: Siopv2Limits,
    ) -> Result<Self, Siopv2Error> {
        let mut request = Self::parse(retrieved.expose_sensitive_compact(), limits)?;
        request.expected_client_id = Some(Zeroizing::new(
            retrieved.expose_sensitive_client_id().to_owned(),
        ));
        request.expected_wallet_nonce = retrieved
            .expose_sensitive_wallet_nonce()
            .map(|nonce| Zeroizing::new(nonce.to_owned()));
        Ok(request)
    }

    /// Parse an inline signed request object with no outer retrieval binding.
    pub fn parse(compact: &str, limits: Siopv2Limits) -> Result<Self, Siopv2Error> {
        if compact.len() > limits.jws().max_compact_bytes() {
            return Err(Siopv2Error::RequestObjectTooLarge);
        }
        let parsed = UnverifiedCompactJws::parse(compact, limits.jws())
            .map_err(|_| Siopv2Error::InvalidRequestObject)?;
        if parsed.protected_header().type_() != Some(REQUEST_OBJECT_JWT_TYPE) {
            return Err(Siopv2Error::InvalidRequestObjectType);
        }
        Ok(Self {
            compact: Zeroizing::new(compact.to_owned()),
            header: parsed.protected_header().clone(),
            expected_client_id: None,
            expected_wallet_nonce: None,
            limits,
        })
    }

    /// Protected header evidence for caller-owned key authorization.
    pub const fn protected_header(&self) -> &ProtectedHeader {
        &self.header
    }

    /// Verify the caller-authorized signature and validate the pinned profile.
    pub fn verify(
        self,
        suites: &SignatureSuiteRegistry,
        key: &JwsVerificationKey<'_>,
        now: i64,
    ) -> Result<ValidatedSiopRequest, Siopv2Error> {
        // Parsing is deliberately repeated here. The JOSE parser owns compact,
        // payload, and signature buffers that are not zeroized on drop; the
        // public unverified state retains only one zeroized compact value and
        // bounded public header evidence until this consuming transition.
        let compact = UnverifiedCompactJws::parse(&self.compact, self.limits.jws())
            .map_err(|_| Siopv2Error::InvalidRequestObject)?;
        let verified = suites
            .verify(&compact, key)
            .map_err(|_| Siopv2Error::InvalidRequestSignature)?;
        let payload = verified.as_compact().payload();
        validate_unique_object_names(
            payload,
            JsonWireLimits {
                max_depth: self.limits.max_json_depth(),
                max_nodes: self.limits.max_json_nodes(),
                max_object_members: self.limits.max_object_members(),
                max_live_key_bytes: self.limits.max_live_key_bytes(),
            },
        )
        .map_err(|_| Siopv2Error::InvalidRequestClaims)?;
        let claims: Claims =
            serde_json::from_slice(payload).map_err(|_| Siopv2Error::InvalidRequestClaims)?;
        validate_strings(&claims, self.limits)?;
        if claims.iss != claims.client_id {
            return Err(Siopv2Error::ClientBindingMismatch);
        }
        if self
            .expected_client_id
            .as_deref()
            .is_some_and(|expected| expected.as_str() != claims.client_id)
        {
            return Err(Siopv2Error::ClientBindingMismatch);
        }
        if self.expected_wallet_nonce.as_deref().map(String::as_str)
            != claims.wallet_nonce.as_deref()
        {
            return Err(Siopv2Error::WalletNonceMismatch);
        }
        if claims.aud != SIOPV2_AUDIENCE
            || claims.response_type != "id_token"
            || claims.response_mode != "direct_post"
            || claims.scope != "openid"
            || claims.id_token_type != "subject_signed_id_token"
        {
            return Err(Siopv2Error::UnsupportedProfile);
        }
        if !is_https_uri(&claims.client_id) || !is_https_uri(&claims.redirect_uri) {
            return Err(Siopv2Error::InvalidRedirectUri);
        }
        if claims.exp <= claims.iat
            || claims.exp.saturating_sub(claims.iat) > self.limits.max_request_lifetime_seconds()
            || claims.iat > now.saturating_add(self.limits.max_clock_skew_seconds())
        {
            return Err(Siopv2Error::InvalidTimeWindow);
        }
        if claims.exp < now.saturating_sub(self.limits.max_clock_skew_seconds()) {
            return Err(Siopv2Error::RequestExpired);
        }
        let replay_fingerprint: [u8; 32] = Sha256::digest(self.compact.as_bytes()).into();
        Ok(ValidatedSiopRequest {
            algorithm: verified.algorithm(),
            client_id: Zeroizing::new(claims.client_id),
            redirect_uri: Zeroizing::new(claims.redirect_uri),
            nonce: Zeroizing::new(claims.nonce),
            state: claims.state.map(Zeroizing::new),
            expires_at: claims.exp,
            replay_fingerprint,
            limits: self.limits,
        })
    }
}

fn validate_strings(claims: &Claims, limits: Siopv2Limits) -> Result<(), Siopv2Error> {
    let values = [
        claims.iss.as_str(),
        claims.aud.as_str(),
        claims.client_id.as_str(),
        claims.redirect_uri.as_str(),
        claims.response_type.as_str(),
        claims.response_mode.as_str(),
        claims.scope.as_str(),
        claims.id_token_type.as_str(),
        claims.nonce.as_str(),
    ];
    if values
        .iter()
        .any(|value| !is_valid_bounded_string(value, limits.max_string_bytes()))
        || claims
            .state
            .as_deref()
            .is_some_and(|value| !is_valid_bounded_string(value, limits.max_string_bytes()))
    {
        return Err(Siopv2Error::InvalidRequestClaims);
    }
    if claims
        .wallet_nonce
        .as_deref()
        .is_some_and(|value| !is_valid_bounded_string(value, limits.max_string_bytes()))
    {
        return Err(Siopv2Error::InvalidRequestClaims);
    }
    Ok(())
}

fn is_valid_bounded_string(value: &str, maximum: usize) -> bool {
    !value.is_empty() && value.len() <= maximum && !value.chars().any(char::is_control)
}

fn is_https_uri(value: &str) -> bool {
    ParsedUri::parse(value).is_ok_and(|parsed| {
        parsed.scheme().as_str().eq_ignore_ascii_case("https")
            && parsed.authority().is_some_and(|authority| {
                !authority.host().is_empty() && authority.userinfo().is_none()
            })
            && parsed.fragment().is_none()
    })
}

/// Signature-verified and profile-validated SIOPv2 request state.
pub struct ValidatedSiopRequest {
    pub(crate) algorithm: JwsAlgorithm,
    pub(crate) client_id: Zeroizing<String>,
    pub(crate) redirect_uri: Zeroizing<String>,
    pub(crate) nonce: Zeroizing<String>,
    pub(crate) state: Option<Zeroizing<String>>,
    pub(crate) expires_at: i64,
    pub(crate) replay_fingerprint: [u8; 32],
    pub(crate) limits: Siopv2Limits,
}

impl ValidatedSiopRequest {
    /// Algorithm used by the relying party's request signature.
    pub const fn algorithm(&self) -> JwsAlgorithm {
        self.algorithm
    }
    /// Explicitly reveal the verified relying-party client identifier.
    pub fn expose_sensitive_client_id(&self) -> &str {
        &self.client_id
    }
    /// Explicitly reveal the verified direct-post destination.
    pub fn expose_sensitive_redirect_uri(&self) -> &str {
        &self.redirect_uri
    }
    /// Explicitly reveal the verified authentication nonce.
    pub fn expose_sensitive_nonce(&self) -> &str {
        &self.nonce
    }
    /// Explicitly reveal optional verified state correlation.
    pub fn expose_sensitive_state(&self) -> Option<&str> {
        self.state.as_deref().map(String::as_str)
    }
    /// Request expiry as caller-supplied Unix time.
    pub const fn expires_at(&self) -> i64 {
        self.expires_at
    }
    /// SHA-256 fingerprint of the exact signed request object for caller-owned replay storage.
    pub const fn replay_fingerprint(&self) -> &[u8; 32] {
        &self.replay_fingerprint
    }
}

impl fmt::Debug for UnverifiedSiopRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("UnverifiedSiopRequest")
            .field("algorithm", &self.header.algorithm())
            .field("compact_len", &self.compact.len())
            .finish_non_exhaustive()
    }
}

impl fmt::Debug for ValidatedSiopRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ValidatedSiopRequest")
            .field("algorithm", &self.algorithm)
            .field("client_id_len", &self.client_id.len())
            .field("redirect_uri_len", &self.redirect_uri.len())
            .field("nonce_len", &self.nonce.len())
            .field("has_state", &self.state.is_some())
            .field("expires_at", &self.expires_at)
            .field("replay_fingerprint", &"[..; 32]")
            .finish()
    }
}
