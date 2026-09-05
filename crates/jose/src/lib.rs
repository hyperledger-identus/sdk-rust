//! Bounded JSON Web Signature Compact Serialization foundations.
//!
//! This crate owns strict wire parsing and staged encoding only. Parsing
//! returns [`UnverifiedCompactJws`]: it does not verify signatures, interpret
//! JWT claims, resolve keys, choose algorithms, or hold private key material.

#![forbid(unsafe_code)]

mod compact;
mod error;
mod header;
mod limits;

pub use compact::{JwsSigningInput, UnverifiedCompactJws};
pub use error::{CAPABILITY, JoseError, error_code};
pub use header::ProtectedHeader;
pub use limits::JwsLimits;

use identus_core::Component;

/// Metadata for the `identus-jose` crate.
pub const COMPONENT: Component = Component {
    name: "identus-jose",
    summary: "Bounded JWS Compact wire parsing and staged encoding.",
};
