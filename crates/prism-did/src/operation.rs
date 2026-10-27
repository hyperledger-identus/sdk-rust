//! Validated, transport-neutral PRISM DID operation domain values.

use std::{collections::BTreeSet, fmt};

use fluent_uri::Uri as ParsedUri;
use identus_crypto::{Ed25519PublicKey, Secp256k1PublicKey, Sha256Digest, X25519PublicKey};
use prost::Message;
use serde::de::{self, DeserializeSeed, MapAccess, SeqAccess, Visitor};

use crate::{
    Error, MAX_CONTEXTS, MAX_ID_CHARACTERS, MAX_SERVICE_ENDPOINT_BYTES,
    MAX_SERVICE_ENDPOINT_CHARACTERS, MAX_SERVICES, MAX_TYPE_CHARACTERS, MAX_VERIFICATION_METHODS,
    OperationError, OperationKind, RawPrismOperation, WireError, proto,
};

mod builder;
mod debug;

/// Supported PRISM verification-method purpose.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum KeyUsage {
    /// Authorizes DID operations and never appears in the DID document.
    Master,
    /// Signs credentials and assertions.
    Issuing,
    /// Establishes shared secrets.
    KeyAgreement,
    /// Authenticates the DID subject.
    Authentication,
    /// Authorizes revocation and never appears in the DID document.
    Revocation,
    /// Authorizes capability invocation.
    CapabilityInvocation,
    /// Authorizes capability delegation.
    CapabilityDelegation,
    /// Authorizes VDR operations and never appears in the DID document.
    Vdr,
}

/// A validated public key supported by the finite PRISM method profile.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PrismPublicKeyMaterial {
    /// secp256k1 public key.
    Secp256k1(Secp256k1PublicKey),
    /// Ed25519 public key.
    Ed25519(Ed25519PublicKey),
    /// X25519 public key.
    X25519(X25519PublicKey),
}

/// A validated PRISM public key and its single protocol purpose.
#[derive(Clone, PartialEq, Eq)]
pub struct PrismPublicKey {
    id: String,
    usage: KeyUsage,
    material: PrismPublicKeyMaterial,
}

impl PrismPublicKey {
    /// Borrow the DID-local fragment identifier without a leading `#`.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Return the key's protocol purpose.
    #[must_use]
    pub const fn usage(&self) -> KeyUsage {
        self.usage
    }

    /// Borrow validated public-key material.
    #[must_use]
    pub const fn material(&self) -> &PrismPublicKeyMaterial {
        &self.material
    }
}

/// A typed service-type candidate, preserving one or many values.
///
/// Use its constructors or an enclosing operation builder to validate public
/// variant payloads before they enter protocol state.
#[derive(Clone, PartialEq, Eq)]
pub enum ServiceType {
    /// One service type token.
    One(String),
    /// A non-empty canonical JSON array of service type tokens.
    Many(Vec<String>),
}

/// One typed service-endpoint candidate value.
///
/// Enclosing endpoint and operation constructors revalidate public variant
/// payloads before they enter protocol state.
#[derive(Clone, PartialEq, Eq)]
pub enum ServiceEndpointValue {
    /// An absolute URI, normalized according to RFC 3986 generic rules.
    Uri(String),
    /// A JSON object with duplicate member names rejected recursively.
    Object(serde_json::Map<String, serde_json::Value>),
}

/// A typed PRISM service-endpoint candidate, preserving one or many values.
///
/// Use its constructors or an enclosing operation builder to validate public
/// variant payloads before they enter protocol state.
#[derive(Clone, PartialEq, Eq)]
pub enum ServiceEndpoint {
    /// One endpoint value.
    One(ServiceEndpointValue),
    /// A non-empty JSON array of endpoint values.
    Many(Vec<ServiceEndpointValue>),
}

/// A validated PRISM service.
#[derive(Clone, PartialEq, Eq)]
pub struct PrismService {
    id: String,
    service_type: ServiceType,
    endpoint: ServiceEndpoint,
}

impl PrismService {
    /// Borrow the DID-local fragment identifier without a leading `#`.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Borrow the service type.
    #[must_use]
    pub const fn service_type(&self) -> &ServiceType {
        &self.service_type
    }

    /// Borrow the service endpoint.
    #[must_use]
    pub const fn endpoint(&self) -> &ServiceEndpoint {
        &self.endpoint
    }

