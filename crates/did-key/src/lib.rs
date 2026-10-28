//! Bounded, effect-free `did:key` creation and local resolution.
//!
//! The first profile accepts canonical base58-btc fingerprints for direct
//! Ed25519, X25519, P-256, and secp256k1 public keys. It never accepts private
//! material, derives another key, or performs I/O.

use std::{collections::BTreeMap, fmt, sync::Arc};

use identus_core::Component;
use identus_crypto::{
    Ed25519PublicKey, EncodeVec, P256PublicKey, Secp256k1PublicKey, X25519PublicKey,
};
use identus_did::{
    Did, DidDocument, DidDocumentMetadata, DidMethod, DidMethodBinding, DidResolutionError,
    DidResolutionErrorKind, DidResolutionFuture, DidResolutionMetadata, DidResolutionResult,
    DidResolver, MediaType, ResolutionOptions, Uri, VerificationMethod, VerificationRelationship,
};
use serde_json::Value;

/// Maximum accepted method-specific identifier length in bytes.
pub const MAX_METHOD_SPECIFIC_ID_BYTES: usize = 128;
/// Maximum accepted decoded multicodec and public-key bytes.
pub const MAX_DECODED_KEY_BYTES: usize = 67;

const DID_PREFIX: &str = "did:key:";
const DID_JSON: &str = "application/did+json";
const DID_LD_JSON: &str = "application/did+ld+json";
const ED25519_PUB: u64 = 0xed;
const X25519_PUB: u64 = 0xec;
const SECP256K1_PUB: u64 = 0xe7;
const P256_PUB: u64 = 0x1200;

/// Metadata for the `identus-did-key` crate.
pub const COMPONENT: Component = Component {
    name: "identus-did-key",
    summary: "Bounded effect-free did:key creation and local resolution.",
};

/// Stable, content-free `did:key` error categories.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// The input is not a `did:key` DID.
    WrongMethod,
    /// The identifier or multibase value is malformed or non-canonical.
    MalformedEncoding,
    /// The multicodec is outside the selected profile.
    UnsupportedCodec,
    /// Public-key bytes have the wrong length or do not encode a valid point.
    InvalidPublicKey,
    /// A public finite input boundary was exceeded.
    ResourceLimit,
    /// Projection into the generic DID document model failed.
    Projection,
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::WrongMethod => "did:key method mismatch",
            Self::MalformedEncoding => "malformed did:key encoding",
            Self::UnsupportedCodec => "unsupported did:key codec",
            Self::InvalidPublicKey => "invalid did:key public key",
            Self::ResourceLimit => "did:key resource limit exceeded",
            Self::Projection => "did:key document projection failed",
        })
    }
}

impl std::error::Error for Error {}

/// One supported caller-owned public key.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum PublicKey {
    /// Ed25519 verification key.
    Ed25519(Ed25519PublicKey),
    /// X25519 key-agreement key.
    X25519(X25519PublicKey),
    /// P-256 verification key.
    P256(P256PublicKey),
    /// secp256k1 verification key.
    Secp256k1(Secp256k1PublicKey),
}

impl From<Ed25519PublicKey> for PublicKey {
    fn from(value: Ed25519PublicKey) -> Self {
        Self::Ed25519(value)
    }
}

impl From<X25519PublicKey> for PublicKey {
    fn from(value: X25519PublicKey) -> Self {
        Self::X25519(value)
    }
}

impl From<P256PublicKey> for PublicKey {
    fn from(value: P256PublicKey) -> Self {
        Self::P256(value)
    }
}

impl From<Secp256k1PublicKey> for PublicKey {
    fn from(value: Secp256k1PublicKey) -> Self {
        Self::Secp256k1(value)
    }
}

impl PublicKey {
    fn codec(&self) -> u64 {
        match self {
            Self::Ed25519(_) => ED25519_PUB,
            Self::X25519(_) => X25519_PUB,
            Self::P256(_) => P256_PUB,
            Self::Secp256k1(_) => SECP256K1_PUB,
        }
    }

    fn canonical_bytes(&self) -> Vec<u8> {
        match self {
            Self::Ed25519(key) => key.encode_vec(),
            Self::X25519(key) => key.encode_vec(),
            Self::P256(key) => key.encode_compressed().to_vec(),
            Self::Secp256k1(key) => key.encode_compressed().to_vec(),
        }
    }

    fn is_agreement(&self) -> bool {
        matches!(self, Self::X25519(_))
    }
}

/// A validated canonical DID and its direct public key.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DidKey {
    did: Did,
    fingerprint: String,
    public_key: PublicKey,
}

