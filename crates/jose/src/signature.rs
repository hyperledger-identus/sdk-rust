//! Bound signing and verification capabilities for compact JWS values.

use std::fmt;

use identus_crypto::{
    Ed25519PrivateKey, Ed25519PublicKey, JwkCurve, JwkKeyType, P256PrivateKey, P256PublicKey,
    PublicKeyJwk, Verifiable,
};
use serde_json::Value;

use crate::{JoseError, JwsSigningInput, UnverifiedCompactJws};

const SIGNATURE_BYTES: usize = 64;

/// Maximum number of verifier suites in one registry.
pub const MAX_SIGNATURE_SUITES: usize = 16;

/// Closed asymmetric JOSE algorithms accepted by this capability slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JwsAlgorithm {
    /// Fully specified EdDSA using the Ed25519 parameter set.
    Ed25519,
    /// ECDSA using P-256 and SHA-256.
    Es256,
    /// Deprecated polymorphic JOSE spelling, restricted here to Ed25519.
    LegacyEdDsa,
}

impl JwsAlgorithm {
    /// Return the exact case-sensitive JOSE registry spelling.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ed25519 => "Ed25519",
            Self::Es256 => "ES256",
            Self::LegacyEdDsa => "EdDSA",
        }
    }

    pub(crate) fn parse(value: &str) -> Result<Self, JoseError> {
        match value {
            "Ed25519" => Ok(Self::Ed25519),
            "ES256" => Ok(Self::Es256),
            "EdDSA" => Ok(Self::LegacyEdDsa),
            _ => Err(JoseError::UnsupportedAlgorithm),
        }
    }

    const fn key_profile(self) -> (JwkKeyType, JwkCurve) {
        match self {
            Self::Ed25519 | Self::LegacyEdDsa => (JwkKeyType::Okp, JwkCurve::Ed25519),
            Self::Es256 => (JwkKeyType::Ec, JwkCurve::P256),
        }
    }
}

impl fmt::Display for JwsAlgorithm {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A static failure class returned by an external signing capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum SignerFailure {
    /// The provider deliberately rejected the operation.
    Rejected,
    /// The provider could not service the operation.
    Unavailable,
}

/// Runtime-neutral external signing capability over public message bytes.
pub trait JwsSigner: Send + Sync {
    /// The one algorithm to which this signer is bound.
    fn algorithm(&self) -> JwsAlgorithm;

    /// Sign the exact supplied JWS signing input.
    fn sign(&self, signing_input: &[u8]) -> Result<[u8; 64], SignerFailure>;
}

impl JwsSigningInput {
    /// Sign this exact input through an external capability.
    pub fn sign_with(&self, signer: &dyn JwsSigner) -> Result<UnverifiedCompactJws, JoseError> {
        let header_algorithm = JwsAlgorithm::parse(self.protected_header().algorithm())?;
        if header_algorithm != signer.algorithm() {
            return Err(JoseError::AlgorithmMismatch);
        }
        self.validate_signature_length(SIGNATURE_BYTES)?;
        let signature = signer
            .sign(self.as_bytes())
            .map_err(|failure| match failure {
                SignerFailure::Rejected => JoseError::SigningRejected,
                SignerFailure::Unavailable => JoseError::SignerUnavailable,
            })?;
        self.clone().attach_signature(signature.to_vec())
    }
}

/// Software Ed25519 signer borrowing a typed crypto private key.
pub struct Ed25519Signer<'key> {
    private_key: &'key Ed25519PrivateKey,
    algorithm: JwsAlgorithm,
}

impl<'key> Ed25519Signer<'key> {
    /// Bind a software key to the fully specified `Ed25519` algorithm.
    #[must_use]
    pub const fn new(private_key: &'key Ed25519PrivateKey) -> Self {
        Self {
            private_key,
            algorithm: JwsAlgorithm::Ed25519,
        }
    }

    /// Bind a software key to explicitly accepted legacy `EdDSA` values.
    #[must_use]
    pub const fn legacy(private_key: &'key Ed25519PrivateKey) -> Self {
        Self {
            private_key,
            algorithm: JwsAlgorithm::LegacyEdDsa,
        }
    }
}

impl JwsSigner for Ed25519Signer<'_> {
    fn algorithm(&self) -> JwsAlgorithm {
        self.algorithm
    }

    fn sign(&self, signing_input: &[u8]) -> Result<[u8; 64], SignerFailure> {
        self.private_key
            .sign(signing_input)
            .try_into()
            .map_err(|_| SignerFailure::Rejected)
    }
}

/// Software ES256 signer borrowing a typed crypto P-256 private key.
pub struct Es256Signer<'key> {
    private_key: &'key P256PrivateKey,
}

impl<'key> Es256Signer<'key> {
    /// Bind a software key to `ES256`.
    #[must_use]
    pub const fn new(private_key: &'key P256PrivateKey) -> Self {
        Self { private_key }
    }
}

