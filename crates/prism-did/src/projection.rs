//! Deterministic projection from active PRISM state to DID Core values.

use std::collections::BTreeMap;

use identus_crypto::EncodeJwk;
use identus_did::{
    ContextEntry, Did, DidDocument, OneOrMany, Service, ServiceEndpoint as DidServiceEndpoint,
    ServiceEndpointValue as DidServiceEndpointValue, Uri, VerificationMethod,
    VerificationRelationship,
};
use serde_json::Value;

use crate::{
    Error, KeyUsage, MAX_OPERATION_BYTES, MAX_PROJECTED_DOCUMENT_BYTES, PrismDid, PrismDidState,
    PrismPublicKey, PrismPublicKeyMaterial, PrismService, ServiceEndpoint, ServiceEndpointValue,
    ServiceType, StateError,
};

const DID_CORE_CONTEXT: &str = "https://www.w3.org/ns/did/v1";

/// Project active PRISM method state into a deterministic DID document.
///
/// Deactivated state has no active DID document and returns `Ok(None)`.
/// Master, revocation, and VDR keys remain protocol state but are never
/// disclosed as DID document verification methods.
pub fn project_did_document(state: &PrismDidState) -> Result<Option<DidDocument>, Error> {
    project_did_document_for(state, state.did().as_did().clone())
}

/// Project active state using the caller-selected equivalent PRISM DID form.
///
/// Local long-form resolution uses the requested long DID consistently for
/// the document controller, verification methods and services while exposing
/// the canonical short form in resolution metadata.
pub fn project_did_document_for(
    state: &PrismDidState,
    controller: Did,
) -> Result<Option<DidDocument>, Error> {
    if state.is_deactivated() {
        return Ok(None);
    }
    let equivalent = PrismDid::parse(controller.as_str())
        .is_ok_and(|candidate| candidate.canonical() == state.did());
    if !equivalent {
        return Err(Error::InvalidState(StateError::InvalidCandidate));
    }
    ensure_projection_budget(state, controller.as_str().len())?;
    let mut methods = Vec::new();
    let mut authentication = Vec::new();
    let mut assertion_method = Vec::new();
    let mut key_agreement = Vec::new();
    let mut capability_invocation = Vec::new();
    let mut capability_delegation = Vec::new();

    for key in state.active_public_keys() {
        let Some(method) = project_key(key, &controller)? else {
            continue;
        };
        let reference = VerificationRelationship::Reference(method.id().clone());
        match key.usage() {
            KeyUsage::Issuing => assertion_method.push(reference),
            KeyUsage::KeyAgreement => key_agreement.push(reference),
            KeyUsage::Authentication => authentication.push(reference),
            KeyUsage::CapabilityInvocation => capability_invocation.push(reference),
            KeyUsage::CapabilityDelegation => capability_delegation.push(reference),
            KeyUsage::Master | KeyUsage::Revocation | KeyUsage::Vdr => unreachable!(),
        }
        methods.push(method);
    }

    let mut contexts = Vec::with_capacity(state.contexts().len() + 1);
    contexts.push(ContextEntry::Uri(parse_uri(DID_CORE_CONTEXT)?));
    contexts.extend(
        state
            .contexts()
            .iter()
            .map(|context| parse_uri(context).map(ContextEntry::Uri))
            .collect::<Result<Vec<_>, _>>()?,
    );
    let services = state
        .active_services()
        .map(|service| project_service(service, controller.as_str()))
        .collect::<Result<Vec<_>, _>>()?;

    let mut builder = DidDocument::builder(controller).context(
        OneOrMany::try_many(contexts)
            .map_err(|_| Error::InvalidState(StateError::InvalidCandidate))?,
    );
    if !methods.is_empty() {
        builder = builder.verification_methods(methods);
    }
    if !authentication.is_empty() {
        builder = builder.authentication(authentication);
    }
    if !assertion_method.is_empty() {
        builder = builder.assertion_method(assertion_method);
    }
    if !key_agreement.is_empty() {
        builder = builder.key_agreement(key_agreement);
    }
    if !capability_invocation.is_empty() {
        builder = builder.capability_invocation(capability_invocation);
    }
    if !capability_delegation.is_empty() {
        builder = builder.capability_delegation(capability_delegation);
    }
    if !services.is_empty() {
        builder = builder.services(services);
    }
    builder
        .build()
        .map(Some)
        .map_err(|_| Error::InvalidState(StateError::InvalidCandidate))
}

