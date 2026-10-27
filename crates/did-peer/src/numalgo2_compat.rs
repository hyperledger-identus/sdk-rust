//! Explicit legacy numalgo-2 compatibility profile.
//!
//! The implementation follows the pinned DIF/Identus encoding. It is kept in
//! a separate module so applications cannot mistake it for the current peer
//! DID profile.

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use identus_did::{
    Did, DidDocument, JsonWireError, JsonWireLimits, MAX_DID_BYTES, MAX_DID_DOCUMENT_BYTES, Uri,
    validate_unique_object_names,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::Error;

const PREFIX: &str = "did:peer:2";
/// Maximum key plus service elements retained by the compatibility profile.
pub const MAX_ELEMENTS: usize = 32;
/// Maximum decoded bytes accepted for one public multicodec key.
pub const MAX_KEY_BYTES: usize = 128;
/// Maximum decoded compact JSON bytes accepted for one service.
pub const MAX_SERVICE_BYTES: usize = 16 * 1_024;
const MAX_SERVICE_JSON_NODES: usize = 4_096;
/// Maximum endpoint maps retained by one compact service.
pub const MAX_SERVICE_ENDPOINTS: usize = 32;
/// Maximum total characters accepted by the compatibility DID parser.
pub const MAX_COMPAT_DID_BYTES: usize = MAX_DID_BYTES;

/// Verification relationship encoded by one numalgo-2 key element.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Purpose {
    /// Authentication / verification (`V`).
    Verification,
    /// Key agreement / encryption (`E`).
    Encryption,
    /// Assertion method (`A`).
    Assertion,
    /// Capability invocation (`I`).
    CapabilityInvocation,
    /// Capability delegation (`D`).
    CapabilityDelegation,
}

impl Purpose {
    const fn code(self) -> char {
        match self {
            Self::Verification => 'V',
            Self::Encryption => 'E',
            Self::Assertion => 'A',
            Self::CapabilityInvocation => 'I',
            Self::CapabilityDelegation => 'D',
        }
    }
}

/// Caller-owned public multicodec key material.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PublicKey {
    purpose: Purpose,
    multibase: String,
}

impl PublicKey {
    /// Validate a public base58btc multicodec key.
    pub fn new(purpose: Purpose, multibase: impl Into<String>) -> Result<Self, Error> {
        let multibase = multibase.into();
        let (_, key_kind) = validate_key(&multibase)?;
        let compatible = matches!(
            (purpose, key_kind),
            (Purpose::Encryption, "X25519" | "secp256k1")
                | (
                    Purpose::Verification
                        | Purpose::Assertion
                        | Purpose::CapabilityInvocation
                        | Purpose::CapabilityDelegation,
                    "Ed25519" | "secp256k1"
                )
        );
        if !compatible {
            return Err(Error::UnsupportedPurpose);
        }
        Ok(Self { purpose, multibase })
    }
}

/// Compact DIDComm service input used by the Identus compatibility profile.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Service {
    /// Service type (`DIDCommMessaging` in normal Identus use).
    #[serde(rename = "t")]
    pub type_: String,
    /// Service endpoint value, using the pinned compact shape.
    #[serde(rename = "s")]
    pub endpoint: ServiceEndpoint,
    /// Optional relative fragment identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Compact numalgo-2 service endpoint.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServiceEndpoint {
    /// One URI endpoint.
    Uri(String),
    /// One DIDComm endpoint map.
    Map(ServiceEndpointMap),
    /// Multiple DIDComm endpoint maps accepted by the compatibility profile.
    Maps(Vec<ServiceEndpointMap>),
}

/// Compact DIDComm endpoint fields, serialized in the pinned `uri`, `a`, `r`
/// order used by existing Identus consumers.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServiceEndpointMap {
    /// Endpoint URI.
    pub uri: String,
    /// Accepted message profiles.
    #[serde(rename = "a", default, skip_serializing_if = "Vec::is_empty")]
    pub accept: Vec<String>,
    /// Routing key identifiers.
    #[serde(rename = "r", default, skip_serializing_if = "Vec::is_empty")]
    pub routing_keys: Vec<String>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum EncodedServices {
    One(Service),
    Many(Vec<Service>),
}

