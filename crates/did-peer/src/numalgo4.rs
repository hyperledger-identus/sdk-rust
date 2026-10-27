//! Current peer DID numalgo-4 short/long-form profile.

use bs58::{decode, encode};
use identus_did::{Did, DidDocument, JsonWireError, JsonWireLimits, validate_unique_object_names};
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};
use std::fmt;

use crate::Error;

const PREFIX: &str = "did:peer:4";
const JSON_MULTICODEC: [u8; 2] = [0x80, 0x04];
const SHA2_256_MULTIHASH: [u8; 2] = [0x12, 0x20];
const MULTIHASH_BYTES: usize = 34;

/// Maximum decoded JSON input-document bytes accepted by this profile.
///
/// Base58 expansion plus the fixed peer-DID hash/prefix remains below the
/// generic 128 KiB DID ceiling at this bound.
pub const MAX_INPUT_DOCUMENT_BYTES: usize = 90 * 1_024;
/// Maximum encoded input-document characters accepted before base decoding.
pub const MAX_ENCODED_DOCUMENT_BYTES: usize = 126 * 1_024;
/// Maximum JSON nesting depth accepted by the input-document parser.
pub const MAX_INPUT_DOCUMENT_DEPTH: usize = 32;
/// Maximum aggregate JSON values accepted by the input-document parser.
pub const MAX_INPUT_DOCUMENT_NODES: usize = 4_096;
/// Maximum members accepted in any one input-document object.
pub const MAX_INPUT_OBJECT_MEMBERS: usize = 128;
/// Maximum aggregate live object-key bytes retained while scanning JSON.
pub const MAX_INPUT_LIVE_KEY_BYTES: usize = 64 * 1_024;

/// A validated numalgo-4 construction input without a root `id`.
#[derive(Clone, PartialEq, Eq)]
pub struct InputDocument {
    value: Value,
    compact: Vec<u8>,
}

impl InputDocument {
    /// Parse, bound and validate a construction input document.
    pub fn from_json_slice(input: &[u8]) -> Result<Self, Error> {
        if input.len() > MAX_INPUT_DOCUMENT_BYTES {
            return Err(Error::ResourceLimit);
        }
        validate_unique_object_names(
            input,
            JsonWireLimits {
                max_depth: MAX_INPUT_DOCUMENT_DEPTH,
                max_nodes: MAX_INPUT_DOCUMENT_NODES,
                max_object_members: MAX_INPUT_OBJECT_MEMBERS,
                max_live_key_bytes: MAX_INPUT_LIVE_KEY_BYTES,
            },
        )
        .map_err(map_json_wire_error)?;
        let value: Value =
            serde_json::from_slice(input).map_err(|_| Error::InvalidInputDocument)?;
        let object = value.as_object().ok_or(Error::InvalidInputDocument)?;
        if object.contains_key("id") || object.len() > 128 {
            return Err(Error::InvalidInputDocument);
        }
        reject_secret_members(&value)?;
        let compact = serde_json::to_vec(&value).map_err(|_| Error::InvalidInputDocument)?;
        if compact.len() > MAX_INPUT_DOCUMENT_BYTES {
            return Err(Error::ResourceLimit);
        }
        // Validate the final generic document shape before retaining input.
        let placeholder = Did::parse("did:peer:4zQmYwAPJzv5CZsnAzt8auVZRnGiHZQ6ZMHTQm14iFe9QqF")
            .map_err(|_| Error::Projection)?;
        contextualize(&value, &placeholder, None)?;
        Ok(Self { value, compact })
    }

    /// Parse a JSON string.
    pub fn from_json_str(input: &str) -> Result<Self, Error> {
        Self::from_json_slice(input.as_bytes())
    }

    /// Borrow the deterministic compact representation hashed during creation.
    #[must_use]
    pub fn compact_json(&self) -> &[u8] {
        &self.compact
    }
}

impl fmt::Debug for InputDocument {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("InputDocument")
            .field("compact_bytes", &self.compact.len())
            .finish_non_exhaustive()
    }
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

/// A validated numalgo-4 long form.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LongForm {
    did: Did,
    short: Did,
    input: InputDocument,
}

