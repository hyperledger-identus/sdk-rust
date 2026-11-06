//! Private protobuf messages for the pinned PRISM storage schema.

#[derive(Clone, PartialEq, prost::Message)]
pub(crate) struct CreateStorageEntry {
    #[prost(bytes = "vec", tag = "1")]
    pub did_prism_hash: Vec<u8>,
    #[prost(bytes = "vec", tag = "50")]
    pub nonce: Vec<u8>,
    #[prost(oneof = "create_storage_entry::Data", tags = "100, 101, 102")]
    pub data: Option<create_storage_entry::Data>,
}

pub(crate) mod create_storage_entry {
    #[derive(Clone, PartialEq, prost::Oneof)]
    pub(crate) enum Data {
        #[prost(bytes = "vec", tag = "100")]
        Bytes(Vec<u8>),
        #[prost(string, tag = "101")]
        Ipfs(String),
        #[prost(message, tag = "102")]
        StatusListEntry(super::StatusListEntry),
    }
}

#[derive(Clone, PartialEq, prost::Message)]
pub(crate) struct UpdateStorageEntry {
    #[prost(bytes = "vec", tag = "2")]
    pub previous_event_hash: Vec<u8>,
    #[prost(oneof = "update_storage_entry::Data", tags = "100, 101, 102")]
    pub data: Option<update_storage_entry::Data>,
}

pub(crate) mod update_storage_entry {
    #[derive(Clone, PartialEq, prost::Oneof)]
    pub(crate) enum Data {
        #[prost(bytes = "vec", tag = "100")]
        Bytes(Vec<u8>),
        #[prost(string, tag = "101")]
        Ipfs(String),
        #[prost(message, tag = "102")]
        StatusListEntry(super::StatusListEntry),
    }
}

#[derive(Clone, PartialEq, prost::Message)]
pub(crate) struct DeactivateStorageEntry {
    #[prost(bytes = "vec", tag = "2")]
    pub previous_event_hash: Vec<u8>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub(crate) struct StatusListEntry {
    #[prost(int64, tag = "1")]
    pub state: i64,
    #[prost(string, tag = "2")]
    pub name: String,
    #[prost(string, tag = "3")]
    pub details: String,
}
