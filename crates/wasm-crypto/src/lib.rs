//! Narrow browser-WASM bindings for selected key operations.
//!
//! Private material remains inside opaque Rust-owned handles. This crate does
//! not provide custody, persistence, private export, transport, or protocol
//! policy. Hierarchical derivation remains profile-explicit and secret state
//! stays inside opaque Rust-owned handles.

use identus_adapters_entropy::GetrandomSystemRandomAdapter;
use identus_crypto::path::DerivationPath;
use identus_crypto::{
    CardanoV2ExtendedPrivateKey, CardanoV2ExtendedPublicKey, Ed25519KeyPair, Ed25519PrivateKey,
    Ed25519PublicKey, EdHDKey, EncodeVec, HDKey, MAX_DERIVATION_PATH_AXES, MAX_HD_SEED_BYTES,
    MIN_HD_SEED_BYTES, Secp256k1PrivateKey, Verifiable, X25519KeyPair, X25519PrivateKey,
    X25519PublicKey,
};
use wasm_bindgen::prelude::*;

/// Largest message accepted by the TypeScript signing and verification bridge.
pub const MAX_KEY_OPERATION_MESSAGE_BYTES: usize = 1_048_576;
/// Exact Ed25519 private/public key byte length.
pub const ED25519_KEY_BYTES: usize = 32;
/// Exact Ed25519 signature byte length.
pub const ED25519_SIGNATURE_BYTES: usize = 64;
/// Exact X25519 private/public key byte length.
pub const X25519_KEY_BYTES: usize = 32;

const API_VERSION: u32 = 1;
const HD_DERIVATION_API_VERSION: u32 = 1;
const CARDANO_V2_SEED_BYTES: usize = 64;

const SECP256K1_BIP32_HARDENED_V1: &str = "secp256k1-bip32-hardened-v1";
const ED25519_SLIP10_HARDENED_V1: &str = "ed25519-slip10-hardened-v1";
const ED25519_CARDANO_V2_V1: &str = "ed25519-cardano-v2-v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BindingError {
    Disposed,
    InvalidPrivateKey,
    InvalidPublicKey,
    InvalidSignature,
    MessageTooLarge,
    NonContributoryPeer,
    EntropyUnavailable,
    InvalidProfile,
    InvalidSeed,
    InvalidPath,
    ForbiddenAxis,
    DepthOverflow,
    DerivationFailed,
    ProjectionFailed,
}

impl BindingError {
    const fn code(self) -> &'static str {
        match self {
            Self::Disposed => "crypto.handle_disposed",
            Self::InvalidPrivateKey => "crypto.invalid_private_key",
            Self::InvalidPublicKey => "crypto.invalid_public_key",
            Self::InvalidSignature => "crypto.invalid_signature",
            Self::MessageTooLarge => "crypto.message_too_large",
            Self::NonContributoryPeer => "crypto.non_contributory_peer",
            Self::EntropyUnavailable => "crypto.entropy_unavailable",
            Self::InvalidProfile => "crypto.hd.invalid_profile",
            Self::InvalidSeed => "crypto.hd.invalid_seed",
            Self::InvalidPath => "crypto.hd.invalid_path",
            Self::ForbiddenAxis => "crypto.hd.forbidden_axis",
            Self::DepthOverflow => "crypto.hd.depth_overflow",
            Self::DerivationFailed => "crypto.hd.derivation_failed",
            Self::ProjectionFailed => "crypto.hd.projection_failed",
        }
    }

    fn into_js(self) -> JsError {
        JsError::new(self.code())
    }
}

fn bounded_message(message: &[u8]) -> Result<&[u8], BindingError> {
    if message.len() > MAX_KEY_OPERATION_MESSAGE_BYTES {
        return Err(BindingError::MessageTooLarge);
    }
    Ok(message)
}

fn ed25519_public(bytes: &[u8]) -> Result<Ed25519PublicKey, BindingError> {
    let key = Ed25519PublicKey::from_slice(bytes).map_err(|_| BindingError::InvalidPublicKey)?;
    if !key.has_canonical_encoding() || key.is_weak() {
        return Err(BindingError::InvalidPublicKey);
    }
    Ok(key)
}

