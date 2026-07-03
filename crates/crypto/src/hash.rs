//! SHA-2 digest primitives, backed by pure `sha2` (wasm-safe; no `ring`).

use sha2::{Digest, Sha256, Sha512};

use crate::error::Error;

/// A 32-byte SHA-256 digest.
#[must_use]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Sha256Digest([u8; 32]);

/// A 64-byte SHA-512 digest.
#[must_use]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Sha512Digest([u8; 64]);

impl Sha256Digest {
    /// Wrap raw bytes as a digest (no hashing; validates the length only).
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let arr: [u8; 32] = bytes.try_into().map_err(|_| Error::InvalidKeySize {
            expected: 32,
            actual: bytes.len(),
            key_type: std::any::type_name::<Self>(),
        })?;
        Ok(Self(arr))
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn as_array(&self) -> &[u8; 32] {
        &self.0
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

impl Sha512Digest {
    /// Wrap raw bytes as a digest (no hashing; validates the length only).
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let arr: [u8; 64] = bytes.try_into().map_err(|_| Error::InvalidKeySize {
            expected: 64,
            actual: bytes.len(),
            key_type: std::any::type_name::<Self>(),
        })?;
        Ok(Self(arr))
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn as_array(&self) -> &[u8; 64] {
        &self.0
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

/// Compute SHA-256 of `bytes`.
pub fn sha256<B: AsRef<[u8]>>(bytes: B) -> Sha256Digest {
    let mut hasher = Sha256::new();
    hasher.update(bytes.as_ref());
    let out = hasher.finalize();
    let arr: [u8; 32] = (&*out).try_into().expect("SHA-256 output is 32 bytes");
    Sha256Digest(arr)
}

/// Compute SHA-512 of `bytes`.
pub fn sha512<B: AsRef<[u8]>>(bytes: B) -> Sha512Digest {
    let mut hasher = Sha512::new();
    hasher.update(bytes.as_ref());
    let out = hasher.finalize();
    let arr: [u8; 64] = (&*out).try_into().expect("SHA-512 output is 64 bytes");
    Sha512Digest(arr)
}

/// Compute HMAC-SHA512 of `input` under `key`.
#[cfg(feature = "derivation")]
#[must_use]
pub fn hmac_sha512(key: &[u8], input: &[u8]) -> [u8; 64] {
    use hmac::Mac;
    let mut mac = <hmac::Hmac<Sha512> as hmac::Mac>::new_from_slice(key)
        .expect("HMAC accepts any key length");
    mac.update(input);
    mac.finalize().into_bytes().into()
}