    pub(crate) fn replace_type(&mut self, service_type: ServiceType) {
        self.service_type = service_type;
    }

    pub(crate) fn replace_endpoint(&mut self, endpoint: ServiceEndpoint) {
        self.endpoint = endpoint;
    }
}

/// A validated PRISM DID create operation.
#[derive(Clone, PartialEq)]
pub struct CreateDidOperation {
    public_keys: Vec<PrismPublicKey>,
    services: Vec<PrismService>,
    contexts: Vec<String>,
}

/// One ordered PRISM DID update-action candidate.
///
/// [`UpdateDidOperation::new`] revalidates every public variant payload before
/// emitting an operation.
#[derive(Clone, PartialEq)]
#[non_exhaustive]
pub enum UpdateAction {
    /// Add a previously unseen key identifier.
    AddKey(PrismPublicKey),
    /// Retire an active key identifier.
    RemoveKey(String),
    /// Add a previously unseen service identifier.
    AddService(PrismService),
    /// Retire an active service identifier.
    RemoveService(String),
    /// Replace either or both mutable service fields.
    UpdateService {
        /// Existing service identifier.
        id: String,
        /// Optional replacement type; empty wire text means unchanged.
        service_type: Option<ServiceType>,
        /// Optional replacement endpoint; empty wire text means unchanged.
        endpoint: Option<ServiceEndpoint>,
    },
    /// Replace the complete additional-context list.
    PatchContext(Vec<String>),
}

/// A validated ordered PRISM DID update operation.
#[derive(Clone, PartialEq)]
pub struct UpdateDidOperation {
    did: crate::CanonicalPrismDid,
    previous_operation_hash: Sha256Digest,
    actions: Vec<UpdateAction>,
}

impl UpdateDidOperation {
    /// Decode and validate an exact raw PRISM update operation.
    pub fn parse(raw: &RawPrismOperation) -> Result<Self, Error> {
        if raw.operation_kind() != OperationKind::UpdateDid {
            return Err(OperationError::InvalidValue.into());
        }
        let wire = proto::UpdateDid::decode(raw.operation_payload())
            .map_err(|_| Error::InvalidWire(WireError::Malformed))?;
        if wire.actions.is_empty() {
            return Err(OperationError::RequiredDataMissing.into());
        }
        let did = canonical_from_suffix(&wire.id)?;
        let previous_operation_hash = Sha256Digest::from_bytes(&wire.previous_operation_hash)
            .map_err(|_| Error::InvalidOperation(OperationError::InvalidValue))?;
        let actions = wire
            .actions
            .into_iter()
            .map(parse_update_action)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            did,
            previous_operation_hash,
            actions,
        })
    }

    /// Borrow the canonical DID targeted by this update.
    #[must_use]
    pub const fn did(&self) -> &crate::CanonicalPrismDid {
        &self.did
    }

    /// Return the exact predecessor operation hash.
    pub const fn previous_operation_hash(&self) -> Sha256Digest {
        self.previous_operation_hash
    }

    /// Borrow actions in signed wire order.
    #[must_use]
    pub fn actions(&self) -> &[UpdateAction] {
        &self.actions
    }
}

/// A validated PRISM DID deactivation operation.
#[derive(Clone, PartialEq, Eq)]
pub struct DeactivateDidOperation {
    did: crate::CanonicalPrismDid,
    previous_operation_hash: Sha256Digest,
}

impl DeactivateDidOperation {
    /// Decode and validate an exact raw PRISM deactivation operation.
    pub fn parse(raw: &RawPrismOperation) -> Result<Self, Error> {
        if raw.operation_kind() != OperationKind::DeactivateDid {
            return Err(OperationError::InvalidValue.into());
        }
        let wire = proto::DeactivateDid::decode(raw.operation_payload())
            .map_err(|_| Error::InvalidWire(WireError::Malformed))?;
        let did = canonical_from_suffix(&wire.id)?;
        let previous_operation_hash = Sha256Digest::from_bytes(&wire.previous_operation_hash)
            .map_err(|_| Error::InvalidOperation(OperationError::InvalidValue))?;
        Ok(Self {
            did,
            previous_operation_hash,
        })
    }

    /// Borrow the canonical DID targeted by this deactivation.
    #[must_use]
    pub const fn did(&self) -> &crate::CanonicalPrismDid {
        &self.did
    }

