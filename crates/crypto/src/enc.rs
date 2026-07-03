//! Encoding trait polymorphism over the crypto key types (ported from neoprism).

/// Encode a key as a variable-length byte vector.
pub trait EncodeVec {
    /// Canonical byte encoding of the key as an owned vector.
    fn encode_vec(&self) -> Vec<u8>;
}

/// Encode a key as a fixed-length byte array of length `N`.
pub trait EncodeArray<const N: usize> {
    /// Canonical byte encoding of the key as a fixed-size array.
    fn encode_array(&self) -> [u8; N];
}

/// Verify a signature against a message.
pub trait Verifiable {
    /// Returns `true` if `signature` is valid for `message` under this key.
    fn verify(&self, message: &[u8], signature: &[u8]) -> bool;
}
