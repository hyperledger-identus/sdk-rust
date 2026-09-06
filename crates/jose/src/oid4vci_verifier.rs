//! Issuer-side verification of the OpenID4VCI 1.0 Final JWT key proof.

use std::{fmt, future::Future, pin::Pin};

use identus_core::WallClock;
use identus_crypto::PublicKeyJwk;
use identus_did::{DereferencingOptions, DidUrl, DidUrlDereferencer, VerificationRelationshipName};
use serde::{Deserialize, Deserializer};

use crate::oid4vci::valid_claim;
use crate::{
    JoseError, JwsAlgorithm, JwsKeyReference, JwsVerificationKey, OID4VCI_PROOF_JWT_TYPE,
    Oid4vciProofJwtClaims, Oid4vciProofJwtClient, Oid4vciProofJwtLimits, SignatureSuiteRegistry,
    UnverifiedCompactJws, VerifiedCompactJws,
};

const AUTHENTICATION: &str = "authentication";
const INVALID_RELATIONSHIP_URI: &str =
    "https://w3id.org/security#INVALID_RELATIONSHIP_FOR_VERIFICATION_METHOD";

/// Static failure returned by an injected X.509 certificate-key provider.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Oid4vciX5cKeyFailure {
    /// The chain or its caller-selected trust policy was rejected.
    Rejected,
    /// The provider could not service the request.
    Unavailable,
}

/// Future returned by [`Oid4vciX5cKeyProvider`].
pub type Oid4vciX5cKeyFuture<'a> =
    Pin<Box<dyn Future<Output = Result<PublicKeyJwk, Oid4vciX5cKeyFailure>> + Send + 'a>>;

/// Caller-owned certificate validation and leaf-key extraction capability.
pub trait Oid4vciX5cKeyProvider: Send + Sync {
    /// Validate one bounded chain and return its trusted leaf public key.
    fn verification_key<'a>(
        &'a self,
        algorithm: JwsAlgorithm,
        chain: &'a [String],
    ) -> Oid4vciX5cKeyFuture<'a>;
}

/// Static failure returned by an injected OpenID Federation trust-chain provider.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Oid4vciTrustChainFailure {
    /// The chain, selected key or caller-owned trust policy was rejected.
    Rejected,
    /// The provider could not service the request.
    Unavailable,
}

/// Future returned by [`Oid4vciTrustChainKeyProvider`].
pub type Oid4vciTrustChainKeyFuture<'a> =
    Pin<Box<dyn Future<Output = Result<PublicKeyJwk, Oid4vciTrustChainFailure>> + Send + 'a>>;

/// Caller-owned OpenID Federation validation and proof-key selection capability.
pub trait Oid4vciTrustChainKeyProvider: Send + Sync {
    /// Validate one bounded chain and return the trusted key selected by `key_id`.
    fn verification_key<'a>(
        &'a self,
        algorithm: JwsAlgorithm,
        key_id: &'a str,
        chain: &'a [String],
    ) -> Oid4vciTrustChainKeyFuture<'a>;
}

/// Static failure returned by an injected key-attestation validator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Oid4vciKeyAttestationFailure {
    /// The attestation, key binding or caller-owned trust policy was rejected.
    Rejected,
    /// The validator could not service the request.
    Unavailable,
}

/// Borrowed bounded evidence supplied to a key-attestation validator.
#[derive(Clone, Copy)]
pub struct Oid4vciKeyAttestationInput<'a> {
    attestation: &'a str,
    proof_key: &'a PublicKeyJwk,
    proof_nonce: Option<&'a str>,
}

impl<'a> Oid4vciKeyAttestationInput<'a> {
    /// Borrow the bounded untrusted key-attestation compact token.
    #[must_use]
    pub const fn attestation(&self) -> &'a str {
        self.attestation
    }

    /// Borrow the exact key that verified the outer proof signature.
    #[must_use]
    pub const fn proof_key(&self) -> &'a PublicKeyJwk {
        self.proof_key
    }

    /// Borrow the validated outer proof nonce, when supplied.
    #[must_use]
    pub const fn proof_nonce(&self) -> Option<&'a str> {
        self.proof_nonce
    }
}