/// A parsed numalgo-2 compatibility DID.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PeerDid {
    did: Did,
    keys: Vec<PublicKey>,
    services: Vec<Service>,
}

impl PeerDid {
    /// Create a compatibility DID from caller-owned public material.
    pub fn create(keys: Vec<PublicKey>, services: Vec<Service>) -> Result<Self, Error> {
        validate_counts(keys.len(), services.len())?;
        if keys.is_empty() {
            return Err(Error::InvalidInputDocument);
        }
        let mut value = String::from(PREFIX);
        for key in &keys {
            value.push('.');
            value.push(key.purpose.code());
            value.push_str(&key.multibase);
        }
        for service in &services {
            validate_service(service)?;
        }
        if !services.is_empty() {
            let json = if services.len() == 1 {
                serde_json::to_vec(&services[0])
            } else {
                serde_json::to_vec(&services)
            }
            .map_err(|_| Error::InvalidInputDocument)?;
            if json.len() > MAX_SERVICE_BYTES {
                return Err(Error::ResourceLimit);
            }
            validate_service_json(&json)?;
            let encoded_len = base64::encoded_len(json.len(), false).ok_or(Error::ResourceLimit)?;
            let next_len = value
                .len()
                .checked_add(2)
                .and_then(|length| length.checked_add(encoded_len))
                .ok_or(Error::ResourceLimit)?;
            if next_len > MAX_COMPAT_DID_BYTES {
                return Err(Error::ResourceLimit);
            }
            value.push_str(".S");
            value.push_str(&URL_SAFE_NO_PAD.encode(json));
        }
        let did = Did::parse(&value).map_err(|_| Error::ResourceLimit)?;
        let peer = Self {
            did,
            keys,
            services,
        };
        peer.resolve()?;
        Ok(peer)
    }

    /// Parse an explicit numalgo-2 compatibility DID.
    pub fn parse(value: &str) -> Result<Self, Error> {
        if value.len() > MAX_COMPAT_DID_BYTES {
            return Err(Error::ResourceLimit);
        }
        if !value.starts_with(PREFIX) {
            return if value.starts_with("did:peer:") {
                Err(Error::UnsupportedAlgorithm)
            } else {
                Err(Error::WrongProfile)
            };
        }
        let suffix = value.strip_prefix(PREFIX).ok_or(Error::WrongProfile)?;
        if !suffix.is_empty() && !suffix.starts_with('.') {
            return Err(Error::UnsupportedAlgorithm);
        }
        if !suffix.starts_with('.') {
            return Err(Error::MalformedEncoding);
        }
        let mut keys = Vec::new();
        let mut services = Vec::new();
        for element in suffix[1..].split('.') {
            if element.len() < 2 {
                return Err(Error::MalformedEncoding);
            }
            if keys.len() + services.len() >= MAX_ELEMENTS {
                return Err(Error::ResourceLimit);
            }
            let Some((&code, payload)) = element.as_bytes().split_first() else {
                return Err(Error::MalformedEncoding);
            };
            if !code.is_ascii() {
                return Err(Error::UnsupportedPurpose);
            }
            let payload = std::str::from_utf8(payload).map_err(|_| Error::MalformedEncoding)?;
            match code {
                b'V' | b'E' | b'A' | b'I' | b'D' => {
                    let purpose = match code {
                        b'V' => Purpose::Verification,
                        b'E' => Purpose::Encryption,
                        b'A' => Purpose::Assertion,
                        b'I' => Purpose::CapabilityInvocation,
                        b'D' => Purpose::CapabilityDelegation,
                        _ => return Err(Error::UnsupportedPurpose),
                    };
                    keys.push(PublicKey::new(purpose, payload)?);
                }
                b'S' => {
                    if payload.len() > MAX_SERVICE_BYTES.saturating_mul(2) {
                        return Err(Error::ResourceLimit);
                    }
                    let bytes = URL_SAFE_NO_PAD
                        .decode(payload)
                        .map_err(|_| Error::MalformedEncoding)?;
                    if bytes.len() > MAX_SERVICE_BYTES {
                        return Err(Error::ResourceLimit);
                    }
                    validate_service_json(&bytes)?;
                    let decoded: EncodedServices =
                        serde_json::from_slice(&bytes).map_err(|_| Error::InvalidInputDocument)?;
                    let decoded = match decoded {
                        EncodedServices::One(service) => vec![service],
                        EncodedServices::Many(services) if services.is_empty() => {
                            return Err(Error::InvalidInputDocument);
                        }
                        EncodedServices::Many(services) => services,
                    };
                    if keys
                        .len()
                        .saturating_add(services.len())
                        .saturating_add(decoded.len())
                        > MAX_ELEMENTS
                    {
                        return Err(Error::ResourceLimit);
                    }
                    for service in &decoded {
                        validate_service(service)?;
                    }
                    services.extend(decoded);
                }
                _ => return Err(Error::UnsupportedPurpose),
            }
        }
        validate_counts(keys.len(), services.len())?;
        if keys.is_empty() {
            return Err(Error::InvalidInputDocument);
        }
        let did = Did::parse(value).map_err(|_| Error::MalformedEncoding)?;
        let peer = Self {
            did,
            keys,
            services,
        };
        peer.resolve()?;
        Ok(peer)
    }

