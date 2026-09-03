//! Bounded, extensible W3C DID document structural model.
//!
//! The model owns syntax, shape and resource invariants. DID method,
//! cryptosuite, resolution, dereferencing and authorization policy belong to
//! adapters above this crate.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use serde_json::Value;

use crate::{
    Did, Error, Uri,
    error::DocumentError,
    wire_json::{JsonWireError, JsonWireLimits, validate_unique_object_names},
};

/// Maximum raw JSON size accepted by [`DidDocument::from_json_slice`].
pub const MAX_DID_DOCUMENT_BYTES: usize = 256 * 1_024;
/// Maximum items in any DID document or extension collection.
pub const MAX_DOCUMENT_ITEMS: usize = 128;
/// Maximum nested levels in an arbitrary JSON extension tree.
pub const MAX_EXTENSION_DEPTH: usize = 32;
/// Maximum JSON nodes across arbitrary data in one DID document.
pub const MAX_EXTENSION_NODES: usize = 4_096;
/// Maximum containers nested in raw DID document JSON during preflight.
pub const MAX_DID_DOCUMENT_WIRE_DEPTH: usize = 64;
/// Maximum JSON values visited during raw DID document preflight.
pub const MAX_DID_DOCUMENT_WIRE_NODES: usize = 16_384;
/// Maximum members permitted in one raw DID document JSON object.
pub const MAX_DID_DOCUMENT_WIRE_OBJECT_MEMBERS: usize = 128;
/// Maximum decoded object-name bytes retained simultaneously during preflight.
pub const MAX_DID_DOCUMENT_WIRE_LIVE_KEY_BYTES: usize = 128 * 1_024;

pub(crate) const MAX_EXTENSION_PROPERTIES: usize = 64;
pub(crate) const MAX_PROPERTY_NAME_BYTES: usize = 256;
pub(crate) const MAX_EXTENSION_STRING_BYTES: usize = 64 * 1_024;
const MAX_OPEN_TYPE_BYTES: usize = 256;
const MAX_MULTIBASE_BYTES: usize = 16 * 1_024;

const DOCUMENT_RESERVED: &[&str] = &[
    "@context",
    "id",
    "controller",
    "alsoKnownAs",
    "verificationMethod",
    "authentication",
    "assertionMethod",
    "keyAgreement",
    "capabilityInvocation",
    "capabilityDelegation",
    "service",
];
const VERIFICATION_RESERVED: &[&str] = &["id", "type", "controller"];
const SERVICE_RESERVED: &[&str] = &["id", "type", "serviceEndpoint"];
const PRIVATE_JWK_MEMBERS: &[&str] = &["d", "p", "q", "dp", "dq", "qi", "oth", "k"];

/// A non-empty value that preserves whether its JSON form was one value or an
/// array.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OneOrMany<T> {
    values: Vec<T>,
    is_many: bool,
}

impl<T> OneOrMany<T> {
    /// Construct the scalar representation.
    #[must_use]
    pub fn one(value: T) -> Self {
        Self {
            values: vec![value],
            is_many: false,
        }
    }

    /// Construct the array representation, rejecting an empty array.
    pub fn try_many(values: Vec<T>) -> Result<Self, Error> {
        if values.is_empty() {
            return Err(invalid(DocumentError::EmptyValue));
        }
        if values.len() > MAX_DOCUMENT_ITEMS {
            return Err(invalid(DocumentError::TooManyItems));
        }
        Ok(Self {
            values,
            is_many: true,
        })
    }

    /// Borrow the normalized non-empty slice.
    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        &self.values
    }

    /// Return `true` when the preserved JSON representation is an array.
    #[must_use]
    pub const fn is_many(&self) -> bool {
        self.is_many
    }

    /// Consume the value and return its normalized items.
    #[must_use]
    pub fn into_vec(self) -> Vec<T> {
        self.values
    }
}

impl<T> From<T> for OneOrMany<T> {
    fn from(value: T) -> Self {
        Self::one(value)
    }
}

impl<T: Serialize> Serialize for OneOrMany<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.is_many {
            self.values.serialize(serializer)
        } else {
            self.values[0].serialize(serializer)
        }
    }
}

