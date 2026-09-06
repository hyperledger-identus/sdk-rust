//! Holder-side construction of the OpenID4VCI 1.0 Final JWT key proof.

use std::fmt;

use serde::Serialize;

use crate::compact::encode_bounded_json;
use crate::header::valid_protected_evidence;
use crate::{
    JoseError, JwsAlgorithm, JwsKeyReference, JwsLimits, JwsSigner, JwsSigningInput,
    JwsVerificationKey, ProtectedHeader, UnverifiedCompactJws,
};

/// Required protected type for the OpenID4VCI JWT key proof.
pub const OID4VCI_PROOF_JWT_TYPE: &str = "openid4vci-proof+jwt";
/// Default maximum bytes in one proof claim string.
pub const DEFAULT_MAX_PROOF_CLAIM_STRING_BYTES: usize = 2_048;

/// Allocation limits for one holder-side OID4VCI proof JWT.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Oid4vciProofJwtLimits {
    jws: JwsLimits,
    max_claim_string_bytes: usize,
}

impl Oid4vciProofJwtLimits {
    /// Combine compact JWS limits with a positive claim-string ceiling.
    pub const fn new(jws: JwsLimits, max_claim_string_bytes: usize) -> Result<Self, JoseError> {
        if max_claim_string_bytes == 0 {
            return Err(JoseError::InvalidLimits);
        }
        Ok(Self {
            jws,
            max_claim_string_bytes,
        })
    }

    /// Return the underlying compact JWS limits.
    #[must_use]
    pub const fn jws(self) -> JwsLimits {
        self.jws
    }

    /// Maximum bytes in one issuer, audience, or nonce string.
    #[must_use]
    pub const fn max_claim_string_bytes(self) -> usize {
        self.max_claim_string_bytes
    }
}

impl Default for Oid4vciProofJwtLimits {
    fn default() -> Self {
        Self {
            jws: JwsLimits::default(),
            max_claim_string_bytes: DEFAULT_MAX_PROOF_CLAIM_STRING_BYTES,
        }
    }
}

/// Optional bounded trust evidence carried by an OID4VCI proof header.
#[derive(Clone, PartialEq, Eq)]
pub struct Oid4vciProofJwtEvidence {
    key_attestation: Option<String>,
    trust_chain: Option<Vec<String>>,
}

impl Oid4vciProofJwtEvidence {
    /// Validate opaque evidence before it can enter a protected signing input.
    pub fn new(
        key_attestation: Option<String>,
        trust_chain: Option<Vec<String>>,
        limits: Oid4vciProofJwtLimits,
    ) -> Result<Self, JoseError> {
        if !valid_protected_evidence(
            key_attestation.as_deref(),
            trust_chain.as_deref(),
            limits.jws(),
        ) {
            return Err(JoseError::InvalidProofEvidence);
        }
        Ok(Self {
            key_attestation,
            trust_chain,
        })
    }

    /// Construct an evidence-free proof configuration.
    #[must_use]
    pub const fn none() -> Self {
        Self {
            key_attestation: None,
            trust_chain: None,
        }
    }

    /// Borrow the untrusted key-attestation compact token, when present.
    #[must_use]
    pub fn key_attestation(&self) -> Option<&str> {
        self.key_attestation.as_deref()
    }

    /// Borrow the untrusted OpenID Federation trust chain, when present.
    #[must_use]
    pub fn trust_chain(&self) -> Option<&[String]> {
        self.trust_chain.as_deref()
    }

    fn validate(&self, limits: Oid4vciProofJwtLimits) -> Result<(), JoseError> {
        if !valid_protected_evidence(
            self.key_attestation.as_deref(),
            self.trust_chain.as_deref(),
            limits.jws(),
        ) {
            return Err(JoseError::InvalidProofEvidence);
        }
        Ok(())
    }
}

impl Default for Oid4vciProofJwtEvidence {
    fn default() -> Self {
        Self::none()
    }
}

impl fmt::Debug for Oid4vciProofJwtEvidence {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Oid4vciProofJwtEvidence")
            .field("has_key_attestation", &self.key_attestation.is_some())
            .field(
                "trust_chain_len",
                &self.trust_chain.as_ref().map_or(0, Vec::len),
            )
            .finish()
    }
}

/// Client identification mode for an OID4VCI key proof.
#[derive(Clone, PartialEq, Eq)]
pub enum Oid4vciProofJwtClient {
    /// Emit the supplied OAuth client identifier as the `iss` claim.
    Identified(String),
    /// Omit `iss` for anonymous access in a pre-authorized-code flow.
    AnonymousPreAuthorized,
}

