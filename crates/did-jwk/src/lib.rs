//! Bounded, effect-free `did:jwk` creation and local resolution.
//!
//! The first profile accepts public Ed25519, X25519, P-256, and secp256k1
//! JWKs. It preserves valid alternate JSON serializations when parsing, while
//! creation emits one deterministic compact representation. The crate owns no
//! private key, custody, network, persistence, or global registration.

use std::{collections::BTreeMap, fmt, sync::Arc};

use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use identus_core::Component;
use identus_crypto::{
    Ed25519PublicKey, JwkCurve, P256PublicKey, PublicKeyJwk, Secp256k1PublicKey, X25519PublicKey,
};
use identus_did::{
    ContextEntry, Did, DidDocument, DidDocumentMetadata, DidMethod, DidMethodBinding,
    DidResolutionError, DidResolutionErrorKind, DidResolutionFuture, DidResolutionMetadata,
    DidResolutionResult, DidResolver, MediaType, OneOrMany, ResolutionOptions, Uri,
    VerificationMethod, VerificationRelationship,
};
use serde::de::{self, DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde_json::{Map, Number, Value};

/// Maximum accepted method-specific base64url carrier length in bytes.
pub const MAX_METHOD_SPECIFIC_ID_BYTES: usize = 21_846;
/// Maximum accepted decoded JWK JSON length in bytes.
pub const MAX_JWK_JSON_BYTES: usize = 16_384;
/// Maximum accepted JSON nesting depth, including the root value.
pub const MAX_JSON_DEPTH: usize = 16;
/// Maximum accepted JSON value nodes, including the root value.
pub const MAX_JSON_NODES: usize = 1_024;
/// Maximum accepted members in the top-level JWK object.
pub const MAX_JWK_MEMBERS: usize = 64;

const DID_PREFIX: &str = "did:jwk:";
const DID_JSON: &str = "application/did+json";
const DID_LD_JSON: &str = "application/did+ld+json";
const DID_CORE_CONTEXT: &str = "https://www.w3.org/ns/did/v1";
const JWS_2020_CONTEXT: &str = "https://w3id.org/security/suites/jws-2020/v1";
const DUPLICATE_MARKER: &str = "did_jwk_duplicate_member";
const LIMIT_MARKER: &str = "did_jwk_resource_limit";
const PRIVATE_MEMBERS: [&str; 8] = ["d", "k", "p", "q", "dp", "dq", "qi", "oth"];

/// Metadata for the `identus-did-jwk` crate.
pub const COMPONENT: Component = Component {
    name: "identus-did-jwk",
    summary: "Bounded effect-free did:jwk creation and local resolution.",
};

/// Stable, content-free `did:jwk` error categories.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// The input is not a `did:jwk` DID.
    WrongMethod,
    /// The method carrier is malformed or not canonical base64url.
    MalformedEncoding,
    /// The decoded payload is malformed JSON or is not a JSON object.
    MalformedJson,
    /// An object contains the same member name more than once.
    DuplicateMember,
    /// The JWK contains private key material.
    PrivateKeyMaterial,
    /// The JWK key family or method profile is unsupported.
    UnsupportedProfile,
    /// The public key does not encode valid usable curve material.
    InvalidPublicKey,
    /// The explicit `use` member is unsupported or incompatible with the key.
    IncompatibleUse,
    /// A public finite input boundary was exceeded.
    ResourceLimit,
    /// Projection into the generic DID document model failed.
    Projection,
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::WrongMethod => "did:jwk method mismatch",
            Self::MalformedEncoding => "malformed did:jwk encoding",
            Self::MalformedJson => "malformed did:jwk JSON",
            Self::DuplicateMember => "duplicate did:jwk JSON member",
            Self::PrivateKeyMaterial => "private did:jwk material is prohibited",
            Self::UnsupportedProfile => "unsupported did:jwk profile",
            Self::InvalidPublicKey => "invalid did:jwk public key",
            Self::IncompatibleUse => "incompatible did:jwk use",
            Self::ResourceLimit => "did:jwk resource limit exceeded",
            Self::Projection => "did:jwk document projection failed",
        })
    }
}

impl std::error::Error for Error {}

/// Relationship policy selected by the JWK `use` member.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum KeyUse {
    /// No `use` member: retain the pinned method's five relationships.
    Unspecified,
    /// `use=sig`: authentication, assertion, and capability relationships.
    Signature,
    /// `use=enc`: key agreement only.
    Encryption,
}

