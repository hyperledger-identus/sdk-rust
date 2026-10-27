use std::io::{self, Write};

use identus_crypto::EncodeVec;
use serde::{Serialize, Serializer, ser::SerializeSeq};

use super::{
    CreateDidOperation, DeactivateDidOperation, KeyUsage, PrismPublicKey, PrismPublicKeyMaterial,
    PrismService, ServiceEndpoint, ServiceEndpointValue, ServiceType, UpdateAction,
    UpdateDidOperation, parse_service, parse_service_endpoint, parse_service_type,
    validate_fragment, validate_uri_preserving_text,
};
use crate::{
    CanonicalPrismDid, Error, MAX_CONTEXTS, MAX_OPERATION_BYTES, MAX_SERVICE_ENDPOINT_BYTES,
    MAX_SERVICES, MAX_TYPE_CHARACTERS, MAX_VERIFICATION_METHODS, OperationError, OperationKind,
    RawPrismOperation, WireError, proto,
};

impl PrismPublicKey {
    /// Construct a validated method key from SDK crypto material.
    pub fn new(
        id: impl Into<String>,
        usage: KeyUsage,
        material: PrismPublicKeyMaterial,
    ) -> Result<Self, Error> {
        let id = id.into();
        validate_fragment(&id)?;
        if matches!(usage, KeyUsage::Master | KeyUsage::Vdr)
            && !matches!(material, PrismPublicKeyMaterial::Secp256k1(_))
        {
            return Err(OperationError::UnsupportedKey.into());
        }
        Ok(Self {
            id,
            usage,
            material,
        })
    }
}

impl ServiceType {
    /// Parse the PRISM scalar or canonical JSON-array wire representation.
    pub fn parse_wire(value: &str) -> Result<Self, Error> {
        parse_service_type(value)
    }

    /// Construct one validated service-type token.
    pub fn one(value: impl Into<String>) -> Result<Self, Error> {
        parse_service_type(&value.into())
    }

    /// Construct a validated non-empty ordered service-type set.
    pub fn many(values: Vec<String>) -> Result<Self, Error> {
        if values.len() > MAX_TYPE_CHARACTERS / 3 {
            return Err(OperationError::ValueTooLong.into());
        }
        let wire = encode_json_bounded(&values, MAX_TYPE_CHARACTERS)?;
        parse_service_type(&wire)
    }
}

impl ServiceEndpoint {
    /// Parse the PRISM URI, JSON-object, or JSON-array wire representation.
    pub fn parse_wire(value: &str) -> Result<Self, Error> {
        parse_service_endpoint(value)
    }

    /// Construct one absolute-URI endpoint while preserving its exact text.
    pub fn uri(value: &str) -> Result<Self, Error> {
        parse_service_endpoint(value)
    }

    /// Construct a bounded endpoint from one JSON object.
    pub fn object(value: serde_json::Map<String, serde_json::Value>) -> Result<Self, Error> {
        let wire = encode_json_bounded(&value, MAX_SERVICE_ENDPOINT_BYTES)?;
        parse_service_endpoint(&wire)
    }

    /// Construct a bounded non-empty ordered endpoint set.
    pub fn many(values: Vec<ServiceEndpointValue>) -> Result<Self, Error> {
        if values.len() > MAX_SERVICE_ENDPOINT_BYTES / 3 {
            return Err(OperationError::ValueTooLong.into());
        }
        let wire = encode_json_bounded(&EndpointValues(&values), MAX_SERVICE_ENDPOINT_BYTES)?;
        parse_service_endpoint(&wire)
    }
}

impl PrismService {
    /// Construct a validated service from typed method values.
    pub fn new(
        id: impl Into<String>,
        service_type: ServiceType,
        endpoint: ServiceEndpoint,
    ) -> Result<Self, Error> {
        parse_service(proto::Service {
            id: id.into(),
            r#type: service_type_to_wire(&service_type)?,
            service_endpoint: service_endpoint_to_wire(&endpoint)?,
        })
    }
}

impl CreateDidOperation {
    /// Construct and revalidate a deterministic create operation.
    pub fn new(
        public_keys: Vec<PrismPublicKey>,
        services: Vec<PrismService>,
        contexts: Vec<String>,
    ) -> Result<Self, Error> {
        if public_keys.len() > MAX_VERIFICATION_METHODS
            || services.len() > MAX_SERVICES
            || contexts.len() > MAX_CONTEXTS
        {
            return Err(OperationError::TooManyItems.into());
        }
        let candidate = Self {
            public_keys,
            services,
            contexts,
        };
        Self::parse(&candidate.to_raw_operation()?)
    }