impl Oid4vciProofJwtClient {
    /// Validate and construct an identified client value.
    pub fn identified(
        client_id: impl AsRef<str>,
        limits: Oid4vciProofJwtLimits,
    ) -> Result<Self, JoseError> {
        let client_id = client_id.as_ref();
        if !valid_claim(client_id, limits) {
            return Err(JoseError::InvalidProofClaims);
        }
        Ok(Self::Identified(client_id.to_owned()))
    }

    pub(crate) fn issuer(&self) -> Option<&str> {
        match self {
            Self::Identified(value) => Some(value),
            Self::AnonymousPreAuthorized => None,
        }
    }
}

impl fmt::Debug for Oid4vciProofJwtClient {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Identified(_) => formatter.write_str("Oid4vciProofJwtClient::Identified(..)"),
            Self::AnonymousPreAuthorized => {
                formatter.write_str("Oid4vciProofJwtClient::AnonymousPreAuthorized")
            }
        }
    }
}

/// Validated public claims for one OID4VCI JWT key proof.
#[derive(Clone, PartialEq, Eq)]
pub struct Oid4vciProofJwtClaims {
    client: Oid4vciProofJwtClient,
    audience: String,
    issued_at: i64,
    nonce: Option<String>,
}

impl Oid4vciProofJwtClaims {
    /// Validate profile claims under the supplied allocation limits.
    pub fn new(
        client: Oid4vciProofJwtClient,
        audience: impl AsRef<str>,
        issued_at: i64,
        nonce: Option<String>,
        limits: Oid4vciProofJwtLimits,
    ) -> Result<Self, JoseError> {
        let audience = audience.as_ref();
        if client
            .issuer()
            .is_some_and(|value| !valid_claim(value, limits))
            || !valid_claim(audience, limits)
            || nonce
                .as_deref()
                .is_some_and(|value| !valid_claim(value, limits))
        {
            return Err(JoseError::InvalidProofClaims);
        }
        let claims = Self {
            client,
            audience: audience.to_owned(),
            issued_at,
            nonce,
        };
        Ok(claims)
    }

    pub(crate) fn from_parsed(
        client: Oid4vciProofJwtClient,
        audience: String,
        issued_at: i64,
        nonce: Option<String>,
        limits: Oid4vciProofJwtLimits,
    ) -> Result<Self, JoseError> {
        let claims = Self {
            client,
            audience,
            issued_at,
            nonce,
        };
        claims.validate(limits)?;
        Ok(claims)
    }

    /// Borrow the client identification mode.
    #[must_use]
    pub const fn client(&self) -> &Oid4vciProofJwtClient {
        &self.client
    }

    /// Borrow the exact Credential Issuer Identifier audience.
    #[must_use]
    pub fn audience(&self) -> &str {
        &self.audience
    }

    /// Return the caller-supplied integer NumericDate issuance time.
    #[must_use]
    pub const fn issued_at(&self) -> i64 {
        self.issued_at
    }

    /// Borrow the server-provided nonce, when present.
    #[must_use]
    pub fn nonce(&self) -> Option<&str> {
        self.nonce.as_deref()
    }

    fn validate(&self, limits: Oid4vciProofJwtLimits) -> Result<(), JoseError> {
        if self
            .client
            .issuer()
            .is_some_and(|value| !valid_claim(value, limits))
            || !valid_claim(&self.audience, limits)
            || self
                .nonce
                .as_deref()
                .is_some_and(|value| !valid_claim(value, limits))
        {
            return Err(JoseError::InvalidProofClaims);
        }
        Ok(())
    }
}

impl fmt::Debug for Oid4vciProofJwtClaims {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Oid4vciProofJwtClaims")
            .field("client", &self.client)
            .field("has_nonce", &self.nonce.is_some())
            .finish_non_exhaustive()
    }
}

/// Stateless holder-side builder for one bounded OID4VCI JWT key proof.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Oid4vciProofJwtBuilder {
    limits: Oid4vciProofJwtLimits,
}

impl Oid4vciProofJwtBuilder {
    /// Construct a builder with explicit allocation limits.
    #[must_use]
    pub const fn new(limits: Oid4vciProofJwtLimits) -> Self {
        Self { limits }
    }