impl fmt::Debug for Oid4vciKeyAttestationInput<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Oid4vciKeyAttestationInput")
            .field("attestation_len", &self.attestation.len())
            .field("has_proof_nonce", &self.proof_nonce.is_some())
            .finish_non_exhaustive()
    }
}

/// Future returned by [`Oid4vciKeyAttestationValidator`].
pub type Oid4vciKeyAttestationFuture<'a> =
    Pin<Box<dyn Future<Output = Result<(), Oid4vciKeyAttestationFailure>> + Send + 'a>>;

/// Caller-owned nested key-attestation validation and trust capability.
pub trait Oid4vciKeyAttestationValidator: Send + Sync {
    /// Validate and trust one attestation bound to the exact outer proof key.
    fn validate<'a>(
        &'a self,
        input: Oid4vciKeyAttestationInput<'a>,
    ) -> Oid4vciKeyAttestationFuture<'a>;
}

/// Static failure returned by an injected replay guard.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Oid4vciProofReplayFailure {
    /// Caller policy rejected this proof as replayed or otherwise inadmissible.
    Rejected,
    /// The replay provider could not service the request.
    Unavailable,
}

/// Future returned by [`Oid4vciProofReplayGuard`].
pub type Oid4vciProofReplayFuture<'a> =
    Pin<Box<dyn Future<Output = Result<(), Oid4vciProofReplayFailure>> + Send + 'a>>;

/// Atomic caller-owned proof replay check-and-record capability.
pub trait Oid4vciProofReplayGuard: Send + Sync {
    /// Atomically accept and record one otherwise valid proof.
    fn accept<'a>(&'a self, input: Oid4vciProofReplayInput<'a>) -> Oid4vciProofReplayFuture<'a>;
}

/// Borrowed bounded evidence supplied to an issuer's replay policy.
#[derive(Clone, Copy)]
pub struct Oid4vciProofReplayInput<'a> {
    proof: &'a VerifiedCompactJws,
    claims: &'a Oid4vciProofJwtClaims,
}

impl<'a> Oid4vciProofReplayInput<'a> {
    /// Borrow the exact cryptographically verified compact proof.
    #[must_use]
    pub const fn proof(&self) -> &'a VerifiedCompactJws {
        self.proof
    }

    /// Borrow the validated recognized proof claims.
    #[must_use]
    pub const fn claims(&self) -> &'a Oid4vciProofJwtClaims {
        self.claims
    }

    /// Borrow the exact header-selected public key reference.
    #[must_use]
    pub fn key_reference(&self) -> &'a JwsKeyReference {
        self.proof
            .as_compact()
            .protected_header()
            .key_reference()
            .expect("verified OID4VCI proof has one key reference")
    }
}

impl fmt::Debug for Oid4vciProofReplayInput<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Oid4vciProofReplayInput")
            .field("compact_len", &self.proof.as_compact().compact().len())
            .field("has_issuer", &self.claims.client().issuer().is_some())
            .field("has_nonce", &self.claims.nonce().is_some())
            .finish_non_exhaustive()
    }
}

/// Explicit expected nonce mode for issuer proof verification.
#[derive(Clone, PartialEq, Eq)]
pub struct Oid4vciProofJwtNonce(Option<String>);

impl Oid4vciProofJwtNonce {
    /// Require an exact non-empty bounded server-provided nonce.
    pub fn required(
        value: impl AsRef<str>,
        limits: Oid4vciProofJwtLimits,
    ) -> Result<Self, JoseError> {
        let value = value.as_ref();
        if !valid_claim(value, limits) {
            return Err(JoseError::InvalidProofPolicy);
        }
        Ok(Self(Some(value.to_owned())))
    }

    /// Require the proof to omit its nonce claim.
    #[must_use]
    pub const fn absent() -> Self {
        Self(None)
    }

    /// Borrow the exact required nonce, or `None` when absence is required.
    #[must_use]
    pub fn expected(&self) -> Option<&str> {
        self.0.as_deref()
    }
}

impl fmt::Debug for Oid4vciProofJwtNonce {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Oid4vciProofJwtNonce")
            .field("required", &self.0.is_some())
            .finish()
    }
}

/// Explicit issuer policy for one OID4VCI proof-verification context.
#[derive(Clone, PartialEq, Eq)]
pub struct Oid4vciProofJwtPolicy {
    client: Oid4vciProofJwtClient,
    audience: String,
    nonce: Oid4vciProofJwtNonce,
    max_age_seconds: u64,
    allowed_clock_skew_seconds: u64,
}