fn ensure_projection_budget(state: &PrismDidState, controller_bytes: usize) -> Result<(), Error> {
    let projected_keys = state
        .active_public_keys()
        .filter(|key| {
            !matches!(
                key.usage(),
                KeyUsage::Master | KeyUsage::Revocation | KeyUsage::Vdr
            )
        })
        .count();
    let projected_services = state.active_services().count();
    // The controller occurs once as the document ID, three times per method
    // (ID, controller and relationship reference), and once per service ID.
    // The complete accepted operation budget conservatively covers all other
    // method-owned strings and key material in the native projection.
    let repetitions = projected_keys
        .checked_mul(3)
        .and_then(|value| value.checked_add(projected_services))
        .and_then(|value| value.checked_add(1))
        .ok_or(Error::InvalidState(StateError::InvalidCandidate))?;
    let estimated = controller_bytes
        .checked_mul(repetitions)
        .and_then(|value| value.checked_add(MAX_OPERATION_BYTES))
        .ok_or(Error::InvalidState(StateError::InvalidCandidate))?;
    if estimated > MAX_PROJECTED_DOCUMENT_BYTES {
        return Err(Error::InvalidState(StateError::InvalidCandidate));
    }
    Ok(())
}

fn project_key(
    key: &PrismPublicKey,
    controller: &identus_did::Did,
) -> Result<Option<VerificationMethod>, Error> {
    if matches!(
        key.usage(),
        KeyUsage::Master | KeyUsage::Revocation | KeyUsage::Vdr
    ) {
        return Ok(None);
    }
    let jwk = match key.material() {
        PrismPublicKeyMaterial::Secp256k1(key) => key.encode_jwk(),
        PrismPublicKeyMaterial::Ed25519(key) => key.encode_jwk(),
        PrismPublicKeyMaterial::X25519(key) => key.encode_jwk(),
    };
    let properties = BTreeMap::from([(
        "publicKeyJwk".to_owned(),
        serde_json::to_value(jwk).map_err(|_| Error::InvalidState(StateError::InvalidCandidate))?,
    )]);
    VerificationMethod::new(
        parse_uri(&format!("{}#{}", controller.as_str(), key.id()))?,
        "JsonWebKey2020".to_owned(),
        controller.clone(),
        properties,
    )
    .map(Some)
    .map_err(|_| Error::InvalidState(StateError::InvalidCandidate))
}

fn project_service(service: &PrismService, did: &str) -> Result<Service, Error> {
    let service_type = match service.service_type() {
        ServiceType::One(value) => OneOrMany::one(value.clone()),
        ServiceType::Many(values) => OneOrMany::try_many(values.clone())
            .map_err(|_| Error::InvalidState(StateError::InvalidCandidate))?,
    };
    let endpoint = match service.endpoint() {
        ServiceEndpoint::One(ServiceEndpointValue::Uri(uri)) => {
            DidServiceEndpoint::Uri(parse_uri(uri)?)
        }
        ServiceEndpoint::One(ServiceEndpointValue::Object(map)) => {
            DidServiceEndpoint::Map(json_map_to_btree(map))
        }
        ServiceEndpoint::Many(values) => DidServiceEndpoint::Set(
            values
                .iter()
                .map(|value| match value {
                    ServiceEndpointValue::Uri(uri) => {
                        parse_uri(uri).map(DidServiceEndpointValue::Uri)
                    }
                    ServiceEndpointValue::Object(map) => {
                        Ok(DidServiceEndpointValue::Map(json_map_to_btree(map)))
                    }
                })
                .collect::<Result<Vec<_>, Error>>()?,
        ),
    };
    Service::new(
        parse_uri(&format!("{did}#{}", service.id()))?,
        service_type,
        endpoint,
        BTreeMap::new(),
    )
    .map_err(|_| Error::InvalidState(StateError::InvalidCandidate))
}

fn parse_uri(value: &str) -> Result<Uri, Error> {
    Uri::parse(value).map_err(|_| Error::InvalidState(StateError::InvalidCandidate))
}