    /// Validate and encode all locally knowable proof data before signing.
    pub fn prepare(
        &self,
        algorithm: JwsAlgorithm,
        key_reference: JwsKeyReference,
        claims: Oid4vciProofJwtClaims,
    ) -> Result<Oid4vciProofSigningInput, JoseError> {
        self.prepare_with_evidence(
            algorithm,
            key_reference,
            claims,
            Oid4vciProofJwtEvidence::none(),
        )
    }

    /// Validate and encode a proof carrying optional attestation evidence.
    pub fn prepare_with_evidence(
        &self,
        algorithm: JwsAlgorithm,
        key_reference: JwsKeyReference,
        claims: Oid4vciProofJwtClaims,
        evidence: Oid4vciProofJwtEvidence,
    ) -> Result<Oid4vciProofSigningInput, JoseError> {
        claims.validate(self.limits)?;
        evidence.validate(self.limits)?;
        if evidence.trust_chain.is_some() && !matches!(key_reference, JwsKeyReference::KeyId(_)) {
            return Err(JoseError::InvalidProofEvidence);
        }
        if let JwsKeyReference::Jwk(public_key) = &key_reference {
            JwsVerificationKey::new(algorithm, public_key)?;
        }
        let header = ProtectedHeader::with_key_reference_and_evidence(
            algorithm.as_str(),
            Some(OID4VCI_PROOF_JWT_TYPE),
            Some(key_reference),
            evidence.key_attestation,
            evidence.trust_chain,
            self.limits.jws(),
        )?;
        let payload = encode_bounded_json(
            &WireClaims::from(&claims),
            self.limits.jws().max_payload_bytes(),
            JoseError::PayloadTooLarge,
            JoseError::InvalidProofClaims,
        )?;
        let input = JwsSigningInput::new(header, payload, self.limits.jws())?;
        Ok(Oid4vciProofSigningInput { input })
    }
}

impl Default for Oid4vciProofJwtBuilder {
    fn default() -> Self {
        Self::new(Oid4vciProofJwtLimits::default())
    }
}

/// Exact encoded OID4VCI proof bytes awaiting an external signature.
#[derive(Clone, PartialEq, Eq)]
pub struct Oid4vciProofSigningInput {
    input: JwsSigningInput,
}

impl Oid4vciProofSigningInput {
    /// Borrow the exact public ASCII bytes the signer must sign.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self.input.as_bytes()
    }

    /// Sign through an algorithm-bound external capability.
    pub fn sign_with(&self, signer: &dyn JwsSigner) -> Result<Oid4vciProofJwt, JoseError> {
        let compact = self.input.sign_with(signer)?;
        Ok(Oid4vciProofJwt { compact })
    }
}

impl fmt::Debug for Oid4vciProofSigningInput {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Oid4vciProofSigningInput")
            .field("signing_input_len", &self.input.as_bytes().len())
            .finish()
    }
}

/// A holder-produced OID4VCI proof JWT that has not been issuer-verified.
#[derive(Clone, PartialEq, Eq)]
pub struct Oid4vciProofJwt {
    compact: UnverifiedCompactJws,
}

impl Oid4vciProofJwt {
    /// Borrow the complete JWS Compact value for protocol transport.
    #[must_use]
    pub fn compact(&self) -> &str {
        self.compact.compact()
    }

    /// Borrow the signed but issuer-unverified compact value.
    #[must_use]
    pub const fn as_unverified(&self) -> &UnverifiedCompactJws {
        &self.compact
    }

    /// Consume the proof wrapper into its issuer-unverified compact value.
    #[must_use]
    pub fn into_unverified(self) -> UnverifiedCompactJws {
        self.compact
    }
}

impl fmt::Debug for Oid4vciProofJwt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Oid4vciProofJwt")
            .field("compact_len", &self.compact.compact().len())
            .finish()
    }
}

#[derive(Serialize)]
struct WireClaims<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    iss: Option<&'a str>,
    aud: &'a str,
    iat: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    nonce: Option<&'a str>,
}

impl<'a> From<&'a Oid4vciProofJwtClaims> for WireClaims<'a> {
    fn from(value: &'a Oid4vciProofJwtClaims) -> Self {
        Self {
            iss: value.client.issuer(),
            aud: &value.audience,
            iat: value.issued_at,
            nonce: value.nonce.as_deref(),
        }
    }
}

pub(crate) fn valid_claim(value: &str, limits: Oid4vciProofJwtLimits) -> bool {
    !value.is_empty()
        && value.len() <= limits.max_claim_string_bytes()
        && !value.chars().any(char::is_control)
}