    /// Return the exact predecessor operation hash.
    pub const fn previous_operation_hash(&self) -> Sha256Digest {
        self.previous_operation_hash
    }
}

impl CreateDidOperation {
    /// Decode and validate an exact raw PRISM create operation.
    pub fn parse(raw: &RawPrismOperation) -> Result<Self, Error> {
        if raw.operation_kind() != OperationKind::CreateDid {
            return Err(OperationError::InvalidValue.into());
        }
        let wire = proto::CreateDid::decode(raw.operation_payload())
            .map_err(|_| Error::InvalidWire(WireError::Malformed))?;
        let data = wire
            .did_data
            .ok_or(Error::InvalidOperation(OperationError::RequiredDataMissing))?;
        if data.public_keys.len() > MAX_VERIFICATION_METHODS || data.services.len() > MAX_SERVICES {
            return Err(OperationError::TooManyItems.into());
        }
        if data.context.len() > MAX_CONTEXTS {
            return Err(OperationError::TooManyItems.into());
        }

        let public_keys = data
            .public_keys
            .into_iter()
            .map(parse_public_key)
            .collect::<Result<Vec<_>, _>>()?;
        ensure_unique(public_keys.iter().map(PrismPublicKey::id))?;
        if !public_keys.iter().any(|key| key.usage == KeyUsage::Master) {
            return Err(OperationError::MissingMasterKey.into());
        }

        let services = data
            .services
            .into_iter()
            .map(parse_service)
            .collect::<Result<Vec<_>, _>>()?;
        ensure_unique(services.iter().map(PrismService::id))?;

        let contexts = data
            .context
            .into_iter()
            .map(|context| normalize_uri(&context))
            .collect::<Result<Vec<_>, _>>()?;
        ensure_unique(contexts.iter().map(String::as_str))?;

        Ok(Self {
            public_keys,
            services,
            contexts,
        })
    }

    /// Borrow keys in signed wire order.
    #[must_use]
    pub fn public_keys(&self) -> &[PrismPublicKey] {
        &self.public_keys
    }

    /// Borrow services in signed wire order.
    #[must_use]
    pub fn services(&self) -> &[PrismService] {
        &self.services
    }

    /// Borrow normalized additional contexts in signed wire order.
    #[must_use]
    pub fn contexts(&self) -> &[String] {
        &self.contexts
    }
}

fn parse_public_key(key: proto::PublicKey) -> Result<PrismPublicKey, Error> {
    validate_fragment(&key.id)?;
    let usage = match proto::KeyUsage::try_from(key.usage) {
        Ok(proto::KeyUsage::Master) => KeyUsage::Master,
        Ok(proto::KeyUsage::Issuing) => KeyUsage::Issuing,
        Ok(proto::KeyUsage::KeyAgreement) => KeyUsage::KeyAgreement,
        Ok(proto::KeyUsage::Authentication) => KeyUsage::Authentication,
        Ok(proto::KeyUsage::Revocation) => KeyUsage::Revocation,
        Ok(proto::KeyUsage::CapabilityInvocation) => KeyUsage::CapabilityInvocation,
        Ok(proto::KeyUsage::CapabilityDelegation) => KeyUsage::CapabilityDelegation,
        Ok(proto::KeyUsage::Vdr) => KeyUsage::Vdr,
        Ok(proto::KeyUsage::Unknown) | Err(_) => return Err(OperationError::UnsupportedKey.into()),
    };
    let key_data = key
        .key_data
        .ok_or(Error::InvalidOperation(OperationError::RequiredDataMissing))?;
    let (curve, uncompressed, compressed) = match key_data {
        proto::public_key::KeyData::Ec(value) => (value.curve, Some((value.x, value.y)), None),
        proto::public_key::KeyData::Compressed(value) => (value.curve, None, Some(value.data)),
    };
    let material = match curve.as_str() {
        "secp256k1" => {
            let encoded = if let Some((x, y)) = uncompressed {
                if x.len() != 32 || y.len() != 32 {
                    return Err(OperationError::UnsupportedKey.into());
                }
                let mut bytes = Vec::with_capacity(65);
                bytes.push(0x04);
                bytes.extend_from_slice(&x);
                bytes.extend_from_slice(&y);
                bytes
            } else {
                compressed.ok_or(Error::InvalidOperation(OperationError::RequiredDataMissing))?
            };
            PrismPublicKeyMaterial::Secp256k1(
                Secp256k1PublicKey::from_slice(&encoded)
                    .map_err(|_| Error::InvalidOperation(OperationError::UnsupportedKey))?,
            )
        }
        "Ed25519" => {
            let encoded = okp_bytes(uncompressed, compressed)?;
            PrismPublicKeyMaterial::Ed25519(
                Ed25519PublicKey::from_slice(&encoded)
                    .map_err(|_| Error::InvalidOperation(OperationError::UnsupportedKey))?,
            )
        }
        "X25519" => {
            let encoded = okp_bytes(uncompressed, compressed)?;
            PrismPublicKeyMaterial::X25519(
                X25519PublicKey::from_slice(&encoded)
                    .map_err(|_| Error::InvalidOperation(OperationError::UnsupportedKey))?,
            )
        }
        _ => return Err(OperationError::UnsupportedKey.into()),
    };
    if matches!(usage, KeyUsage::Master | KeyUsage::Vdr)
        && !matches!(material, PrismPublicKeyMaterial::Secp256k1(_))
    {
        return Err(OperationError::UnsupportedKey.into());
    }
    Ok(PrismPublicKey {
        id: key.id,
        usage,
        material,
    })
}

