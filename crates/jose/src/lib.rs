//! Bounded JSON Web Signature Compact and signature-capability foundations.
//!
//! Parsing returns [`UnverifiedCompactJws`]. A caller-selected bounded
//! [`SignatureSuiteRegistry`] can produce [`VerifiedCompactJws`] after exact
//! algorithm/key binding and cryptographic verification. Protocol claims,
//! identity resolution, trust, time, and replay policy belong to higher-level
//! crates. This crate holds no private key material and selects no trust policy.

#![forbid(unsafe_code)]

mod compact;
mod error;
mod error_contract;
mod header;
mod limits;
mod signature;

pub use compact::{JwsSigningInput, UnverifiedCompactJws};
pub use error::{CAPABILITY, JoseError, error_code};
pub use header::{
    JwsKeyId, JwsKeyReference, JwsX5c, MAX_TRUST_CHAIN_ENTRIES, MAX_X5C_CERTIFICATES,
    ProtectedHeader,
};
pub use limits::JwsLimits;
pub use signature::{
    Ed25519SignatureSuite, Ed25519Signer, Es256SignatureSuite, Es256Signer, JwsAlgorithm,
    JwsSignatureSuite, JwsSigner, JwsVerificationKey, LegacyEdDsaSignatureSuite,
    SignatureSuiteRegistry, SignerFailure, VerifiedCompactJws,
};

use identus_core::Component;

/// Metadata for the `identus-jose` crate.
pub const COMPONENT: Component = Component {
    name: "identus-jose",
    summary: "Bounded protocol-neutral JWS Compact and signature capabilities.",
};