/// A validated `did:jwk`, its public JWK, and relationship policy.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DidJwk {
    did: Did,
    public_key: PublicKeyJwk,
    key_use: KeyUse,
}

impl DidJwk {
    /// Create a deterministic DID from a supported public JWK.
    pub fn create(public_key: PublicKeyJwk) -> Result<Self, Error> {
        let key_use = validate_jwk(&public_key)?;
        ensure_member_limit(&public_key)?;
        let json = serde_json::to_vec(&public_key).map_err(|_| Error::Projection)?;
        if json.len() > MAX_JWK_JSON_BYTES {
            return Err(Error::ResourceLimit);
        }
        let carrier = URL_SAFE_NO_PAD.encode(&json);
        if carrier.len() > MAX_METHOD_SPECIFIC_ID_BYTES {
            return Err(Error::ResourceLimit);
        }
        let did = Did::parse(&format!("{DID_PREFIX}{carrier}")).map_err(|_| Error::Projection)?;
        Ok(Self {
            did,
            public_key,
            key_use,
        })
    }

    /// Parse and validate a `did:jwk` string.
    pub fn parse(input: &str) -> Result<Self, Error> {
        if let Some(carrier) = input.strip_prefix(DID_PREFIX)
            && carrier.len() > MAX_METHOD_SPECIFIC_ID_BYTES
        {
            return Err(Error::ResourceLimit);
        }
        let did = Did::parse(input).map_err(|_| Error::MalformedEncoding)?;
        Self::from_did(did)
    }

    /// Validate an already syntax-checked DID.
    pub fn from_did(did: Did) -> Result<Self, Error> {
        if did.method() != "jwk" {
            return Err(Error::WrongMethod);
        }
        let carrier = did.method_specific_id();
        if carrier.len() > MAX_METHOD_SPECIFIC_ID_BYTES {
            return Err(Error::ResourceLimit);
        }
        if carrier.is_empty()
            || carrier
                .bytes()
                .any(|byte| !byte.is_ascii_alphanumeric() && byte != b'-' && byte != b'_')
        {
            return Err(Error::MalformedEncoding);
        }
        let predicted = decoded_len_upper_bound(carrier.len()).ok_or(Error::ResourceLimit)?;
        if predicted > MAX_JWK_JSON_BYTES {
            return Err(Error::ResourceLimit);
        }
        let json = URL_SAFE_NO_PAD
            .decode(carrier.as_bytes())
            .map_err(|_| Error::MalformedEncoding)?;
        if json.len() > MAX_JWK_JSON_BYTES || URL_SAFE_NO_PAD.encode(&json) != carrier {
            return Err(if json.len() > MAX_JWK_JSON_BYTES {
                Error::ResourceLimit
            } else {
                Error::MalformedEncoding
            });
        }
        let value = parse_bounded_json(&json)?;
        let object = value.as_object().ok_or(Error::MalformedJson)?;
        reject_private_or_profile_members(object)?;
        let public_key: PublicKeyJwk =
            serde_json::from_value(value).map_err(|_| Error::UnsupportedProfile)?;
        let key_use = validate_jwk(&public_key)?;
        Ok(Self {
            did,
            public_key,
            key_use,
        })
    }

    /// Borrow the exact validated DID representation.
    #[must_use]
    pub const fn did(&self) -> &Did {
        &self.did
    }

    /// Borrow the validated public JWK.
    #[must_use]
    pub const fn public_key(&self) -> &PublicKeyJwk {
        &self.public_key
    }

    /// Return the selected relationship policy.
    #[must_use]
    pub const fn key_use(&self) -> KeyUse {
        self.key_use
    }

    /// Resolve this DID to its deterministic context-free DID document.
    pub fn resolve(&self) -> Result<DidDocument, Error> {
        self.resolve_document(false)
    }