impl<'de, T> Deserialize<'de> for OneOrMany<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Wire<T> {
            One(T),
            Many(Vec<T>),
        }

        match Wire::deserialize(deserializer)? {
            Wire::One(value) => Ok(Self::one(value)),
            Wire::Many(values) => Self::try_many(values).map_err(de::Error::custom),
        }
    }
}

/// One JSON-LD context entry retained by the representation-neutral model.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContextEntry {
    /// A context URI.
    Uri(Uri),
    /// An inline JSON-LD context definition.
    Object(BTreeMap<String, Value>),
}

/// A verification method used directly inside a relationship or by reference.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VerificationRelationship {
    /// An RFC 3986 reference to a verification method resource.
    Reference(Uri),
    /// A verification method scoped to this relationship.
    Embedded(VerificationMethod),
}

/// One value in a mixed service endpoint set.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServiceEndpointValue {
    /// A URI endpoint.
    Uri(Uri),
    /// A service-type-defined endpoint map.
    Map(BTreeMap<String, Value>),
}

/// A W3C DID Core service endpoint representation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum ServiceEndpoint {
    /// A single URI endpoint.
    Uri(Uri),
    /// A single service-type-defined endpoint map.
    Map(BTreeMap<String, Value>),
    /// A non-empty mixed set of URI and map endpoints.
    Set(Vec<ServiceEndpointValue>),
}

impl<'de> Deserialize<'de> for ServiceEndpoint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Wire {
            Uri(Uri),
            Map(BTreeMap<String, Value>),
            Set(Vec<ServiceEndpointValue>),
        }

        let endpoint = match Wire::deserialize(deserializer)? {
            Wire::Uri(uri) => Self::Uri(uri),
            Wire::Map(map) => Self::Map(map),
            Wire::Set(values) => Self::Set(values),
        };
        endpoint
            .validate(&mut JsonBudget::default())
            .map_err(de::Error::custom)?;
        Ok(endpoint)
    }
}

impl ServiceEndpoint {
    fn validate(&self, budget: &mut JsonBudget) -> Result<(), Error> {
        match self {
            Self::Uri(_) => Ok(()),
            Self::Map(map) => validate_json_map(map, &[], budget),
            Self::Set(values) => {
                validate_collection(values)?;
                for (index, value) in values.iter().enumerate() {
                    if values[..index].contains(value) {
                        return Err(invalid(DocumentError::DuplicateSetMember));
                    }
                    if let ServiceEndpointValue::Map(map) = value {
                        validate_json_map(map, &[], budget)?;
                    }
                }
                Ok(())
            }
        }
    }
}

/// An extensible W3C verification method with public material only.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct VerificationMethod {
    id: Uri,
    #[serde(rename = "type")]
    type_: String,
    controller: Did,
    #[serde(flatten)]
    properties: BTreeMap<String, Value>,
}

impl VerificationMethod {
    /// Construct and validate a verification method.
    pub fn new(
        id: Uri,
        type_: String,
        controller: Did,
        properties: BTreeMap<String, Value>,
    ) -> Result<Self, Error> {
        let method = Self {
            id,
            type_,
            controller,
            properties,
        };
        method.validate(&mut JsonBudget::default())?;
        Ok(method)
    }

    /// Borrow the verification method identifier.
    #[must_use]
    pub const fn id(&self) -> &Uri {
        &self.id
    }

    /// Borrow the open verification method type name.
    #[must_use]
    pub fn method_type(&self) -> &str {
        &self.type_
    }

    /// Borrow the controlling DID.
    #[must_use]
    pub const fn controller(&self) -> &Did {
        &self.controller
    }

    /// Borrow all suite-defined verification properties.
    #[must_use]
    pub const fn properties(&self) -> &BTreeMap<String, Value> {
        &self.properties
    }

    /// Borrow the recognized public JWK map, when present.
    #[must_use]
    pub fn public_key_jwk(&self) -> Option<&serde_json::Map<String, Value>> {
        self.properties
            .get("publicKeyJwk")
            .and_then(Value::as_object)
    }

    /// Borrow the recognized multibase public key string, when present.
    #[must_use]
    pub fn public_key_multibase(&self) -> Option<&str> {
        self.properties
            .get("publicKeyMultibase")
            .and_then(Value::as_str)
    }

