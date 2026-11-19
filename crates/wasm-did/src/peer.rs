//! Explicit WASM ownership boundary for the two supported peer DID profiles.

use identus_did::DidDocument;
use identus_did_peer::{
    Error as PeerError,
    numalgo2_compat::{PeerDid, PublicKey, Purpose, Service, ServiceEndpoint, ServiceEndpointMap},
    numalgo4::{InputDocument, LongForm, ShortForm},
};
use serde::Deserialize;
use serde_json::Value;
use wasm_bindgen::prelude::*;

use crate::{DidParseError, contains_unsafe_javascript_integer};

const NUMALGO4_PROFILE: &str = "numalgo4";
const NUMALGO2_COMPATIBILITY_PROFILE: &str = "numalgo2-compatibility";
const MAX_NUMALGO2_INPUT_BYTES: usize = 24 * 1_024;

/// Owned JavaScript result for numalgo-4 creation or local resolution.
#[derive(Debug)]
#[wasm_bindgen]
pub struct PeerDidNumalgo4View {
    long_form: String,
    short_form: String,
    input_document: String,
    resolved_document: String,
}

#[wasm_bindgen]
impl PeerDidNumalgo4View {
    /// Stable explicit profile identity.
    #[wasm_bindgen(getter)]
    pub fn profile(&self) -> String {
        NUMALGO4_PROFILE.to_owned()
    }

    /// Verified long-form peer DID.
    #[wasm_bindgen(getter, js_name = longForm)]
    pub fn long_form(&self) -> String {
        self.long_form.clone()
    }

    /// Derived short-form peer DID.
    #[wasm_bindgen(getter, js_name = shortForm)]
    pub fn short_form(&self) -> String {
        self.short_form.clone()
    }

    /// Compact normalized public input document without a root `id`.
    #[wasm_bindgen(getter, js_name = inputDocument)]
    pub fn input_document(&self) -> String {
        self.input_document.clone()
    }

    /// Compact normalized locally resolved DID Document.
    #[wasm_bindgen(getter, js_name = resolvedDocument)]
    pub fn resolved_document(&self) -> String {
        self.resolved_document.clone()
    }
}

/// Owned JavaScript result for short-form validation without resolution.
#[derive(Debug)]
#[wasm_bindgen]
pub struct PeerDidNumalgo4ShortView {
    short_form: String,
}

#[wasm_bindgen]
impl PeerDidNumalgo4ShortView {
    /// Stable explicit profile identity.
    #[wasm_bindgen(getter)]
    pub fn profile(&self) -> String {
        NUMALGO4_PROFILE.to_owned()
    }

    /// Validated short-form peer DID.
    #[wasm_bindgen(getter, js_name = shortForm)]
    pub fn short_form(&self) -> String {
        self.short_form.clone()
    }
}

/// Owned JavaScript result for explicit numalgo-2 compatibility operations.
#[derive(Debug)]
#[wasm_bindgen]
pub struct PeerDidNumalgo2CompatibilityView {
    did: String,
    resolved_document: String,
}

#[wasm_bindgen]
impl PeerDidNumalgo2CompatibilityView {
    /// Stable explicit compatibility-profile identity.
    #[wasm_bindgen(getter)]
    pub fn profile(&self) -> String {
        NUMALGO2_COMPATIBILITY_PROFILE.to_owned()
    }

    /// Validated compatibility peer DID.
    #[wasm_bindgen(getter)]
    pub fn did(&self) -> String {
        self.did.clone()
    }

