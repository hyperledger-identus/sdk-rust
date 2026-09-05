//! Bounded JSON Web Signature Compact and signature-capability foundations.
//!
//! Parsing returns [`UnverifiedCompactJws`]. A caller-selected bounded
//! [`SignatureSuiteRegistry`] can produce [`VerifiedCompactJws`] after exact
//! algorithm/key binding and cryptographic verification. The crate does not
//! interpret JWT claims, resolve or authorize DID keys, decide trust, or hold
//! private key material.

#![forbid(unsafe_code)]

mod compact;
mod error;
mod header;
mod limits;
mod signature;

pub use compact::{JwsSigningInput, UnverifiedCompactJws};
pub use error::{CAPABILITY, JoseError, error_code};
pub use header::ProtectedHeader;
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
    summary: "Bounded JWS Compact and signature capabilities.",
};