    fn resolve_document(&self, json_ld: bool) -> Result<DidDocument, Error> {
        let method_uri = Uri::parse(&format!("{}#0", self.did)).map_err(|_| Error::Projection)?;
        let jwk = serde_json::to_value(&self.public_key).map_err(|_| Error::Projection)?;
        let mut properties = BTreeMap::new();
        properties.insert("publicKeyJwk".to_owned(), jwk);
        let method = VerificationMethod::new(
            method_uri.clone(),
            "JsonWebKey2020".to_owned(),
            self.did.clone(),
            properties,
        )
        .map_err(|_| Error::Projection)?;
        let relationship = || VerificationRelationship::Reference(method_uri.clone());
        let builder = DidDocument::builder(self.did.clone());
        let builder = if json_ld {
            let contexts = OneOrMany::try_many(vec![
                ContextEntry::Uri(
                    Uri::parse(DID_CORE_CONTEXT).expect("fixed DID Core context URI is valid"),
                ),
                ContextEntry::Uri(
                    Uri::parse(JWS_2020_CONTEXT).expect("fixed JWS 2020 context URI is valid"),
                ),
            ])
            .expect("fixed did:jwk JSON-LD contexts are non-empty");
            builder.context(contexts)
        } else {
            builder
        };
        let builder = builder.verification_methods(vec![method]);
        let builder = match self.key_use {
            KeyUse::Unspecified => builder
                .authentication(vec![relationship()])
                .assertion_method(vec![relationship()])
                .capability_invocation(vec![relationship()])
                .capability_delegation(vec![relationship()])
                .key_agreement(vec![relationship()]),
            KeyUse::Signature => builder
                .authentication(vec![relationship()])
                .assertion_method(vec![relationship()])
                .capability_invocation(vec![relationship()])
                .capability_delegation(vec![relationship()]),
            KeyUse::Encryption => builder.key_agreement(vec![relationship()]),
        };
        builder.build().map_err(|_| Error::Projection)
    }
}

/// Deterministic local resolver for explicit registry composition.
#[derive(Clone, Copy, Debug, Default)]
pub struct DidJwkResolver;

impl DidJwkResolver {
    /// Consume this resolver into an explicit `jwk` method binding.
    #[must_use]
    pub fn into_method_binding(self) -> DidMethodBinding {
        DidMethodBinding::new(
            DidMethod::parse("jwk").expect("fixed method name is valid"),
            Arc::new(self),
        )
    }
}

impl DidResolver for DidJwkResolver {
    fn resolve<'a>(
        &'a self,
        did: &'a Did,
        options: &'a ResolutionOptions,
    ) -> DidResolutionFuture<'a> {
        Box::pin(async move { resolve_result(did, options) })
    }
}

fn decoded_len_upper_bound(encoded_len: usize) -> Option<usize> {
    let complete = encoded_len.checked_div(4)?.checked_mul(3)?;
    match encoded_len % 4 {
        0 => Some(complete),
        2 => complete.checked_add(1),
        3 => complete.checked_add(2),
        1 => None,
        _ => unreachable!("remainder modulo four"),
    }
}

fn ensure_member_limit(jwk: &PublicKeyJwk) -> Result<(), Error> {
    let structural = if jwk.y().is_some() { 4usize } else { 3usize };
    structural
        .checked_add(jwk.extensions().len())
        .filter(|count| *count <= MAX_JWK_MEMBERS)
        .map(|_| ())
        .ok_or(Error::ResourceLimit)
}

fn reject_private_or_profile_members(object: &Map<String, Value>) -> Result<(), Error> {
    if object.len() > MAX_JWK_MEMBERS {
        return Err(Error::ResourceLimit);
    }
    if PRIVATE_MEMBERS
        .iter()
        .any(|name| object.contains_key(*name))
    {
        return Err(Error::PrivateKeyMaterial);
    }
    if object.contains_key("key_ops") {
        return Err(Error::UnsupportedProfile);
    }
    Ok(())
}

fn validate_jwk(jwk: &PublicKeyJwk) -> Result<KeyUse, Error> {
    ensure_member_limit(jwk)?;
    if jwk.extensions().contains_key("key_ops") {
        return Err(Error::UnsupportedProfile);
    }
    if PRIVATE_MEMBERS
        .iter()
        .any(|name| jwk.extensions().contains_key(*name))
    {
        return Err(Error::PrivateKeyMaterial);
    }
    let key_use = match jwk.extensions().get("use") {
        None => KeyUse::Unspecified,
        Some(Value::String(value)) if value == "sig" => KeyUse::Signature,
        Some(Value::String(value)) if value == "enc" => KeyUse::Encryption,
        Some(_) => return Err(Error::IncompatibleUse),
    };
    validate_key_material(jwk)?;
    match (jwk.crv(), key_use) {
        (JwkCurve::Ed25519 | JwkCurve::P256 | JwkCurve::Secp256k1, KeyUse::Encryption)
        | (JwkCurve::X25519, KeyUse::Signature) => Err(Error::IncompatibleUse),
        (JwkCurve::Ed25519 | JwkCurve::X25519 | JwkCurve::P256 | JwkCurve::Secp256k1, _) => {
            Ok(key_use)
        }
    }
}