    /// Encode this validated create operation into its deterministic raw wire form.
    pub fn to_raw_operation(&self) -> Result<RawPrismOperation, Error> {
        let mut data = ProtoWriter::new();
        for key in &self.public_keys {
            data.push_message(2, encode_public_key(key)?)?;
        }
        for service in &self.services {
            data.push_message(3, encode_service(service)?)?;
        }
        for context in &self.contexts {
            data.push_bytes(4, context.as_bytes())?;
        }
        let mut create = ProtoWriter::new();
        create.push_message(1, data.finish())?;
        let payload = create.finish();
        RawPrismOperation::from_payload(OperationKind::CreateDid, &payload)
    }
}

impl UpdateDidOperation {
    /// Construct and revalidate an ordered deterministic update operation.
    pub fn new(
        did: CanonicalPrismDid,
        previous_operation_hash: identus_crypto::Sha256Digest,
        actions: Vec<UpdateAction>,
    ) -> Result<Self, Error> {
        let candidate = Self {
            did,
            previous_operation_hash,
            actions,
        };
        Self::parse(&candidate.to_raw_operation()?)
    }

    /// Encode this validated update operation into its deterministic raw wire form.
    pub fn to_raw_operation(&self) -> Result<RawPrismOperation, Error> {
        let mut update = ProtoWriter::new();
        update.push_bytes(1, self.previous_operation_hash.as_bytes())?;
        update.push_bytes(2, self.did.suffix().as_bytes())?;
        for action in &self.actions {
            update.push_message(3, encode_update_action(action)?)?;
        }
        let payload = update.finish();
        RawPrismOperation::from_payload(OperationKind::UpdateDid, &payload)
    }
}

impl DeactivateDidOperation {
    /// Construct a deterministic deactivation operation.
    pub const fn new(
        did: CanonicalPrismDid,
        previous_operation_hash: identus_crypto::Sha256Digest,
    ) -> Self {
        Self {
            did,
            previous_operation_hash,
        }
    }

    /// Encode this validated deactivation operation into its deterministic raw wire form.
    pub fn to_raw_operation(&self) -> Result<RawPrismOperation, Error> {
        let mut deactivate = ProtoWriter::new();
        deactivate.push_bytes(1, self.previous_operation_hash.as_bytes())?;
        deactivate.push_bytes(2, self.did.suffix().as_bytes())?;
        let payload = deactivate.finish();
        RawPrismOperation::from_payload(OperationKind::DeactivateDid, &payload)
    }
}

fn encode_public_key(key: &PrismPublicKey) -> Result<Vec<u8>, Error> {
    let usage = match key.usage {
        KeyUsage::Master => proto::KeyUsage::Master,
        KeyUsage::Issuing => proto::KeyUsage::Issuing,
        KeyUsage::KeyAgreement => proto::KeyUsage::KeyAgreement,
        KeyUsage::Authentication => proto::KeyUsage::Authentication,
        KeyUsage::Revocation => proto::KeyUsage::Revocation,
        KeyUsage::CapabilityInvocation => proto::KeyUsage::CapabilityInvocation,
        KeyUsage::CapabilityDelegation => proto::KeyUsage::CapabilityDelegation,
        KeyUsage::Vdr => proto::KeyUsage::Vdr,
    };
    let (curve, data) = match &key.material {
        PrismPublicKeyMaterial::Secp256k1(key) => ("secp256k1", key.encode_compressed().to_vec()),
        PrismPublicKeyMaterial::Ed25519(key) => ("Ed25519", key.encode_vec()),
        PrismPublicKeyMaterial::X25519(key) => ("X25519", key.encode_vec()),
    };
    let mut compressed = ProtoWriter::new();
    compressed.push_bytes(1, curve.as_bytes())?;
    compressed.push_bytes(2, &data)?;
    let mut encoded = ProtoWriter::new();
    encoded.push_bytes(1, key.id.as_bytes())?;
    encoded.push_varint(2, usage as u64)?;
    encoded.push_message(9, compressed.finish())?;
    Ok(encoded.finish())
}

