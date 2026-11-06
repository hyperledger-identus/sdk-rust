use identus_crypto::sha256;

/// Object-safe disclosure digest capability for deterministic fault and oracle tests.
pub trait DisclosureHasher: Send + Sync {
    /// Return the SHA-256-profile digest of exact encoded disclosure bytes.
    fn hash(&self, bytes: &[u8]) -> [u8; 32];
}

/// SHA-256 implementation delegated to the existing Identus crypto primitive.
#[derive(Debug, Default, Clone, Copy)]
pub struct Sha256DisclosureHasher;

impl DisclosureHasher for Sha256DisclosureHasher {
    fn hash(&self, bytes: &[u8]) -> [u8; 32] {
        *sha256(bytes).as_array()
    }
}