fn parse_update_action(action: proto::UpdateDidAction) -> Result<UpdateAction, Error> {
    use proto::update_did_action::Action;

    match action
        .action
        .ok_or(Error::InvalidOperation(OperationError::RequiredDataMissing))?
    {
        Action::AddKey(action) => action
            .key
            .ok_or(Error::InvalidOperation(OperationError::RequiredDataMissing))
            .and_then(parse_public_key)
            .map(UpdateAction::AddKey),
        Action::RemoveKey(action) => {
            validate_fragment(&action.key_id)?;
            Ok(UpdateAction::RemoveKey(action.key_id))
        }
        Action::AddService(action) => action
            .service
            .ok_or(Error::InvalidOperation(OperationError::RequiredDataMissing))
            .and_then(parse_service)
            .map(UpdateAction::AddService),
        Action::RemoveService(action) => {
            validate_fragment(&action.service_id)?;
            Ok(UpdateAction::RemoveService(action.service_id))
        }
        Action::UpdateService(action) => {
            validate_fragment(&action.service_id)?;
            let service_type = if action.r#type.is_empty() {
                None
            } else {
                Some(parse_service_type(&action.r#type)?)
            };
            let endpoint = if action.service_endpoints.is_empty() {
                None
            } else {
                Some(parse_service_endpoint(&action.service_endpoints)?)
            };
            Ok(UpdateAction::UpdateService {
                id: action.service_id,
                service_type,
                endpoint,
            })
        }
        Action::PatchContext(action) => {
            if action.context.len() > MAX_CONTEXTS {
                return Err(OperationError::TooManyItems.into());
            }
            let contexts = action
                .context
                .into_iter()
                .map(|context| normalize_uri(&context))
                .collect::<Result<Vec<_>, _>>()?;
            ensure_unique(contexts.iter().map(String::as_str))?;
            Ok(UpdateAction::PatchContext(contexts))
        }
    }
}

fn canonical_from_suffix(suffix: &str) -> Result<crate::CanonicalPrismDid, Error> {
    crate::CanonicalPrismDid::parse(&format!("did:prism:{suffix}"))
        .map_err(|_| Error::InvalidOperation(OperationError::InvalidValue))
}

fn okp_bytes(
    uncompressed: Option<(Vec<u8>, Vec<u8>)>,
    compressed: Option<Vec<u8>>,
) -> Result<Vec<u8>, Error> {
    match (uncompressed, compressed) {
        (Some((x, y)), None) if y.is_empty() => Ok(x),
        (None, Some(data)) => Ok(data),
        _ => Err(OperationError::UnsupportedKey.into()),
    }
}

fn parse_service(service: proto::Service) -> Result<PrismService, Error> {
    validate_fragment(&service.id)?;
    let service_type = parse_service_type(&service.r#type)?;
    let endpoint = parse_service_endpoint(&service.service_endpoint)?;
    Ok(PrismService {
        id: service.id,
        service_type,
        endpoint,
    })
}

fn parse_service_type(value: &str) -> Result<ServiceType, Error> {
    if value.chars().count() > MAX_TYPE_CHARACTERS || !value.is_ascii() {
        return Err(OperationError::ValueTooLong.into());
    }
    if value.starts_with('[') {
        let values: Vec<String> = serde_json::from_str(value)
            .map_err(|_| Error::InvalidOperation(OperationError::InvalidValue))?;
        if values.is_empty()
            || values.iter().any(|item| !valid_service_type_token(item))
            || serde_json::to_string(&values).ok().as_deref() != Some(value)
        {
            return Err(OperationError::InvalidValue.into());
        }
        ensure_unique(values.iter().map(String::as_str))?;
        Ok(ServiceType::Many(values))
    } else if valid_service_type_token(value) {
        Ok(ServiceType::One(value.to_owned()))
    } else {
        Err(OperationError::InvalidValue.into())
    }
}

fn valid_service_type_token(value: &str) -> bool {
    !value.is_empty()
        && !value.starts_with(' ')
        && !value.ends_with(' ')
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b' '))
}