    /// Compact normalized locally resolved DID Document.
    #[wasm_bindgen(getter, js_name = resolvedDocument)]
    pub fn resolved_document(&self) -> String {
        self.resolved_document.clone()
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Numalgo2Input {
    keys: Vec<Numalgo2KeyInput>,
    #[serde(default)]
    services: Vec<Numalgo2ServiceInput>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Numalgo2KeyInput {
    purpose: Numalgo2PurposeInput,
    multibase: String,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
enum Numalgo2PurposeInput {
    Verification,
    Encryption,
    Assertion,
    CapabilityInvocation,
    CapabilityDelegation,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Numalgo2ServiceInput {
    #[serde(rename = "type")]
    type_: String,
    endpoint: Numalgo2EndpointInput,
    #[serde(default)]
    id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Numalgo2EndpointInput {
    Uri(String),
    Map(Numalgo2EndpointMapInput),
    Maps(Vec<Numalgo2EndpointMapInput>),
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct Numalgo2EndpointMapInput {
    uri: String,
    #[serde(default)]
    accept: Option<Vec<String>>,
    #[serde(default)]
    routing_keys: Option<Vec<String>>,
}

/// Create and locally resolve the current numalgo-4 profile.
#[wasm_bindgen(js_name = createPeerDidNumalgo4)]
pub fn create_peer_did_numalgo4(
    input_document_json: &str,
) -> Result<PeerDidNumalgo4View, DidParseError> {
    let input = InputDocument::from_json_str(input_document_json).map_err(DidParseError::peer)?;
    let long = LongForm::create(input).map_err(DidParseError::peer)?;
    project_numalgo4(&long, long.resolve().map_err(DidParseError::peer)?)
}

/// Parse, verify, and locally resolve a numalgo-4 long form.
#[wasm_bindgen(js_name = parsePeerDidNumalgo4Long)]
pub fn parse_peer_did_numalgo4_long(long_form: &str) -> Result<PeerDidNumalgo4View, DidParseError> {
    let long = LongForm::parse(long_form).map_err(DidParseError::peer)?;
    project_numalgo4(&long, long.resolve().map_err(DidParseError::peer)?)
}

/// Validate a numalgo-4 short form without implying resolvability.
#[wasm_bindgen(js_name = parsePeerDidNumalgo4Short)]
pub fn parse_peer_did_numalgo4_short(
    short_form: &str,
) -> Result<PeerDidNumalgo4ShortView, DidParseError> {
    let short = ShortForm::parse(short_form).map_err(DidParseError::peer)?;
    Ok(PeerDidNumalgo4ShortView {
        short_form: short.did().to_string(),
    })
}

/// Resolve a short form only with matching caller-supplied long-form material.
#[wasm_bindgen(js_name = resolvePeerDidNumalgo4Short)]
pub fn resolve_peer_did_numalgo4_short(
    short_form: &str,
    long_form: &str,
) -> Result<PeerDidNumalgo4View, DidParseError> {
    let short = ShortForm::parse(short_form).map_err(DidParseError::peer)?;
    let long = LongForm::parse(long_form).map_err(DidParseError::peer)?;
    let document = short.resolve_with(&long).map_err(DidParseError::peer)?;
    project_numalgo4(&long, document)
}

/// Create and locally resolve an explicit numalgo-2 compatibility DID.
#[wasm_bindgen(js_name = createPeerDidNumalgo2Compatibility)]
pub fn create_peer_did_numalgo2_compatibility(
    input_json: &str,
) -> Result<PeerDidNumalgo2CompatibilityView, DidParseError> {
    if input_json.len() > MAX_NUMALGO2_INPUT_BYTES {
        return Err(DidParseError::peer(PeerError::ResourceLimit));
    }
    let input: Numalgo2Input = serde_json::from_str(input_json)
        .map_err(|_| DidParseError::peer(PeerError::InvalidInputDocument))?;
    let keys = input
        .keys
        .into_iter()
        .map(|key| PublicKey::new(map_purpose(key.purpose), key.multibase))
        .collect::<Result<Vec<_>, _>>()
        .map_err(DidParseError::peer)?;
    let services = input.services.into_iter().map(map_service).collect();
    let peer = PeerDid::create_sdk_ts_compatibility(keys, services).map_err(DidParseError::peer)?;
    project_numalgo2(&peer)
}

/// Parse and locally resolve an explicit numalgo-2 compatibility DID.
#[wasm_bindgen(js_name = parsePeerDidNumalgo2Compatibility)]
pub fn parse_peer_did_numalgo2_compatibility(
    did: &str,
) -> Result<PeerDidNumalgo2CompatibilityView, DidParseError> {
    let peer = PeerDid::parse(did).map_err(DidParseError::peer)?;
    project_numalgo2(&peer)
}

fn map_purpose(purpose: Numalgo2PurposeInput) -> Purpose {
    match purpose {
        Numalgo2PurposeInput::Verification => Purpose::Verification,
        Numalgo2PurposeInput::Encryption => Purpose::Encryption,
        Numalgo2PurposeInput::Assertion => Purpose::Assertion,
        Numalgo2PurposeInput::CapabilityInvocation => Purpose::CapabilityInvocation,
        Numalgo2PurposeInput::CapabilityDelegation => Purpose::CapabilityDelegation,
    }
}

fn map_service(input: Numalgo2ServiceInput) -> Service {
    Service {
        type_: input.type_,
        endpoint: match input.endpoint {
            Numalgo2EndpointInput::Uri(uri) => ServiceEndpoint::Uri(uri),
            Numalgo2EndpointInput::Map(endpoint) => ServiceEndpoint::Map(map_endpoint(endpoint)),
            Numalgo2EndpointInput::Maps(endpoints) => {
                ServiceEndpoint::Maps(endpoints.into_iter().map(map_endpoint).collect())
            }
        },
        id: input.id,
    }
}

fn map_endpoint(input: Numalgo2EndpointMapInput) -> ServiceEndpointMap {
    ServiceEndpointMap {
        uri: input.uri,
        accept: input.accept,
        routing_keys: input.routing_keys,
    }
}

fn project_numalgo4(
    long: &LongForm,
    document: DidDocument,
) -> Result<PeerDidNumalgo4View, DidParseError> {
    let input_document = std::str::from_utf8(long.input_document().compact_json())
        .map_err(|_| DidParseError::peer(PeerError::Projection))?
        .to_owned();
    reject_unsafe_numbers(&input_document)?;
    Ok(PeerDidNumalgo4View {
        long_form: long.did().to_string(),
        short_form: long.short_form().to_string(),
        input_document,
        resolved_document: serialize_document(&document)?,
    })
}

fn project_numalgo2(peer: &PeerDid) -> Result<PeerDidNumalgo2CompatibilityView, DidParseError> {
    Ok(PeerDidNumalgo2CompatibilityView {
        did: peer.did().to_string(),
        resolved_document: serialize_document(&peer.resolve().map_err(DidParseError::peer)?)?,
    })
}

fn serialize_document(document: &DidDocument) -> Result<String, DidParseError> {
    let serialized =
        serde_json::to_string(document).map_err(|_| DidParseError::peer(PeerError::Projection))?;
    reject_unsafe_numbers(&serialized)?;
    Ok(serialized)
}

fn reject_unsafe_numbers(serialized: &str) -> Result<(), DidParseError> {
    let value: Value =
        serde_json::from_str(serialized).map_err(|_| DidParseError::peer(PeerError::Projection))?;
    if contains_unsafe_javascript_integer(&value) {
        Err(DidParseError::unsupported_lossy_mapping())
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PEER_INVALID_INPUT_DOCUMENT_CODE, PEER_UNSUPPORTED_CODEC_CODE};

    const ED25519_KEY: &str = "z6MkiTBz1y5mVhWQBkf9yGkZc1V1Dk2d5HfU7J4rLQvW8x9A";

    #[test]
    fn numalgo4_creation_and_short_resolution_share_identity() {
        let created = create_peer_did_numalgo4(r#"{"hello":"world"}"#).unwrap();
        assert_eq!(created.profile(), NUMALGO4_PROFILE);
        assert!(created.long_form().starts_with("did:peer:4"));
        assert_eq!(created.long_form().matches(':').count(), 3);
        assert_eq!(created.short_form().matches(':').count(), 2);

        let resolved =
            resolve_peer_did_numalgo4_short(&created.short_form(), &created.long_form()).unwrap();
        assert_eq!(resolved.short_form(), created.short_form());
        assert!(resolved.resolved_document().contains(&created.short_form()));
    }

    #[test]
    fn explicit_profile_mismatch_is_redacted() {
        let error = parse_peer_did_numalgo4_long("did:peer:2.invalid").unwrap_err();
        assert_eq!(error.code(), crate::PEER_UNSUPPORTED_ALGORITHM_CODE);
        assert!(!error.code().contains("invalid"));
    }

    #[test]
    fn numalgo2_input_is_closed_before_method_validation() {
        let input = format!(
            r#"{{"keys":[{{"purpose":"verification","multibase":"{ED25519_KEY}","private":"secret"}}]}}"#,
        );
        assert_eq!(
            create_peer_did_numalgo2_compatibility(&input)
                .unwrap_err()
                .code(),
            PEER_INVALID_INPUT_DOCUMENT_CODE
        );
    }

    #[test]
    fn numalgo2_rejects_an_unsupported_public_codec() {
        let input = r#"{"keys":[{"purpose":"verification","multibase":"z1111"}]}"#;
        assert_eq!(
            create_peer_did_numalgo2_compatibility(input)
                .unwrap_err()
                .code(),
            PEER_UNSUPPORTED_CODEC_CODE
        );
    }
}