impl Oid4vciProofJwtPolicy {
    /// Validate and own the complete issuer policy.
    pub fn new(
        client: Oid4vciProofJwtClient,
        audience: impl AsRef<str>,
        nonce: Oid4vciProofJwtNonce,
        max_age_seconds: u64,
        allowed_clock_skew_seconds: u64,
        limits: Oid4vciProofJwtLimits,
    ) -> Result<Self, JoseError> {
        let audience = audience.as_ref();
        if client
            .issuer()
            .is_some_and(|value| !valid_claim(value, limits))
            || !valid_claim(audience, limits)
            || nonce
                .expected()
                .is_some_and(|value| !valid_claim(value, limits))
            || max_age_seconds
                .checked_add(allowed_clock_skew_seconds)
                .is_none()
        {
            return Err(JoseError::InvalidProofPolicy);
        }
        Ok(Self {
            client,
            audience: audience.to_owned(),
            nonce,
            max_age_seconds,
            allowed_clock_skew_seconds,
        })
    }

    /// Borrow the expected client mode.
    #[must_use]
    pub const fn client(&self) -> &Oid4vciProofJwtClient {
        &self.client
    }

    /// Borrow the expected Credential Issuer audience.
    #[must_use]
    pub fn audience(&self) -> &str {
        &self.audience
    }

    /// Borrow the exact nonce policy.
    #[must_use]
    pub const fn nonce(&self) -> &Oid4vciProofJwtNonce {
        &self.nonce
    }

    /// Maximum accepted age before clock skew is applied.
    #[must_use]
    pub const fn max_age_seconds(&self) -> u64 {
        self.max_age_seconds
    }

    /// Maximum accepted clock skew in either direction.
    #[must_use]
    pub const fn allowed_clock_skew_seconds(&self) -> u64 {
        self.allowed_clock_skew_seconds
    }
}

impl fmt::Debug for Oid4vciProofJwtPolicy {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Oid4vciProofJwtPolicy")
            .field("client", &self.client)
            .field("nonce", &self.nonce)
            .field("max_age_seconds", &self.max_age_seconds)
            .field(
                "allowed_clock_skew_seconds",
                &self.allowed_clock_skew_seconds,
            )
            .finish_non_exhaustive()
    }
}

/// A bounded profile-parsed proof whose signature and policy are unverified.
#[derive(Clone, PartialEq, Eq)]
pub struct Oid4vciParsedProofJwt {
    compact: UnverifiedCompactJws,
    claims: Oid4vciProofJwtClaims,
}

impl Oid4vciParsedProofJwt {
    /// Borrow the exact bounded signature-unverified compact value.
    #[must_use]
    pub const fn as_unverified(&self) -> &UnverifiedCompactJws {
        &self.compact
    }

    /// Borrow the validated recognized claims.
    #[must_use]
    pub const fn claims(&self) -> &Oid4vciProofJwtClaims {
        &self.claims
    }
}

impl fmt::Debug for Oid4vciParsedProofJwt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Oid4vciParsedProofJwt")
            .field("compact_len", &self.compact.compact().len())
            .field("has_issuer", &self.claims.client().issuer().is_some())
            .field("has_nonce", &self.claims.nonce().is_some())
            .finish_non_exhaustive()
    }
}

/// A proof whose exact signature verifies with its header-selected public key.
#[derive(Clone, PartialEq, Eq)]
pub struct Oid4vciVerifiedProofJwt {
    proof: VerifiedCompactJws,
    claims: Oid4vciProofJwtClaims,
    proof_key: PublicKeyJwk,
}

/// A cryptographically verified proof whose optional trust evidence is accepted.
#[derive(Clone, PartialEq, Eq)]
pub struct Oid4vciTrustedProofJwt {
    verified: Oid4vciVerifiedProofJwt,
}

impl Oid4vciTrustedProofJwt {
    /// Borrow the underlying cryptographically verified proof.
    #[must_use]
    pub const fn verified(&self) -> &Oid4vciVerifiedProofJwt {
        &self.verified
    }

