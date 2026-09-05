//! Bounded JSON Web Signature Compact and signature-capability foundations.
//!
//! Parsing returns [`UnverifiedCompactJws`]. A caller-selected bounded
//! [`SignatureSuiteRegistry`] can produce [`VerifiedCompactJws`] after exact
//! algorithm/key binding and cryptographic verification. Its narrow
//! OpenID4VCI profile constructs holder proofs and separates issuer parsing,
//! key-bound signature verification and policy authorization. DID,
//! certificate, clock and replay behavior enters through caller-owned ports;
//! the crate does not decide trust or hold private key material.

#![forbid(unsafe_code)]

mod compact;
mod error;
mod header;
mod limits;
mod oid4vci;
mod oid4vci_verifier;
mod signature;

pub use compact::{JwsSigningInput, UnverifiedCompactJws};
pub use error::{CAPABILITY, JoseError, error_code};
pub use header::{JwsKeyReference, MAX_X5C_CERTIFICATES, ProtectedHeader};
pub use limits::JwsLimits;
pub use oid4vci::{
    DEFAULT_MAX_PROOF_CLAIM_STRING_BYTES, OID4VCI_PROOF_JWT_TYPE, Oid4vciProofJwt,
    Oid4vciProofJwtBuilder, Oid4vciProofJwtClaims, Oid4vciProofJwtClient, Oid4vciProofJwtLimits,
    Oid4vciProofSigningInput,
};
pub use oid4vci_verifier::{
    Oid4vciAuthorizedProofJwt, Oid4vciParsedProofJwt, Oid4vciProofJwtNonce, Oid4vciProofJwtPolicy,
    Oid4vciProofJwtVerifier, Oid4vciProofReplayFailure, Oid4vciProofReplayFuture,
    Oid4vciProofReplayGuard, Oid4vciProofReplayInput, Oid4vciVerifiedProofJwt,
    Oid4vciX5cKeyFailure, Oid4vciX5cKeyFuture, Oid4vciX5cKeyProvider,
};
pub use signature::{
    Ed25519SignatureSuite, Ed25519Signer, Es256SignatureSuite, Es256Signer, JwsAlgorithm,
    JwsSignatureSuite, JwsSigner, JwsVerificationKey, LegacyEdDsaSignatureSuite,
    SignatureSuiteRegistry, SignerFailure, VerifiedCompactJws,
};

use identus_core::Component;

/// Metadata for the `identus-jose` crate.
pub const COMPONENT: Component = Component {
    name: "identus-jose",
    summary: "Bounded JWS Compact, signatures, and narrow proof profiles.",
};
