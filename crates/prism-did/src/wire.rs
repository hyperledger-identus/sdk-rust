//! Bounded duplicate-aware PRISM protobuf envelope scanning.

use std::{fmt, ops::Range, sync::Arc};

use identus_crypto::{Sha256Digest, sha256};

use crate::{
    Error, MAX_OPERATION_BYTES, MAX_SIGNATURE_BYTES, MAX_SIGNED_OPERATION_BYTES,
    MAX_SIGNED_WITH_BYTES, WireError, operation::validate_fragment,
};

const MAX_PROTOBUF_FIELD_NUMBER: u64 = (1 << 29) - 1;

/// Stable discriminator for an operation carried by a PRISM envelope.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum OperationKind {
    /// Create a PRISM DID.
    CreateDid,
    /// Update a PRISM DID.
    UpdateDid,
    /// Announce a protocol version.
    ProtocolVersionUpdate,
    /// Deactivate a PRISM DID.
    DeactivateDid,
    /// Create an opaque VDR/storage entry owned by another capability.
    CreateVdrEntry,
    /// Update an opaque VDR/storage entry owned by another capability.
    UpdateVdrEntry,
    /// Deactivate an opaque VDR/storage entry owned by another capability.
    DeactivateVdrEntry,
}

/// An owned exact `PrismOperation` with a bounded recognized oneof payload.
#[derive(Clone, PartialEq, Eq)]
pub struct RawPrismOperation {
    bytes: Arc<[u8]>,
    payload: Range<usize>,
    kind: OperationKind,
}

impl RawPrismOperation {
    /// Scan exact operation bytes without decoding or re-encoding them.
    pub fn parse(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() > MAX_OPERATION_BYTES {
            return Err(WireError::OperationTooLarge.into());
        }
        let (kind, payload) = scan_operation(bytes)?;
        Ok(Self {
            bytes: Arc::from(bytes),
            payload,
            kind,
        })
    }

    /// Return the complete exact operation bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Return the recognized operation kind.
    #[must_use]
    pub const fn operation_kind(&self) -> OperationKind {
        self.kind
    }

    /// Return exact oneof payload bytes.
    #[must_use]
    pub fn operation_payload(&self) -> &[u8] {
        &self.bytes[self.payload.clone()]
    }

    /// Hash the complete exact operation bytes.
    pub fn operation_hash(&self) -> Sha256Digest {
        sha256(self.as_bytes())
    }

    pub(crate) fn from_payload(kind: OperationKind, payload: &[u8]) -> Result<Self, Error> {
        if payload.len() > MAX_OPERATION_BYTES {
            return Err(WireError::OperationTooLarge.into());
        }
        let field: u64 = match kind {
            OperationKind::CreateDid => 1,
            OperationKind::UpdateDid => 2,
            OperationKind::ProtocolVersionUpdate => 5,
            OperationKind::DeactivateDid => 6,
            OperationKind::CreateVdrEntry => 7,
            OperationKind::UpdateVdrEntry => 8,
            OperationKind::DeactivateVdrEntry => 9,
        };
        let mut bytes = Vec::with_capacity(payload.len().saturating_add(12));
        prost::encoding::encode_varint(field << 3 | 2, &mut bytes);
        prost::encoding::encode_varint(payload.len() as u64, &mut bytes);
        bytes.extend_from_slice(payload);
        Self::parse(&bytes)
    }
}

impl fmt::Debug for RawPrismOperation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RawPrismOperation")
            .field("kind", &self.kind)
            .field("operation_bytes", &self.bytes.len())
            .field("payload_bytes", &self.payload.len())
            .finish()
    }
}

/// An owned signed envelope retaining exact inbound protobuf bytes.
#[derive(Clone, PartialEq, Eq)]
pub struct SignedPrismOperation {
    bytes: Arc<[u8]>,
    signed_with: Range<usize>,
    signature: Range<usize>,
    operation: Range<usize>,
    payload: Range<usize>,
    kind: OperationKind,
}

