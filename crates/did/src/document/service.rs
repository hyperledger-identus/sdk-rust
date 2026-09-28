use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Deserializer, Serialize, de};
use serde_json::Value;

use crate::{
    Error, Uri,
    error::DocumentError,
    json_cleanup::{RejectionGuard, drop_json_values_iteratively},
};

use super::{
    JsonBudget, MAX_OPEN_TYPE_BYTES, OneOrMany, SERVICE_RESERVED, invalid, validate_collection,
    validate_json_map, validate_open_string,
};

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

        let endpoint = RejectionGuard::new(
            match Wire::deserialize(deserializer)? {
                Wire::Uri(uri) => Self::Uri(uri),
                Wire::Map(map) => Self::Map(map),
                Wire::Set(values) => Self::Set(values),
            },
            drop_service_endpoint_json,
        );
        endpoint
            .owner()
            .validate(&mut JsonBudget::default())
            .map_err(de::Error::custom)?;
        Ok(endpoint.into_owner())
    }
}

impl ServiceEndpoint {
    pub(super) fn validate(&self, budget: &mut JsonBudget) -> Result<(), Error> {
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
        let service = RejectionGuard::new(
            Self {
                id,
                type_,
                endpoint,
                extensions,
            },
            drop_service_json,
        );
        service.owner().validate(&mut JsonBudget::default())?;
        Ok(service.into_owner())
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

    pub(super) fn validate(&self, budget: &mut JsonBudget) -> Result<(), Error> {
        validate_open_set(self.type_.as_slice())?;
        self.endpoint.validate(budget)?;
        validate_json_map(&self.extensions, SERVICE_RESERVED, budget)
    }
}

fn drop_service_json(service: Service) {
    let mut roots = Vec::new();
    collect_service_json(service, &mut roots);
    drop_json_values_iteratively(roots);
}

pub(super) fn collect_service_json(service: Service, roots: &mut Vec<Value>) {
    let Service {
        id: _,
        type_: _,
        endpoint,
        extensions,
    } = service;
    collect_service_endpoint_json(endpoint, roots);
    roots.extend(extensions.into_values());
}

fn drop_service_endpoint_json(endpoint: ServiceEndpoint) {
    let mut roots = Vec::new();
    collect_service_endpoint_json(endpoint, &mut roots);
    drop_json_values_iteratively(roots);
}

fn collect_service_endpoint_json(endpoint: ServiceEndpoint, roots: &mut Vec<Value>) {
    match endpoint {
        ServiceEndpoint::Uri(_) => {}
        ServiceEndpoint::Map(map) => roots.extend(map.into_values()),
        ServiceEndpoint::Set(values) => {
            for value in values {
                if let ServiceEndpointValue::Map(map) = value {
                    roots.extend(map.into_values());
                }
            }
        }
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