impl LongForm {
    /// Create a long form from a validated input document.
    pub fn create(input: InputDocument) -> Result<Self, Error> {
        let mut document_bytes = Vec::with_capacity(JSON_MULTICODEC.len() + input.compact.len());
        document_bytes.extend_from_slice(&JSON_MULTICODEC);
        document_bytes.extend_from_slice(&input.compact);
        let encoded_document = format!("z{}", encode(document_bytes).into_string());
        if encoded_document.len() > MAX_ENCODED_DOCUMENT_BYTES {
            return Err(Error::ResourceLimit);
        }

        let digest = Sha256::digest(encoded_document.as_bytes());
        let mut multihash = Vec::with_capacity(MULTIHASH_BYTES);
        multihash.extend_from_slice(&SHA2_256_MULTIHASH);
        multihash.extend_from_slice(&digest);
        let hash = format!("z{}", encode(multihash).into_string());
        let short = Did::parse(&format!("{PREFIX}{hash}")).map_err(|_| Error::Projection)?;
        let did = Did::parse(&format!("{PREFIX}{hash}:{encoded_document}"))
            .map_err(|_| Error::ResourceLimit)?;
        let long = Self { did, short, input };
        long.resolve()?;
        ShortForm(long.short.clone()).resolve_with(&long)?;
        Ok(long)
    }

    /// Parse and verify a numalgo-4 long form.
    pub fn parse(value: &str) -> Result<Self, Error> {
        if !value.starts_with(PREFIX) {
            return classify_wrong_profile(value);
        }
        let remainder = &value[PREFIX.len()..];
        let (hash, encoded_document) = remainder.split_once(':').ok_or(Error::WrongProfile)?;
        if hash.contains(':') || encoded_document.contains(':') {
            return Err(Error::MalformedEncoding);
        }
        if encoded_document.len() > MAX_ENCODED_DOCUMENT_BYTES {
            return Err(Error::ResourceLimit);
        }
        validate_hash(hash, encoded_document)?;
        let decoded = decode_multibase(encoded_document, MAX_INPUT_DOCUMENT_BYTES + 2)?;
        if !decoded.starts_with(&JSON_MULTICODEC) {
            return Err(Error::UnsupportedCodec);
        }
        let input = InputDocument::from_json_slice(&decoded[JSON_MULTICODEC.len()..])?;
        let did = Did::parse(value).map_err(|_| Error::MalformedEncoding)?;
        let short = Did::parse(&format!("{PREFIX}{hash}")).map_err(|_| Error::MalformedEncoding)?;
        let long = Self { did, short, input };
        long.resolve()?;
        ShortForm(long.short.clone()).resolve_with(&long)?;
        Ok(long)
    }

    /// Borrow the long-form DID.
    #[must_use]
    pub const fn did(&self) -> &Did {
        &self.did
    }

    /// Borrow the derived short-form DID.
    #[must_use]
    pub const fn short_form(&self) -> &Did {
        &self.short
    }

    /// Borrow the decoded construction input.
    #[must_use]
    pub const fn input_document(&self) -> &InputDocument {
        &self.input
    }

    /// Resolve the long form locally.
    pub fn resolve(&self) -> Result<DidDocument, Error> {
        contextualize(&self.input.value, &self.did, Some(&self.short))
    }
}

/// A validated numalgo-4 short form.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShortForm(Did);

impl ShortForm {
    /// Parse a short form without performing resolution I/O.
    pub fn parse(value: &str) -> Result<Self, Error> {
        if !value.starts_with(PREFIX) {
            return classify_wrong_profile(value);
        }
        let hash = &value[PREFIX.len()..];
        if hash.is_empty() || hash.contains(':') {
            return Err(Error::WrongProfile);
        }
        validate_multihash(hash)?;
        let did = Did::parse(value).map_err(|_| Error::MalformedEncoding)?;
        Ok(Self(did))
    }

    /// Borrow the short-form DID.
    #[must_use]
    pub const fn did(&self) -> &Did {
        &self.0
    }

    /// Reject effect-free resolution without matching long-form material.
    pub const fn resolve(&self) -> Result<DidDocument, Error> {
        Err(Error::LongFormRequired)
    }

    /// Resolve using caller-supplied matching long-form material.
    pub fn resolve_with(&self, long: &LongForm) -> Result<DidDocument, Error> {
        if self.0 != long.short {
            return Err(Error::HashMismatch);
        }
        contextualize(&long.input.value, &self.0, Some(&long.did))
    }
}

fn validate_hash(hash: &str, encoded_document: &str) -> Result<(), Error> {
    let claimed = validate_multihash(hash)?;
    let actual = Sha256::digest(encoded_document.as_bytes());
    let mut difference = 0_u8;
    for (left, right) in claimed.iter().zip(actual.iter()) {
        difference |= left ^ right;
    }
    if difference == 0 {
        Ok(())
    } else {
        Err(Error::HashMismatch)
    }
}

fn validate_multihash(hash: &str) -> Result<[u8; 32], Error> {
    let decoded = decode_multibase(hash, MULTIHASH_BYTES)?;
    if decoded.len() != MULTIHASH_BYTES || !decoded.starts_with(&SHA2_256_MULTIHASH) {
        return Err(Error::UnsupportedCodec);
    }
    let mut digest = [0_u8; 32];
    digest.copy_from_slice(&decoded[2..]);
    Ok(digest)
}