impl SignedPrismOperation {
    /// Scan and validate a signed envelope without reconstructing signed bytes.
    pub fn parse(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() > MAX_SIGNED_OPERATION_BYTES {
            return Err(WireError::EnvelopeTooLarge.into());
        }
        let ranges = scan_envelope(bytes)?;
        Ok(Self {
            bytes: Arc::from(bytes),
            signed_with: ranges.signed_with,
            signature: ranges.signature,
            operation: ranges.operation,
            payload: ranges.payload,
            kind: ranges.kind,
        })
    }

    /// Return the complete exact signed envelope.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Return the exact UTF-8 signing key identifier.
    #[must_use]
    pub fn signed_with(&self) -> &str {
        std::str::from_utf8(&self.bytes[self.signed_with.clone()])
            .expect("wire scanner validated signed_with UTF-8")
    }

    /// Return exact signature bytes.
    #[must_use]
    pub fn signature(&self) -> &[u8] {
        &self.bytes[self.signature.clone()]
    }

    /// Return exact nested `PrismOperation` bytes used for signature verification and hashing.
    #[must_use]
    pub fn operation_bytes(&self) -> &[u8] {
        &self.bytes[self.operation.clone()]
    }

    /// Return the recognized operation kind.
    #[must_use]
    pub const fn operation_kind(&self) -> OperationKind {
        self.kind
    }

    /// Return exact oneof payload bytes for typed DID decoding or an outer VDR capability.
    #[must_use]
    pub fn operation_payload(&self) -> &[u8] {
        &self.bytes[self.payload.clone()]
    }

    /// Hash the exact nested operation bytes to obtain the PRISM operation hash / DID suffix.
    pub fn operation_hash(&self) -> Sha256Digest {
        sha256(self.operation_bytes())
    }

    /// Hash the exact signed envelope bytes to obtain its operation identifier.
    pub fn operation_id(&self) -> Sha256Digest {
        sha256(self.as_bytes())
    }

    /// Assemble a deterministic signed envelope around an exact raw operation.
    ///
    /// The caller remains responsible for obtaining the signature from an
    /// external key owner. The signature must cover [`RawPrismOperation::as_bytes`].
    pub fn from_signature(
        signed_with: &str,
        signature: &[u8],
        operation: &RawPrismOperation,
    ) -> Result<Self, Error> {
        if signature.is_empty() {
            return Err(WireError::RequiredFieldMissing.into());
        }
        if signed_with.len() > MAX_SIGNED_WITH_BYTES || signature.len() > MAX_SIGNATURE_BYTES {
            return Err(WireError::Malformed.into());
        }
        validate_fragment(signed_with)?;
        let mut bytes = Vec::with_capacity(
            signed_with
                .len()
                .saturating_add(signature.len())
                .saturating_add(operation.as_bytes().len())
                .saturating_add(24),
        );
        push_length_delimited(&mut bytes, 1, signed_with.as_bytes());
        push_length_delimited(&mut bytes, 2, signature);
        push_length_delimited(&mut bytes, 3, operation.as_bytes());
        Self::parse(&bytes)
    }
}

impl fmt::Debug for SignedPrismOperation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SignedPrismOperation")
            .field("kind", &self.kind)
            .field("signed_with_bytes", &self.signed_with.len())
            .field("signature_bytes", &self.signature.len())
            .field("operation_bytes", &self.operation.len())
            .finish_non_exhaustive()
    }
}

struct EnvelopeRanges {
    signed_with: Range<usize>,
    signature: Range<usize>,
    operation: Range<usize>,
    payload: Range<usize>,
    kind: OperationKind,
}

