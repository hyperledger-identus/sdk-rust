use zeroize::{Zeroize, ZeroizeOnDrop};

const SECRET_SIZE: usize = 32;

/// One explicitly exposed copy of HD-key secret material.
///
/// The owned bytes are erased on drop and omitted from [`Debug`](std::fmt::Debug).
/// Raw bytes are available only through [`Self::expose_secret_bytes`]. Any
/// further copy made from that borrow is caller-owned and outside this value's
/// erasure boundary.
#[must_use = "dropping the exposure value immediately erases its owned secret copy"]
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct HdKeySecretBytes([u8; SECRET_SIZE]);

impl HdKeySecretBytes {
    pub(super) const fn new(bytes: [u8; SECRET_SIZE]) -> Self {
        Self(bytes)
    }

    /// Borrow the raw secret bytes.
    ///
    /// The borrow cannot outlive this zeroizing owner. A caller that copies the
    /// array must protect and erase that independent copy.
    #[must_use]
    pub const fn expose_secret_bytes(&self) -> &[u8; SECRET_SIZE] {
        &self.0
    }
}

impl std::fmt::Debug for HdKeySecretBytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HdKeySecretBytes").finish_non_exhaustive()
    }
}