fn decode_multibase(value: &str, max_decoded: usize) -> Result<Vec<u8>, Error> {
    let payload = value.strip_prefix('z').ok_or(Error::MalformedEncoding)?;
    if payload.is_empty() || payload.len() > MAX_ENCODED_DOCUMENT_BYTES {
        return Err(Error::ResourceLimit);
    }
    let estimate = payload.len().saturating_mul(733) / 1_000 + 1;
    if estimate > max_decoded {
        return Err(Error::ResourceLimit);
    }
    let decoded = decode(payload)
        .into_vec()
        .map_err(|_| Error::MalformedEncoding)?;
    if decoded.len() > max_decoded {
        return Err(Error::ResourceLimit);
    }
    Ok(decoded)
}

fn contextualize(
    input: &Value,
    subject: &Did,
    counterpart: Option<&Did>,
) -> Result<DidDocument, Error> {
    let mut object = input
        .as_object()
        .cloned()
        .ok_or(Error::InvalidInputDocument)?;
    object.insert("id".into(), Value::String(subject.to_string()));
    contextualize_methods(&mut object, subject)?;
    contextualize_services(&mut object, subject)?;
    if let Some(counterpart) = counterpart {
        let aliases = object
            .entry("alsoKnownAs")
            .or_insert_with(|| Value::Array(Vec::new()))
            .as_array_mut()
            .ok_or(Error::InvalidInputDocument)?;
        if aliases.len() >= 128 {
            return Err(Error::ResourceLimit);
        }
        aliases.push(Value::String(counterpart.to_string()));
    }
    let bytes = serde_json::to_vec(&Value::Object(object)).map_err(|_| Error::Projection)?;
    DidDocument::from_json_slice(&bytes).map_err(|_| Error::Projection)
}

fn reject_secret_members(value: &Value) -> Result<(), Error> {
    match value {
        Value::Object(object) => {
            for (name, value) in object {
                let normalized: String = name
                    .chars()
                    .filter(|character| character.is_ascii_alphanumeric())
                    .flat_map(char::to_lowercase)
                    .collect();
                if normalized.contains("privatekey")
                    || normalized.contains("secretkey")
                    || normalized == "secret"
                    || normalized.contains("seed")
                    || normalized.contains("mnemonic")
                {
                    return Err(Error::InvalidInputDocument);
                }
                reject_secret_members(value)?;
            }
        }
        Value::Array(values) => {
            for value in values {
                reject_secret_members(value)?;
            }
        }
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
    }
    Ok(())
}

fn contextualize_methods(object: &mut Map<String, Value>, subject: &Did) -> Result<(), Error> {
    if let Some(methods) = object.get_mut("verificationMethod") {
        contextualize_method_array(methods, subject, false)?;
    }
    for name in [
        "authentication",
        "assertionMethod",
        "keyAgreement",
        "capabilityInvocation",
        "capabilityDelegation",
    ] {
        if let Some(values) = object.get_mut(name) {
            contextualize_method_array(values, subject, true)?;
        }
    }
    Ok(())
}

fn contextualize_method_array(
    value: &mut Value,
    subject: &Did,
    allow_references: bool,
) -> Result<(), Error> {
    let values = value.as_array_mut().ok_or(Error::InvalidInputDocument)?;
    if values.len() > 128 {
        return Err(Error::ResourceLimit);
    }
    for value in values {
        if allow_references && value.is_string() {
            contextualize_fragment(value, subject)?;
            continue;
        }
        let method = value.as_object_mut().ok_or(Error::InvalidInputDocument)?;
        let id = method.get_mut("id").ok_or(Error::InvalidInputDocument)?;
        contextualize_fragment(id, subject)?;
        match method.get_mut("controller") {
            None => {
                method.insert("controller".into(), Value::String(subject.to_string()));
            }
            Some(controller) => {
                if controller
                    .as_str()
                    .is_some_and(|value| value.starts_with('#'))
                {
                    contextualize_fragment(controller, subject)?;
                }
            }
        }
    }
    Ok(())
}

fn contextualize_services(object: &mut Map<String, Value>, subject: &Did) -> Result<(), Error> {
    let Some(services) = object.get_mut("service") else {
        return Ok(());
    };
    let services = services.as_array_mut().ok_or(Error::InvalidInputDocument)?;
    if services.len() > 128 {
        return Err(Error::ResourceLimit);
    }
    for service in services {
        let service = service.as_object_mut().ok_or(Error::InvalidInputDocument)?;
        contextualize_fragment(
            service.get_mut("id").ok_or(Error::InvalidInputDocument)?,
            subject,
        )?;
    }
    Ok(())
}