    fn validate(&self, budget: &mut JsonBudget) -> Result<(), Error> {
        validate_open_string(&self.type_, MAX_OPEN_TYPE_BYTES)?;
        if self.properties.is_empty() {
            return Err(invalid(DocumentError::EmptyValue));
        }
        validate_json_map(&self.properties, VERIFICATION_RESERVED, budget)?;

        let jwk = self.properties.get("publicKeyJwk");
        let multibase = self.properties.get("publicKeyMultibase");
        if jwk.is_some() && multibase.is_some() {
            return Err(invalid(DocumentError::MultipleVerificationMaterial));
        }
        if let Some(jwk) = jwk {
            validate_public_jwk(jwk)?;
        }
        if let Some(multibase) = multibase {
            let Some(multibase) = multibase.as_str() else {
                return Err(invalid(DocumentError::InvalidPropertyShape));
            };
            if multibase.is_empty()
                || multibase.len() > MAX_MULTIBASE_BYTES
                || !multibase.bytes().all(|byte| byte.is_ascii_graphic())
            {
                return Err(invalid(DocumentError::InvalidString));
            }
        }
        Ok(())
    }
}

impl<'de> Deserialize<'de> for VerificationMethod {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            id: Uri,
            #[serde(rename = "type")]
            type_: String,
            controller: Did,
            #[serde(flatten)]
            properties: BTreeMap<String, Value>,
        }

        let wire = Wire::deserialize(deserializer)?;
        Self::new(wire.id, wire.type_, wire.controller, wire.properties).map_err(de::Error::custom)
    }
}

/// An extensible W3C DID service.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Service {
    id: Uri,
    #[serde(rename = "type")]
    type_: OneOrMany<String>,
    #[serde(rename = "serviceEndpoint")]
    endpoint: ServiceEndpoint,
    #[serde(flatten)]
    extensions: BTreeMap<String, Value>,
}

impl Service {
    /// Construct and validate a service.
    pub fn new(
        id: Uri,
        type_: OneOrMany<String>,
        endpoint: ServiceEndpoint,
        extensions: BTreeMap<String, Value>,
    ) -> Result<Self, Error> {
        let service = Self {
            id,
            type_,
            endpoint,
            extensions,
        };
        service.validate(&mut JsonBudget::default())?;
        Ok(service)
    }

    /// Borrow the service identifier.
    #[must_use]
    pub const fn id(&self) -> &Uri {
        &self.id
    }

    /// Borrow service type names while retaining their wire cardinality.
    #[must_use]
    pub const fn service_types(&self) -> &OneOrMany<String> {
        &self.type_
    }

    /// Borrow the service endpoint representation.
    #[must_use]
    pub const fn endpoint(&self) -> &ServiceEndpoint {
        &self.endpoint
    }

    /// Borrow service extension entries.
    #[must_use]
    pub const fn extensions(&self) -> &BTreeMap<String, Value> {
        &self.extensions
    }

    fn validate(&self, budget: &mut JsonBudget) -> Result<(), Error> {
        validate_open_set(self.type_.as_slice())?;
        self.endpoint.validate(budget)?;
        validate_json_map(&self.extensions, SERVICE_RESERVED, budget)
    }
}

impl<'de> Deserialize<'de> for Service {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            id: Uri,
            #[serde(rename = "type")]
            type_: OneOrMany<String>,
            #[serde(rename = "serviceEndpoint")]
            endpoint: ServiceEndpoint,
            #[serde(flatten)]
            extensions: BTreeMap<String, Value>,
        }

        let wire = Wire::deserialize(deserializer)?;
        Self::new(wire.id, wire.type_, wire.endpoint, wire.extensions).map_err(de::Error::custom)
    }
}