    /// Borrow the compatibility DID.
    #[must_use]
    pub const fn did(&self) -> &Did {
        &self.did
    }

    /// Resolve locally into the generic DID document model.
    pub fn resolve(&self) -> Result<DidDocument, Error> {
        self.preflight_projection()?;
        let mut verification_methods = Vec::with_capacity(self.keys.len());
        let mut authentication = Vec::new();
        let mut assertion_method = Vec::new();
        let mut key_agreement = Vec::new();
        let mut capability_invocation = Vec::new();
        let mut capability_delegation = Vec::new();

        for (index, key) in self.keys.iter().enumerate() {
            let (method_type, _) = validate_key(&key.multibase)?;
            let id = format!("{}#key-{}", self.did, index + 1);
            verification_methods.push(json!({
                "id": id,
                "type": method_type,
                "controller": self.did,
                "publicKeyMultibase": key.multibase,
            }));
            let reference = Value::String(id.clone());
            match key.purpose {
                Purpose::Verification => authentication.push(reference),
                Purpose::Encryption => key_agreement.push(reference),
                Purpose::Assertion => assertion_method.push(reference),
                Purpose::CapabilityInvocation => capability_invocation.push(reference),
                Purpose::CapabilityDelegation => capability_delegation.push(reference),
            }
        }

        let mut document = serde_json::Map::new();
        document.insert("id".into(), Value::String(self.did.to_string()));
        document.insert(
            "verificationMethod".into(),
            Value::Array(verification_methods),
        );
        insert_nonempty(&mut document, "authentication", authentication);
        insert_nonempty(&mut document, "assertionMethod", assertion_method);
        insert_nonempty(&mut document, "keyAgreement", key_agreement);
        insert_nonempty(&mut document, "capabilityInvocation", capability_invocation);
        insert_nonempty(&mut document, "capabilityDelegation", capability_delegation);

        let mut projected_services = Vec::with_capacity(self.services.len());
        for (index, service) in self.services.iter().enumerate() {
            let endpoint = expand_service_endpoint(&service.endpoint);
            let fragment = match &service.id {
                Some(id) if id.starts_with('#') && id.len() > 1 => id.clone(),
                Some(_) => return Err(Error::InvalidInputDocument),
                None if index == 0 => "#service".to_owned(),
                None => format!("#service-{index}"),
            };
            projected_services.push(json!({
                "id": format!("{}{fragment}", self.did),
                "type": if service.type_ == "dm" { "DIDCommMessaging" } else { service.type_.as_str() },
                "serviceEndpoint": endpoint,
            }));
        }
        insert_nonempty(&mut document, "service", projected_services);
        let bytes = serde_json::to_vec(&Value::Object(document)).map_err(|_| Error::Projection)?;
        DidDocument::from_json_slice(&bytes).map_err(|_| Error::Projection)
    }