fn encode_service(service: &PrismService) -> Result<Vec<u8>, Error> {
    let service_type = service_type_to_wire(&service.service_type)?;
    let endpoint = service_endpoint_to_wire(&service.endpoint)?;
    let mut encoded = ProtoWriter::new();
    encoded.push_bytes(1, service.id.as_bytes())?;
    encoded.push_bytes(2, service_type.as_bytes())?;
    encoded.push_bytes(3, endpoint.as_bytes())?;
    Ok(encoded.finish())
}

fn service_type_to_wire(value: &ServiceType) -> Result<String, Error> {
    match value {
        ServiceType::One(value) => match parse_service_type(value)? {
            ServiceType::One(value) => Ok(value),
            ServiceType::Many(_) => Err(OperationError::InvalidValue.into()),
        },
        ServiceType::Many(values) => encode_json_bounded(values, MAX_TYPE_CHARACTERS),
    }
}

fn service_endpoint_to_wire(value: &ServiceEndpoint) -> Result<String, Error> {
    match value {
        ServiceEndpoint::One(ServiceEndpointValue::Uri(value)) => {
            match parse_service_endpoint(value)? {
                ServiceEndpoint::One(ServiceEndpointValue::Uri(value)) => Ok(value),
                _ => Err(OperationError::InvalidValue.into()),
            }
        }
        ServiceEndpoint::One(ServiceEndpointValue::Object(value)) => {
            encode_json_bounded(value, MAX_SERVICE_ENDPOINT_BYTES)
        }
        ServiceEndpoint::Many(values) => {
            encode_json_bounded(&EndpointValues(values), MAX_SERVICE_ENDPOINT_BYTES)
        }
    }
}

fn encode_update_action(action: &UpdateAction) -> Result<Vec<u8>, Error> {
    let (field, payload) = match action {
        UpdateAction::AddKey(key) => {
            let mut payload = ProtoWriter::new();
            payload.push_message(1, encode_public_key(key)?)?;
            (1, payload.finish())
        }
        UpdateAction::RemoveKey(id) => {
            validate_fragment(id)?;
            let mut payload = ProtoWriter::new();
            payload.push_bytes(1, id.as_bytes())?;
            (2, payload.finish())
        }
        UpdateAction::AddService(service) => {
            let mut payload = ProtoWriter::new();
            payload.push_message(1, encode_service(service)?)?;
            (3, payload.finish())
        }
        UpdateAction::RemoveService(id) => {
            validate_fragment(id)?;
            let mut payload = ProtoWriter::new();
            payload.push_bytes(1, id.as_bytes())?;
            (4, payload.finish())
        }
        UpdateAction::UpdateService {
            id,
            service_type,
            endpoint,
        } => {
            validate_fragment(id)?;
            let mut payload = ProtoWriter::new();
            payload.push_bytes(1, id.as_bytes())?;
            if let Some(service_type) = service_type {
                payload.push_bytes(2, service_type_to_wire(service_type)?.as_bytes())?;
            }
            if let Some(endpoint) = endpoint {
                payload.push_bytes(3, service_endpoint_to_wire(endpoint)?.as_bytes())?;
            }
            (5, payload.finish())
        }
        UpdateAction::PatchContext(contexts) => {
            let mut payload = ProtoWriter::new();
            for context in contexts {
                if context.len() > MAX_OPERATION_BYTES {
                    return Err(WireError::OperationTooLarge.into());
                }
                let validated = validate_uri_preserving_text(context)?;
                payload.push_bytes(1, validated.as_bytes())?;
            }
            (6, payload.finish())
        }
    };
    let mut encoded = ProtoWriter::new();
    encoded.push_message(field, payload)?;
    Ok(encoded.finish())
}

struct ProtoWriter {
    bytes: Vec<u8>,
}

impl ProtoWriter {
    fn new() -> Self {
        Self { bytes: Vec::new() }
    }

    fn push_bytes(&mut self, field: u64, value: &[u8]) -> Result<(), Error> {
        let additional = varint_len(field << 3 | 2)
            .saturating_add(varint_len(value.len() as u64))
            .saturating_add(value.len());
        self.reserve(additional)?;
        prost::encoding::encode_varint(field << 3 | 2, &mut self.bytes);
        prost::encoding::encode_varint(value.len() as u64, &mut self.bytes);
        self.bytes.extend_from_slice(value);
        Ok(())
    }

    fn push_message(&mut self, field: u64, value: Vec<u8>) -> Result<(), Error> {
        self.push_bytes(field, &value)
    }