fn x25519_public(bytes: &[u8]) -> Result<X25519PublicKey, BindingError> {
    let key = X25519PublicKey::from_slice(bytes).map_err(|_| BindingError::InvalidPublicKey)?;
    if !key.has_canonical_encoding() {
        return Err(BindingError::InvalidPublicKey);
    }
    Ok(key)
}

/// Return the exact private binding contract version.
#[wasm_bindgen]
#[must_use]
pub fn crypto_binding_api_version() -> u32 {
    API_VERSION
}

/// Return the exact HD-derivation binding contract version.
#[wasm_bindgen]
#[must_use]
pub fn hd_derivation_binding_api_version() -> u32 {
    HD_DERIVATION_API_VERSION
}

enum HdPrivateKey {
    Secp256k1(HDKey),
    Ed25519Slip10(EdHDKey),
    Ed25519CardanoV2 {
        key: CardanoV2ExtendedPrivateKey,
        depth: u32,
    },
}

impl HdPrivateKey {
    fn from_seed(profile: &str, seed: &[u8]) -> Result<Self, BindingError> {
        match profile {
            SECP256K1_BIP32_HARDENED_V1 => {
                require_standard_seed(seed)?;
                HDKey::init_from_seed(seed)
                    .map(Self::Secp256k1)
                    .map_err(|_| BindingError::DerivationFailed)
            }
            ED25519_SLIP10_HARDENED_V1 => {
                require_standard_seed(seed)?;
                EdHDKey::init_from_seed(seed)
                    .map(Self::Ed25519Slip10)
                    .map_err(|_| BindingError::DerivationFailed)
            }
            ED25519_CARDANO_V2_V1 => {
                if seed.len() != CARDANO_V2_SEED_BYTES {
                    return Err(BindingError::InvalidSeed);
                }
                CardanoV2ExtendedPrivateKey::from_seed(seed)
                    .map(|key| Self::Ed25519CardanoV2 { key, depth: 0 })
                    .map_err(|_| BindingError::DerivationFailed)
            }
            _ => Err(BindingError::InvalidProfile),
        }
    }

    const fn profile(&self) -> &'static str {
        match self {
            Self::Secp256k1(_) => SECP256K1_BIP32_HARDENED_V1,
            Self::Ed25519Slip10(_) => ED25519_SLIP10_HARDENED_V1,
            Self::Ed25519CardanoV2 { .. } => ED25519_CARDANO_V2_V1,
        }
    }

    const fn depth(&self) -> u32 {
        match self {
            Self::Secp256k1(key) => key.depth,
            Self::Ed25519Slip10(key) => key.depth,
            Self::Ed25519CardanoV2 { depth, .. } => *depth,
        }
    }

    fn derive(&self, path: &str) -> Result<Self, BindingError> {
        let path = parse_path(path)?;
        let depth = checked_depth(self.depth(), path.len())?;
        match self {
            Self::Secp256k1(key) => {
                require_hardened(&path)?;
                key.derive(&path.to_string())
                    .map(Self::Secp256k1)
                    .map_err(|_| BindingError::DerivationFailed)
            }
            Self::Ed25519Slip10(key) => {
                require_hardened(&path)?;
                key.derive(&path.to_string())
                    .map(Self::Ed25519Slip10)
                    .map_err(|_| BindingError::DerivationFailed)
            }
            Self::Ed25519CardanoV2 { key, .. } => key
                .derive_path(&path)
                .map(|key| Self::Ed25519CardanoV2 { key, depth })
                .map_err(|_| BindingError::DerivationFailed),
        }
    }

    fn public_key_bytes(&self) -> Result<Vec<u8>, BindingError> {
        match self {
            Self::Secp256k1(key) => {
                let secret = key.expose_private_key();
                Secp256k1PrivateKey::from_slice(secret.expose_secret_bytes())
                    .map(|key| key.to_public_key().encode_vec())
                    .map_err(|_| BindingError::ProjectionFailed)
            }
            Self::Ed25519Slip10(key) => {
                let secret = key.expose_private_key();
                Ed25519PrivateKey::from_slice(secret.expose_secret_bytes())
                    .map(|key| key.to_public_key().encode_vec())
                    .map_err(|_| BindingError::ProjectionFailed)
            }
            Self::Ed25519CardanoV2 { key, .. } => key
                .to_public_key()
                .map(|key| key.to_bytes().to_vec())
                .map_err(|_| BindingError::ProjectionFailed),
        }
    }

    fn cardano_public(&self) -> Result<HdPublicKey, BindingError> {
        match self {
            Self::Ed25519CardanoV2 { key, depth } => key
                .to_public_key()
                .map(|key| HdPublicKey { key, depth: *depth })
                .map_err(|_| BindingError::ProjectionFailed),
            _ => Err(BindingError::ForbiddenAxis),
        }
    }
}