fn scan_envelope(bytes: &[u8]) -> Result<EnvelopeRanges, WireError> {
    let mut cursor = 0;
    let mut signed_with = None;
    let mut signature = None;
    let mut operation = None;

    while cursor < bytes.len() {
        let (key, next) = read_varint(bytes, cursor)?;
        cursor = next;
        let field = key >> 3;
        let wire_type = (key & 0x07) as u8;
        if !valid_field_number(field) {
            return Err(WireError::Malformed);
        }
        if matches!(field, 1..=3) {
            if wire_type != 2 {
                return Err(WireError::Malformed);
            }
            let range = read_length_delimited(bytes, &mut cursor)?;
            let slot = match field {
                1 => &mut signed_with,
                2 => &mut signature,
                3 => &mut operation,
                _ => unreachable!(),
            };
            if slot.replace(range).is_some() {
                return Err(WireError::DuplicateSingularField);
            }
        } else {
            cursor = skip_value(bytes, cursor, wire_type)?;
        }
    }

    let signed_with = signed_with.ok_or(WireError::RequiredFieldMissing)?;
    let signature = signature.ok_or(WireError::RequiredFieldMissing)?;
    let operation = operation.ok_or(WireError::RequiredFieldMissing)?;
    if signed_with.is_empty() || signature.is_empty() || operation.is_empty() {
        return Err(WireError::RequiredFieldMissing);
    }
    if signed_with.len() > MAX_SIGNED_WITH_BYTES || signature.len() > MAX_SIGNATURE_BYTES {
        return Err(WireError::Malformed);
    }
    std::str::from_utf8(&bytes[signed_with.clone()]).map_err(|_| WireError::Malformed)?;
    if operation.len() > MAX_OPERATION_BYTES {
        return Err(WireError::OperationTooLarge);
    }
    let (kind, relative_payload) = scan_operation(&bytes[operation.clone()])?;
    let payload = operation.start + relative_payload.start..operation.start + relative_payload.end;
    Ok(EnvelopeRanges {
        signed_with,
        signature,
        operation,
        payload,
        kind,
    })
}

fn scan_operation(bytes: &[u8]) -> Result<(OperationKind, Range<usize>), WireError> {
    let mut cursor = 0;
    let mut operation = None;
    while cursor < bytes.len() {
        let (key, next) = read_varint(bytes, cursor)?;
        cursor = next;
        let field = key >> 3;
        let wire_type = (key & 0x07) as u8;
        if !valid_field_number(field) {
            return Err(WireError::Malformed);
        }
        let kind = match field {
            1 => Some(OperationKind::CreateDid),
            2 => Some(OperationKind::UpdateDid),
            5 => Some(OperationKind::ProtocolVersionUpdate),
            6 => Some(OperationKind::DeactivateDid),
            7 => Some(OperationKind::CreateVdrEntry),
            8 => Some(OperationKind::UpdateVdrEntry),
            9 => Some(OperationKind::DeactivateVdrEntry),
            _ => None,
        };
        if let Some(kind) = kind {
            if wire_type != 2 {
                return Err(WireError::Malformed);
            }
            if operation.is_some() {
                return Err(WireError::DuplicateOperationKind);
            }
            operation = Some((kind, read_length_delimited(bytes, &mut cursor)?));
        } else {
            cursor = skip_value(bytes, cursor, wire_type)?;
        }
    }
    operation.ok_or(WireError::OperationMissing)
}

pub(crate) fn scan_did_payload(kind: OperationKind, bytes: &[u8]) -> Result<(), WireError> {
    match kind {
        OperationKind::CreateDid => scan_create_did(bytes),
        OperationKind::UpdateDid => scan_update_did(bytes),
        OperationKind::DeactivateDid => scan_deactivate_did(bytes),
        _ => Ok(()),
    }
}

fn scan_create_did(bytes: &[u8]) -> Result<(), WireError> {
    let mut seen = 0_u64;
    scan_fields(bytes, |field, wire_type, value| match field {
        1 => {
            claim(&mut seen, field, wire_type, 2)?;
            scan_did_creation_data(value.expect("length-delimited field has a value"))
        }
        _ => Ok(()),
    })
}

fn scan_did_creation_data(bytes: &[u8]) -> Result<(), WireError> {
    scan_fields(bytes, |field, wire_type, value| match field {
        2 => {
            expect_wire_type(wire_type, 2)?;
            scan_public_key(value.expect("length-delimited field has a value"))
        }
        3 => {
            expect_wire_type(wire_type, 2)?;
            scan_service(value.expect("length-delimited field has a value"))
        }
        4 => expect_wire_type(wire_type, 2),
        _ => Ok(()),
    })
}