    fn preflight_projection(&self) -> Result<(), Error> {
        let did_repetitions = self.keys.iter().try_fold(1_usize, |count, _| {
            count.checked_add(3).ok_or(Error::ResourceLimit)
        })?;
        let did_repetitions = did_repetitions
            .checked_add(self.services.len())
            .ok_or(Error::ResourceLimit)?;
        let mut projected_bytes = self
            .did
            .as_str()
            .len()
            .checked_mul(did_repetitions)
            .and_then(|bytes| {
                bytes.checked_add(
                    2_048 + (self.keys.len() + self.services.len()).saturating_mul(512),
                )
            })
            .ok_or(Error::ResourceLimit)?;
        for key in &self.keys {
            projected_bytes = projected_bytes
                .checked_add(key.multibase.len())
                .ok_or(Error::ResourceLimit)?;
        }
        for service in &self.services {
            projected_bytes = projected_bytes
                .checked_add(json_string_encoded_len(&service.type_))
                .and_then(|bytes| {
                    bytes.checked_add(service.id.as_deref().map_or(0, json_string_encoded_len))
                })
                .ok_or(Error::ResourceLimit)?;
            match &service.endpoint {
                ServiceEndpoint::Uri(uri) => {
                    projected_bytes = projected_bytes
                        .checked_add(json_string_encoded_len(uri))
                        .ok_or(Error::ResourceLimit)?;
                }
                ServiceEndpoint::Map(endpoint) => {
                    projected_bytes = projected_bytes
                        .checked_add(projected_endpoint_map_bytes(endpoint)?)
                        .ok_or(Error::ResourceLimit)?;
                }
                ServiceEndpoint::Maps(endpoints) => {
                    for endpoint in endpoints {
                        projected_bytes = projected_bytes
                            .checked_add(projected_endpoint_map_bytes(endpoint)?)
                            .ok_or(Error::ResourceLimit)?;
                    }
                }
            }
        }
        if projected_bytes > MAX_DID_DOCUMENT_BYTES {
            return Err(Error::ResourceLimit);
        }
        Ok(())
    }
}

fn json_string_encoded_len(value: &str) -> usize {
    value.chars().fold(2_usize, |length, character| {
        length.saturating_add(match character {
            '"' | '\\' | '\u{0008}' | '\t' | '\n' | '\u{000c}' | '\r' => 2,
            '\u{0000}'..='\u{001f}' => 6,
            _ => character.len_utf8(),
        })
    })
}

fn projected_endpoint_map_bytes(endpoint: &ServiceEndpointMap) -> Result<usize, Error> {
    std::iter::once(&endpoint.uri)
        .chain(endpoint.accept.iter())
        .chain(endpoint.routing_keys.iter())
        .try_fold(0_usize, |bytes, value| {
            bytes
                .checked_add(json_string_encoded_len(value))
                .ok_or(Error::ResourceLimit)
        })
}

fn validate_counts(keys: usize, services: usize) -> Result<(), Error> {
    if keys.saturating_add(services) > MAX_ELEMENTS {
        Err(Error::ResourceLimit)
    } else {
        Ok(())
    }
}

fn validate_service_json(input: &[u8]) -> Result<(), Error> {
    validate_unique_object_names(
        input,
        JsonWireLimits {
            max_depth: 8,
            max_nodes: MAX_SERVICE_JSON_NODES,
            max_object_members: 16,
            max_live_key_bytes: 4_096,
        },
    )
    .map_err(map_json_wire_error)
}

