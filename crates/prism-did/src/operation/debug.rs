use std::fmt;

use super::{
    CreateDidOperation, DeactivateDidOperation, PrismPublicKey, PrismPublicKeyMaterial,
    PrismService, ServiceEndpoint, ServiceEndpointValue, ServiceType, UpdateAction,
    UpdateDidOperation,
};

impl fmt::Debug for PrismPublicKeyMaterial {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Secp256k1(_) => "PrismPublicKeyMaterial::Secp256k1(..)",
            Self::Ed25519(_) => "PrismPublicKeyMaterial::Ed25519(..)",
            Self::X25519(_) => "PrismPublicKeyMaterial::X25519(..)",
        })
    }
}

impl fmt::Debug for PrismPublicKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PrismPublicKey")
            .field("usage", &self.usage)
            .field("material", &self.material)
            .finish_non_exhaustive()
    }
}

impl fmt::Debug for ServiceType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::One(_) => formatter.write_str("ServiceType::One(..)"),
            Self::Many(values) => formatter
                .debug_tuple("ServiceType::Many")
                .field(&values.len())
                .finish(),
        }
    }
}

impl fmt::Debug for ServiceEndpointValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Uri(_) => formatter.write_str("ServiceEndpointValue::Uri(..)"),
            Self::Object(values) => formatter
                .debug_tuple("ServiceEndpointValue::Object")
                .field(&values.len())
                .finish(),
        }
    }
}

impl fmt::Debug for ServiceEndpoint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::One(value) => formatter
                .debug_tuple("ServiceEndpoint::One")
                .field(value)
                .finish(),
            Self::Many(values) => formatter
                .debug_tuple("ServiceEndpoint::Many")
                .field(&values.len())
                .finish(),
        }
    }
}

impl fmt::Debug for PrismService {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PrismService")
            .field("service_type", &self.service_type)
            .field("endpoint", &self.endpoint)
            .finish_non_exhaustive()
    }
}

impl fmt::Debug for CreateDidOperation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CreateDidOperation")
            .field("public_keys", &self.public_keys.len())
            .field("services", &self.services.len())
            .field("contexts", &self.contexts.len())
            .finish()
    }
}

impl fmt::Debug for UpdateAction {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AddKey(key) => formatter
                .debug_tuple("UpdateAction::AddKey")
                .field(key)
                .finish(),
            Self::RemoveKey(_) => formatter.write_str("UpdateAction::RemoveKey(..)"),
            Self::AddService(service) => formatter
                .debug_tuple("UpdateAction::AddService")
                .field(service)
                .finish(),
            Self::RemoveService(_) => formatter.write_str("UpdateAction::RemoveService(..)"),
            Self::UpdateService {
                service_type,
                endpoint,
                ..
            } => formatter
                .debug_struct("UpdateAction::UpdateService")
                .field("replaces_type", &service_type.is_some())
                .field("replaces_endpoint", &endpoint.is_some())
                .finish_non_exhaustive(),
            Self::PatchContext(contexts) => formatter
                .debug_tuple("UpdateAction::PatchContext")
                .field(&contexts.len())
                .finish(),
        }
    }
}

impl fmt::Debug for UpdateDidOperation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("UpdateDidOperation")
            .field("actions", &self.actions.len())
            .finish_non_exhaustive()
    }
}

impl fmt::Debug for DeactivateDidOperation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DeactivateDidOperation")
            .finish_non_exhaustive()
    }
}