fn scan_public_key(bytes: &[u8]) -> Result<(), WireError> {
    let mut seen = 0_u64;
    let mut key_data_seen = false;
    scan_fields(bytes, |field, wire_type, value| match field {
        1 => claim(&mut seen, field, wire_type, 2),
        2 => claim(&mut seen, field, wire_type, 0),
        8 | 9 => {
            expect_wire_type(wire_type, 2)?;
            if key_data_seen {
                return Err(WireError::DuplicateSingularField);
            }
            key_data_seen = true;
            let value = value.expect("length-delimited field has a value");
            if field == 8 {
                scan_ec_key_data(value)
            } else {
                scan_compressed_ec_key_data(value)
            }
        }
        _ => Ok(()),
    })
}

fn scan_ec_key_data(bytes: &[u8]) -> Result<(), WireError> {
    let mut seen = 0_u64;
    scan_fields(bytes, |field, wire_type, _| match field {
        1..=3 => claim(&mut seen, field, wire_type, 2),
        _ => Ok(()),
    })
}

fn scan_compressed_ec_key_data(bytes: &[u8]) -> Result<(), WireError> {
    let mut seen = 0_u64;
    scan_fields(bytes, |field, wire_type, _| match field {
        1 | 2 => claim(&mut seen, field, wire_type, 2),
        _ => Ok(()),
    })
}

fn scan_service(bytes: &[u8]) -> Result<(), WireError> {
    let mut seen = 0_u64;
    scan_fields(bytes, |field, wire_type, _| match field {
        1..=3 => claim(&mut seen, field, wire_type, 2),
        _ => Ok(()),
    })
}

fn scan_update_did(bytes: &[u8]) -> Result<(), WireError> {
    let mut seen = 0_u64;
    scan_fields(bytes, |field, wire_type, value| match field {
        1 | 2 => claim(&mut seen, field, wire_type, 2),
        3 => {
            expect_wire_type(wire_type, 2)?;
            scan_update_action(value.expect("length-delimited field has a value"))
        }
        _ => Ok(()),
    })
}

fn scan_update_action(bytes: &[u8]) -> Result<(), WireError> {
    let mut action_seen = false;
    scan_fields(bytes, |field, wire_type, value| match field {
        1..=6 => {
            expect_wire_type(wire_type, 2)?;
            if action_seen {
                return Err(WireError::DuplicateSingularField);
            }
            action_seen = true;
            let value = value.expect("length-delimited field has a value");
            match field {
                1 => scan_wrapped_message(value, scan_public_key),
                2 | 4 => scan_single_string(value),
                3 => scan_wrapped_message(value, scan_service),
                5 => scan_update_service(value),
                6 => scan_repeated_string(value),
                _ => unreachable!(),
            }
        }
        _ => Ok(()),
    })
}

fn scan_wrapped_message(
    bytes: &[u8],
    nested: fn(&[u8]) -> Result<(), WireError>,
) -> Result<(), WireError> {
    let mut seen = 0_u64;
    scan_fields(bytes, |field, wire_type, value| match field {
        1 => {
            claim(&mut seen, field, wire_type, 2)?;
            nested(value.expect("length-delimited field has a value"))
        }
        _ => Ok(()),
    })
}

fn scan_single_string(bytes: &[u8]) -> Result<(), WireError> {
    let mut seen = 0_u64;
    scan_fields(bytes, |field, wire_type, _| match field {
        1 => claim(&mut seen, field, wire_type, 2),
        _ => Ok(()),
    })
}

fn scan_repeated_string(bytes: &[u8]) -> Result<(), WireError> {
    scan_fields(bytes, |field, wire_type, _| match field {
        1 => expect_wire_type(wire_type, 2),
        _ => Ok(()),
    })
}

fn scan_update_service(bytes: &[u8]) -> Result<(), WireError> {
    let mut seen = 0_u64;
    scan_fields(bytes, |field, wire_type, _| match field {
        1..=3 => claim(&mut seen, field, wire_type, 2),
        _ => Ok(()),
    })
}

fn scan_deactivate_did(bytes: &[u8]) -> Result<(), WireError> {
    let mut seen = 0_u64;
    scan_fields(bytes, |field, wire_type, _| match field {
        1 | 2 => claim(&mut seen, field, wire_type, 2),
        _ => Ok(()),
    })
}