    /// Borrow the recognized claims carried by the trusted proof.
    #[must_use]
    pub const fn claims(&self) -> &Oid4vciProofJwtClaims {
        self.verified.claims()
    }
}

impl fmt::Debug for Oid4vciTrustedProofJwt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Oid4vciTrustedProofJwt")
            .field("algorithm", &self.verified.proof.algorithm())
            .field(
                "has_key_attestation",
                &self
                    .verified
                    .proof
                    .as_compact()
                    .protected_header()
                    .key_attestation()
                    .is_some(),
            )
            .field(
                "trust_chain_len",
                &self
                    .verified
                    .proof
                    .as_compact()
                    .protected_header()
                    .trust_chain()
                    .map_or(0, <[String]>::len),
            )
            .finish_non_exhaustive()
    }
}

impl Oid4vciVerifiedProofJwt {
    /// Borrow the cryptographically verified compact evidence.
    #[must_use]
    pub const fn proof(&self) -> &VerifiedCompactJws {
        &self.proof
    }

    /// Borrow the validated recognized proof claims.
    #[must_use]
    pub const fn claims(&self) -> &Oid4vciProofJwtClaims {
        &self.claims
    }
}

impl fmt::Debug for Oid4vciVerifiedProofJwt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Oid4vciVerifiedProofJwt")
            .field("algorithm", &self.proof.algorithm())
            .field("compact_len", &self.proof.as_compact().compact().len())
            .field("has_issuer", &self.claims.client().issuer().is_some())
            .field("has_nonce", &self.claims.nonce().is_some())
            .finish_non_exhaustive()
    }
}

/// A cryptographically verified proof accepted by explicit issuer policy.
#[derive(Clone, PartialEq, Eq)]
pub struct Oid4vciAuthorizedProofJwt {
    trusted: Oid4vciTrustedProofJwt,
}

impl Oid4vciAuthorizedProofJwt {
    /// Borrow the underlying key-bound cryptographic evidence.
    #[must_use]
    pub const fn verified(&self) -> &Oid4vciVerifiedProofJwt {
        self.trusted.verified()
    }

    /// Borrow the trust-evaluated proof evidence.
    #[must_use]
    pub const fn trusted(&self) -> &Oid4vciTrustedProofJwt {
        &self.trusted
    }

    /// Borrow the issuer-authorized recognized claims.
    #[must_use]
    pub const fn claims(&self) -> &Oid4vciProofJwtClaims {
        self.trusted.claims()
    }
}

impl fmt::Debug for Oid4vciAuthorizedProofJwt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Oid4vciAuthorizedProofJwt")
            .field("algorithm", &self.trusted.verified.proof.algorithm())
            .field(
                "compact_len",
                &self.trusted.verified.proof.as_compact().compact().len(),
            )
            .finish_non_exhaustive()
    }
}

/// Stateless issuer verifier over caller-owned key and policy capabilities.
pub struct Oid4vciProofJwtVerifier<'a> {
    limits: Oid4vciProofJwtLimits,
    suites: &'a SignatureSuiteRegistry,
    did_dereferencer: Option<&'a dyn DidUrlDereferencer>,
    x5c_provider: Option<&'a dyn Oid4vciX5cKeyProvider>,
    trust_chain_provider: Option<&'a dyn Oid4vciTrustChainKeyProvider>,
    key_attestation_validator: Option<&'a dyn Oid4vciKeyAttestationValidator>,
}

impl<'a> Oid4vciProofJwtVerifier<'a> {
    /// Bind explicit limits, algorithm allowlist and optional key providers.
    #[must_use]
    pub const fn new(
        limits: Oid4vciProofJwtLimits,
        suites: &'a SignatureSuiteRegistry,
        did_dereferencer: Option<&'a dyn DidUrlDereferencer>,
        x5c_provider: Option<&'a dyn Oid4vciX5cKeyProvider>,
    ) -> Self {
        Self {
            limits,
            suites,
            did_dereferencer,
            x5c_provider,
            trust_chain_provider: None,
            key_attestation_validator: None,
        }
    }

    /// Add the caller-owned OpenID Federation trust-chain capability.
    #[must_use]
    pub const fn with_trust_chain_provider(
        mut self,
        provider: &'a dyn Oid4vciTrustChainKeyProvider,
    ) -> Self {
        self.trust_chain_provider = Some(provider);
        self
    }

