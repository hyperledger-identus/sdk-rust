//! Bounded, effect-free peer DID method profiles.
//!
//! The current numalgo-4 profile and the legacy numalgo-2 compatibility
//! profile are intentionally separate modules. This crate never generates or
//! stores private keys and never performs network or persistence work.

use core::fmt;

use identus_core::Component;

pub mod numalgo2_compat;
pub mod numalgo4;

/// Metadata for the `identus-did-peer` crate.
pub const COMPONENT: Component = Component {
    name: "identus-did-peer",
    summary: "Bounded current and compatibility peer DID method profiles.",
};

/// Stable peer DID error categories.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// The selected API received another peer DID profile.
    WrongProfile,
    /// The DID selects an unsupported historical or unknown algorithm.
    UnsupportedAlgorithm,
    /// The method-specific identifier or encoded material is malformed.
    MalformedEncoding,
    /// The encoded material uses an unsupported multicodec.
    UnsupportedCodec,
    /// A numalgo-2 purpose code is unsupported or incompatible with its key.
    UnsupportedPurpose,
    /// A numalgo-4 input document violates the pinned profile.
    InvalidInputDocument,
    /// A numalgo-4 long form does not match its short-form hash.
    HashMismatch,
    /// Resolution of a short form requires matching caller-supplied long form.
    LongFormRequired,
    /// A public finite resource boundary was exceeded.
    ResourceLimit,
    /// Projection into the generic DID document model failed.
    Projection,
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::WrongProfile => "peer DID profile mismatch",
            Self::UnsupportedAlgorithm => "unsupported peer DID algorithm",
            Self::MalformedEncoding => "malformed peer DID encoding",
            Self::UnsupportedCodec => "unsupported peer DID codec",
            Self::UnsupportedPurpose => "unsupported peer DID key purpose",
            Self::InvalidInputDocument => "invalid peer DID input document",
            Self::HashMismatch => "peer DID hash mismatch",
            Self::LongFormRequired => "matching peer DID long form required",
            Self::ResourceLimit => "peer DID resource limit exceeded",
            Self::Projection => "peer DID document projection failed",
        })
    }
}

impl std::error::Error for Error {}
