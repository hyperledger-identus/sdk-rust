//! Validated canonical and long-form PRISM DID values.

use std::{fmt, str::FromStr};

use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use identus_crypto::HexStr;
use identus_did::Did;

use crate::{
    Error, MAX_OPERATION_BYTES, OperationKind, PrismDidState, RawPrismOperation, SyntaxError,
};

const PREFIX: &str = "did:prism:";
const SUFFIX_HEX_BYTES: usize = 64;

/// A canonical short-form PRISM DID whose suffix is a SHA-256 digest.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CanonicalPrismDid(Did);

impl CanonicalPrismDid {
    /// Parse and validate a canonical short-form PRISM DID.
    pub fn parse(value: &str) -> Result<Self, Error> {
        let did =
            Did::parse(value).map_err(|_| Error::InvalidSyntax(SyntaxError::InvalidMethod))?;
        validate_method(&did)?;
        validate_suffix(did.method_specific_id())?;
        Ok(Self(did))
    }

    /// Construct a canonical DID from a raw 32-byte suffix digest.
    pub fn from_digest(digest: &[u8; 32]) -> Self {
        let digest = HexStr::try_from_bytes(digest)
            .expect("a SHA-256 digest fits the bounded crypto hex codec");
        let value = format!("{PREFIX}{digest}");
        Self(Did::parse(&value).expect("fixed PRISM prefix and SHA-256 hex form a valid DID"))
    }

    /// Borrow the generic DID value.
    #[must_use]
    pub const fn as_did(&self) -> &Did {
        &self.0
    }

    /// Borrow the lowercase hex suffix.
    #[must_use]
    pub fn suffix(&self) -> &str {
        self.0.method_specific_id()
    }

    /// Consume this value as a generic DID.
    #[must_use]
    pub fn into_did(self) -> Did {
        self.0
    }
}

impl fmt::Display for CanonicalPrismDid {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl fmt::Debug for CanonicalPrismDid {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CanonicalPrismDid")
            .finish_non_exhaustive()
    }
}

impl FromStr for CanonicalPrismDid {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

/// A self-certifying PRISM DID carrying its exact create-operation bytes.
#[derive(Clone, PartialEq, Eq)]
pub struct LongFormPrismDid {
    did: Did,
    canonical: CanonicalPrismDid,
    initial_state: RawPrismOperation,
}

impl LongFormPrismDid {
    /// Parse and validate a long-form PRISM DID.
    pub fn parse(value: &str) -> Result<Self, Error> {
        let did =
            Did::parse(value).map_err(|_| Error::InvalidSyntax(SyntaxError::InvalidMethod))?;
        validate_method(&did)?;
        let (suffix, encoded) = did
            .method_specific_id()
            .split_once(':')
            .ok_or(Error::InvalidSyntax(SyntaxError::InvalidLongForm))?;
        if encoded.is_empty() || encoded.contains(':') || encoded.contains('=') {
            return Err(Error::InvalidSyntax(SyntaxError::InvalidLongForm));
        }
        validate_suffix(suffix)?;
        if decoded_len_upper_bound(encoded.len()) > MAX_OPERATION_BYTES {
            return Err(Error::InvalidSyntax(SyntaxError::TooLong));
        }
        let bytes = URL_SAFE_NO_PAD
            .decode(encoded)
            .map_err(|_| Error::InvalidSyntax(SyntaxError::InvalidLongForm))?;
        if URL_SAFE_NO_PAD.encode(&bytes) != encoded {
            return Err(Error::InvalidSyntax(SyntaxError::InvalidLongForm));
        }
        let initial_state = RawPrismOperation::parse(&bytes)?;
        if initial_state.operation_kind() != OperationKind::CreateDid {
            return Err(Error::InvalidSyntax(SyntaxError::InvalidLongForm));
        }
        let expected = HexStr::try_from_bytes(initial_state.operation_hash().as_bytes())
            .expect("a SHA-256 digest fits the bounded crypto hex codec");
        if suffix != expected.as_str() {
            return Err(Error::InvalidSyntax(SyntaxError::InitialStateHashMismatch));
        }
        PrismDidState::from_initial_state(&initial_state)?;
        let canonical = CanonicalPrismDid::parse(&format!("{PREFIX}{suffix}"))?;
        Ok(Self {
            did,
            canonical,
            initial_state,
        })
    }

    /// Construct a long-form DID from exact validated create-operation bytes.
    pub fn from_initial_state(initial_state: RawPrismOperation) -> Result<Self, Error> {
        if initial_state.operation_kind() != OperationKind::CreateDid {
            return Err(Error::InvalidSyntax(SyntaxError::InvalidLongForm));
        }
        PrismDidState::from_initial_state(&initial_state)?;
        let canonical = CanonicalPrismDid::from_digest(initial_state.operation_hash().as_array());
        let encoded = URL_SAFE_NO_PAD.encode(initial_state.as_bytes());
        let did = Did::parse(&format!("{canonical}:{encoded}"))
            .map_err(|_| Error::InvalidSyntax(SyntaxError::TooLong))?;
        Ok(Self {
            did,
            canonical,
            initial_state,
        })
    }