fn claim(seen: &mut u64, field: u64, wire_type: u8, expected: u8) -> Result<(), WireError> {
    expect_wire_type(wire_type, expected)?;
    let bit = 1_u64
        .checked_shl(u32::try_from(field).map_err(|_| WireError::Malformed)?)
        .ok_or(WireError::Malformed)?;
    if *seen & bit != 0 {
        return Err(WireError::DuplicateSingularField);
    }
    *seen |= bit;
    Ok(())
}

fn expect_wire_type(actual: u8, expected: u8) -> Result<(), WireError> {
    if actual == expected {
        Ok(())
    } else {
        Err(WireError::Malformed)
    }
}

fn scan_fields(
    bytes: &[u8],
    mut visit: impl FnMut(u64, u8, Option<&[u8]>) -> Result<(), WireError>,
) -> Result<(), WireError> {
    let mut cursor = 0;
    while cursor < bytes.len() {
        let (key, next) = read_varint(bytes, cursor)?;
        cursor = next;
        let field = key >> 3;
        let wire_type = (key & 0x07) as u8;
        if !valid_field_number(field) {
            return Err(WireError::Malformed);
        }
        if wire_type == 2 {
            let range = read_length_delimited(bytes, &mut cursor)?;
            visit(field, wire_type, Some(&bytes[range]))?;
        } else {
            cursor = skip_value(bytes, cursor, wire_type)?;
            visit(field, wire_type, None)?;
        }
    }
    Ok(())
}

const fn valid_field_number(field: u64) -> bool {
    field > 0 && field <= MAX_PROTOBUF_FIELD_NUMBER
}

fn read_varint(bytes: &[u8], start: usize) -> Result<(u64, usize), WireError> {
    let mut value = 0_u64;
    for offset in 0..10 {
        let index = start.checked_add(offset).ok_or(WireError::Malformed)?;
        let byte = *bytes.get(index).ok_or(WireError::Malformed)?;
        if offset == 9 && byte > 1 {
            return Err(WireError::Malformed);
        }
        value |= u64::from(byte & 0x7f) << (offset * 7);
        if byte & 0x80 == 0 {
            return Ok((value, index + 1));
        }
    }
    Err(WireError::Malformed)
}

fn read_length_delimited(bytes: &[u8], cursor: &mut usize) -> Result<Range<usize>, WireError> {
    let (length, start) = read_varint(bytes, *cursor)?;
    let length = usize::try_from(length).map_err(|_| WireError::Malformed)?;
    let end = start.checked_add(length).ok_or(WireError::Malformed)?;
    if end > bytes.len() {
        return Err(WireError::Malformed);
    }
    *cursor = end;
    Ok(start..end)
}

fn skip_value(bytes: &[u8], cursor: usize, wire_type: u8) -> Result<usize, WireError> {
    match wire_type {
        0 => read_varint(bytes, cursor).map(|(_, next)| next),
        1 => cursor
            .checked_add(8)
            .filter(|end| *end <= bytes.len())
            .ok_or(WireError::Malformed),
        2 => {
            let mut next = cursor;
            read_length_delimited(bytes, &mut next)?;
            Ok(next)
        }
        5 => cursor
            .checked_add(4)
            .filter(|end| *end <= bytes.len())
            .ok_or(WireError::Malformed),
        _ => Err(WireError::Malformed),
    }
}