    /// Add the caller-owned key-attestation validation capability.
    #[must_use]
    pub const fn with_key_attestation_validator(
        mut self,
        validator: &'a dyn Oid4vciKeyAttestationValidator,
    ) -> Self {
        self.key_attestation_validator = Some(validator);
        self
    }

    /// Parse and validate the bounded profile without invoking providers.
    pub fn parse(&self, value: &str) -> Result<Oid4vciParsedProofJwt, JoseError> {
        let compact = UnverifiedCompactJws::parse(value, self.limits.jws())?;
        let header = compact.protected_header();
        if header.type_() != Some(OID4VCI_PROOF_JWT_TYPE) {
            return Err(JoseError::InvalidProofType);
        }
        if header.key_reference().is_none() {
            return Err(JoseError::MissingProofKeyReference);
        }
        if header.trust_chain().is_some()
            && !matches!(header.key_reference(), Some(JwsKeyReference::KeyId(_)))
        {
            return Err(JoseError::InvalidProofEvidence);
        }
        let algorithm = JwsAlgorithm::parse(header.algorithm())?;
        if !self.suites.contains(algorithm) {
            return Err(JoseError::AlgorithmNotAllowed);
        }
        let wire: ParsedClaims =
            serde_json::from_slice(compact.payload()).map_err(|_| JoseError::InvalidProofClaims)?;
        let client = match wire.iss {
            Some(value) if valid_claim(&value, self.limits) => {
                Oid4vciProofJwtClient::Identified(value)
            }
            Some(_) => return Err(JoseError::InvalidProofClaims),
            None => Oid4vciProofJwtClient::AnonymousPreAuthorized,
        };
        let claims = Oid4vciProofJwtClaims::from_parsed(
            client,
            wire.aud,
            wire.iat,
            wire.nonce,
            self.limits,
        )?;
        Ok(Oid4vciParsedProofJwt { compact, claims })
    }

    /// Resolve the header-selected key and verify the exact JWS signature.
    pub async fn verify_signature(
        &self,
        parsed: Oid4vciParsedProofJwt,
    ) -> Result<Oid4vciVerifiedProofJwt, JoseError> {
        let header = parsed.compact.protected_header();
        let algorithm = JwsAlgorithm::parse(header.algorithm())?;
        let selected = header
            .key_reference()
            .ok_or(JoseError::MissingProofKeyReference)?;
        let (proof, proof_key) = match selected {
            JwsKeyReference::Jwk(public_key) => {
                let key = JwsVerificationKey::new(algorithm, public_key)?;
                (
                    self.suites.verify(&parsed.compact, &key)?,
                    public_key.clone(),
                )
            }
            JwsKeyReference::KeyId(value) => {
                let public_key = if let Some(chain) = header.trust_chain() {
                    self.resolve_trust_chain_key(algorithm, value, chain)
                        .await?
                } else {
                    self.resolve_did_key(value).await?
                };
                let key = JwsVerificationKey::new(algorithm, &public_key)?;
                (self.suites.verify(&parsed.compact, &key)?, public_key)
            }
            JwsKeyReference::X5c(value) => {
                let public_key = self.resolve_x5c_key(algorithm, value).await?;
                let key = JwsVerificationKey::new(algorithm, &public_key)?;
                (self.suites.verify(&parsed.compact, &key)?, public_key)
            }
        };
        Ok(Oid4vciVerifiedProofJwt {
            proof,
            claims: parsed.claims,
            proof_key,
        })
    }

    /// Validate optional proof trust evidence through caller-owned capabilities.
    pub async fn validate_trust(
        &self,
        verified: Oid4vciVerifiedProofJwt,
    ) -> Result<Oid4vciTrustedProofJwt, JoseError> {
        if let Some(attestation) = verified
            .proof
            .as_compact()
            .protected_header()
            .key_attestation()
        {
            let validator = self
                .key_attestation_validator
                .ok_or(JoseError::KeyAttestationProviderRequired)?;
            validator
                .validate(Oid4vciKeyAttestationInput {
                    attestation,
                    proof_key: &verified.proof_key,
                    proof_nonce: verified.claims.nonce(),
                })
                .await
                .map_err(|failure| match failure {
                    Oid4vciKeyAttestationFailure::Rejected => JoseError::KeyAttestationRejected,
                    Oid4vciKeyAttestationFailure::Unavailable => {
                        JoseError::KeyAttestationProviderUnavailable
                    }
                })?;
        }
        Ok(Oid4vciTrustedProofJwt { verified })
    }