fn json_map_to_btree(map: &serde_json::Map<String, Value>) -> BTreeMap<String, Value> {
    map.iter()
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect()
}

#[cfg(test)]
mod tests {
    use identus_crypto::Secp256k1PrivateKey;
    use prost::Message;
    use serde_json::json;

    use super::*;
    use crate::{
        MAX_LONG_FORM_DID_BYTES, MAX_SERVICES, MAX_VERIFICATION_METHODS, RawPrismOperation, proto,
    };

    fn state_with_document_keys() -> PrismDidState {
        let secp = Secp256k1PrivateKey::from_slice(&[1; 32])
            .unwrap()
            .to_public_key()
            .encode_compressed();
        let key = |id: &str, usage: proto::KeyUsage| proto::PublicKey {
            id: id.to_owned(),
            usage: usage as i32,
            key_data: Some(proto::public_key::KeyData::Compressed(
                proto::CompressedEcKeyData {
                    curve: "secp256k1".to_owned(),
                    data: secp.to_vec(),
                },
            )),
        };
        let create = proto::CreateDid {
            did_data: Some(proto::DidCreationData {
                public_keys: vec![
                    key("master", proto::KeyUsage::Master),
                    key("issuing", proto::KeyUsage::Issuing),
                    key("authentication", proto::KeyUsage::Authentication),
                    key("revocation", proto::KeyUsage::Revocation),
                    key("vdr", proto::KeyUsage::Vdr),
                ],
                services: vec![proto::Service {
                    id: "linked".to_owned(),
                    r#type: "LinkedDomains".to_owned(),
                    service_endpoint: "https://example.com/profile".to_owned(),
                }],
                context: vec!["HTTPS://EXAMPLE.COM/a/../context".to_owned()],
            }),
        };
        let payload = create.encode_to_vec();
        let mut bytes = vec![0x0a];
        prost::encoding::encode_varint(payload.len() as u64, &mut bytes);
        bytes.extend_from_slice(&payload);
        PrismDidState::from_initial_state(&RawPrismOperation::parse(&bytes).unwrap()).unwrap()
    }

    #[test]
    fn projection_is_stable_and_omits_protocol_only_keys() {
        let state = state_with_document_keys();
        let first = project_did_document(&state).unwrap().unwrap();
        let second = project_did_document(&state).unwrap().unwrap();
        let json = serde_json::to_value(&first).unwrap();

        assert_eq!(json, serde_json::to_value(&second).unwrap());
        assert_eq!(json["@context"][0], DID_CORE_CONTEXT);
        assert_eq!(json["@context"][1], "https://example.com/context");
        assert_eq!(json["verificationMethod"].as_array().unwrap().len(), 2);
        assert_eq!(json["assertionMethod"].as_array().unwrap().len(), 1);
        assert_eq!(json["authentication"].as_array().unwrap().len(), 1);
        assert_eq!(json["service"][0]["type"], "LinkedDomains");
        assert_eq!(
            json["service"][0]["serviceEndpoint"],
            "https://example.com/profile"
        );
        let encoded = serde_json::to_string(&json).unwrap();
        assert!(!encoded.contains("master"));
        assert!(!encoded.contains("revocation"));
        assert!(!encoded.contains("vdr"));
        assert_ne!(json, json!(null));
    }

    #[test]
    fn projection_requires_an_exact_equivalent_prism_identifier() {
        let state = state_with_document_keys();
        let prefix_collision = Did::parse(&format!("{}0", state.did())).unwrap();

        assert_eq!(
            project_did_document_for(&state, prefix_collision),
            Err(Error::InvalidState(StateError::InvalidCandidate))
        );
    }

    #[test]
    fn projection_budget_covers_the_supported_maximum_long_form_shape() {
        let repetitions = 1 + 3 * (MAX_VERIFICATION_METHODS - 1) + MAX_SERVICES;
        let estimate = MAX_LONG_FORM_DID_BYTES * repetitions + MAX_OPERATION_BYTES;

        assert_eq!(MAX_LONG_FORM_DID_BYTES, 128_075);
        assert_eq!(estimate, 25_454_850);
        assert!(estimate <= MAX_PROJECTED_DOCUMENT_BYTES);
    }
}