fn require_standard_seed(seed: &[u8]) -> Result<(), BindingError> {
    if (MIN_HD_SEED_BYTES..=MAX_HD_SEED_BYTES).contains(&seed.len()) {
        Ok(())
    } else {
        Err(BindingError::InvalidSeed)
    }
}

fn parse_path(path: &str) -> Result<DerivationPath, BindingError> {
    DerivationPath::from_path(path).map_err(|_| BindingError::InvalidPath)
}

fn checked_depth(current: u32, axes: usize) -> Result<u32, BindingError> {
    let axes = u32::try_from(axes).map_err(|_| BindingError::DepthOverflow)?;
    current
        .checked_add(axes)
        .filter(|depth| *depth <= MAX_DERIVATION_PATH_AXES as u32)
        .ok_or(BindingError::DepthOverflow)
}

fn require_hardened(path: &DerivationPath) -> Result<(), BindingError> {
    if path
        .axes()
        .iter()
        .all(identus_crypto::path::DerivationAxis::is_hardened)
    {
        Ok(())
    } else {
        Err(BindingError::ForbiddenAxis)
    }
}

struct HdPublicKey {
    key: CardanoV2ExtendedPublicKey,
    depth: u32,
}

impl HdPublicKey {
    fn derive(&self, path: &str) -> Result<Self, BindingError> {
        let path = parse_path(path)?;
        if path
            .axes()
            .iter()
            .any(identus_crypto::path::DerivationAxis::is_hardened)
        {
            return Err(BindingError::ForbiddenAxis);
        }
        let depth = checked_depth(self.depth, path.len())?;
        self.key
            .derive_path(&path)
            .map(|key| Self { key, depth })
            .map_err(|_| BindingError::DerivationFailed)
    }
}

/// Opaque profile-specific hierarchical private-key handle.
#[wasm_bindgen]
pub struct HdPrivateHandle {
    private: Option<HdPrivateKey>,
}

#[wasm_bindgen]
impl HdPrivateHandle {
    /// Import one bounded seed into a named derivation profile.
    #[wasm_bindgen(js_name = fromSeed)]
    pub fn from_seed(profile: &str, seed: &mut [u8]) -> Result<HdPrivateHandle, JsError> {
        let result = HdPrivateKey::from_seed(profile, seed).map(|private| Self {
            private: Some(private),
        });
        seed.fill(0);
        result.map_err(BindingError::into_js)
    }

    /// Return the immutable profile identifier.
    pub fn profile(&self) -> Result<String, JsError> {
        self.private
            .as_ref()
            .map(|private| private.profile().to_owned())
            .ok_or_else(|| BindingError::Disposed.into_js())
    }

    /// Return the current derivation depth.
    pub fn depth(&self) -> Result<u32, JsError> {
        self.private
            .as_ref()
            .map(HdPrivateKey::depth)
            .ok_or_else(|| BindingError::Disposed.into_js())
    }

    /// Derive a new opaque child while retaining this parent.
    pub fn derive(&self, path: &str) -> Result<HdPrivateHandle, JsError> {
        self.private
            .as_ref()
            .ok_or(BindingError::Disposed)
            .and_then(|private| private.derive(path))
            .map(|private| Self {
                private: Some(private),
            })
            .map_err(BindingError::into_js)
    }

    /// Project copied profile-specific public bytes.
    #[wasm_bindgen(js_name = publicKeyBytes)]
    pub fn public_key_bytes(&self) -> Result<Vec<u8>, JsError> {
        self.private
            .as_ref()
            .ok_or(BindingError::Disposed)
            .and_then(HdPrivateKey::public_key_bytes)
            .map_err(BindingError::into_js)
    }

