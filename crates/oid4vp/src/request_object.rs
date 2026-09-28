use std::fmt;

use identus_jose::{
    JwsAlgorithm, JwsVerificationKey, ProtectedHeader, SignatureSuiteRegistry, UnverifiedCompactJws,
};
use zeroize::Zeroizing;

use crate::{
    Oid4vpError, RequestObjectValidationLimits, request_object_json::parse_request_object_claims,
};

/// Required protected `typ` for an OpenID4VP Request Object.
pub const REQUEST_OBJECT_JWT_TYPE: &str = "oauth-authz-req+jwt";

/// A bounded compact Request Object whose signature has not been verified.
///
/// Header values are untrusted inputs intended only to help the caller locate
/// a candidate verification key. The caller remains responsible for proving
/// that key is authorized for the exact client identifier.
pub struct UnverifiedRequestObject {
    compact: Zeroizing<String>,
    protected_header: ProtectedHeader,
    outer_client_id: Zeroizing<String>,
    expected_wallet_nonce: Option<Zeroizing<String>>,
    limits: RequestObjectValidationLimits,
}

impl UnverifiedRequestObject {
    pub(crate) fn parse(
        compact: &str,
        outer_client_id: Zeroizing<String>,
        expected_wallet_nonce: Option<Zeroizing<String>>,
        limits: RequestObjectValidationLimits,
    ) -> Result<Self, Oid4vpError> {
        let parsed = UnverifiedCompactJws::parse(compact, limits.jws())
            .map_err(|_| Oid4vpError::InvalidRequestObject)?;
        if parsed.protected_header().type_() != Some(REQUEST_OBJECT_JWT_TYPE) {
            return Err(Oid4vpError::InvalidRequestObjectType);
        }
        let protected_header = parsed.protected_header().clone();
        Ok(Self {
            compact: Zeroizing::new(compact.to_owned()),
            protected_header,
            outer_client_id,
            expected_wallet_nonce,
            limits,
        })
    }

    /// Borrow the validated but untrusted protected header for key discovery.
    pub const fn protected_header(&self) -> &ProtectedHeader {
        &self.protected_header
    }

    /// Explicitly reveal the outer client identifier for caller-owned key
    /// authorization and trust resolution.
    pub fn expose_sensitive_outer_client_id(&self) -> &str {
        &self.outer_client_id
    }

    /// Verify the exact compact JWS and correlate signed request claims.
    ///
    /// This proves cryptographic validity with `key`; it does not prove that
    /// the caller associated that key with the client through DID, X.509,
    /// federation, verifier attestation, or another trust framework.
    pub fn verify(
        self,
        suites: &SignatureSuiteRegistry,
        key: &JwsVerificationKey<'_>,
    ) -> Result<VerifiedRequestObject, Oid4vpError> {
        let parsed = UnverifiedCompactJws::parse(&self.compact, self.limits.jws())
            .map_err(|_| Oid4vpError::InvalidRequestObject)?;
        let verified = suites
            .verify(&parsed, key)
            .map_err(|_| Oid4vpError::InvalidRequestObjectSignature)?;
        let algorithm = verified.algorithm();
        let payload = Zeroizing::new(verified.as_compact().payload().to_vec());
        let claims = parse_request_object_claims(&payload, self.limits)?;
        let client_id = claims
            .client_id
            .ok_or(Oid4vpError::RequestObjectClientIdMismatch)?;
        if client_id.as_str() != self.outer_client_id.as_str() {
            return Err(Oid4vpError::RequestObjectClientIdMismatch);
        }
        if let Some(expected) = self.expected_wallet_nonce.as_ref()
            && claims.wallet_nonce.as_ref().map(|nonce| nonce.as_str()) != Some(expected.as_str())
        {
            return Err(Oid4vpError::RequestObjectWalletNonceMismatch);
        }

        Ok(VerifiedRequestObject {
            protected_header: self.protected_header,
            algorithm,
            payload,
            client_id,
            wallet_nonce: claims.wallet_nonce,
        })
    }
}

impl fmt::Debug for UnverifiedRequestObject {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("UnverifiedRequestObject")
            .field("algorithm", &self.protected_header.algorithm())
            .field(
                "has_key_reference",
                &self.protected_header.key_reference().is_some(),
            )
            .field("compact_len", &self.compact.len())
            .field("client_id_len", &self.outer_client_id.len())
            .field(
                "expects_wallet_nonce",
                &self.expected_wallet_nonce.is_some(),
            )
            .finish()
    }
}

/// A signed Request Object with Final type, client-id and nonce correlation.
///
/// This is not yet verifier-trust, audience, freshness, OAuth, DCQL, consent,
/// or product-authorization evidence.
pub struct VerifiedRequestObject {
    protected_header: ProtectedHeader,
    algorithm: JwsAlgorithm,
    payload: Zeroizing<Vec<u8>>,
    client_id: Zeroizing<String>,
    wallet_nonce: Option<Zeroizing<String>>,
}

impl VerifiedRequestObject {
    pub(crate) fn into_sensitive_payload(self) -> Zeroizing<Vec<u8>> {
        self.payload
    }

    /// Exact algorithm accepted by the selected signature suite and key.
    pub const fn algorithm(&self) -> JwsAlgorithm {
        self.algorithm
    }

    /// Borrow the signature-protected header.
    pub const fn protected_header(&self) -> &ProtectedHeader {
        &self.protected_header
    }

    /// Explicitly reveal the exact verified payload for later bounded stages.
    pub fn expose_sensitive_payload(&self) -> &[u8] {
        &self.payload
    }

    /// Explicitly reveal the correlated signed client identifier.
    pub fn expose_sensitive_client_id(&self) -> &str {
        &self.client_id
    }

    /// Explicitly reveal the signed wallet nonce, when present.
    pub fn expose_sensitive_wallet_nonce(&self) -> Option<&str> {
        self.wallet_nonce.as_ref().map(|nonce| nonce.as_str())
    }

    /// Exact verified payload byte length.
    pub fn payload_len(&self) -> usize {
        self.payload.len()
    }
}

impl fmt::Debug for VerifiedRequestObject {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("VerifiedRequestObject")
            .field("algorithm", &self.algorithm)
            .field(
                "has_key_reference",
                &self.protected_header.key_reference().is_some(),
            )
            .field("payload_len", &self.payload.len())
            .field("client_id_len", &self.client_id.len())
            .field("has_wallet_nonce", &self.wallet_nonce.is_some())
            .finish()
    }
}
