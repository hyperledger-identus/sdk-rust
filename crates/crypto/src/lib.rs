//! Cryptography and key-management boundaries.
//!
//! This crate will own cryptographic key-management parity: `Ed25519`, `X25519`, `Secp256k1`,
//! mnemonic restoration, `JWK`, `JOSE`/`COSE` integration, signer ports,
//! and hardware/`KMS` adapter traits.

/// Component metadata.
pub const COMPONENT: identus_core::Component = identus_core::Component {
    name: "identus-crypto",
    summary: "Cryptography, keys, and signer abstraction.",
};