    /// Create an opaque Cardano V2 public-derivation handle.
    #[wasm_bindgen(js_name = cardanoPublicHandle)]
    pub fn cardano_public_handle(&self) -> Result<HdPublicHandle, JsError> {
        self.private
            .as_ref()
            .ok_or(BindingError::Disposed)
            .and_then(HdPrivateKey::cardano_public)
            .map(|public| HdPublicHandle {
                public: Some(public),
            })
            .map_err(BindingError::into_js)
    }

    /// Idempotently destroy private material owned by this handle.
    pub fn dispose(&mut self) {
        self.private = None;
    }
}

/// Opaque Cardano V2 public child-derivation handle.
#[wasm_bindgen]
pub struct HdPublicHandle {
    public: Option<HdPublicKey>,
}

#[wasm_bindgen]
impl HdPublicHandle {
    /// Return the immutable Cardano V2 profile identifier.
    pub fn profile(&self) -> Result<String, JsError> {
        self.public
            .as_ref()
            .map(|_| ED25519_CARDANO_V2_V1.to_owned())
            .ok_or_else(|| BindingError::Disposed.into_js())
    }

    /// Return the current derivation depth.
    pub fn depth(&self) -> Result<u32, JsError> {
        self.public
            .as_ref()
            .map(|public| public.depth)
            .ok_or_else(|| BindingError::Disposed.into_js())
    }

    /// Derive a new all-soft public child while retaining this parent.
    pub fn derive(&self, path: &str) -> Result<HdPublicHandle, JsError> {
        self.public
            .as_ref()
            .ok_or(BindingError::Disposed)
            .and_then(|public| public.derive(path))
            .map(|public| Self {
                public: Some(public),
            })
            .map_err(BindingError::into_js)
    }

    /// Project copied 64-byte public-key-plus-chain-code bytes.
    #[wasm_bindgen(js_name = publicKeyBytes)]
    pub fn public_key_bytes(&self) -> Result<Vec<u8>, JsError> {
        self.public
            .as_ref()
            .map(|public| public.key.to_bytes().to_vec())
            .ok_or_else(|| BindingError::Disposed.into_js())
    }

    /// Idempotently release this public derivation owner.
    pub fn dispose(&mut self) {
        self.public = None;
    }
}

/// Opaque Ed25519 private-key handle.
#[wasm_bindgen]
pub struct Ed25519PrivateHandle {
    private: Option<Ed25519PrivateKey>,
}

#[wasm_bindgen]
impl Ed25519PrivateHandle {
    /// Generate a private key using the reviewed system entropy adapter.
    pub fn generate() -> Result<Ed25519PrivateHandle, JsError> {
        let pair = Ed25519KeyPair::generate(&mut GetrandomSystemRandomAdapter)
            .map_err(|_| BindingError::EntropyUnavailable.into_js())?;
        Ok(Self {
            private: Some(pair.private().clone()),
        })
    }

    /// Import exactly 32 private seed bytes.
    #[wasm_bindgen(js_name = importBytes)]
    pub fn import_bytes(bytes: &[u8]) -> Result<Ed25519PrivateHandle, JsError> {
        let private = Ed25519PrivateKey::from_slice(bytes)
            .map_err(|_| BindingError::InvalidPrivateKey.into_js())?;
        Ok(Self {
            private: Some(private),
        })
    }

    /// Project the exact 32-byte public key.
    #[wasm_bindgen(js_name = publicKeyBytes)]
    pub fn public_key_bytes(&self) -> Result<Vec<u8>, JsError> {
        self.private
            .as_ref()
            .map(|private| private.to_public_key().encode_vec())
            .ok_or_else(|| BindingError::Disposed.into_js())
    }

    /// Sign one bounded message.
    pub fn sign(&self, message: &[u8]) -> Result<Vec<u8>, JsError> {
        let message = bounded_message(message).map_err(BindingError::into_js)?;
        self.private
            .as_ref()
            .map(|private| private.sign(message))
            .ok_or_else(|| BindingError::Disposed.into_js())
    }

    /// Idempotently destroy private material owned by this handle.
    pub fn dispose(&mut self) {
        self.private = None;
    }
}

/// Strictly verify one bounded Ed25519 signature.
#[wasm_bindgen(js_name = verifyEd25519)]
pub fn verify_ed25519(
    public_key: &[u8],
    message: &[u8],
    signature: &[u8],
) -> Result<bool, JsError> {
    let message = bounded_message(message).map_err(BindingError::into_js)?;
    if signature.len() != ED25519_SIGNATURE_BYTES {
        return Err(BindingError::InvalidSignature.into_js());
    }
    let public_key = ed25519_public(public_key).map_err(BindingError::into_js)?;
    Ok(public_key.verify(message, signature))
}