impl JwsSigner for Es256Signer<'_> {
    fn algorithm(&self) -> JwsAlgorithm {
        JwsAlgorithm::Es256
    }

    fn sign(&self, signing_input: &[u8]) -> Result<[u8; 64], SignerFailure> {
        Ok(self.private_key.sign_fixed(signing_input))
    }
}

/// A public JWK explicitly bound to one JOSE algorithm.
pub struct JwsVerificationKey<'key> {
    algorithm: JwsAlgorithm,
    public_key: &'key PublicKeyJwk,
}

impl<'key> JwsVerificationKey<'key> {
    /// Bind a public JWK to exactly one accepted algorithm.
    pub fn new(algorithm: JwsAlgorithm, public_key: &'key PublicKeyJwk) -> Result<Self, JoseError> {
        let (expected_type, expected_curve) = algorithm.key_profile();
        if public_key.kty() != expected_type || public_key.crv() != expected_curve {
            return Err(JoseError::InvalidVerificationKey);
        }
        if let Some(declared) = public_key.extensions().get("alg") {
            match declared {
                Value::String(value) if value == algorithm.as_str() => {}
                _ => return Err(JoseError::AlgorithmMismatch),
            }
        }
        Ok(Self {
            algorithm,
            public_key,
        })
    }

    /// The algorithm selected for this key use.
    #[must_use]
    pub const fn algorithm(&self) -> JwsAlgorithm {
        self.algorithm
    }

    /// Borrow the validated public-only JWK.
    #[must_use]
    pub const fn public_key(&self) -> &PublicKeyJwk {
        self.public_key
    }
}

impl fmt::Debug for JwsVerificationKey<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("JwsVerificationKey")
            .field("algorithm", &self.algorithm)
            .field("key_type", &self.public_key.kty())
            .field("curve", &self.public_key.crv())
            .finish_non_exhaustive()
    }
}

/// Object-safe verification suite selected by an explicit registry entry.
pub trait JwsSignatureSuite: Send + Sync {
    /// The one algorithm implemented by this suite.
    fn algorithm(&self) -> JwsAlgorithm;

    /// Verify exact JWS bytes with an already algorithm-bound public JWK.
    fn verify(
        &self,
        signing_input: &[u8],
        signature: &[u8],
        public_key: &PublicKeyJwk,
    ) -> Result<(), JoseError>;
}

/// Built-in fully specified Ed25519 verification suite.
#[derive(Debug, Default, Clone, Copy)]
pub struct Ed25519SignatureSuite;

impl JwsSignatureSuite for Ed25519SignatureSuite {
    fn algorithm(&self) -> JwsAlgorithm {
        JwsAlgorithm::Ed25519
    }

    fn verify(
        &self,
        signing_input: &[u8],
        signature: &[u8],
        public_key: &PublicKeyJwk,
    ) -> Result<(), JoseError> {
        verify_ed25519(signing_input, signature, public_key)
    }
}

/// Built-in legacy `EdDSA` suite, restricted to strict Ed25519.
#[derive(Debug, Default, Clone, Copy)]
pub struct LegacyEdDsaSignatureSuite;

impl JwsSignatureSuite for LegacyEdDsaSignatureSuite {
    fn algorithm(&self) -> JwsAlgorithm {
        JwsAlgorithm::LegacyEdDsa
    }

    fn verify(
        &self,
        signing_input: &[u8],
        signature: &[u8],
        public_key: &PublicKeyJwk,
    ) -> Result<(), JoseError> {
        verify_ed25519(signing_input, signature, public_key)
    }
}

fn verify_ed25519(
    signing_input: &[u8],
    signature: &[u8],
    public_key: &PublicKeyJwk,
) -> Result<(), JoseError> {
    if signature.len() != SIGNATURE_BYTES {
        return Err(JoseError::InvalidSignatureLength);
    }
    let bytes = public_key.x().to_bytes();
    let key =
        Ed25519PublicKey::from_slice(&bytes).map_err(|_| JoseError::InvalidVerificationKey)?;
    if key.verify(signing_input, signature) {
        Ok(())
    } else {
        Err(JoseError::SignatureInvalid)
    }
}

/// Built-in ES256 verification suite using raw 64-byte `r || s` signatures.
#[derive(Debug, Default, Clone, Copy)]
pub struct Es256SignatureSuite;

impl JwsSignatureSuite for Es256SignatureSuite {
    fn algorithm(&self) -> JwsAlgorithm {
        JwsAlgorithm::Es256
    }

    fn verify(
        &self,
        signing_input: &[u8],
        signature: &[u8],
        public_key: &PublicKeyJwk,
    ) -> Result<(), JoseError> {
        let signature: &[u8; SIGNATURE_BYTES] = signature
            .try_into()
            .map_err(|_| JoseError::InvalidSignatureLength)?;
        let x = public_key.x().to_bytes();
        let y = public_key
            .y()
            .ok_or(JoseError::InvalidVerificationKey)?
            .to_bytes();
        let mut encoded = [0_u8; 65];
        encoded[0] = 4;
        encoded[1..33].copy_from_slice(&x);
        encoded[33..65].copy_from_slice(&y);
        let key =
            P256PublicKey::from_slice(&encoded).map_err(|_| JoseError::InvalidVerificationKey)?;
        if key.verify_fixed(signing_input, signature) {
            Ok(())
        } else {
            Err(JoseError::SignatureInvalid)
        }
    }
}