    fn push_varint(&mut self, field: u64, value: u64) -> Result<(), Error> {
        let additional = varint_len(field << 3).saturating_add(varint_len(value));
        self.reserve(additional)?;
        prost::encoding::encode_varint(field << 3, &mut self.bytes);
        prost::encoding::encode_varint(value, &mut self.bytes);
        Ok(())
    }

    fn reserve(&mut self, additional: usize) -> Result<(), Error> {
        let next = self
            .bytes
            .len()
            .checked_add(additional)
            .ok_or(Error::InvalidWire(WireError::OperationTooLarge))?;
        if next > MAX_OPERATION_BYTES {
            return Err(WireError::OperationTooLarge.into());
        }
        self.bytes.reserve(additional);
        Ok(())
    }

    fn finish(self) -> Vec<u8> {
        self.bytes
    }
}

const fn varint_len(mut value: u64) -> usize {
    let mut length = 1;
    while value >= 0x80 {
        value >>= 7;
        length += 1;
    }
    length
}

struct BoundedJsonWriter {
    bytes: Vec<u8>,
    limit: usize,
}

impl Write for BoundedJsonWriter {
    fn write(&mut self, value: &[u8]) -> io::Result<usize> {
        let next = self
            .bytes
            .len()
            .checked_add(value.len())
            .ok_or_else(|| io::Error::other("bounded JSON length overflow"))?;
        if next > self.limit {
            return Err(io::Error::other("bounded JSON limit exceeded"));
        }
        self.bytes.extend_from_slice(value);
        Ok(value.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn encode_json_bounded<T: Serialize + ?Sized>(value: &T, limit: usize) -> Result<String, Error> {
    let mut writer = BoundedJsonWriter {
        bytes: Vec::new(),
        limit,
    };
    serde_json::to_writer(&mut writer, value)
        .map_err(|_| Error::InvalidOperation(OperationError::ValueTooLong))?;
    String::from_utf8(writer.bytes)
        .map_err(|_| Error::InvalidOperation(OperationError::InvalidValue))
}

struct EndpointValue<'a>(&'a ServiceEndpointValue);

impl Serialize for EndpointValue<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.0 {
            ServiceEndpointValue::Uri(value) => serializer.serialize_str(value),
            ServiceEndpointValue::Object(value) => value.serialize(serializer),
        }
    }
}

struct EndpointValues<'a>(&'a [ServiceEndpointValue]);

impl Serialize for EndpointValues<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut sequence = serializer.serialize_seq(Some(self.0.len()))?;
        for value in self.0 {
            sequence.serialize_element(&EndpointValue(value))?;
        }
        sequence.end()
    }
}

#[cfg(test)]
mod tests {
    use identus_crypto::{Secp256k1PrivateKey, Verifiable};

    use super::*;
    use crate::{MAX_SIGNATURE_BYTES, MAX_SIGNED_WITH_BYTES, PrismDidState, SignedPrismOperation};

    fn master(private: &Secp256k1PrivateKey, id: &str) -> PrismPublicKey {
        PrismPublicKey::new(
            id,
            KeyUsage::Master,
            PrismPublicKeyMaterial::Secp256k1(private.to_public_key()),
        )
        .unwrap()
    }