fn parse_service_endpoint(value: &str) -> Result<ServiceEndpoint, Error> {
    if value.len() > MAX_SERVICE_ENDPOINT_BYTES
        || value.chars().count() > MAX_SERVICE_ENDPOINT_CHARACTERS
        || value.is_empty()
    {
        return Err(OperationError::ValueTooLong.into());
    }
    if value.starts_with('{') || value.starts_with('[') {
        let json = parse_json_without_duplicate_names(value)?;
        return match json {
            serde_json::Value::Object(map) => {
                Ok(ServiceEndpoint::One(ServiceEndpointValue::Object(map)))
            }
            serde_json::Value::Array(values) if !values.is_empty() => values
                .into_iter()
                .map(endpoint_from_json)
                .collect::<Result<Vec<_>, _>>()
                .and_then(|values| {
                    for (index, value) in values.iter().enumerate() {
                        if values[..index].contains(value) {
                            return Err(OperationError::DuplicateValue.into());
                        }
                    }
                    Ok(ServiceEndpoint::Many(values))
                }),
            _ => Err(OperationError::InvalidValue.into()),
        };
    }
    normalize_uri(value).map(|uri| ServiceEndpoint::One(ServiceEndpointValue::Uri(uri)))
}

fn endpoint_from_json(value: serde_json::Value) -> Result<ServiceEndpointValue, Error> {
    match value {
        serde_json::Value::String(uri) => normalize_uri(&uri).map(ServiceEndpointValue::Uri),
        serde_json::Value::Object(map) => Ok(ServiceEndpointValue::Object(map)),
        _ => Err(OperationError::InvalidValue.into()),
    }
}

fn normalize_uri(value: &str) -> Result<String, Error> {
    let parsed = ParsedUri::parse(value)
        .map_err(|_| Error::InvalidOperation(OperationError::InvalidValue))?;
    let normalized = parsed.normalize().to_string();
    identus_did::Uri::parse(&normalized)
        .map_err(|_| Error::InvalidOperation(OperationError::InvalidValue))?;
    Ok(normalized)
}

pub(crate) fn validate_fragment(value: &str) -> Result<(), Error> {
    if value.is_empty() || value.chars().count() > MAX_ID_CHARACTERS || !value.is_ascii() {
        return Err(OperationError::InvalidValue.into());
    }
    let bytes = value.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        let byte = bytes[index];
        if byte == b'%' {
            if index + 2 >= bytes.len()
                || !bytes[index + 1].is_ascii_hexdigit()
                || !bytes[index + 2].is_ascii_hexdigit()
            {
                return Err(OperationError::InvalidValue.into());
            }
            index += 3;
            continue;
        }
        if !(byte.is_ascii_alphanumeric()
            || matches!(
                byte,
                b'-' | b'.'
                    | b'_'
                    | b'~'
                    | b'!'
                    | b'$'
                    | b'&'
                    | b'\''
                    | b'('
                    | b')'
                    | b'*'
                    | b'+'
                    | b','
                    | b';'
                    | b'='
                    | b':'
                    | b'@'
                    | b'/'
                    | b'?'
            ))
        {
            return Err(OperationError::InvalidValue.into());
        }
        index += 1;
    }
    Ok(())
}

fn ensure_unique<'a>(values: impl IntoIterator<Item = &'a str>) -> Result<(), Error> {
    let mut seen = BTreeSet::new();
    for value in values {
        if !seen.insert(value) {
            return Err(OperationError::DuplicateValue.into());
        }
    }
    Ok(())
}