/// An immutable, validated W3C DID document structural model.
///
/// Use [`Self::from_json_slice`] or [`Self::from_json_str`] for untrusted raw
/// JSON. In addition to semantic validation, those entry points enforce the
/// byte ceiling and reject duplicate object names before map materialization.
/// Generic serde deserialization cannot recover duplicates already discarded
/// by an upstream format or map representation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct DidDocument {
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    context: Option<OneOrMany<ContextEntry>>,
    id: Did,
    #[serde(skip_serializing_if = "Option::is_none")]
    controller: Option<OneOrMany<Did>>,
    #[serde(rename = "alsoKnownAs", skip_serializing_if = "Option::is_none")]
    also_known_as: Option<Vec<Uri>>,
    #[serde(rename = "verificationMethod", skip_serializing_if = "Option::is_none")]
    verification_methods: Option<Vec<VerificationMethod>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authentication: Option<Vec<VerificationRelationship>>,
    #[serde(rename = "assertionMethod", skip_serializing_if = "Option::is_none")]
    assertion_method: Option<Vec<VerificationRelationship>>,
    #[serde(rename = "keyAgreement", skip_serializing_if = "Option::is_none")]
    key_agreement: Option<Vec<VerificationRelationship>>,
    #[serde(
        rename = "capabilityInvocation",
        skip_serializing_if = "Option::is_none"
    )]
    capability_invocation: Option<Vec<VerificationRelationship>>,
    #[serde(
        rename = "capabilityDelegation",
        skip_serializing_if = "Option::is_none"
    )]
    capability_delegation: Option<Vec<VerificationRelationship>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<Vec<Service>>,
    #[serde(flatten)]
    extensions: BTreeMap<String, Value>,
}

impl DidDocument {
    /// Begin native construction for a validated DID subject.
    #[must_use]
    pub fn builder(id: Did) -> DidDocumentBuilder {
        DidDocumentBuilder::new(id)
    }

    /// Deserialize and validate a bounded JSON byte slice.
    pub fn from_json_slice(input: &[u8]) -> Result<Self, Error> {
        if input.len() > MAX_DID_DOCUMENT_BYTES {
            return Err(invalid(DocumentError::TooLarge));
        }
        validate_unique_object_names(
            input,
            JsonWireLimits {
                max_depth: MAX_DID_DOCUMENT_WIRE_DEPTH,
                max_nodes: MAX_DID_DOCUMENT_WIRE_NODES,
                max_object_members: MAX_DID_DOCUMENT_WIRE_OBJECT_MEMBERS,
                max_live_key_bytes: MAX_DID_DOCUMENT_WIRE_LIVE_KEY_BYTES,
            },
        )
        .map_err(|reason| invalid(map_wire_error(reason)))?;
        serde_json::from_slice(input).map_err(|_| invalid(DocumentError::MalformedJson))
    }

    /// Deserialize and validate a bounded JSON string.
    pub fn from_json_str(input: &str) -> Result<Self, Error> {
        Self::from_json_slice(input.as_bytes())
    }

    /// Borrow the DID subject.
    #[must_use]
    pub const fn id(&self) -> &Did {
        &self.id
    }

    /// Borrow the optional context representation.
    #[must_use]
    pub const fn context(&self) -> Option<&OneOrMany<ContextEntry>> {
        self.context.as_ref()
    }

    /// Borrow optional controller values.
    #[must_use]
    pub const fn controller(&self) -> Option<&OneOrMany<Did>> {
        self.controller.as_ref()
    }

    /// Borrow aliases, distinguishing absence from a present set.
    #[must_use]
    pub fn also_known_as(&self) -> Option<&[Uri]> {
        self.also_known_as.as_deref()
    }

    /// Borrow top-level verification methods.
    #[must_use]
    pub fn verification_methods(&self) -> Option<&[VerificationMethod]> {
        self.verification_methods.as_deref()
    }

    /// Borrow authentication relationships.
    #[must_use]
    pub fn authentication(&self) -> Option<&[VerificationRelationship]> {
        self.authentication.as_deref()
    }

    /// Borrow assertion relationships.
    #[must_use]
    pub fn assertion_method(&self) -> Option<&[VerificationRelationship]> {
        self.assertion_method.as_deref()
    }

    /// Borrow key-agreement relationships.
    #[must_use]
    pub fn key_agreement(&self) -> Option<&[VerificationRelationship]> {
        self.key_agreement.as_deref()
    }

    /// Borrow capability-invocation relationships.
    #[must_use]
    pub fn capability_invocation(&self) -> Option<&[VerificationRelationship]> {
        self.capability_invocation.as_deref()
    }

    /// Borrow capability-delegation relationships.
    #[must_use]
    pub fn capability_delegation(&self) -> Option<&[VerificationRelationship]> {
        self.capability_delegation.as_deref()
    }

    /// Borrow services.
    #[must_use]
    pub fn services(&self) -> Option<&[Service]> {
        self.service.as_deref()
    }