/// Caller-owned, bounded allowlist and dispatcher of JWS verifier suites.
pub struct SignatureSuiteRegistry {
    suites: Vec<RegisteredSuite>,
    capacity: usize,
}

struct RegisteredSuite {
    algorithm: JwsAlgorithm,
    verifier: Box<dyn JwsSignatureSuite>,
}

impl SignatureSuiteRegistry {
    /// Create an empty registry with a positive capacity no greater than 16.
    pub fn new(capacity: usize) -> Result<Self, JoseError> {
        if capacity == 0 || capacity > MAX_SIGNATURE_SUITES {
            return Err(JoseError::InvalidRegistryCapacity);
        }
        Ok(Self {
            suites: Vec::with_capacity(capacity),
            capacity,
        })
    }

    /// Create the recommended allowlist containing `Ed25519` and `ES256`.
    #[must_use]
    pub fn recommended() -> Self {
        Self {
            suites: vec![
                RegisteredSuite {
                    algorithm: JwsAlgorithm::Ed25519,
                    verifier: Box::new(Ed25519SignatureSuite),
                },
                RegisteredSuite {
                    algorithm: JwsAlgorithm::Es256,
                    verifier: Box::new(Es256SignatureSuite),
                },
            ],
            capacity: MAX_SIGNATURE_SUITES,
        }
    }

    /// Add one verifier suite to this explicit allowlist.
    pub fn register(&mut self, suite: impl JwsSignatureSuite + 'static) -> Result<(), JoseError> {
        let algorithm = suite.algorithm();
        if self.contains(algorithm) {
            return Err(JoseError::DuplicateAlgorithm);
        }
        if self.suites.len() == self.capacity {
            return Err(JoseError::RegistryFull);
        }
        self.suites.push(RegisteredSuite {
            algorithm,
            verifier: Box::new(suite),
        });
        Ok(())
    }

    /// Return whether the allowlist contains one exact algorithm.
    #[must_use]
    pub fn contains(&self, algorithm: JwsAlgorithm) -> bool {
        self.suites.iter().any(|suite| suite.algorithm == algorithm)
    }

    /// Number of registered verifier suites.
    #[must_use]
    pub fn len(&self) -> usize {
        self.suites.len()
    }

    /// Return whether no verifier suite is registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.suites.is_empty()
    }

    /// Verify an unverified compact value with one explicitly bound key.
    pub fn verify(
        &self,
        compact: &UnverifiedCompactJws,
        key: &JwsVerificationKey<'_>,
    ) -> Result<VerifiedCompactJws, JoseError> {
        let header_algorithm = JwsAlgorithm::parse(compact.protected_header().algorithm())?;
        if header_algorithm != key.algorithm {
            return Err(JoseError::AlgorithmMismatch);
        }
        let suite = self
            .suites
            .iter()
            .find(|suite| suite.algorithm == header_algorithm)
            .ok_or(JoseError::AlgorithmNotAllowed)?;
        if compact.signature().len() != SIGNATURE_BYTES {
            return Err(JoseError::InvalidSignatureLength);
        }
        suite
            .verifier
            .verify(compact.signing_input(), compact.signature(), key.public_key)?;
        Ok(VerifiedCompactJws {
            compact: compact.clone(),
            algorithm: header_algorithm,
        })
    }
}

impl fmt::Debug for SignatureSuiteRegistry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let algorithms: Vec<_> = self.suites.iter().map(|suite| suite.algorithm).collect();
        formatter
            .debug_struct("SignatureSuiteRegistry")
            .field("algorithms", &algorithms)
            .field("capacity", &self.capacity)
            .finish()
    }
}

/// A compact JWS accepted by one registered suite and bound public key.
#[derive(Clone, PartialEq, Eq)]
pub struct VerifiedCompactJws {
    compact: UnverifiedCompactJws,
    algorithm: JwsAlgorithm,
}

impl VerifiedCompactJws {
    /// The exact algorithm accepted during verification.
    #[must_use]
    pub const fn algorithm(&self) -> JwsAlgorithm {
        self.algorithm
    }

    /// Borrow the original bounded compact value and its decoded data.
    #[must_use]
    pub const fn as_compact(&self) -> &UnverifiedCompactJws {
        &self.compact
    }

    /// Consume the evidence wrapper and return the original compact value.
    #[must_use]
    pub fn into_compact(self) -> UnverifiedCompactJws {
        self.compact
    }
}

impl fmt::Debug for VerifiedCompactJws {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("VerifiedCompactJws")
            .field("algorithm", &self.algorithm)
            .field("payload_len", &self.compact.payload().len())
            .field("compact_len", &self.compact.compact().len())
            .finish()
    }
}