impl DidKey {
    /// Create a canonical DID from a supported public key.
    pub fn create(public_key: impl Into<PublicKey>) -> Result<Self, Error> {
        let public_key = public_key.into();
        let fingerprint = encode_fingerprint(&public_key);
        let did =
            Did::parse(&format!("{DID_PREFIX}{fingerprint}")).map_err(|_| Error::Projection)?;
        Ok(Self {
            did,
            fingerprint,
            public_key,
        })
    }

    /// Parse and validate a canonical `did:key` string.
    pub fn parse(input: &str) -> Result<Self, Error> {
        if let Some(fingerprint) = input.strip_prefix(DID_PREFIX)
            && fingerprint.len() > MAX_METHOD_SPECIFIC_ID_BYTES
        {
            return Err(Error::ResourceLimit);
        }
        let did = Did::parse(input).map_err(|_| Error::MalformedEncoding)?;
        Self::from_did(did)
    }

    /// Parse and validate an already syntax-checked DID.
    pub fn from_did(did: Did) -> Result<Self, Error> {
        if did.method() != "key" {
            return Err(Error::WrongMethod);
        }
        let fingerprint = did.method_specific_id();
        if fingerprint.len() > MAX_METHOD_SPECIFIC_ID_BYTES {
            return Err(Error::ResourceLimit);
        }
        let public_key = decode_fingerprint(fingerprint)?;
        if encode_fingerprint(&public_key) != fingerprint {
            return Err(Error::MalformedEncoding);
        }
        Ok(Self {
            fingerprint: fingerprint.to_owned(),
            did,
            public_key,
        })
    }

    /// Borrow the canonical DID.
    #[must_use]
    pub const fn did(&self) -> &Did {
        &self.did
    }

    /// Borrow the canonical multibase fingerprint.
    #[must_use]
    pub fn fingerprint(&self) -> &str {
        &self.fingerprint
    }

    /// Borrow the validated public key.
    #[must_use]
    pub const fn public_key(&self) -> &PublicKey {
        &self.public_key
    }

    /// Resolve this DID to its deterministic DID document.
    pub fn resolve(&self) -> Result<DidDocument, Error> {
        let method_uri = Uri::parse(&format!("{}#{}", self.did, self.fingerprint))
            .map_err(|_| Error::Projection)?;
        let mut properties = BTreeMap::new();
        properties.insert(
            "publicKeyMultibase".to_owned(),
            Value::String(self.fingerprint.clone()),
        );
        let method = VerificationMethod::new(
            method_uri.clone(),
            "Multikey".to_owned(),
            self.did.clone(),
            properties,
        )
        .map_err(|_| Error::Projection)?;
        let relationship = || VerificationRelationship::Reference(method_uri.clone());
        let builder = DidDocument::builder(self.did.clone()).verification_methods(vec![method]);
        let builder = if self.public_key.is_agreement() {
            builder.key_agreement(vec![relationship()])
        } else {
            builder
                .authentication(vec![relationship()])
                .assertion_method(vec![relationship()])
                .capability_invocation(vec![relationship()])
                .capability_delegation(vec![relationship()])
        };
        builder.build().map_err(|_| Error::Projection)
    }
}

/// Deterministic local resolver for explicit registry composition.
#[derive(Clone, Copy, Debug, Default)]
pub struct DidKeyResolver;

impl DidKeyResolver {
    /// Consume this resolver into an explicit `key` method binding.
    #[must_use]
    pub fn into_method_binding(self) -> DidMethodBinding {
        DidMethodBinding::new(
            DidMethod::parse("key").expect("fixed method name is valid"),
            Arc::new(self),
        )
    }
}

impl DidResolver for DidKeyResolver {
    fn resolve<'a>(
        &'a self,
        did: &'a Did,
        options: &'a ResolutionOptions,
    ) -> DidResolutionFuture<'a> {
        Box::pin(async move { resolve_result(did, options) })
    }
}

fn encode_fingerprint(public_key: &PublicKey) -> String {
    let mut bytes = encode_uvarint(public_key.codec());
    bytes.extend(public_key.canonical_bytes());
    format!("z{}", bs58::encode(bytes).into_string())
}