fn validate_key(multibase: &str) -> Result<(&'static str, &'static str), Error> {
    let payload = multibase
        .strip_prefix('z')
        .ok_or(Error::MalformedEncoding)?;
    if payload.is_empty() || payload.len() > 256 {
        return Err(Error::ResourceLimit);
    }
    let bytes = bs58::decode(payload)
        .into_vec()
        .map_err(|_| Error::MalformedEncoding)?;
    if bytes.len() > MAX_KEY_BYTES {
        return Err(Error::ResourceLimit);
    }
    match bytes.as_slice() {
        [0xed, 0x01, key @ ..] if key.len() == 32 => Ok(("Multikey", "Ed25519")),
        [0xec, 0x01, key @ ..] if key.len() == 32 => Ok(("Multikey", "X25519")),
        [0xe7, 0x01, key @ ..] if key.len() == 33 => Ok(("Multikey", "secp256k1")),
        _ => Err(Error::UnsupportedCodec),
    }
}

fn validate_service(service: &Service) -> Result<(), Error> {
    if service.type_.is_empty() || service.type_.len() > 128 {
        return Err(Error::InvalidInputDocument);
    }
    let mut retained_string_bytes = service
        .type_
        .len()
        .checked_add(service.id.as_deref().map_or(0, str::len))
        .ok_or(Error::ResourceLimit)?;
    match &service.endpoint {
        ServiceEndpoint::Uri(uri) if uri.is_empty() || uri.len() > 8_192 => {
            return Err(Error::InvalidInputDocument);
        }
        ServiceEndpoint::Uri(uri) => {
            retained_string_bytes = retained_string_bytes
                .checked_add(uri.len())
                .ok_or(Error::ResourceLimit)?;
            Uri::parse(uri).map_err(|_| Error::InvalidInputDocument)?;
        }
        ServiceEndpoint::Map(endpoint) => {
            validate_endpoint_map(endpoint, &mut retained_string_bytes)?;
        }
        ServiceEndpoint::Maps(endpoints) => {
            if endpoints.is_empty() || endpoints.len() > MAX_SERVICE_ENDPOINTS {
                return Err(Error::ResourceLimit);
            }
            for endpoint in endpoints {
                validate_endpoint_map(endpoint, &mut retained_string_bytes)?;
            }
        }
    }
    if retained_string_bytes > MAX_SERVICE_BYTES {
        return Err(Error::ResourceLimit);
    }
    if service
        .id
        .as_deref()
        .is_some_and(|id| !id.starts_with('#') || id.len() < 2 || id.len() > 256)
    {
        return Err(Error::InvalidInputDocument);
    }
    Ok(())
}

fn validate_endpoint_map(
    endpoint: &ServiceEndpointMap,
    retained_string_bytes: &mut usize,
) -> Result<(), Error> {
    if endpoint.uri.is_empty()
        || endpoint.uri.len() > 8_192
        || endpoint.accept.len() > 32
        || endpoint.routing_keys.len() > 32
    {
        return Err(Error::ResourceLimit);
    }
    Uri::parse(&endpoint.uri).map_err(|_| Error::InvalidInputDocument)?;
    for value in std::iter::once(&endpoint.uri)
        .chain(endpoint.accept.iter())
        .chain(endpoint.routing_keys.iter())
    {
        *retained_string_bytes = retained_string_bytes
            .checked_add(value.len())
            .ok_or(Error::ResourceLimit)?;
        if *retained_string_bytes > MAX_SERVICE_BYTES {
            return Err(Error::ResourceLimit);
        }
    }
    Ok(())
}

const fn map_json_wire_error(error: JsonWireError) -> Error {
    match error {
        JsonWireError::DuplicateName | JsonWireError::Malformed => Error::InvalidInputDocument,
        JsonWireError::TooDeep
        | JsonWireError::TooManyNodes
        | JsonWireError::TooManyMembers
        | JsonWireError::TooManyLiveKeyBytes => Error::ResourceLimit,
        _ => Error::InvalidInputDocument,
    }
}