fn contextualize_fragment(value: &mut Value, subject: &Did) -> Result<(), Error> {
    let fragment = value.as_str().ok_or(Error::InvalidInputDocument)?;
    if !fragment.starts_with('#') || fragment.len() < 2 {
        return Err(Error::InvalidInputDocument);
    }
    *value = Value::String(format!("{subject}{fragment}"));
    Ok(())
}

fn classify_wrong_profile<T>(value: &str) -> Result<T, Error> {
    if value.starts_with("did:peer:") {
        Err(Error::UnsupportedAlgorithm)
    } else {
        Err(Error::WrongProfile)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r##"{"verificationMethod":[{"id":"#key-1","type":"Multikey","publicKeyMultibase":"z6MkiToqovww7vYtxm1xNM15u9JzqzUFZ1k7s7MazYJUyAxv"}],"authentication":["#key-1"],"service":[{"id":"#messages","type":"DIDCommMessaging","serviceEndpoint":"https://example.com/didcomm"}]}"##;

    #[test]
    fn creates_parses_and_resolves_long_and_short_forms() {
        let input = InputDocument::from_json_str(INPUT).expect("valid input");
        let long = LongForm::create(input).expect("long form");
        let reparsed = LongForm::parse(long.did().as_str()).expect("parse long");
        assert_eq!(reparsed, long);
        assert_eq!(long.resolve().expect("resolve long").id(), long.did());

        let short = ShortForm::parse(long.short_form().as_str()).expect("short");
        assert_eq!(
            short.resolve_with(&long).expect("resolve short").id(),
            short.did()
        );
    }

    #[test]
    fn matches_reference_implementation_minimal_vector() {
        let input = InputDocument::from_json_str(r#"{"hello":"world"}"#).expect("valid input");
        let long = LongForm::create(input).expect("long form");
        assert_eq!(
            long.did().as_str(),
            "did:peer:4zQmb7xLdVY9TXx8oov5XgpGUmGELgqiAV2699s43i6Qdm3M:zQSJgiFTYiCHjQ9MktwNThRXM7a"
        );
    }

    #[test]
    fn rejects_tampered_document_and_cross_profile_inputs() {
        let input = InputDocument::from_json_str(INPUT).expect("valid input");
        let long = LongForm::create(input).expect("long form");
        let mut tampered = long.did().to_string();
        let last = tampered.pop().expect("last character");
        tampered.push(if last == '1' { '2' } else { '1' });
        assert_eq!(LongForm::parse(&tampered), Err(Error::HashMismatch));
        assert_eq!(
            ShortForm::parse("did:peer:2.Vz6Mkabc"),
            Err(Error::UnsupportedAlgorithm)
        );
    }

    #[test]
    fn rejects_root_id_and_oversized_input() {
        assert_eq!(
            InputDocument::from_json_str(r#"{"id":"did:example:123"}"#),
            Err(Error::InvalidInputDocument)
        );
        assert_eq!(
            InputDocument::from_json_slice(&vec![b' '; MAX_INPUT_DOCUMENT_BYTES + 1]),
            Err(Error::ResourceLimit)
        );
        assert_eq!(
            InputDocument::from_json_str(r#"{"claim":1,"claim":2}"#),
            Err(Error::InvalidInputDocument)
        );
    }

    #[test]
    fn rejects_secret_shaped_members_at_any_depth() {
        for input in [
            r#"{"privateKey":"secret"}"#,
            r#"{"extension":{"secret_key":"secret"}}"#,
            r#"{"extension":[{"mnemonic":"words"}]}"#,
        ] {
            assert_eq!(
                InputDocument::from_json_str(input),
                Err(Error::InvalidInputDocument)
            );
        }
    }

    #[test]
    fn rejects_input_that_only_overflows_after_counterpart_alias() {
        let aliases = (0..128)
            .map(|index| format!(r#""https://example.com/{index}""#))
            .collect::<Vec<_>>()
            .join(",");
        let input = format!(r#"{{"alsoKnownAs":[{aliases}]}}"#);
        let input = InputDocument::from_json_str(&input).expect("input is valid before projection");
        assert_eq!(LongForm::create(input), Err(Error::ResourceLimit));
    }

    #[test]
    fn property_generated_input_sizes_preserve_short_long_linkage() {
        for size in 0..=512 {
            let input = format!(r#"{{"claim":"{}"}}"#, "a".repeat(size));
            let long = LongForm::create(InputDocument::from_json_str(&input).expect("input"))
                .expect("create");
            let parsed = LongForm::parse(long.did().as_str()).expect("parse");
            assert_eq!(parsed.short_form(), long.short_form());
            assert_eq!(parsed.resolve().expect("resolve").id(), parsed.did());
        }
    }
}
