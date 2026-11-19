//! Explicit WASM ownership boundary for the portable PRISM DID method.

use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use identus_crypto::{Ed25519PublicKey, Secp256k1PublicKey, X25519PublicKey};
use identus_prism_did::{
    CreateDidOperation, KeyUsage, LongFormPrismDid, PrismDid, PrismDidState, PrismPublicKey,
    PrismPublicKeyMaterial, PrismService, ServiceEndpoint, ServiceEndpointValue, ServiceType,
    SignedPrismOperation, project_did_document_for,
};
use serde::Deserialize;
use serde_json::Value;
use wasm_bindgen::prelude::*;

use crate::{DidParseError, contains_unsafe_javascript_integer};

const CANONICAL_FORM: &str = "canonical";
const LONG_FORM: &str = "long-form";
const MAX_CREATE_INPUT_BYTES: usize = 64 * 1_024;
const MAX_HISTORY_INPUT_BYTES: usize = 24 * 1_024 * 1_024;
const MAX_HISTORY_OPERATIONS: usize = 4_096;

/// Owned JavaScript result for validated PRISM identifier operations.
#[derive(Debug)]
#[wasm_bindgen]
pub struct PrismDidView {
    requested_did: String,
    canonical_did: String,
    form: &'static str,
    resolved_document: Option<String>,
}

#[wasm_bindgen]
impl PrismDidView {
    #[wasm_bindgen(getter, js_name = requestedDid)]
    pub fn requested_did(&self) -> String {
        self.requested_did.clone()
    }

    #[wasm_bindgen(getter, js_name = canonicalDid)]
    pub fn canonical_did(&self) -> String {
        self.canonical_did.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn form(&self) -> String {
        self.form.to_owned()
    }

    #[wasm_bindgen(getter, js_name = resolvedDocument)]
    pub fn resolved_document(&self) -> Option<String> {
        self.resolved_document.clone()
    }
}

/// Owned JavaScript result for authenticated PRISM history reduction.
#[derive(Debug)]
#[wasm_bindgen]
pub struct PrismDidStateView {
    requested_did: String,
    canonical_did: String,
    deactivated: bool,
    resolved_document: Option<String>,
}

#[wasm_bindgen]
impl PrismDidStateView {
    #[wasm_bindgen(getter, js_name = requestedDid)]
    pub fn requested_did(&self) -> String {
        self.requested_did.clone()
    }

