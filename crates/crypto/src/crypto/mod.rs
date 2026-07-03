//! Curve modules (feature-gated, mirroring neoprism's gating).

#[cfg(feature = "ed25519")]
pub mod ed25519;
#[cfg(feature = "secp256k1")]
pub mod secp256k1;
#[cfg(feature = "secp256r1")]
pub mod secp256r1;
#[cfg(feature = "x25519")]
pub mod x25519;