    #[test]
    fn builders_round_trip_create_update_deactivate_and_signature_envelopes() {
        let private = Secp256k1PrivateKey::from_slice(&[1; 32]).unwrap();
        let service = PrismService::new(
            "linked",
            ServiceType::one("LinkedDomains").unwrap(),
            ServiceEndpoint::uri("HTTPS://EXAMPLE.COM/a/../profile").unwrap(),
        )
        .unwrap();
        let create = CreateDidOperation::new(
            vec![master(&private, "master")],
            vec![service],
            vec!["HTTPS://EXAMPLE.COM/a/../context".to_owned()],
        )
        .unwrap();
        assert_eq!(create.contexts(), &["HTTPS://EXAMPLE.COM/a/../context"]);
        let raw_create = create.to_raw_operation().unwrap();
        assert_eq!(CreateDidOperation::parse(&raw_create).unwrap(), create);
        let signature = private.sign(raw_create.as_bytes());
        assert!(
            private
                .to_public_key()
                .verify(raw_create.as_bytes(), &signature)
        );
        let signed_create =
            SignedPrismOperation::from_signature("master", &signature, &raw_create).unwrap();
        let mut state = PrismDidState::from_signed_create(&signed_create).unwrap();

        let update = UpdateDidOperation::new(
            state.did().clone(),
            state.last_operation_hash(),
            vec![UpdateAction::PatchContext(vec![
                "HTTPS://EXAMPLE.COM/b/../updated".to_owned(),
            ])],
        )
        .unwrap();
        let raw_update = update.to_raw_operation().unwrap();
        assert_eq!(UpdateDidOperation::parse(&raw_update).unwrap(), update);
        let signed_update = SignedPrismOperation::from_signature(
            "master",
            &private.sign(raw_update.as_bytes()),
            &raw_update,
        )
        .unwrap();
        state.apply_signed(&signed_update).unwrap();
        assert_eq!(state.contexts(), &["HTTPS://EXAMPLE.COM/b/../updated"]);

        let deactivate =
            DeactivateDidOperation::new(state.did().clone(), state.last_operation_hash());
        let raw_deactivate = deactivate.to_raw_operation().unwrap();
        assert_eq!(
            DeactivateDidOperation::parse(&raw_deactivate).unwrap(),
            deactivate
        );
        let signed_deactivate = SignedPrismOperation::from_signature(
            "master",
            &private.sign(raw_deactivate.as_bytes()),
            &raw_deactivate,
        )
        .unwrap();
        state.apply_signed(&signed_deactivate).unwrap();
        assert!(state.is_deactivated());
    }

    #[test]
    fn create_builder_reproduces_the_pinned_preprod_wire_form() {
        let expected = hex::decode("0a3f0a3d123b0a076d61737465723010014a2e0a09736563703235366b31122102e45998e18117113835b972b43784fb6b6470b722cb2dc171c5fba00265bc80ee").unwrap();
        let raw = RawPrismOperation::parse(&expected).unwrap();
        let create = CreateDidOperation::parse(&raw).unwrap();

        assert_eq!(create.to_raw_operation().unwrap().as_bytes(), expected);
    }

    #[test]
    fn builders_revalidate_public_enum_payloads() {
        let private = Secp256k1PrivateKey::from_slice(&[1; 32]).unwrap();
        let invalid = PrismService {
            id: "service".to_owned(),
            service_type: ServiceType::Many(Vec::new()),
            endpoint: ServiceEndpoint::One(ServiceEndpointValue::Uri("relative".to_owned())),
        };
        assert!(
            CreateDidOperation::new(vec![master(&private, "master")], vec![invalid], Vec::new(),)
                .is_err()
        );
    }

    #[test]
    fn builders_reject_oversized_values_before_output_allocation() {
        let private = Secp256k1PrivateKey::from_slice(&[1; 32]).unwrap();
        let huge_context = format!("https://example.com/{}", "x".repeat(MAX_OPERATION_BYTES));
        assert_eq!(
            CreateDidOperation::new(
                vec![master(&private, "master")],
                Vec::new(),
                vec![huge_context],
            ),
            Err(Error::InvalidWire(WireError::OperationTooLarge))
        );

        assert!(ServiceType::many(vec!["x".repeat(MAX_TYPE_CHARACTERS + 1)]).is_err());
        let mut object = serde_json::Map::new();
        object.insert(
            "value".to_owned(),
            serde_json::Value::String("x".repeat(MAX_SERVICE_ENDPOINT_BYTES + 1)),
        );
        assert!(ServiceEndpoint::object(object).is_err());

        assert_eq!(
            CreateDidOperation::new(
                vec![master(&private, "master")],
                Vec::new(),
                vec!["https://example.com".to_owned(); MAX_CONTEXTS + 1],
            ),
            Err(Error::InvalidOperation(OperationError::TooManyItems))
        );

        let create =
            CreateDidOperation::new(vec![master(&private, "master")], Vec::new(), Vec::new())
                .unwrap()
                .to_raw_operation()
                .unwrap();
        assert!(
            SignedPrismOperation::from_signature(
                &"k".repeat(MAX_SIGNED_WITH_BYTES + 1),
                &[1],
                &create,
            )
            .is_err()
        );
        assert!(
            SignedPrismOperation::from_signature("master", &[1; MAX_SIGNATURE_BYTES + 1], &create,)
                .is_err()
        );
        assert!(SignedPrismOperation::from_signature("#master", &[1], &create).is_err());
    }
}