    /// Clone this document while replacing its service selection.
    ///
    /// The replacement is validated through the same cross-document path as
    /// native construction and deserialization. This is useful for algorithms
    /// that project a subset of an already validated DID document.
    pub fn with_services(&self, services: Option<Vec<Service>>) -> Result<Self, Error> {
        let mut document = self.clone();
        document.service = services;
        document.validate()?;
        Ok(document)
    }

    /// Borrow document extension entries.
    #[must_use]
    pub const fn extensions(&self) -> &BTreeMap<String, Value> {
        &self.extensions
    }

    fn validate(&self) -> Result<(), Error> {
        let mut budget = JsonBudget::default();

        if let Some(context) = &self.context {
            for entry in context.as_slice() {
                if let ContextEntry::Object(map) = entry {
                    validate_json_map(map, &[], &mut budget)?;
                }
            }
        }
        if let Some(controller) = &self.controller {
            validate_unique_by(controller.as_slice(), Did::as_str)?;
        }
        if let Some(aliases) = &self.also_known_as {
            validate_collection(aliases)?;
            validate_unique_by(aliases, Uri::as_str)?;
        }

        let mut method_ids = BTreeSet::new();
        if let Some(methods) = &self.verification_methods {
            validate_collection(methods)?;
            for method in methods {
                if !method_ids.insert(method.id().as_str()) {
                    return Err(invalid(DocumentError::DuplicateVerificationMethod));
                }
                method.validate(&mut budget)?;
            }
        }

        for relationship in [
            self.authentication.as_deref(),
            self.assertion_method.as_deref(),
            self.key_agreement.as_deref(),
            self.capability_invocation.as_deref(),
            self.capability_delegation.as_deref(),
        ]
        .into_iter()
        .flatten()
        {
            validate_relationship(relationship, &mut method_ids, &mut budget)?;
        }

        if let Some(services) = &self.service {
            validate_collection(services)?;
            let mut ids = BTreeSet::new();
            for service in services {
                if !ids.insert(service.id().as_str()) {
                    return Err(invalid(DocumentError::DuplicateService));
                }
                service.validate(&mut budget)?;
            }
        }
        validate_json_map(&self.extensions, DOCUMENT_RESERVED, &mut budget)
    }
}

const fn map_wire_error(reason: JsonWireError) -> DocumentError {
    match reason {
        JsonWireError::DuplicateName => DocumentError::DuplicateJsonProperty,
        JsonWireError::TooDeep => DocumentError::ExtensionTooDeep,
        JsonWireError::TooManyNodes | JsonWireError::TooManyLiveKeyBytes => {
            DocumentError::ExtensionTooLarge
        }
        JsonWireError::TooManyMembers => DocumentError::TooManyProperties,
        JsonWireError::Malformed => DocumentError::MalformedJson,
    }
}

impl<'de> Deserialize<'de> for DidDocument {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            #[serde(rename = "@context")]
            context: Option<OneOrMany<ContextEntry>>,
            id: Did,
            controller: Option<OneOrMany<Did>>,
            #[serde(rename = "alsoKnownAs")]
            also_known_as: Option<Vec<Uri>>,
            #[serde(rename = "verificationMethod")]
            verification_methods: Option<Vec<VerificationMethod>>,
            authentication: Option<Vec<VerificationRelationship>>,
            #[serde(rename = "assertionMethod")]
            assertion_method: Option<Vec<VerificationRelationship>>,
            #[serde(rename = "keyAgreement")]
            key_agreement: Option<Vec<VerificationRelationship>>,
            #[serde(rename = "capabilityInvocation")]
            capability_invocation: Option<Vec<VerificationRelationship>>,
            #[serde(rename = "capabilityDelegation")]
            capability_delegation: Option<Vec<VerificationRelationship>>,
            service: Option<Vec<Service>>,
            #[serde(flatten)]
            extensions: BTreeMap<String, Value>,
        }

        let wire = Wire::deserialize(deserializer)?;
        let document = Self {
            context: wire.context,
            id: wire.id,
            controller: wire.controller,
            also_known_as: wire.also_known_as,
            verification_methods: wire.verification_methods,
            authentication: wire.authentication,
            assertion_method: wire.assertion_method,
            key_agreement: wire.key_agreement,
            capability_invocation: wire.capability_invocation,
            capability_delegation: wire.capability_delegation,
            service: wire.service,
            extensions: wire.extensions,
        };
        document.validate().map_err(de::Error::custom)?;
        Ok(document)
    }
}