    /// Borrow the complete generic long-form DID.
    #[must_use]
    pub const fn as_did(&self) -> &Did {
        &self.did
    }

    /// Borrow the equivalent canonical short-form DID.
    #[must_use]
    pub const fn canonical(&self) -> &CanonicalPrismDid {
        &self.canonical
    }

    /// Borrow the exact create operation carried by this identifier.
    #[must_use]
    pub const fn initial_state(&self) -> &RawPrismOperation {
        &self.initial_state
    }
}

impl fmt::Display for LongFormPrismDid {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.did.fmt(formatter)
    }
}

impl fmt::Debug for LongFormPrismDid {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LongFormPrismDid")
            .field("initial_state", &self.initial_state)
            .finish_non_exhaustive()
    }
}

impl FromStr for LongFormPrismDid {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

/// Either canonical or self-certifying long-form PRISM DID syntax.
#[derive(Clone, PartialEq, Eq)]
pub enum PrismDid {
    /// Canonical short form.
    Canonical(CanonicalPrismDid),
    /// Long form with embedded create state.
    LongForm(LongFormPrismDid),
}

impl PrismDid {
    /// Parse either supported PRISM DID form.
    pub fn parse(value: &str) -> Result<Self, Error> {
        let did =
            Did::parse(value).map_err(|_| Error::InvalidSyntax(SyntaxError::InvalidMethod))?;
        validate_method(&did)?;
        if did.method_specific_id().contains(':') {
            LongFormPrismDid::parse(value).map(Self::LongForm)
        } else {
            CanonicalPrismDid::parse(value).map(Self::Canonical)
        }
    }

    /// Borrow the generic DID value in the caller-supplied form.
    #[must_use]
    pub const fn as_did(&self) -> &Did {
        match self {
            Self::Canonical(did) => did.as_did(),
            Self::LongForm(did) => did.as_did(),
        }
    }

    /// Borrow the canonical short form.
    #[must_use]
    pub const fn canonical(&self) -> &CanonicalPrismDid {
        match self {
            Self::Canonical(did) => did,
            Self::LongForm(did) => did.canonical(),
        }
    }
}

impl fmt::Display for PrismDid {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_did().fmt(formatter)
    }
}

impl fmt::Debug for PrismDid {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Canonical(_) => "PrismDid::Canonical(..)",
            Self::LongForm(_) => "PrismDid::LongForm(..)",
        })
    }
}

impl FromStr for PrismDid {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

fn validate_method(did: &Did) -> Result<(), Error> {
    if did.method() == "prism" {
        Ok(())
    } else {
        Err(Error::InvalidSyntax(SyntaxError::InvalidMethod))
    }
}

fn validate_suffix(value: &str) -> Result<(), Error> {
    if value.len() == SUFFIX_HEX_BYTES
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        Ok(())
    } else {
        Err(Error::InvalidSyntax(SyntaxError::InvalidSuffix))
    }
}

const fn decoded_len_upper_bound(encoded_len: usize) -> usize {
    encoded_len.saturating_mul(3).saturating_add(3) / 4
}

#[cfg(test)]
mod tests {
    use prost::Message;

    use super::*;

    fn raw_create() -> RawPrismOperation {
        RawPrismOperation::parse(&hex::decode("0a3f0a3d123b0a076d61737465723010014a2e0a09736563703235366b31122102e45998e18117113835b972b43784fb6b6470b722cb2dc171c5fba00265bc80ee").unwrap()).unwrap()
    }