/// Opaque X25519 private-key handle.
#[wasm_bindgen]
pub struct X25519PrivateHandle {
    private: Option<X25519PrivateKey>,
}

#[wasm_bindgen]
impl X25519PrivateHandle {
    /// Generate a private key using the reviewed system entropy adapter.
    pub fn generate() -> Result<X25519PrivateHandle, JsError> {
        let pair = X25519KeyPair::generate(&mut GetrandomSystemRandomAdapter)
            .map_err(|_| BindingError::EntropyUnavailable.into_js())?;
        Ok(Self {
            private: Some(pair.private().clone()),
        })
    }

    /// Import exactly 32 private scalar bytes.
    #[wasm_bindgen(js_name = importBytes)]
    pub fn import_bytes(bytes: &[u8]) -> Result<X25519PrivateHandle, JsError> {
        let private = X25519PrivateKey::from_slice(bytes)
            .map_err(|_| BindingError::InvalidPrivateKey.into_js())?;
        Ok(Self {
            private: Some(private),
        })
    }

    /// Project the exact 32-byte public key.
    #[wasm_bindgen(js_name = publicKeyBytes)]
    pub fn public_key_bytes(&self) -> Result<Vec<u8>, JsError> {
        self.private
            .as_ref()
            .map(|private| private.to_public_key().encode_vec())
            .ok_or_else(|| BindingError::Disposed.into_js())
    }

    /// Derive a contributory 32-byte shared secret.
    #[wasm_bindgen(js_name = deriveSharedSecret)]
    pub fn derive_shared_secret(&self, peer_public_key: &[u8]) -> Result<Vec<u8>, JsError> {
        let peer = x25519_public(peer_public_key).map_err(BindingError::into_js)?;
        if !peer.is_contributory() {
            return Err(BindingError::NonContributoryPeer.into_js());
        }
        self.private
            .as_ref()
            .map(|private| private.derive_shared(&peer))
            .ok_or_else(|| BindingError::Disposed.into_js())
    }