    #[wasm_bindgen(getter, js_name = canonicalDid)]
    pub fn canonical_did(&self) -> String {
        self.canonical_did.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn deactivated(&self) -> bool {
        self.deactivated
    }

    #[wasm_bindgen(getter, js_name = resolvedDocument)]
    pub fn resolved_document(&self) -> Option<String> {
        self.resolved_document.clone()
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CreateInput {
    keys: Vec<KeyInput>,
    #[serde(default)]
    services: Vec<ServiceInput>,
    #[serde(default)]
    contexts: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct KeyInput {
    id: String,
    usage: UsageInput,
    curve: CurveInput,
    public_key: String,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
enum UsageInput {
    Master,
    Issuing,
    KeyAgreement,
    Authentication,
    Revocation,
    CapabilityInvocation,
    CapabilityDelegation,
    Vdr,
}

#[derive(Clone, Copy, Debug, Deserialize)]
enum CurveInput {
    #[serde(rename = "secp256k1")]
    Secp256k1,
    #[serde(rename = "Ed25519")]
    Ed25519,
    #[serde(rename = "X25519")]
    X25519,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct ServiceInput {
    id: String,
    #[serde(rename = "type")]
    type_: OneOrManyString,
    service_endpoint: Value,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum OneOrManyString {
    One(String),
    Many(Vec<String>),
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct HistoryInput {
    operations: Vec<String>,
}

/// Validate a canonical or self-certifying long-form PRISM DID.
#[wasm_bindgen(js_name = parsePrismDid)]
pub fn parse_prism_did(value: &str) -> Result<PrismDidView, DidParseError> {
    let did = PrismDid::parse(value).map_err(DidParseError::prism)?;
    Ok(project_identifier(&did, None))
}

/// Create and locally project a self-certifying long-form PRISM DID.
#[wasm_bindgen(js_name = createPrismDid)]
pub fn create_prism_did(input_json: &str) -> Result<PrismDidView, DidParseError> {
    if input_json.len() > MAX_CREATE_INPUT_BYTES {
        return Err(invalid_operation());
    }
    let input: CreateInput = serde_json::from_str(input_json).map_err(|_| invalid_operation())?;
    let keys = input
        .keys
        .into_iter()
        .map(map_key)
        .collect::<Result<Vec<_>, _>>()?;
    let services = input
        .services
        .into_iter()
        .map(map_service)
        .collect::<Result<Vec<_>, _>>()?;
    let create =
        CreateDidOperation::new(keys, services, input.contexts).map_err(DidParseError::prism)?;
    let raw = create.to_raw_operation().map_err(DidParseError::prism)?;
    let long = LongFormPrismDid::from_initial_state(raw).map_err(DidParseError::prism)?;
    project_long_form(long)
}

/// Resolve only the embedded initial state of a long-form PRISM DID.
#[wasm_bindgen(js_name = resolvePrismDidLongForm)]
pub fn resolve_prism_did_long_form(value: &str) -> Result<PrismDidView, DidParseError> {
    let long = LongFormPrismDid::parse(value).map_err(DidParseError::prism)?;
    project_long_form(long)
}

/// Authentically reduce a bounded exact signed-operation history.
#[wasm_bindgen(js_name = reducePrismDidHistory)]
pub fn reduce_prism_did_history(
    value: &str,
    history_json: &str,
) -> Result<PrismDidStateView, DidParseError> {
    if history_json.len() > MAX_HISTORY_INPUT_BYTES {
        return Err(invalid_wire());
    }
    let requested = PrismDid::parse(value).map_err(DidParseError::prism)?;
    let history: HistoryInput = serde_json::from_str(history_json).map_err(|_| invalid_wire())?;
    if history.operations.len() > MAX_HISTORY_OPERATIONS {
        return Err(invalid_wire());
    }
    let signed = history
        .operations
        .into_iter()
        .map(|encoded| {
            if encoded.is_empty() || encoded.contains('=') {
                return Err(invalid_wire());
            }
            let bytes = URL_SAFE_NO_PAD
                .decode(&encoded)
                .map_err(|_| invalid_wire())?;
            if URL_SAFE_NO_PAD.encode(&bytes) != encoded {
                return Err(invalid_wire());
            }
            SignedPrismOperation::parse(&bytes).map_err(DidParseError::prism)
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut state = match &requested {
        PrismDid::Canonical(_) => {
            let (first, rest) = signed.split_first().ok_or_else(invalid_state)?;
            let state = PrismDidState::from_signed_create(first).map_err(DidParseError::prism)?;
            if state.did() != requested.canonical() {
                return Err(invalid_state());
            }
            (state, rest)
        }
        PrismDid::LongForm(long) => {
            let state = PrismDidState::from_initial_state(long.initial_state())
                .map_err(DidParseError::prism)?;
            (state, signed.as_slice())
        }
    };
    for operation in state.1 {
        state
            .0
            .apply_signed(operation)
            .map_err(DidParseError::prism)?;
    }
    project_state(&requested, state.0)
}

fn project_identifier(did: &PrismDid, document: Option<String>) -> PrismDidView {
    PrismDidView {
        requested_did: did.to_string(),
        canonical_did: did.canonical().to_string(),
        form: match did {
            PrismDid::Canonical(_) => CANONICAL_FORM,
            PrismDid::LongForm(_) => LONG_FORM,
        },
        resolved_document: document,
    }
}

fn project_long_form(long: LongFormPrismDid) -> Result<PrismDidView, DidParseError> {
    let state =
        PrismDidState::from_initial_state(long.initial_state()).map_err(DidParseError::prism)?;
    let did = PrismDid::LongForm(long);
    let document = project_document(&did, &state)?;
    Ok(project_identifier(&did, document))
}

fn project_state(did: &PrismDid, state: PrismDidState) -> Result<PrismDidStateView, DidParseError> {
    Ok(PrismDidStateView {
        requested_did: did.to_string(),
        canonical_did: did.canonical().to_string(),
        deactivated: state.is_deactivated(),
        resolved_document: project_document(did, &state)?,
    })
}

fn project_document(
    did: &PrismDid,
    state: &PrismDidState,
) -> Result<Option<String>, DidParseError> {
    project_did_document_for(state, did.as_did().clone())
        .map_err(DidParseError::prism)?
        .map(|document| {
            let serialized = serde_json::to_string(&document).map_err(|_| invalid_state())?;
            let value: Value = serde_json::from_str(&serialized).map_err(|_| invalid_state())?;
            if contains_unsafe_javascript_integer(&value) {
                return Err(DidParseError::unsupported_lossy_mapping());
            }
            Ok(serialized)
        })
        .transpose()
}

fn map_key(input: KeyInput) -> Result<PrismPublicKey, DidParseError> {
    if input.public_key.is_empty() || input.public_key.contains('=') {
        return Err(invalid_operation());
    }
    let bytes = URL_SAFE_NO_PAD
        .decode(&input.public_key)
        .map_err(|_| invalid_operation())?;
    if URL_SAFE_NO_PAD.encode(&bytes) != input.public_key {
        return Err(invalid_operation());
    }
    let material = match input.curve {
        CurveInput::Secp256k1 => PrismPublicKeyMaterial::Secp256k1(
            Secp256k1PublicKey::from_slice(&bytes).map_err(|_| invalid_operation())?,
        ),
        CurveInput::Ed25519 => PrismPublicKeyMaterial::Ed25519(
            Ed25519PublicKey::from_slice(&bytes).map_err(|_| invalid_operation())?,
        ),
        CurveInput::X25519 => PrismPublicKeyMaterial::X25519(
            X25519PublicKey::from_slice(&bytes).map_err(|_| invalid_operation())?,
        ),
    };
    PrismPublicKey::new(input.id, map_usage(input.usage), material).map_err(DidParseError::prism)
}

fn map_usage(usage: UsageInput) -> KeyUsage {
    match usage {
        UsageInput::Master => KeyUsage::Master,
        UsageInput::Issuing => KeyUsage::Issuing,
        UsageInput::KeyAgreement => KeyUsage::KeyAgreement,
        UsageInput::Authentication => KeyUsage::Authentication,
        UsageInput::Revocation => KeyUsage::Revocation,
        UsageInput::CapabilityInvocation => KeyUsage::CapabilityInvocation,
        UsageInput::CapabilityDelegation => KeyUsage::CapabilityDelegation,
        UsageInput::Vdr => KeyUsage::Vdr,
    }
}

fn map_service(input: ServiceInput) -> Result<PrismService, DidParseError> {
    let service_type = match input.type_ {
        OneOrManyString::One(value) => ServiceType::one(value),
        OneOrManyString::Many(values) => ServiceType::many(values),
    }
    .map_err(DidParseError::prism)?;
    let endpoint = map_endpoint(input.service_endpoint)?;
    PrismService::new(input.id, service_type, endpoint).map_err(DidParseError::prism)
}

fn map_endpoint(value: Value) -> Result<ServiceEndpoint, DidParseError> {
    match value {
        Value::String(value) => ServiceEndpoint::uri(&value).map_err(DidParseError::prism),
        Value::Object(value) => ServiceEndpoint::object(value).map_err(DidParseError::prism),
        Value::Array(values) => {
            let values = values
                .into_iter()
                .map(|value| match value {
                    Value::String(value) => Ok(ServiceEndpointValue::Uri(value)),
                    Value::Object(value) => Ok(ServiceEndpointValue::Object(value)),
                    _ => Err(invalid_operation()),
                })
                .collect::<Result<Vec<_>, _>>()?;
            ServiceEndpoint::many(values).map_err(DidParseError::prism)
        }
        _ => Err(invalid_operation()),
    }
}

fn invalid_wire() -> DidParseError {
    DidParseError::prism(identus_prism_did::Error::InvalidWire(
        identus_prism_did::WireError::Malformed,
    ))
}

fn invalid_operation() -> DidParseError {
    DidParseError::prism(identus_prism_did::Error::InvalidOperation(
        identus_prism_did::OperationError::InvalidValue,
    ))
}

fn invalid_state() -> DidParseError {
    DidParseError::prism(identus_prism_did::Error::InvalidState(
        identus_prism_did::StateError::InvalidCandidate,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::engine::general_purpose::URL_SAFE_NO_PAD;

    const PUBLIC_KEY_HEX: &str =
        "02e45998e18117113835b972b43784fb6b6470b722cb2dc171c5fba00265bc80ee";
    const SIGNED_CREATE_HEX: &str = "0a076d61737465723012483046022100b8e57b812375ea9e3c3ae79459c8695a149419bf68c2e9e18a185dddce4b08af022100aa5fa6dc7dee7c0d0b91ce05c204527e84bba9ef87e278708512d0c91ac2403c1a410a3f0a3d123b0a076d61737465723010014a2e0a09736563703235366b31122102e45998e18117113835b972b43784fb6b6470b722cb2dc171c5fba00265bc80ee";

    #[test]
    fn public_material_create_and_local_projection_round_trip() {
        let key = URL_SAFE_NO_PAD.encode(hex::decode(PUBLIC_KEY_HEX).unwrap());
        let input = format!(
            r#"{{"keys":[{{"id":"master0","usage":"master","curve":"secp256k1","publicKey":"{key}"}}],"services":[],"contexts":[]}}"#
        );
        let created = create_prism_did(&input).unwrap();
        assert_eq!(created.form(), LONG_FORM);
        assert!(created.requested_did().starts_with("did:prism:"));
        assert!(created.resolved_document().is_some());
        let parsed = resolve_prism_did_long_form(&created.requested_did()).unwrap();
        assert_eq!(parsed.canonical_did(), created.canonical_did());
    }

    #[test]
    fn canonical_signed_history_is_authenticated_and_projected() {
        let operation = URL_SAFE_NO_PAD.encode(hex::decode(SIGNED_CREATE_HEX).unwrap());
        let history = format!(r#"{{"operations":["{operation}"]}}"#);
        let state = reduce_prism_did_history(
            "did:prism:44fb82ad3544a8c8acb763c01a6fc7d92d81c38104096319345a4618d1a4275c",
            &history,
        )
        .unwrap();
        assert!(!state.deactivated());
        assert!(state.resolved_document().is_some());
    }

    #[test]
    fn history_rejects_non_canonical_base64url() {
        let error = reduce_prism_did_history(
            "did:prism:44fb82ad3544a8c8acb763c01a6fc7d92d81c38104096319345a4618d1a4275c",
            r#"{"operations":["AA=="]}"#,
        )
        .unwrap_err();
        assert_eq!(error.code(), crate::PRISM_INVALID_WIRE_CODE);
    }
}