fn expand_service_endpoint(value: &ServiceEndpoint) -> Value {
    match value {
        ServiceEndpoint::Uri(uri) => Value::String(uri.clone()),
        ServiceEndpoint::Map(endpoint) => json!({
            "uri": endpoint.uri,
            "accept": endpoint.accept,
            "routingKeys": endpoint.routing_keys,
        }),
        ServiceEndpoint::Maps(endpoints) => Value::Array(
            endpoints
                .iter()
                .map(|endpoint| {
                    json!({
                        "uri": endpoint.uri,
                        "accept": endpoint.accept,
                        "routingKeys": endpoint.routing_keys,
                    })
                })
                .collect(),
        ),
    }
}

fn insert_nonempty(map: &mut serde_json::Map<String, Value>, name: &str, values: Vec<Value>) {
    if !values.is_empty() {
        map.insert(name.to_owned(), Value::Array(values));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VERIFICATION_KEY: &str = "z6MkiToqovww7vYtxm1xNM15u9JzqzUFZ1k7s7MazYJUyAxv";
    const AGREEMENT_KEY: &str = "z6LSj72tK8brWgZja8NLRwfiAcBXQWZJL6n1AP1zFjvVUtqN";

    #[test]
    fn creates_parses_and_resolves_compatibility_profile() {
        let keys = vec![
            PublicKey::new(Purpose::Verification, VERIFICATION_KEY).expect("key"),
            PublicKey::new(Purpose::Encryption, AGREEMENT_KEY).expect("key"),
        ];
        let service = Service {
            type_: "DIDCommMessaging".into(),
            endpoint: ServiceEndpoint::Map(ServiceEndpointMap {
                uri: "https://example.com/didcomm".into(),
                accept: vec!["didcomm/v2".into()],
                routing_keys: vec![],
            }),
            id: None,
        };
        let peer = PeerDid::create(keys, vec![service]).expect("create");
        let parsed = PeerDid::parse(peer.did().as_str()).expect("parse");
        assert_eq!(parsed, peer);
        let document = parsed.resolve().expect("resolve");
        assert_eq!(document.id(), peer.did());
        assert_eq!(document.verification_methods().expect("methods").len(), 2);
        assert_eq!(document.authentication().expect("authentication").len(), 1);
        assert!(document.assertion_method().is_none());
    }

    #[test]
    fn creates_parses_and_resolves_multiple_endpoint_maps() {
        let key = PublicKey::new(Purpose::Verification, VERIFICATION_KEY).expect("key");
        let endpoint = |uri: &str| ServiceEndpointMap {
            uri: uri.into(),
            accept: vec!["didcomm/v2".into()],
            routing_keys: vec![],
        };
        let service = Service {
            type_: "DIDCommMessaging".into(),
            endpoint: ServiceEndpoint::Maps(vec![
                endpoint("https://one.example/didcomm"),
                endpoint("https://two.example/didcomm"),
            ]),
            id: None,
        };
        let peer = PeerDid::create(vec![key], vec![service]).expect("create");
        let parsed = PeerDid::parse(peer.did().as_str()).expect("parse");
        assert_eq!(parsed, peer);
        assert_eq!(
            parsed.resolve().expect("resolve").services().unwrap().len(),
            1
        );
    }

    #[test]
    fn accepts_omitted_endpoint_options_and_preserves_the_omission() {
        let compact =
            URL_SAFE_NO_PAD.encode(br#"{"t":"dm","s":{"uri":"https://example.com/didcomm"}}"#);
        let encoded = format!("{PREFIX}.V{VERIFICATION_KEY}.S{compact}");
        let parsed = PeerDid::parse(&encoded).expect("parse omitted options");
        let recreated = PeerDid::create(parsed.keys.clone(), parsed.services.clone())
            .expect("recreate omitted options");
        assert_eq!(recreated.did().as_str(), encoded);
    }

    #[test]
    fn creates_and_parses_multiple_services_in_one_compact_element() {
        let key = PublicKey::new(Purpose::Verification, VERIFICATION_KEY).expect("key");
        let service = |type_: &str, uri: &str| Service {
            type_: type_.into(),
            endpoint: ServiceEndpoint::Map(ServiceEndpointMap {
                uri: uri.into(),
                accept: vec![],
                routing_keys: vec![],
            }),
            id: None,
        };
        let peer = PeerDid::create(
            vec![key],
            vec![
                service("DIDCommMessaging", "https://one.example/didcomm"),
                service("LinkedDomains", "https://two.example/domain"),
            ],
        )
        .expect("create service array");
        assert_eq!(peer.did().as_str().matches(".S").count(), 1);
        let parsed = PeerDid::parse(peer.did().as_str()).expect("parse service array");
        assert_eq!(parsed.services.len(), 2);
        assert_eq!(parsed, peer);
    }

    #[test]
    fn rejects_other_profiles_and_unknown_codecs() {
        assert_eq!(
            PeerDid::parse("did:peer:4zQmabc"),
            Err(Error::UnsupportedAlgorithm)
        );
        assert_eq!(
            PeerDid::parse("did:peer:20.Vz6Mkabc"),
            Err(Error::UnsupportedAlgorithm)
        );
        assert_eq!(
            PublicKey::new(Purpose::Verification, "z1111"),
            Err(Error::UnsupportedCodec)
        );
    }

    #[test]
    fn rejects_non_ascii_purpose_without_panicking() {
        assert_eq!(
            PeerDid::parse("did:peer:2.é"),
            Err(Error::UnsupportedPurpose)
        );
    }

    #[test]
    fn truncated_elements_are_malformed_not_resource_exhaustion() {
        assert_eq!(PeerDid::parse("did:peer:2."), Err(Error::MalformedEncoding));
        assert_eq!(
            PeerDid::parse("did:peer:2.V"),
            Err(Error::MalformedEncoding)
        );
    }

    #[test]
    fn rejects_oversized_raw_did_before_element_scans() {
        let oversized = format!("{PREFIX}.{}", "V".repeat(MAX_COMPAT_DID_BYTES));
        assert_eq!(PeerDid::parse(&oversized), Err(Error::ResourceLimit));
    }

    #[test]
    fn rejects_invalid_service_uri_and_unknown_service_fields() {
        let key = PublicKey::new(Purpose::Verification, VERIFICATION_KEY).expect("key");
        let service = Service {
            type_: "DIDCommMessaging".into(),
            endpoint: ServiceEndpoint::Uri("not a URI".into()),
            id: None,
        };
        assert_eq!(
            PeerDid::create(vec![key], vec![service]),
            Err(Error::InvalidInputDocument)
        );

        let secret_service = URL_SAFE_NO_PAD
            .encode(br#"{"t":"DIDCommMessaging","s":"https://example.com","privateKey":"secret"}"#);
        let value = format!("did:peer:2.V{VERIFICATION_KEY}.S{secret_service}");
        assert_eq!(PeerDid::parse(&value), Err(Error::InvalidInputDocument));
    }

    #[test]
    fn rejects_oversized_service_fields_before_serialization() {
        let key = PublicKey::new(Purpose::Verification, VERIFICATION_KEY).expect("key");
        let service = Service {
            type_: "DIDCommMessaging".into(),
            endpoint: ServiceEndpoint::Map(ServiceEndpointMap {
                uri: "https://example.com/didcomm".into(),
                accept: vec!["a".repeat(MAX_SERVICE_BYTES)],
                routing_keys: vec![],
            }),
            id: None,
        };
        assert_eq!(
            PeerDid::create(vec![key], vec![service]),
            Err(Error::ResourceLimit)
        );
    }

    #[test]
    fn rejects_aggregate_projection_growth_before_building_document() {
        let key = PublicKey::new(Purpose::Verification, VERIFICATION_KEY).expect("key");
        let services = (0..10)
            .map(|_| Service {
                type_: "DIDCommMessaging".into(),
                endpoint: ServiceEndpoint::Map(ServiceEndpointMap {
                    uri: "https://example.com/didcomm".into(),
                    accept: vec!["a".repeat(8_000)],
                    routing_keys: vec![],
                }),
                id: None,
            })
            .collect();
        assert_eq!(
            PeerDid::create(vec![key], services),
            Err(Error::ResourceLimit)
        );
    }

    #[test]
    fn created_service_array_stays_within_the_parser_work_budget() {
        let key = PublicKey::new(Purpose::Verification, VERIFICATION_KEY).expect("key");
        let services = (0..26)
            .map(|index| Service {
                type_: "DIDCommMessaging".into(),
                endpoint: ServiceEndpoint::Map(ServiceEndpointMap {
                    uri: format!("https://example.com/{index}"),
                    accept: vec!["didcomm/v2".into(), "didcomm/aip2;env=rfc587".into()],
                    routing_keys: vec!["did:example:one".into(), "did:example:two".into()],
                }),
                id: None,
            })
            .collect();
        let peer = PeerDid::create(vec![key], services).expect("create bounded service array");
        assert_eq!(
            PeerDid::parse(peer.did().as_str()).expect("parse created DID"),
            peer
        );
    }

    #[test]
    fn rejects_total_did_growth_before_encoding_the_next_service() {
        let key = PublicKey::new(Purpose::Verification, VERIFICATION_KEY).expect("key");
        let services = (0..8)
            .map(|index| Service {
                type_: "DIDCommMessaging".into(),
                endpoint: ServiceEndpoint::Map(ServiceEndpointMap {
                    uri: format!("https://example.com/{index}"),
                    accept: vec!["a".repeat(16_000)],
                    routing_keys: vec![],
                }),
                id: None,
            })
            .collect();
        assert_eq!(
            PeerDid::create(vec![key], services),
            Err(Error::ResourceLimit)
        );
    }

    #[test]
    fn parses_affinidi_numalgo2_compatibility_vector() {
        const VECTOR: &str = "did:peer:2.Vz6MkiToqovww7vYtxm1xNM15u9JzqzUFZ1k7s7MazYJUyAxv.EzQ3shQLqRUza6AMJFbPuMdvFRFWm1wKviQRnQSC1fScovJN4s.SeyJ0IjoiRElEQ29tbU1lc3NhZ2luZyIsInMiOnsidXJpIjoiaHR0cHM6Ly8xMjcuMC4wLjE6NzAzNyIsImEiOlsiZGlkY29tbS92MiJdLCJyIjpbXX19";
        let peer = PeerDid::parse(VECTOR).expect("oracle vector");
        assert_eq!(peer.did().as_str(), VECTOR);
        let recreated =
            PeerDid::create(peer.keys.clone(), peer.services.clone()).expect("recreate");
        assert_eq!(
            PeerDid::parse(recreated.did().as_str()).expect("parse recreated"),
            recreated
        );
        let document = peer.resolve().expect("resolve");
        assert_eq!(document.verification_methods().expect("methods").len(), 2);
        assert_eq!(document.services().expect("services").len(), 1);
    }

    #[test]
    fn property_each_supported_purpose_round_trips() {
        for purpose in [
            Purpose::Verification,
            Purpose::Assertion,
            Purpose::CapabilityInvocation,
            Purpose::CapabilityDelegation,
        ] {
            let peer = PeerDid::create(
                vec![PublicKey::new(purpose, VERIFICATION_KEY).expect("key")],
                vec![],
            )
            .expect("create");
            assert_eq!(PeerDid::parse(peer.did().as_str()).expect("parse"), peer);
            peer.resolve().expect("resolve");
        }
        let peer = PeerDid::create(
            vec![PublicKey::new(Purpose::Encryption, AGREEMENT_KEY).expect("key")],
            vec![],
        )
        .expect("create");
        assert_eq!(PeerDid::parse(peer.did().as_str()).expect("parse"), peer);
        peer.resolve().expect("resolve");
    }
}