    /// Idempotently destroy private material owned by this handle.
    pub fn dispose(&mut self) {
        self.private = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn decode(value: &str) -> Vec<u8> {
        hex::decode(value).expect("fixture hex")
    }

    #[test]
    fn rfc_8032_vector_is_strictly_compatible() {
        let private = Ed25519PrivateKey::from_slice(&decode(
            "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60",
        ))
        .expect("private key");
        assert_eq!(
            private.to_public_key().encode_vec(),
            decode("d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a")
        );
        let signature = private.sign(&[]);
        assert_eq!(
            signature,
            decode(
                "e5564300c360ac729086e2cc806e828a84877f1eb8e5d974d873e065224901555fb8821590a33bacc61e39701cf9b46bd25bf5f0595bbe24655141438e7a100b"
            )
        );
        assert!(private.to_public_key().verify(&[], &signature));
    }

    #[test]
    fn rfc_7748_vector_is_contributory_and_compatible() {
        let private = X25519PrivateKey::from_slice(&decode(
            "77076d0a7318a57d3c16c17251b26645df4c2f87ebc0992ab177fba51db92c2a",
        ))
        .expect("private key");
        let peer = x25519_public(&decode(
            "de9edb7d7b7dc1b4d35b61c2ece435373f8343c85b78674dadfc7e146f882b4f",
        ))
        .expect("peer public key");
        assert_eq!(
            private.to_public_key().encode_vec(),
            decode("8520f0098930a754748b7ddcb43ef75a0dbf3a0d26381af4eba4a98eaa9b4e6a")
        );
        assert_eq!(
            private.derive_shared(&peer),
            decode("4a5d9d5ba4ce2de1728e3bf480350f25e07e21c947d19e3376f09b3c1e161742")
        );
    }

    #[test]
    fn strict_boundaries_and_low_order_peer_fail_closed() {
        assert!(bounded_message(&vec![0; MAX_KEY_OPERATION_MESSAGE_BYTES]).is_ok());
        assert_eq!(
            bounded_message(&vec![0; MAX_KEY_OPERATION_MESSAGE_BYTES + 1]),
            Err(BindingError::MessageTooLarge)
        );
        assert_eq!(
            ed25519_public(&[0; ED25519_KEY_BYTES]),
            Err(BindingError::InvalidPublicKey)
        );
        let low_order = x25519_public(&[0; X25519_KEY_BYTES]).expect("canonical key");
        assert!(!low_order.is_contributory());
    }

    #[test]
    fn malformed_lengths_and_signature_mutation_fail_closed() {
        assert!(Ed25519PrivateKey::from_slice(&[0; ED25519_KEY_BYTES - 1]).is_err());
        assert!(X25519PrivateKey::from_slice(&[0; X25519_KEY_BYTES + 1]).is_err());
        assert_eq!(
            ed25519_public(&[0; ED25519_KEY_BYTES - 1]),
            Err(BindingError::InvalidPublicKey)
        );
        assert_eq!(
            x25519_public(&[0; X25519_KEY_BYTES + 1]),
            Err(BindingError::InvalidPublicKey)
        );

        let private = Ed25519PrivateKey::from_slice(&decode(
            "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60",
        ))
        .expect("private key");
        let mut signature = private.sign(b"bounded");
        signature[0] ^= 1;
        assert!(!private.to_public_key().verify(b"bounded", &signature));
    }

    #[test]
    fn hd_profiles_match_published_public_projections() {
        let seed = decode("000102030405060708090a0b0c0d0e0f");
        let secp = HdPrivateKey::from_seed(SECP256K1_BIP32_HARDENED_V1, &seed).unwrap();
        assert_eq!(
            secp.public_key_bytes().unwrap(),
            decode("0339a36013301597daef41fbe593a02cc513d0b55527ec2df1050e2e8ff49c85c2")
        );
        assert_eq!(
            secp.derive("m/0'").unwrap().public_key_bytes().unwrap(),
            decode("035a784662a4a20a65bf6aab9ae98a6c068a81c52e4b032c0fb5400c706cfccc56")
        );

        let slip10 = HdPrivateKey::from_seed(ED25519_SLIP10_HARDENED_V1, &seed).unwrap();
        assert_eq!(
            slip10.public_key_bytes().unwrap(),
            decode("a4b2856bfec510abab89753fac1ac0e1112364e7d250545963f135f2a33188ed")
        );
        assert_eq!(
            slip10.derive("m/0'").unwrap().public_key_bytes().unwrap(),
            decode("8c8a13df77a28f3445213a0f432fde644acaa215fc72dcdf300d5efaa85d350c")
        );
    }

    #[test]
    fn hd_profile_rules_and_resource_boundaries_fail_closed() {
        let seed = [0x42; 16];
        assert!(matches!(
            HdPrivateKey::from_seed("ambiguous", &seed),
            Err(BindingError::InvalidProfile)
        ));
        assert!(matches!(
            HdPrivateKey::from_seed(SECP256K1_BIP32_HARDENED_V1, &[0; 15]),
            Err(BindingError::InvalidSeed)
        ));
        let secp = HdPrivateKey::from_seed(SECP256K1_BIP32_HARDENED_V1, &seed).unwrap();
        assert!(matches!(
            secp.derive("m/0"),
            Err(BindingError::ForbiddenAxis)
        ));
        assert!(matches!(
            secp.derive("M/0'"),
            Err(BindingError::InvalidPath)
        ));

        let cardano = HdPrivateKey::from_seed(ED25519_CARDANO_V2_V1, &[7; 64]).unwrap();
        let public = cardano.cardano_public().unwrap();
        assert!(matches!(
            public.derive("m/0'"),
            Err(BindingError::ForbiddenAxis)
        ));
        assert_eq!(checked_depth(254, 1), Ok(255));
        assert_eq!(checked_depth(255, 1), Err(BindingError::DepthOverflow));
    }

    #[test]
    fn cardano_soft_public_derivation_matches_private_derivation() {
        let cardano = HdPrivateKey::from_seed(ED25519_CARDANO_V2_V1, &[7; 64]).unwrap();
        let private_child = cardano.derive("m/0").unwrap().public_key_bytes().unwrap();
        let public_child = cardano
            .cardano_public()
            .unwrap()
            .derive("m/0")
            .unwrap()
            .key
            .to_bytes();
        assert_eq!(private_child, public_child);
    }
}
