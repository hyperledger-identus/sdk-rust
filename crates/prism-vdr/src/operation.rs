//! Validated PRISM storage operation values and authorization.

use std::fmt;

use identus_crypto::{Sha256Digest, Verifiable};
use identus_prism_did::{
    CanonicalPrismDid, KeyUsage, OperationKind, PrismDidState, PrismPublicKeyMaterial,
    SignedPrismOperation,
};
use prost::Message;

use crate::{
    Error, ErrorKind, MAX_IPFS_BYTES, MAX_NONCE_BYTES, MAX_STATUS_TEXT_BYTES, MAX_VDR_DATA_BYTES,
    proto, wire,
};

/// Opaque storage content defined by the PRISM wire profile.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum VdrData {
    /// No data field was supplied.
    None,
    /// Opaque bounded bytes.
    Bytes(Vec<u8>),
    /// Opaque bounded IPFS reference.
    Ipfs(String),
    /// Experimental structured status-list payload.
    StatusList(StatusListEntry),
}

impl fmt::Debug for VdrData {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => formatter.write_str("None"),
            Self::Bytes(value) => formatter
                .debug_struct("Bytes")
                .field("byte_len", &value.len())
                .finish(),
            Self::Ipfs(value) => formatter
                .debug_struct("Ipfs")
                .field("byte_len", &value.len())
                .finish(),
            Self::StatusList(value) => fmt::Debug::fmt(value, formatter),
        }
    }
}

/// Bounded PRISM status-list payload.
#[derive(Clone, PartialEq, Eq)]
pub struct StatusListEntry {
    state: i64,
    name: String,
    details: String,
}

impl StatusListEntry {
    /// Return the opaque protocol state number.
    #[must_use]
    pub const fn state(&self) -> i64 {
        self.state
    }

    /// Borrow the optional name text.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Borrow the optional details text.
    #[must_use]
    pub fn details(&self) -> &str {
        &self.details
    }
}

impl fmt::Debug for StatusListEntry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("StatusListEntry")
            .field("state", &self.state)
            .field("name_bytes", &self.name.len())
            .field("details_bytes", &self.details.len())
            .finish()
    }
}

/// Validated create-entry operation.
#[derive(Clone, PartialEq, Eq)]
pub struct CreateVdrEntry {
    owner: CanonicalPrismDid,
    nonce: Vec<u8>,
    data: VdrData,
    operation_hash: Sha256Digest,
}

/// Validated update-entry operation.
#[derive(Clone, PartialEq, Eq)]
pub struct UpdateVdrEntry {
    predecessor: Sha256Digest,
    data: VdrData,
    operation_hash: Sha256Digest,
}

/// Validated deactivate-entry operation.
#[derive(Clone, PartialEq, Eq)]
pub struct DeactivateVdrEntry {
    predecessor: Sha256Digest,
    operation_hash: Sha256Digest,
}

/// One validated PRISM VDR operation.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum VdrOperation {
    /// Create a root entry.
    Create(CreateVdrEntry),
    /// Replace or amend an existing entry.
    Update(UpdateVdrEntry),
    /// Mark an existing entry terminal.
    Deactivate(DeactivateVdrEntry),
}

impl fmt::Debug for CreateVdrEntry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CreateVdrEntry")
            .field("nonce_bytes", &self.nonce.len())
            .field("data", &self.data)
            .finish_non_exhaustive()
    }
}

impl fmt::Debug for UpdateVdrEntry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("UpdateVdrEntry")
            .field("data", &self.data)
            .finish_non_exhaustive()
    }
}

impl fmt::Debug for DeactivateVdrEntry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DeactivateVdrEntry")
            .finish_non_exhaustive()
    }
}

impl fmt::Debug for VdrOperation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Create(value) => fmt::Debug::fmt(value, formatter),
            Self::Update(value) => fmt::Debug::fmt(value, formatter),
            Self::Deactivate(value) => fmt::Debug::fmt(value, formatter),
        }
    }
}

impl VdrOperation {
    /// Decode a bounded signed PRISM VDR envelope without re-encoding signed bytes.
    pub fn parse(signed: &SignedPrismOperation) -> Result<Self, Error> {
        let hash = signed.operation_hash();
        match signed.operation_kind() {
            OperationKind::CreateVdrEntry => parse_create(signed.operation_payload(), hash),
            OperationKind::UpdateVdrEntry => parse_update(signed.operation_payload(), hash),
            OperationKind::DeactivateVdrEntry => parse_deactivate(signed.operation_payload(), hash),
            _ => Err(Error::new(ErrorKind::InvalidOperation)),
        }
    }

    /// Return the exact nested-operation hash.
    pub const fn operation_hash(&self) -> Sha256Digest {
        match self {
            Self::Create(value) => value.operation_hash,
            Self::Update(value) => value.operation_hash,
            Self::Deactivate(value) => value.operation_hash,
        }
    }

    /// Return the predecessor for update or deactivation.
    #[must_use]
    pub const fn predecessor(&self) -> Option<Sha256Digest> {
        match self {
            Self::Create(_) => None,
            Self::Update(value) => Some(value.predecessor),
            Self::Deactivate(value) => Some(value.predecessor),
        }
    }