fn parse_json_without_duplicate_names(value: &str) -> Result<serde_json::Value, Error> {
    let mut deserializer = serde_json::Deserializer::from_str(value);
    let parsed = UniqueJson
        .deserialize(&mut deserializer)
        .map_err(|_| Error::InvalidOperation(OperationError::InvalidValue))?;
    deserializer
        .end()
        .map_err(|_| Error::InvalidOperation(OperationError::InvalidValue))?;
    Ok(parsed)
}

struct UniqueJson;

impl<'de> DeserializeSeed<'de> for UniqueJson {
    type Value = serde_json::Value;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(UniqueJsonVisitor)
    }
}

struct UniqueJsonVisitor;

impl<'de> Visitor<'de> for UniqueJsonVisitor {
    type Value = serde_json::Value;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a JSON value without duplicate object member names")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
        Ok(value.into())
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> {
        Ok(value.into())
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> {
        Ok(value.into())
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        serde_json::Number::from_f64(value)
            .map(serde_json::Value::Number)
            .ok_or_else(|| E::custom("non-finite JSON number"))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
        Ok(value.into())
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E> {
        Ok(value.into())
    }

    fn visit_none<E>(self) -> Result<Self::Value, E> {
        Ok(serde_json::Value::Null)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E> {
        Ok(serde_json::Value::Null)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        UniqueJson.deserialize(deserializer)
    }

    fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut values = Vec::new();
        while let Some(value) = sequence.next_element_seed(UniqueJson)? {
            values.push(value);
        }
        Ok(serde_json::Value::Array(values))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut values = serde_json::Map::new();
        while let Some(key) = map.next_key::<String>()? {
            if values.contains_key(&key) {
                return Err(de::Error::custom("duplicate JSON object member"));
            }
            values.insert(key, map.next_value_seed(UniqueJson)?);
        }
        Ok(serde_json::Value::Object(values))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn raw_create_from(data: proto::DidCreationData) -> RawPrismOperation {
        let payload = proto::CreateDid {
            did_data: Some(data),
        }
        .encode_to_vec();
        let mut operation = vec![0x0a];
        prost::encoding::encode_varint(payload.len() as u64, &mut operation);
        operation.extend_from_slice(&payload);
        RawPrismOperation::parse(&operation).unwrap()
    }

    fn raw_message(field: u8, payload: &[u8]) -> RawPrismOperation {
        let mut operation = vec![field << 3 | 2];
        prost::encoding::encode_varint(payload.len() as u64, &mut operation);
        operation.extend_from_slice(payload);
        RawPrismOperation::parse(&operation).unwrap()
    }

    fn master(id: &str) -> proto::PublicKey {
        proto::PublicKey {
            id: id.to_owned(),
            usage: proto::KeyUsage::Master as i32,
            key_data: Some(proto::public_key::KeyData::Compressed(
                proto::CompressedEcKeyData {
                    curve: "secp256k1".to_owned(),
                    data: hex::decode(
                        "02e45998e18117113835b972b43784fb6b6470b722cb2dc171c5fba00265bc80ee",
                    )
                    .unwrap(),
                },
            )),
        }
    }

    fn preprod_create() -> RawPrismOperation {
        RawPrismOperation::parse(&hex::decode("0a3f0a3d123b0a076d61737465723010014a2e0a09736563703235366b31122102e45998e18117113835b972b43784fb6b6470b722cb2dc171c5fba00265bc80ee").unwrap()).unwrap()
    }

    #[test]
    fn parses_pinned_preprod_create_operation() {
        let operation = CreateDidOperation::parse(&preprod_create()).unwrap();
        assert_eq!(operation.public_keys().len(), 1);
        assert_eq!(operation.public_keys()[0].id(), "master0");
        assert_eq!(operation.public_keys()[0].usage(), KeyUsage::Master);
        assert!(operation.services().is_empty());
        assert!(operation.contexts().is_empty());
    }

    #[test]
    fn debug_redacts_operation_domain_values() {
        let create = CreateDidOperation::parse(&preprod_create()).unwrap();
        let create_debug = format!("{create:?}");
        assert!(!create_debug.contains("master0"));
        let key_debug = format!("{:?}", create.public_keys()[0]);
        assert!(!key_debug.contains("master0"));
        assert!(!key_debug.contains("e45998"));

        let service = PrismService {
            id: "private-service-id".to_owned(),
            service_type: ServiceType::One("PrivateServiceType".to_owned()),
            endpoint: ServiceEndpoint::One(ServiceEndpointValue::Uri(
                "https://private.example/path".to_owned(),
            )),
        };
        let service_debug = format!("{service:?}");
        for sensitive in [
            "private-service-id",
            "PrivateServiceType",
            "private.example",
        ] {
            assert!(!service_debug.contains(sensitive));
        }

        let action = UpdateAction::UpdateService {
            id: "private-service-id".to_owned(),
            service_type: Some(ServiceType::One("PrivateServiceType".to_owned())),
            endpoint: Some(ServiceEndpoint::One(ServiceEndpointValue::Uri(
                "https://private.example/path".to_owned(),
            ))),
        };
        let action_debug = format!("{action:?}");
        assert!(!action_debug.contains("private-service-id"));
        assert!(!action_debug.contains("private.example"));

        let did_suffix = "44fb82ad3544a8c8acb763c01a6fc7d92d81c38104096319345a4618d1a4275c";
        let update = proto::UpdateDid {
            previous_operation_hash: vec![0xabu8; 32],
            id: did_suffix.to_owned(),
            actions: vec![proto::UpdateDidAction {
                action: Some(proto::update_did_action::Action::PatchContext(
                    proto::PatchContextAction {
                        context: vec!["https://private.example/context".to_owned()],
                    },
                )),
            }],
        };
        let update = UpdateDidOperation::parse(&raw_message(2, &update.encode_to_vec())).unwrap();
        let update_debug = format!("{update:?}");
        assert!(!update_debug.contains(did_suffix));
        assert!(!update_debug.contains("private.example"));
        assert!(!update_debug.contains("abab"));

        let deactivate = proto::DeactivateDid {
            previous_operation_hash: vec![0xcdu8; 32],
            id: did_suffix.to_owned(),
        };
        let deactivate =
            DeactivateDidOperation::parse(&raw_message(6, &deactivate.encode_to_vec())).unwrap();
        let deactivate_debug = format!("{deactivate:?}");
        assert!(!deactivate_debug.contains(did_suffix));
        assert!(!deactivate_debug.contains("cdcd"));
    }

    #[test]
    fn endpoint_rejects_duplicate_names_at_any_depth() {
        assert!(parse_service_endpoint(r#"{"uri":"https://a","uri":"https://b"}"#).is_err());
        assert!(parse_service_endpoint(r#"{"nested":{"x":1,"x":2}}"#).is_err());
    }

    #[test]
    fn contexts_are_normalized_before_duplicate_detection() {
        assert_eq!(
            normalize_uri("HTTPS://EXAMPLE.COM/a/../b").unwrap(),
            "https://example.com/b"
        );
        assert!(ensure_unique(["https://example.com", "https://example.com"]).is_err());
    }

    #[test]
    fn fragment_profile_is_finite_and_percent_aware() {
        assert!(validate_fragment("key-1%20x").is_ok());
        assert!(validate_fragment("key%2").is_err());
        assert!(validate_fragment("#key").is_err());
        assert!(validate_fragment("").is_err());
    }

    #[test]
    fn rejects_duplicate_ids_and_missing_master() {
        let duplicate = raw_create_from(proto::DidCreationData {
            public_keys: vec![master("same"), master("same")],
            services: Vec::new(),
            context: Vec::new(),
        });
        assert_eq!(
            CreateDidOperation::parse(&duplicate),
            Err(Error::InvalidOperation(OperationError::DuplicateValue))
        );

        let mut issuing = master("issuing");
        issuing.usage = proto::KeyUsage::Issuing as i32;
        let no_master = raw_create_from(proto::DidCreationData {
            public_keys: vec![issuing],
            services: Vec::new(),
            context: Vec::new(),
        });
        assert_eq!(
            CreateDidOperation::parse(&no_master),
            Err(Error::InvalidOperation(OperationError::MissingMasterKey))
        );
    }

    #[test]
    fn rejects_normalization_equivalent_contexts() {
        let raw = raw_create_from(proto::DidCreationData {
            public_keys: vec![master("master")],
            services: Vec::new(),
            context: vec![
                "https://EXAMPLE.com/a/../context".to_owned(),
                "https://example.com/context".to_owned(),
            ],
        });
        assert_eq!(
            CreateDidOperation::parse(&raw),
            Err(Error::InvalidOperation(OperationError::DuplicateValue))
        );
    }

    #[test]
    fn master_and_vdr_keys_must_use_secp256k1() {
        let mut invalid = master("master");
        invalid.key_data = Some(proto::public_key::KeyData::Compressed(
            proto::CompressedEcKeyData {
                curve: "Ed25519".to_owned(),
                data: vec![7; 32],
            },
        ));
        let raw = raw_create_from(proto::DidCreationData {
            public_keys: vec![invalid],
            services: Vec::new(),
            context: Vec::new(),
        });
        assert_eq!(
            CreateDidOperation::parse(&raw),
            Err(Error::InvalidOperation(OperationError::UnsupportedKey))
        );
    }

    #[test]
    fn validates_service_type_and_endpoint_profiles() {
        let raw = raw_create_from(proto::DidCreationData {
            public_keys: vec![master("master")],
            services: vec![proto::Service {
                id: "didcomm".to_owned(),
                r#type: r#"["DIDCommMessaging","Profile"]"#.to_owned(),
                service_endpoint:
                    r#"["HTTPS://EXAMPLE.COM/a/../didcomm",{"accept":["didcomm/v2"]}]"#.to_owned(),
            }],
            context: Vec::new(),
        });
        let operation = CreateDidOperation::parse(&raw).unwrap();
        assert_eq!(operation.services()[0].id(), "didcomm");
        assert_eq!(
            operation.services()[0].service_type(),
            &ServiceType::Many(vec!["DIDCommMessaging".to_owned(), "Profile".to_owned()])
        );
        let ServiceEndpoint::Many(values) = operation.services()[0].endpoint() else {
            panic!("expected multiple service endpoints");
        };
        assert_eq!(
            values[0],
            ServiceEndpointValue::Uri("https://example.com/didcomm".to_owned())
        );
    }

    #[test]
    fn update_preserves_action_order_and_rejects_missing_oneof() {
        let did_suffix = "44fb82ad3544a8c8acb763c01a6fc7d92d81c38104096319345a4618d1a4275c";
        let wire = proto::UpdateDid {
            previous_operation_hash: vec![3; 32],
            id: did_suffix.to_owned(),
            actions: vec![
                proto::UpdateDidAction {
                    action: Some(proto::update_did_action::Action::RemoveKey(
                        proto::RemoveKeyAction {
                            key_id: "old".to_owned(),
                        },
                    )),
                },
                proto::UpdateDidAction {
                    action: Some(proto::update_did_action::Action::PatchContext(
                        proto::PatchContextAction {
                            context: vec!["HTTPS://EXAMPLE.COM/a/../context".to_owned()],
                        },
                    )),
                },
            ],
        };
        let raw = raw_message(2, &wire.encode_to_vec());
        let operation = UpdateDidOperation::parse(&raw).unwrap();
        assert_eq!(operation.did().suffix(), did_suffix);
        assert!(matches!(operation.actions()[0], UpdateAction::RemoveKey(_)));
        assert_eq!(
            operation.actions()[1],
            UpdateAction::PatchContext(vec!["https://example.com/context".to_owned()])
        );

        let invalid = proto::UpdateDid {
            actions: vec![proto::UpdateDidAction { action: None }],
            ..wire
        };
        assert_eq!(
            UpdateDidOperation::parse(&raw_message(2, &invalid.encode_to_vec())),
            Err(Error::InvalidOperation(OperationError::RequiredDataMissing))
        );
    }

    #[test]
    fn deactivation_requires_canonical_target_and_hash() {
        let valid = proto::DeactivateDid {
            previous_operation_hash: vec![9; 32],
            id: "44fb82ad3544a8c8acb763c01a6fc7d92d81c38104096319345a4618d1a4275c".to_owned(),
        };
        let operation =
            DeactivateDidOperation::parse(&raw_message(6, &valid.encode_to_vec())).unwrap();
        assert_eq!(operation.previous_operation_hash().as_bytes(), &[9; 32]);

        let invalid = proto::DeactivateDid {
            previous_operation_hash: vec![9; 31],
            ..valid
        };
        assert_eq!(
            DeactivateDidOperation::parse(&raw_message(6, &invalid.encode_to_vec())),
            Err(Error::InvalidOperation(OperationError::InvalidValue))
        );
    }
}