/// Native builder that converges on the same validation path as serde.
#[derive(Clone, Debug)]
pub struct DidDocumentBuilder {
    document: DidDocument,
}

impl DidDocumentBuilder {
    /// Start a document for `id` with every optional property absent.
    #[must_use]
    pub fn new(id: Did) -> Self {
        Self {
            document: DidDocument {
                context: None,
                id,
                controller: None,
                also_known_as: None,
                verification_methods: None,
                authentication: None,
                assertion_method: None,
                key_agreement: None,
                capability_invocation: None,
                capability_delegation: None,
                service: None,
                extensions: BTreeMap::new(),
            },
        }
    }

    /// Set the optional representation context.
    #[must_use]
    pub fn context(mut self, context: OneOrMany<ContextEntry>) -> Self {
        self.document.context = Some(context);
        self
    }

    /// Set one or more controllers.
    #[must_use]
    pub fn controller(mut self, controller: OneOrMany<Did>) -> Self {
        self.document.controller = Some(controller);
        self
    }

    /// Set the alias set.
    #[must_use]
    pub fn also_known_as(mut self, aliases: Vec<Uri>) -> Self {
        self.document.also_known_as = Some(aliases);
        self
    }

    /// Set top-level verification methods.
    #[must_use]
    pub fn verification_methods(mut self, methods: Vec<VerificationMethod>) -> Self {
        self.document.verification_methods = Some(methods);
        self
    }

    /// Set authentication relationships.
    #[must_use]
    pub fn authentication(mut self, values: Vec<VerificationRelationship>) -> Self {
        self.document.authentication = Some(values);
        self
    }

    /// Set assertion relationships.
    #[must_use]
    pub fn assertion_method(mut self, values: Vec<VerificationRelationship>) -> Self {
        self.document.assertion_method = Some(values);
        self
    }

    /// Set key-agreement relationships.
    #[must_use]
    pub fn key_agreement(mut self, values: Vec<VerificationRelationship>) -> Self {
        self.document.key_agreement = Some(values);
        self
    }

    /// Set capability-invocation relationships.
    #[must_use]
    pub fn capability_invocation(mut self, values: Vec<VerificationRelationship>) -> Self {
        self.document.capability_invocation = Some(values);
        self
    }

    /// Set capability-delegation relationships.
    #[must_use]
    pub fn capability_delegation(mut self, values: Vec<VerificationRelationship>) -> Self {
        self.document.capability_delegation = Some(values);
        self
    }

    /// Set services.
    #[must_use]
    pub fn services(mut self, services: Vec<Service>) -> Self {
        self.document.service = Some(services);
        self
    }

    /// Set document extension entries.
    #[must_use]
    pub fn extensions(mut self, extensions: BTreeMap<String, Value>) -> Self {
        self.document.extensions = extensions;
        self
    }

    /// Validate all cross-document invariants and return the immutable model.
    pub fn build(self) -> Result<DidDocument, Error> {
        self.document.validate()?;
        Ok(self.document)
    }
}

fn validate_relationship<'a>(
    values: &'a [VerificationRelationship],
    method_ids: &mut BTreeSet<&'a str>,
    budget: &mut JsonBudget,
) -> Result<(), Error> {
    validate_collection(values)?;
    let mut relationship_ids = BTreeSet::new();
    for value in values {
        let id = match value {
            VerificationRelationship::Reference(reference) => reference.as_str(),
            VerificationRelationship::Embedded(method) => {
                method.validate(budget)?;
                if !method_ids.insert(method.id().as_str()) {
                    return Err(invalid(DocumentError::DuplicateVerificationMethod));
                }
                method.id().as_str()
            }
        };
        if !relationship_ids.insert(id) {
            return Err(invalid(DocumentError::DuplicateSetMember));
        }
    }
    Ok(())
}

fn validate_public_jwk(value: &Value) -> Result<(), Error> {
    let Some(jwk) = value.as_object() else {
        return Err(invalid(DocumentError::InvalidPropertyShape));
    };
    if PRIVATE_JWK_MEMBERS
        .iter()
        .any(|member| jwk.contains_key(*member))
    {
        return Err(invalid(DocumentError::PrivateKeyMaterial));
    }
    let Some(key_type) = jwk.get("kty").and_then(Value::as_str) else {
        return Err(invalid(DocumentError::InvalidPropertyShape));
    };
    validate_open_string(key_type, MAX_OPEN_TYPE_BYTES)
}