fn decode_fingerprint(fingerprint: &str) -> Result<PublicKey, Error> {
    let payload = fingerprint
        .strip_prefix('z')
        .ok_or(Error::MalformedEncoding)?;
    if payload.is_empty() {
        return Err(Error::MalformedEncoding);
    }
    if payload.len() > MAX_METHOD_SPECIFIC_ID_BYTES - 1 {
        return Err(Error::ResourceLimit);
    }
    let mut decoded = [0u8; MAX_DECODED_KEY_BYTES];
    let decoded_len =
        bs58::decode(payload)
            .onto(&mut decoded[..])
            .map_err(|error| match error {
                bs58::decode::Error::BufferTooSmall => Error::ResourceLimit,
                _ => Error::MalformedEncoding,
            })?;
    let bytes = &decoded[..decoded_len];
    let (codec, prefix_len) = decode_uvarint(bytes)?;
    let key = &bytes[prefix_len..];
    match codec {
        ED25519_PUB if key.len() == 32 => Ed25519PublicKey::from_slice(key)
            .map(PublicKey::Ed25519)
            .map_err(|_| Error::InvalidPublicKey),
        X25519_PUB if key.len() == 32 => X25519PublicKey::from_slice(key)
            .map(PublicKey::X25519)
            .map_err(|_| Error::InvalidPublicKey),
        P256_PUB if key.len() == 33 => P256PublicKey::from_slice(key)
            .map(PublicKey::P256)
            .map_err(|_| Error::InvalidPublicKey),
        SECP256K1_PUB if key.len() == 33 => Secp256k1PublicKey::from_slice(key)
            .map(PublicKey::Secp256k1)
            .map_err(|_| Error::InvalidPublicKey),
        ED25519_PUB | X25519_PUB | P256_PUB | SECP256K1_PUB => Err(Error::InvalidPublicKey),
        _ => Err(Error::UnsupportedCodec),
    }
}

fn encode_uvarint(mut value: u64) -> Vec<u8> {
    let mut encoded = Vec::with_capacity(2);
    loop {
        let mut byte = (value & 0x7f) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        encoded.push(byte);
        if value == 0 {
            return encoded;
        }
    }
}

fn decode_uvarint(bytes: &[u8]) -> Result<(u64, usize), Error> {
    let mut value = 0u64;
    for (index, byte) in bytes.iter().copied().take(10).enumerate() {
        if index == 9 && byte > 1 {
            return Err(Error::MalformedEncoding);
        }
        let shift = index * 7;
        value |= u64::from(byte & 0x7f)
            .checked_shl(u32::try_from(shift).map_err(|_| Error::MalformedEncoding)?)
            .ok_or(Error::MalformedEncoding)?;
        if byte & 0x80 == 0 {
            let length = index + 1;
            if encode_uvarint(value).as_slice() != &bytes[..length] {
                return Err(Error::MalformedEncoding);
            }
            return Ok((value, length));
        }
    }
    Err(Error::MalformedEncoding)
}

fn resolve_result(did: &Did, options: &ResolutionOptions) -> DidResolutionResult {
    if options.version_id().is_some()
        || options.version_time().is_some()
        || !options.extensions().is_empty()
    {
        return resolution_failure(DidResolutionErrorKind::FeatureNotSupported);
    }
    let selected = options.accept().map_or(DID_JSON, MediaType::as_str);
    if !matches!(selected, DID_JSON | DID_LD_JSON) {
        return resolution_failure(DidResolutionErrorKind::RepresentationNotSupported);
    }
    let parsed = match DidKey::from_did(did.clone()) {
        Ok(value) => value,
        Err(Error::WrongMethod) => {
            return resolution_failure(DidResolutionErrorKind::MethodNotSupported);
        }
        Err(_) => return resolution_failure(DidResolutionErrorKind::InvalidDid),
    };
    let document = match parsed.resolve() {
        Ok(document) => document,
        Err(_) => return resolution_failure(DidResolutionErrorKind::InvalidDidDocument),
    };
    let metadata = match MediaType::parse(selected)
        .and_then(|media_type| DidResolutionMetadata::new(Some(media_type), None, BTreeMap::new()))
    {
        Ok(metadata) => metadata,
        Err(_) => return resolution_failure(DidResolutionErrorKind::InternalError),
    };
    DidResolutionResult::success(metadata, document, DidDocumentMetadata::empty())
        .unwrap_or_else(|_| resolution_failure(DidResolutionErrorKind::InvalidDidDocument))
}

fn resolution_failure(kind: DidResolutionErrorKind) -> DidResolutionResult {
    let metadata = DidResolutionMetadata::new(
        None,
        Some(DidResolutionError::standard(kind)),
        BTreeMap::new(),
    )
    .expect("standard did:key resolution error is valid");
    DidResolutionResult::failure(metadata).expect("standard did:key resolution failure is valid")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uvarint_rejects_overlong_encodings() {
        assert_eq!(
            decode_uvarint(&[0xed, 0x81, 0x00]),
            Err(Error::MalformedEncoding)
        );
    }

    #[test]
    fn errors_are_content_free() {
        assert_eq!(
            Error::InvalidPublicKey.to_string(),
            "invalid did:key public key"
        );
        assert!(!Error::ResourceLimit.to_string().contains("did:key:z"));
    }
}
