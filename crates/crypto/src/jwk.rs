//! JWK (JSON Web Key) struct and the `EncodeJwk` trait (ported from neoprism).

use crate::base64::Base64UrlStrNoPad;

/// A minimal JWK representation (`kty`, `crv`, `x`, `y`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Jwk {
    /// Key type, e.g. `"OKP"` or `"EC"`.
    pub kty: String,
    /// Curve name, e.g. `"Ed25519"`, `"X25519"`, `"secp256k1"`, `"P-256"`.
    pub crv: String,
    /// The `x` coordinate (base64url-no-pad), if present.
    pub x: Option<Base64UrlStrNoPad>,
    /// The `y` coordinate (base64url-no-pad), if present.
    pub y: Option<Base64UrlStrNoPad>,
}

/// Encode a public key as a [`Jwk`].
pub trait EncodeJwk {
    /// Encode this public key as a [`Jwk`].
    fn encode_jwk(&self) -> Jwk;
}