    /// Verify owner state and the VDR-key signature over exact nested bytes.
    pub fn authorize(
        &self,
        signed: &SignedPrismOperation,
        owner: &CanonicalPrismDid,
        state: &PrismDidState,
    ) -> Result<(), Error> {
        if signed.operation_hash() != self.operation_hash()
            || state.did() != owner
            || state.is_deactivated()
            || matches!(self, Self::Create(value) if &value.owner != owner)
        {
            return Err(Error::new(ErrorKind::Unauthorized));
        }
        let Some(key) = state
            .active_public_keys()
            .find(|key| key.id() == signed.signed_with() && key.usage() == KeyUsage::Vdr)
        else {
            return Err(Error::new(ErrorKind::Unauthorized));
        };
        let PrismPublicKeyMaterial::Secp256k1(key) = key.material() else {
            return Err(Error::new(ErrorKind::Unauthorized));
        };
        if key.verify(signed.operation_bytes(), signed.signature()) {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Unauthorized))
        }
    }
}

impl CreateVdrEntry {
    /// Borrow the owning canonical DID.
    #[must_use]
    pub const fn owner(&self) -> &CanonicalPrismDid {
        &self.owner
    }

    /// Borrow the bounded nonce.
    #[must_use]
    pub fn nonce(&self) -> &[u8] {
        &self.nonce
    }

    /// Borrow the storage content.
    #[must_use]
    pub const fn data(&self) -> &VdrData {
        &self.data
    }
}

impl UpdateVdrEntry {
    /// Return the exact predecessor hash.
    pub const fn predecessor(&self) -> Sha256Digest {
        self.predecessor
    }

    /// Borrow the replacement or delta content.
    #[must_use]
    pub const fn data(&self) -> &VdrData {
        &self.data
    }
}

impl DeactivateVdrEntry {
    /// Return the exact predecessor hash.
    pub const fn predecessor(&self) -> Sha256Digest {
        self.predecessor
    }
}

fn parse_create(bytes: &[u8], operation_hash: Sha256Digest) -> Result<VdrOperation, Error> {
    wire::scan_payload(bytes, true, true)?;
    let value =
        proto::CreateStorageEntry::decode(bytes).map_err(|_| Error::new(ErrorKind::InvalidWire))?;
    let digest: [u8; 32] = value
        .did_prism_hash
        .try_into()
        .map_err(|_| Error::new(ErrorKind::InvalidOperation))?;
    if value.nonce.len() > MAX_NONCE_BYTES {
        return Err(Error::new(ErrorKind::ResourceLimitExceeded));
    }
    Ok(VdrOperation::Create(CreateVdrEntry {
        owner: CanonicalPrismDid::from_digest(&digest),
        nonce: value.nonce,
        data: parse_create_data(value.data)?,
        operation_hash,
    }))
}

fn parse_update(bytes: &[u8], operation_hash: Sha256Digest) -> Result<VdrOperation, Error> {
    wire::scan_payload(bytes, false, true)?;
    let value =
        proto::UpdateStorageEntry::decode(bytes).map_err(|_| Error::new(ErrorKind::InvalidWire))?;
    Ok(VdrOperation::Update(UpdateVdrEntry {
        predecessor: digest(&value.previous_event_hash)?,
        data: parse_update_data(value.data)?,
        operation_hash,
    }))
}

fn parse_deactivate(bytes: &[u8], operation_hash: Sha256Digest) -> Result<VdrOperation, Error> {
    wire::scan_payload(bytes, false, false)?;
    let value = proto::DeactivateStorageEntry::decode(bytes)
        .map_err(|_| Error::new(ErrorKind::InvalidWire))?;
    Ok(VdrOperation::Deactivate(DeactivateVdrEntry {
        predecessor: digest(&value.previous_event_hash)?,
        operation_hash,
    }))
}

fn digest(bytes: &[u8]) -> Result<Sha256Digest, Error> {
    Sha256Digest::from_bytes(bytes).map_err(|_| Error::new(ErrorKind::InvalidOperation))
}

fn parse_create_data(value: Option<proto::create_storage_entry::Data>) -> Result<VdrData, Error> {
    match value {
        None => Ok(VdrData::None),
        Some(proto::create_storage_entry::Data::Bytes(value)) => bytes_data(value),
        Some(proto::create_storage_entry::Data::Ipfs(value)) => ipfs_data(value),
        Some(proto::create_storage_entry::Data::StatusListEntry(value)) => status_data(value),
    }
}

fn parse_update_data(value: Option<proto::update_storage_entry::Data>) -> Result<VdrData, Error> {
    match value {
        None => Ok(VdrData::None),
        Some(proto::update_storage_entry::Data::Bytes(value)) => bytes_data(value),
        Some(proto::update_storage_entry::Data::Ipfs(value)) => ipfs_data(value),
        Some(proto::update_storage_entry::Data::StatusListEntry(value)) => status_data(value),
    }
}

fn bytes_data(value: Vec<u8>) -> Result<VdrData, Error> {
    if value.len() > MAX_VDR_DATA_BYTES {
        Err(Error::new(ErrorKind::ResourceLimitExceeded))
    } else {
        Ok(VdrData::Bytes(value))
    }
}

fn ipfs_data(value: String) -> Result<VdrData, Error> {
    if value.len() > MAX_IPFS_BYTES {
        Err(Error::new(ErrorKind::ResourceLimitExceeded))
    } else {
        Ok(VdrData::Ipfs(value))
    }
}

fn status_data(value: proto::StatusListEntry) -> Result<VdrData, Error> {
    if value.name.len() > MAX_STATUS_TEXT_BYTES || value.details.len() > MAX_STATUS_TEXT_BYTES {
        return Err(Error::new(ErrorKind::ResourceLimitExceeded));
    }
    Ok(VdrData::StatusList(StatusListEntry {
        state: value.state,
        name: value.name,
        details: value.details,
    }))
}