    /// Apply explicit issuer claims, freshness and replay policy.
    pub async fn authorize(
        &self,
        verified: Oid4vciVerifiedProofJwt,
        policy: &Oid4vciProofJwtPolicy,
        clock: &dyn WallClock,
        replay: &dyn Oid4vciProofReplayGuard,
    ) -> Result<Oid4vciAuthorizedProofJwt, JoseError> {
        validate_policy_with_clock(&verified.claims, policy, clock)?;
        let trusted = self.validate_trust(verified).await?;
        accept_replay(trusted, replay).await
    }

    /// Apply issuer claims, freshness and replay policy to trust-evaluated proof.
    pub async fn authorize_trusted(
        &self,
        trusted: Oid4vciTrustedProofJwt,
        policy: &Oid4vciProofJwtPolicy,
        clock: &dyn WallClock,
        replay: &dyn Oid4vciProofReplayGuard,
    ) -> Result<Oid4vciAuthorizedProofJwt, JoseError> {
        validate_policy_with_clock(trusted.claims(), policy, clock)?;
        accept_replay(trusted, replay).await
    }

    /// Compose parsing, key-bound signature verification and issuer policy.
    pub async fn verify_and_authorize(
        &self,
        value: &str,
        policy: &Oid4vciProofJwtPolicy,
        clock: &dyn WallClock,
        replay: &dyn Oid4vciProofReplayGuard,
    ) -> Result<Oid4vciAuthorizedProofJwt, JoseError> {
        let parsed = self.parse(value)?;
        validate_policy_with_clock(&parsed.claims, policy, clock)?;
        let verified = self.verify_signature(parsed).await?;
        let trusted = self.validate_trust(verified).await?;
        accept_replay(trusted, replay).await
    }

    async fn resolve_did_key(&self, value: &str) -> Result<PublicKeyJwk, JoseError> {
        let did_url = DidUrl::parse(value).map_err(|_| JoseError::UnsupportedProofKeyReference)?;
        if did_url.fragment().is_none() || !did_url.path().is_empty() || did_url.query().is_some() {
            return Err(JoseError::UnsupportedProofKeyReference);
        }
        let dereferencer = self
            .did_dereferencer
            .ok_or(JoseError::UnsupportedProofKeyReference)?;
        let relationship = VerificationRelationshipName::parse(AUTHENTICATION)
            .map_err(|_| JoseError::ProofKeyResolutionFailed)?;
        let options = DereferencingOptions::builder()
            .verification_relationship(relationship)
            .build()
            .map_err(|_| JoseError::ProofKeyResolutionFailed)?;
        let result = dereferencer.dereference(&did_url, &options).await;
        if let Some(error) = result.metadata().error() {
            if error.type_uri().as_str() == INVALID_RELATIONSHIP_URI {
                return Err(JoseError::ProofKeyNotAuthorized);
            }
            return Err(JoseError::ProofKeyResolutionFailed);
        }
        let method = result
            .content()
            .ok_or(JoseError::ProofKeyResolutionFailed)?
            .to_verification_method()
            .map_err(|_| JoseError::ProofKeyResolutionFailed)?;
        if method.id().as_str() != value {
            return Err(JoseError::ProofKeyResolutionFailed);
        }
        let jwk = method
            .public_key_jwk()
            .ok_or(JoseError::UnsupportedProofKeyReference)?;
        serde_json::from_value(serde_json::Value::Object(jwk.clone()))
            .map_err(|_| JoseError::InvalidVerificationKey)
    }

    async fn resolve_x5c_key(
        &self,
        algorithm: JwsAlgorithm,
        chain: &[String],
    ) -> Result<PublicKeyJwk, JoseError> {
        let provider = self.x5c_provider.ok_or(JoseError::X5cProviderRequired)?;
        provider
            .verification_key(algorithm, chain)
            .await
            .map_err(|failure| match failure {
                Oid4vciX5cKeyFailure::Rejected => JoseError::X5cRejected,
                Oid4vciX5cKeyFailure::Unavailable => JoseError::X5cProviderUnavailable,
            })
    }