fn validate_key_material(jwk: &PublicKeyJwk) -> Result<(), Error> {
    let x = jwk.x().to_bytes();
    match jwk.crv() {
        JwkCurve::Ed25519 => {
            let key = Ed25519PublicKey::from_slice(&x).map_err(|_| Error::InvalidPublicKey)?;
            if !key.has_canonical_encoding() || key.is_weak() {
                return Err(Error::InvalidPublicKey);
            }
        }
        JwkCurve::X25519 => {
            let key = X25519PublicKey::from_slice(&x).map_err(|_| Error::InvalidPublicKey)?;
            if !key.has_canonical_encoding() || !key.is_contributory() {
                return Err(Error::InvalidPublicKey);
            }
        }
        JwkCurve::P256 | JwkCurve::Secp256k1 => {
            let y = jwk.y().ok_or(Error::UnsupportedProfile)?.to_bytes();
            if x.len() != 32 || y.len() != 32 {
                return Err(Error::InvalidPublicKey);
            }
            let mut point = [0u8; 65];
            point[0] = 4;
            point[1..33].copy_from_slice(&x);
            point[33..].copy_from_slice(&y);
            match jwk.crv() {
                JwkCurve::P256 => {
                    P256PublicKey::from_slice(&point).map_err(|_| Error::InvalidPublicKey)?;
                }
                JwkCurve::Secp256k1 => {
                    Secp256k1PublicKey::from_slice(&point).map_err(|_| Error::InvalidPublicKey)?;
                }
                _ => unreachable!("EC branch fixes the curve"),
            }
        }
    }
    Ok(())
}

fn parse_bounded_json(input: &[u8]) -> Result<Value, Error> {
    let mut budget = JsonBudget::default();
    let mut deserializer = serde_json::Deserializer::from_slice(input);
    let value = JsonSeed {
        budget: &mut budget,
        depth: 1,
        top_level: true,
    }
    .deserialize(&mut deserializer)
    .map_err(classify_json_error)?;
    deserializer.end().map_err(|_| Error::MalformedJson)?;
    Ok(value)
}

fn classify_json_error(error: serde_json::Error) -> Error {
    let message = error.to_string();
    if message.contains(DUPLICATE_MARKER) {
        Error::DuplicateMember
    } else if message.contains(LIMIT_MARKER) {
        Error::ResourceLimit
    } else {
        Error::MalformedJson
    }
}

#[derive(Default)]
struct JsonBudget {
    nodes: usize,
}

struct JsonSeed<'a> {
    budget: &'a mut JsonBudget,
    depth: usize,
    top_level: bool,
}

impl<'de> DeserializeSeed<'de> for JsonSeed<'_> {
    type Value = Value;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        if self.depth > MAX_JSON_DEPTH {
            return Err(de::Error::custom(LIMIT_MARKER));
        }
        self.budget.nodes = self
            .budget
            .nodes
            .checked_add(1)
            .filter(|nodes| *nodes <= MAX_JSON_NODES)
            .ok_or_else(|| de::Error::custom(LIMIT_MARKER))?;
        deserializer.deserialize_any(JsonVisitor {
            budget: self.budget,
            depth: self.depth,
            top_level: self.top_level,
        })
    }
}

struct JsonVisitor<'a> {
    budget: &'a mut JsonBudget,
    depth: usize,
    top_level: bool,
}

