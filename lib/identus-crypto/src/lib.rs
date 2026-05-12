pub mod curve;
pub mod schnorr;

/// Re-exported so callers of `schnorr::sign()` can satisfy the `CryptoRng` bound.
pub use rand;