fn push_length_delimited(output: &mut Vec<u8>, field: u64, value: &[u8]) {
    prost::encoding::encode_varint(field << 3 | 2, output);
    prost::encoding::encode_varint(value.len() as u64, output);
    output.extend_from_slice(value);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn decode_hex(value: &str) -> Vec<u8> {
        hex::decode(value).expect("test fixture is valid hex")
    }

    #[test]
    fn preserves_exact_preprod_operation_and_hashes() {
        let bytes = decode_hex(
            "0a076d61737465723012483046022100b8e57b812375ea9e3c3ae79459c8695a149419bf68c2e9e18a185dddce4b08af022100aa5fa6dc7dee7c0d0b91ce05c204527e84bba9ef87e278708512d0c91ac2403c1a410a3f0a3d123b0a076d61737465723010014a2e0a09736563703235366b31122102e45998e18117113835b972b43784fb6b6470b722cb2dc171c5fba00265bc80ee",
        );
        let parsed = SignedPrismOperation::parse(&bytes).unwrap();

        assert_eq!(parsed.as_bytes(), bytes);
        assert_eq!(parsed.signed_with(), "master0");
        assert_eq!(parsed.operation_kind(), OperationKind::CreateDid);
        assert_eq!(parsed.operation_bytes().len(), 65);
        assert_eq!(
            hex::encode(parsed.operation_id().as_bytes()),
            "5873d42d38408e10f5e9dc624c0fc18fbf4b5a7871046501eb7f7e4997f11360"
        );
        assert_eq!(
            hex::encode(parsed.operation_hash().as_bytes()),
            "44fb82ad3544a8c8acb763c01a6fc7d92d81c38104096319345a4618d1a4275c"
        );
        let raw = RawPrismOperation::parse(parsed.operation_bytes()).unwrap();
        let rebuilt =
            SignedPrismOperation::from_signature(parsed.signed_with(), parsed.signature(), &raw)
                .unwrap();
        assert_eq!(rebuilt.as_bytes(), bytes);
    }

    #[test]
    fn debug_redacts_wire_values() {
        let bytes = decode_hex(
            "0a076d61737465723012473045022036b87a5180bf8ccd3e0c7590c5e6649cb00d5b9937b730ccb0d679d63911c78a022100f68c1a3698f328a1599025195cbc926c7f3f87bb723d0249e8182c93049cbb681a410a3f0a3d123b0a076d61737465723010014a2e0a09736563703235366b31122103376eae12841185739e97c1d4bf190aa97fe8227c40f31d4c49afb722ebde484c",
        );
        let parsed = SignedPrismOperation::parse(&bytes).unwrap();
        let debug = format!("{parsed:?}");

        assert!(!debug.contains("master0"));
        assert!(!debug.contains("36b87a"));
        assert!(debug.contains("operation_bytes"));
    }

    #[test]
    fn rejects_duplicate_security_relevant_fields_and_kinds() {
        let operation = [0x0a, 0x00];
        let mut envelope = vec![0x0a, 0x01, b'k', 0x12, 0x01, 0xaa, 0x1a, 0x02];
        envelope.extend_from_slice(&operation);
        let mut duplicate_field = envelope.clone();
        duplicate_field.extend_from_slice(&[0x0a, 0x01, b'x']);
        assert_eq!(
            SignedPrismOperation::parse(&duplicate_field),
            Err(Error::InvalidWire(WireError::DuplicateSingularField))
        );

        let operation = [0x0a, 0x00, 0x12, 0x00];
        let mut duplicate_kind = vec![0x0a, 0x01, b'k', 0x12, 0x01, 0xaa, 0x1a, 0x04];
        duplicate_kind.extend_from_slice(&operation);
        assert_eq!(
            SignedPrismOperation::parse(&duplicate_kind),
            Err(Error::InvalidWire(WireError::DuplicateOperationKind))
        );
    }

    #[test]
    fn rejects_malformed_and_oversized_inputs() {
        assert_eq!(
            SignedPrismOperation::parse(&vec![0; MAX_SIGNED_OPERATION_BYTES + 1]),
            Err(Error::InvalidWire(WireError::EnvelopeTooLarge))
        );
        assert_eq!(
            SignedPrismOperation::parse(&[0x0a, 0x80]),
            Err(Error::InvalidWire(WireError::Malformed))
        );
        assert_eq!(
            SignedPrismOperation::parse(&[0x0a, 0x00, 0x12, 0x00, 0x1a, 0x00]),
            Err(Error::InvalidWire(WireError::RequiredFieldMissing))
        );

        let mut invalid_field = Vec::new();
        prost::encoding::encode_varint((MAX_PROTOBUF_FIELD_NUMBER + 1) << 3, &mut invalid_field);
        invalid_field.push(0);
        assert_eq!(
            SignedPrismOperation::parse(&invalid_field),
            Err(Error::InvalidWire(WireError::Malformed))
        );

        let mut operation = vec![0x0a, 0x00];
        operation.extend_from_slice(&invalid_field);
        assert_eq!(
            RawPrismOperation::parse(&operation),
            Err(Error::InvalidWire(WireError::Malformed))
        );
        assert_eq!(
            scan_fields(&invalid_field, |_, _, _| Ok(())),
            Err(WireError::Malformed)
        );
    }
}