    fn unprojectable_create() -> RawPrismOperation {
        let endpoint = format!(r#"{{"x":[{}]}}"#, vec!["0"; 100].join(","));
        let create = crate::proto::CreateDid {
            did_data: Some(crate::proto::DidCreationData {
                public_keys: vec![crate::proto::PublicKey {
                    id: "master".to_owned(),
                    usage: crate::proto::KeyUsage::Master as i32,
                    key_data: Some(crate::proto::public_key::KeyData::Compressed(
                        crate::proto::CompressedEcKeyData {
                            curve: "secp256k1".to_owned(),
                            data: hex::decode(
                                "02e45998e18117113835b972b43784fb6b6470b722cb2dc171c5fba00265bc80ee",
                            )
                            .unwrap(),
                        },
                    )),
                }],
                services: (0..crate::MAX_SERVICES)
                    .map(|index| crate::proto::Service {
                        id: format!("service-{index}"),
                        r#type: "LinkedDomains".to_owned(),
                        service_endpoint: endpoint.clone(),
                    })
                    .collect(),
                context: Vec::new(),
            }),
        };
        RawPrismOperation::from_payload(OperationKind::CreateDid, &create.encode_to_vec()).unwrap()
    }

    #[test]
    fn canonical_suffix_is_strict_lowercase_sha256_hex() {
        let valid = "did:prism:44fb82ad3544a8c8acb763c01a6fc7d92d81c38104096319345a4618d1a4275c";
        assert_eq!(CanonicalPrismDid::parse(valid).unwrap().to_string(), valid);
        assert!(
            CanonicalPrismDid::parse(
                "did:example:44fb82ad3544a8c8acb763c01a6fc7d92d81c38104096319345a4618d1a4275c"
            )
            .is_err()
        );
        assert!(
            CanonicalPrismDid::parse(
                "did:prism:44FB82AD3544A8C8ACB763C01A6FC7D92D81C38104096319345A4618D1A4275C"
            )
            .is_err()
        );
    }

    #[test]
    fn debug_redacts_short_and_long_form_identifiers() {
        let canonical = CanonicalPrismDid::from_digest(&[0xabu8; 32]);
        let long = LongFormPrismDid::from_initial_state(raw_create()).unwrap();

        assert!(!format!("{canonical:?}").contains(canonical.suffix()));
        assert!(!format!("{long:?}").contains(long.canonical().suffix()));
        assert!(
            !format!("{:?}", PrismDid::LongForm(long.clone())).contains(long.as_did().as_str())
        );
    }

    #[test]
    fn long_form_round_trip_preserves_exact_initial_state() {
        let raw = raw_create();
        let exact = raw.as_bytes().to_vec();
        let long = LongFormPrismDid::from_initial_state(raw).unwrap();
        let reparsed = LongFormPrismDid::parse(&long.to_string()).unwrap();

        assert_eq!(reparsed.initial_state().as_bytes(), exact);
        assert_eq!(
            reparsed.canonical().suffix(),
            "44fb82ad3544a8c8acb763c01a6fc7d92d81c38104096319345a4618d1a4275c"
        );
        assert!(matches!(
            PrismDid::parse(&long.to_string()),
            Ok(PrismDid::LongForm(_))
        ));
    }

    #[test]
    fn maximum_bounded_create_state_round_trips_as_a_long_form_did() {
        let mut bytes = raw_create().as_bytes().to_vec();
        let unknown_payload_len = MAX_OPERATION_BYTES - bytes.len() - 4;
        prost::encoding::encode_varint(15_u64 << 3 | 2, &mut bytes);
        prost::encoding::encode_varint(unknown_payload_len as u64, &mut bytes);
        bytes.resize(bytes.len() + unknown_payload_len, 0);
        assert_eq!(bytes.len(), MAX_OPERATION_BYTES);

        let operation = RawPrismOperation::parse(&bytes).unwrap();
        let long = LongFormPrismDid::from_initial_state(operation).unwrap();
        assert_eq!(long.as_did().as_str().len(), crate::MAX_LONG_FORM_DID_BYTES);

        let reparsed = LongFormPrismDid::parse(long.as_did().as_str()).unwrap();
        assert_eq!(reparsed.initial_state().as_bytes(), bytes);
    }

    #[test]
    fn long_form_rejects_hash_mismatch_and_non_create_state() {
        let long = LongFormPrismDid::from_initial_state(raw_create())
            .unwrap()
            .to_string();
        let mismatched = long.replacen("44fb", "54fb", 1);
        assert_eq!(
            LongFormPrismDid::parse(&mismatched),
            Err(Error::InvalidSyntax(SyntaxError::InitialStateHashMismatch))
        );

        let update = RawPrismOperation::parse(&[0x12, 0x00]).unwrap();
        assert_eq!(
            LongFormPrismDid::from_initial_state(update),
            Err(Error::InvalidSyntax(SyntaxError::InvalidLongForm))
        );
    }

    #[test]
    fn long_form_rejects_semantically_invalid_create_state() {
        let missing_creation_data = RawPrismOperation::parse(&[0x0a, 0x00]).unwrap();
        assert_eq!(
            LongFormPrismDid::from_initial_state(missing_creation_data),
            Err(Error::InvalidOperation(
                crate::OperationError::RequiredDataMissing
            ))
        );

        let invalid = RawPrismOperation::parse(&[0x0a, 0x02, 0x0a, 0x00]).unwrap();
        let canonical = CanonicalPrismDid::from_digest(invalid.operation_hash().as_array());
        let encoded = URL_SAFE_NO_PAD.encode(invalid.as_bytes());
        assert_eq!(
            LongFormPrismDid::parse(&format!("{canonical}:{encoded}")),
            Err(Error::InvalidOperation(
                crate::OperationError::MissingMasterKey
            ))
        );
    }

    #[test]
    fn long_form_rejects_initial_state_that_cannot_project() {
        let raw = unprojectable_create();
        let canonical = CanonicalPrismDid::from_digest(raw.operation_hash().as_array());
        let encoded = URL_SAFE_NO_PAD.encode(raw.as_bytes());

        assert_eq!(
            LongFormPrismDid::from_initial_state(raw),
            Err(Error::InvalidState(crate::StateError::InvalidCandidate))
        );
        assert_eq!(
            LongFormPrismDid::parse(&format!("{canonical}:{encoded}")),
            Err(Error::InvalidState(crate::StateError::InvalidCandidate))
        );
    }
}
