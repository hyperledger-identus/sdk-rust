//! Narrow browser-WASM bindings for selected key operations.
//!
//! Private material remains inside opaque Rust-owned handles. This crate does
//! not provide custody, persistence, private export, derivation, transport, or
//! protocol policy.

use identus_adapters_entropy::GetrandomSystemRandomAdapter;
use identus_crypto::{
    Ed25519KeyPair, Ed25519PrivateKey, Ed25519PublicKey, EncodeVec, Verifiable, X25519KeyPair,
    X25519PrivateKey, X25519PublicKey,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BindingError {
    Disposed,
    InvalidPrivateKey,
    InvalidPublicKey,
    InvalidSignature,
    MessageTooLarge,
    NonContributoryPeer,
    EntropyUnavailable,
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
}