impl<'de> Visitor<'de> for JsonVisitor<'_> {
    type Value = Value;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("bounded JSON")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
        Ok(Value::Bool(value))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> {
        Ok(Value::Number(Number::from(value)))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> {
        Ok(Value::Number(Number::from(value)))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Number::from_f64(value)
            .map(Value::Number)
            .ok_or_else(|| de::Error::custom("non-finite JSON number"))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
        Ok(Value::String(value.to_owned()))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E> {
        Ok(Value::String(value))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E> {
        Ok(Value::Null)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E> {
        Ok(Value::Null)
    }

    fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut values = Vec::new();
        while let Some(value) = sequence.next_element_seed(JsonSeed {
            budget: self.budget,
            depth: self.depth + 1,
            top_level: false,
        })? {
            values.push(value);
        }
        Ok(Value::Array(values))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut values = Map::new();
        while let Some(key) = map.next_key::<String>()? {
            if values.contains_key(&key) {
                return Err(de::Error::custom(DUPLICATE_MARKER));
            }
            if self.top_level && values.len() >= MAX_JWK_MEMBERS {
                return Err(de::Error::custom(LIMIT_MARKER));
            }
            let value = map.next_value_seed(JsonSeed {
                budget: self.budget,
                depth: self.depth + 1,
                top_level: false,
            })?;
            values.insert(key, value);
        }
        Ok(Value::Object(values))
    }
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
    if did.method() == "jwk" && did.method_specific_id().len() > MAX_METHOD_SPECIFIC_ID_BYTES {
        return resolution_failure(DidResolutionErrorKind::InvalidDid);
    }
    let parsed = match DidJwk::from_did(did.clone()) {
        Ok(value) => value,
        Err(Error::WrongMethod) => {
            return resolution_failure(DidResolutionErrorKind::MethodNotSupported);
        }
        Err(_) => return resolution_failure(DidResolutionErrorKind::InvalidDid),
    };
    let document = match parsed.resolve_document(selected == DID_LD_JSON) {
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
    .expect("standard did:jwk resolution error is valid");
    DidResolutionResult::failure(metadata).expect("standard did:jwk resolution failure is valid")
}

#[cfg(test)]
mod tests {
    use super::*;
    use identus_crypto::{Ed25519PrivateKey, EncodeJwk, X25519PrivateKey};

    #[test]
    fn constructor_parser_round_trip_is_deterministic() {
        let jwk = Ed25519PrivateKey::from_slice(&[7; 32])
            .unwrap()
            .to_public_key()
            .encode_jwk();
        let created = DidJwk::create(jwk).unwrap();
        assert_eq!(DidJwk::parse(created.did().as_str()).unwrap(), created);
    }

    #[test]
    fn nested_duplicate_members_are_rejected() {
        let json = br#"{"kty":"OKP","crv":"Ed25519","x":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA","meta":{"a":1,"a":2}}"#;
        let did = format!("{DID_PREFIX}{}", URL_SAFE_NO_PAD.encode(json));
        assert_eq!(DidJwk::parse(&did), Err(Error::DuplicateMember));
    }

    #[test]
    fn key_ops_and_private_members_are_rejected() {
        for member in [r#""key_ops":["verify"]"#, r#""d":"secret""#] {
            let json = format!(
                r#"{{"kty":"OKP","crv":"Ed25519","x":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",{member}}}"#
            );
            let did = format!("{DID_PREFIX}{}", URL_SAFE_NO_PAD.encode(json));
            assert!(matches!(
                DidJwk::parse(&did),
                Err(Error::UnsupportedProfile | Error::PrivateKeyMaterial)
            ));
        }
    }

    #[test]
    fn use_must_match_the_curve_when_explicit() {
        let ed = Ed25519PrivateKey::from_slice(&[8; 32])
            .unwrap()
            .to_public_key()
            .encode_jwk();
        let mut value = serde_json::to_value(ed).unwrap();
        value["use"] = Value::String("enc".to_owned());
        let json = serde_json::to_vec(&value).unwrap();
        let did = format!("{DID_PREFIX}{}", URL_SAFE_NO_PAD.encode(json));
        assert_eq!(DidJwk::parse(&did), Err(Error::IncompatibleUse));

        let x = X25519PrivateKey::from_slice(&[9; 32])
            .unwrap()
            .to_public_key()
            .encode_jwk();
        let mut value = serde_json::to_value(x).unwrap();
        value["use"] = Value::String("sig".to_owned());
        let json = serde_json::to_vec(&value).unwrap();
        let did = format!("{DID_PREFIX}{}", URL_SAFE_NO_PAD.encode(json));
        assert_eq!(DidJwk::parse(&did), Err(Error::IncompatibleUse));
    }

    #[test]
    fn errors_are_content_free() {
        assert_eq!(
            Error::InvalidPublicKey.to_string(),
            "invalid did:jwk public key"
        );
        assert!(!Error::MalformedJson.to_string().contains("secret"));
    }
}