fn validate_open_set(values: &[String]) -> Result<(), Error> {
    validate_collection(values)?;
    let mut unique = BTreeSet::new();
    for value in values {
        validate_open_string(value, MAX_OPEN_TYPE_BYTES)?;
        if !unique.insert(value.as_str()) {
            return Err(invalid(DocumentError::DuplicateSetMember));
        }
    }
    Ok(())
}

fn validate_open_string(value: &str, max_bytes: usize) -> Result<(), Error> {
    if value.is_empty()
        || value.len() > max_bytes
        || value.chars().any(char::is_control)
        || value.trim() != value
    {
        return Err(invalid(DocumentError::InvalidString));
    }
    Ok(())
}

fn validate_collection<T>(values: &[T]) -> Result<(), Error> {
    if values.is_empty() {
        return Err(invalid(DocumentError::EmptyValue));
    }
    if values.len() > MAX_DOCUMENT_ITEMS {
        return Err(invalid(DocumentError::TooManyItems));
    }
    Ok(())
}

fn validate_unique_by<T, F>(values: &[T], key: F) -> Result<(), Error>
where
    F: Fn(&T) -> &str,
{
    let mut unique = BTreeSet::new();
    for value in values {
        if !unique.insert(key(value)) {
            return Err(invalid(DocumentError::DuplicateSetMember));
        }
    }
    Ok(())
}

#[derive(Default)]
pub(crate) struct JsonBudget {
    nodes: usize,
}

impl JsonBudget {
    fn visit(&mut self) -> Result<(), Error> {
        self.nodes += 1;
        if self.nodes > MAX_EXTENSION_NODES {
            return Err(invalid(DocumentError::ExtensionTooLarge));
        }
        Ok(())
    }
}

pub(crate) fn validate_json_map(
    map: &BTreeMap<String, Value>,
    reserved: &[&str],
    budget: &mut JsonBudget,
) -> Result<(), Error> {
    budget.visit()?;
    if map.len() > MAX_EXTENSION_PROPERTIES {
        return Err(invalid(DocumentError::TooManyProperties));
    }
    for (key, value) in map {
        if key.is_empty() || key.len() > MAX_PROPERTY_NAME_BYTES {
            return Err(invalid(DocumentError::InvalidPropertyName));
        }
        if reserved.contains(&key.as_str()) {
            return Err(invalid(DocumentError::ReservedProperty));
        }
        validate_json_value(value, 1, budget)?;
    }
    Ok(())
}

fn validate_json_object(
    map: &serde_json::Map<String, Value>,
    depth: usize,
    budget: &mut JsonBudget,
) -> Result<(), Error> {
    if map.len() > MAX_EXTENSION_PROPERTIES {
        return Err(invalid(DocumentError::TooManyProperties));
    }
    for (key, value) in map {
        if key.is_empty() || key.len() > MAX_PROPERTY_NAME_BYTES {
            return Err(invalid(DocumentError::InvalidPropertyName));
        }
        validate_json_value(value, depth, budget)?;
    }
    Ok(())
}

pub(crate) fn validate_json_value(
    value: &Value,
    depth: usize,
    budget: &mut JsonBudget,
) -> Result<(), Error> {
    if depth > MAX_EXTENSION_DEPTH {
        return Err(invalid(DocumentError::ExtensionTooDeep));
    }
    budget.visit()?;
    match value {
        Value::String(value) if value.len() > MAX_EXTENSION_STRING_BYTES => {
            Err(invalid(DocumentError::InvalidString))
        }
        Value::Array(values) => {
            if values.len() > MAX_DOCUMENT_ITEMS {
                return Err(invalid(DocumentError::TooManyItems));
            }
            for value in values {
                validate_json_value(value, depth + 1, budget)?;
            }
            Ok(())
        }
        Value::Object(map) => validate_json_object(map, depth + 1, budget),
        _ => Ok(()),
    }
}

const fn invalid(reason: DocumentError) -> Error {
    Error::InvalidDocument(reason)
}
