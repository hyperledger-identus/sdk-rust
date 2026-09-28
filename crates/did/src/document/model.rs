use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Deserializer, Serialize, de};
use serde_json::Value;

use crate::{
    Did, Error, Uri,
    error::DocumentError,
    json_cleanup::{RejectionGuard, drop_json_values_iteratively},
    wire_json::{JsonWireError, JsonWireLimits, validate_unique_object_names},
};

use super::{
    ContextEntry, DOCUMENT_RESERVED, JsonBudget, MAX_DID_DOCUMENT_BYTES,
    MAX_DID_DOCUMENT_WIRE_DEPTH, MAX_DID_DOCUMENT_WIRE_LIVE_KEY_BYTES, MAX_DID_DOCUMENT_WIRE_NODES,
    MAX_DID_DOCUMENT_WIRE_OBJECT_MEMBERS, OneOrMany, Service, VerificationMethod,
    VerificationRelationship, collect_service_json, collect_verification_method_json, invalid,
    validate_collection, validate_json_map,
};

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
            validate_collection(context.as_slice())?;
            for entry in context.as_slice() {
                if let ContextEntry::Object(object) = entry {
                    validate_json_map(object.as_map(), &[], &mut budget)?;
                }
            }
        }
        if let Some(controller) = &self.controller {
            validate_collection(controller.as_slice())?;
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
        let document = RejectionGuard::new(
            Self {
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
            },
            drop_did_document_json,
        );
        document.owner().validate().map_err(de::Error::custom)?;
        Ok(document.into_owner())
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
        let document = RejectionGuard::new(self.document, drop_did_document_json);
        document.owner().validate()?;
        Ok(document.into_owner())
    }
}

fn drop_did_document_json(document: DidDocument) {
    let DidDocument {
        context,
        id: _,
        controller: _,
        also_known_as: _,
        verification_methods,
        authentication,
        assertion_method,
        key_agreement,
        capability_invocation,
        capability_delegation,
        service,
        extensions,
    } = document;
    let mut roots = Vec::new();

    if let Some(context) = context {
        for entry in context.into_vec() {
            if let ContextEntry::Object(object) = entry {
                roots.extend(object.into_map().into_values());
            }
        }
    }
    if let Some(methods) = verification_methods {
        for method in methods {
            collect_verification_method_json(method, &mut roots);
        }
    }
    for relationships in [
        authentication,
        assertion_method,
        key_agreement,
        capability_invocation,
        capability_delegation,
    ]
    .into_iter()
    .flatten()
    {
        for relationship in relationships {
            if let VerificationRelationship::Embedded(method) = relationship {
                collect_verification_method_json(method, &mut roots);
            }
        }
    }
    if let Some(services) = service {
        for service in services {
            collect_service_json(service, &mut roots);
        }
    }
    roots.extend(extensions.into_values());

    drop_json_values_iteratively(roots);
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