    async fn resolve_trust_chain_key(
        &self,
        algorithm: JwsAlgorithm,
        key_id: &str,
        chain: &[String],
    ) -> Result<PublicKeyJwk, JoseError> {
        let provider = self
            .trust_chain_provider
            .ok_or(JoseError::TrustChainProviderRequired)?;
        provider
            .verification_key(algorithm, key_id, chain)
            .await
            .map_err(|failure| match failure {
                Oid4vciTrustChainFailure::Rejected => JoseError::TrustChainRejected,
                Oid4vciTrustChainFailure::Unavailable => JoseError::TrustChainProviderUnavailable,
            })
    }
}

impl fmt::Debug for Oid4vciProofJwtVerifier<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Oid4vciProofJwtVerifier")
            .field("limits", &self.limits)
            .field("algorithm_count", &self.suites.len())
            .field("has_did_dereferencer", &self.did_dereferencer.is_some())
            .field("has_x5c_provider", &self.x5c_provider.is_some())
            .field(
                "has_trust_chain_provider",
                &self.trust_chain_provider.is_some(),
            )
            .field(
                "has_key_attestation_validator",
                &self.key_attestation_validator.is_some(),
            )
            .finish()
    }
}

#[derive(Deserialize)]
struct ParsedClaims {
    #[serde(default, deserialize_with = "deserialize_optional_string")]
    iss: Option<String>,
    aud: String,
    iat: i64,
    #[serde(default, deserialize_with = "deserialize_optional_string")]
    nonce: Option<String>,
}

fn deserialize_optional_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(deserializer).map(Some)
}

fn validate_claim_policy(
    claims: &Oid4vciProofJwtClaims,
    policy: &Oid4vciProofJwtPolicy,
) -> Result<(), JoseError> {
    if claims.client().issuer() != policy.client.issuer() {
        return Err(JoseError::ProofClientMismatch);
    }
    if claims.audience() != policy.audience {
        return Err(JoseError::ProofAudienceMismatch);
    }
    if claims.nonce() != policy.nonce.expected() {
        return Err(JoseError::ProofNonceMismatch);
    }
    Ok(())
}

fn validate_policy_with_clock(
    claims: &Oid4vciProofJwtClaims,
    policy: &Oid4vciProofJwtPolicy,
    clock: &dyn WallClock,
) -> Result<(), JoseError> {
    validate_claim_policy(claims, policy)?;
    let now = clock
        .now()
        .map_err(|_| JoseError::ProofClockUnavailable)?
        .whole_seconds();
    validate_freshness(claims.issued_at(), now, policy)
}

async fn accept_replay(
    trusted: Oid4vciTrustedProofJwt,
    replay: &dyn Oid4vciProofReplayGuard,
) -> Result<Oid4vciAuthorizedProofJwt, JoseError> {
    replay
        .accept(Oid4vciProofReplayInput {
            proof: &trusted.verified.proof,
            claims: &trusted.verified.claims,
        })
        .await
        .map_err(|failure| match failure {
            Oid4vciProofReplayFailure::Rejected => JoseError::ProofReplayRejected,
            Oid4vciProofReplayFailure::Unavailable => JoseError::ProofReplayUnavailable,
        })?;
    Ok(Oid4vciAuthorizedProofJwt { trusted })
}

fn validate_freshness(
    issued_at: i64,
    now: u64,
    policy: &Oid4vciProofJwtPolicy,
) -> Result<(), JoseError> {
    let issued_at = u64::try_from(issued_at).map_err(|_| JoseError::ProofStale)?;
    let latest = now
        .checked_add(policy.allowed_clock_skew_seconds)
        .ok_or(JoseError::InvalidProofPolicy)?;
    if issued_at > latest {
        return Err(JoseError::ProofIssuedInFuture);
    }
    let accepted_age = policy
        .max_age_seconds
        .checked_add(policy.allowed_clock_skew_seconds)
        .ok_or(JoseError::InvalidProofPolicy)?;
    if now
        .checked_sub(issued_at)
        .is_some_and(|age| age > accepted_age)
    {
        return Err(JoseError::ProofStale);
    }
    Ok(())
}
