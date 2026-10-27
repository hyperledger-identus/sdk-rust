//! Private checked-in prost messages for the pinned PRISM DID wire profile.
//!
//! These structs are an implementation detail. Public SDK contracts use the
//! validated domain types and exact-byte facades in sibling modules.

#[derive(Clone, PartialEq, prost::Message)]
pub(crate) struct CreateDid {
    #[prost(message, optional, tag = "1")]
    pub did_data: Option<DidCreationData>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub(crate) struct DidCreationData {
    #[prost(message, repeated, tag = "2")]
    pub public_keys: Vec<PublicKey>,
    #[prost(message, repeated, tag = "3")]
    pub services: Vec<Service>,
    #[prost(string, repeated, tag = "4")]
    pub context: Vec<String>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub(crate) struct UpdateDid {
    #[prost(bytes = "vec", tag = "1")]
    pub previous_operation_hash: Vec<u8>,
    #[prost(string, tag = "2")]
    pub id: String,
    #[prost(message, repeated, tag = "3")]
    pub actions: Vec<UpdateDidAction>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub(crate) struct DeactivateDid {
    #[prost(bytes = "vec", tag = "1")]
    pub previous_operation_hash: Vec<u8>,
    #[prost(string, tag = "2")]
    pub id: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub(crate) struct PublicKey {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(enumeration = "KeyUsage", tag = "2")]
    pub usage: i32,
    #[prost(oneof = "public_key::KeyData", tags = "8, 9")]
    pub key_data: Option<public_key::KeyData>,
}

pub(crate) mod public_key {
    #[derive(Clone, PartialEq, prost::Oneof)]
    pub(crate) enum KeyData {
        #[prost(message, tag = "8")]
        Ec(super::EcKeyData),
        #[prost(message, tag = "9")]
        Compressed(super::CompressedEcKeyData),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, prost::Enumeration)]
#[repr(i32)]
pub(crate) enum KeyUsage {
    Unknown = 0,
    Master = 1,
    Issuing = 2,
    KeyAgreement = 3,
    Authentication = 4,
    Revocation = 5,
    CapabilityInvocation = 6,
    CapabilityDelegation = 7,
    Vdr = 8,
}

#[derive(Clone, PartialEq, prost::Message)]
pub(crate) struct EcKeyData {
    #[prost(string, tag = "1")]
    pub curve: String,
    #[prost(bytes = "vec", tag = "2")]
    pub x: Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub y: Vec<u8>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub(crate) struct CompressedEcKeyData {
    #[prost(string, tag = "1")]
    pub curve: String,
    #[prost(bytes = "vec", tag = "2")]
    pub data: Vec<u8>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub(crate) struct Service {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(string, tag = "2")]
    pub r#type: String,
    #[prost(string, tag = "3")]
    pub service_endpoint: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub(crate) struct UpdateDidAction {
    #[prost(oneof = "update_did_action::Action", tags = "1, 2, 3, 4, 5, 6")]
    pub action: Option<update_did_action::Action>,
}

pub(crate) mod update_did_action {
    #[derive(Clone, PartialEq, prost::Oneof)]
    pub(crate) enum Action {
        #[prost(message, tag = "1")]
        AddKey(super::AddKeyAction),
        #[prost(message, tag = "2")]
        RemoveKey(super::RemoveKeyAction),
        #[prost(message, tag = "3")]
        AddService(super::AddServiceAction),
        #[prost(message, tag = "4")]
        RemoveService(super::RemoveServiceAction),
        #[prost(message, tag = "5")]
        UpdateService(super::UpdateServiceAction),
        #[prost(message, tag = "6")]
        PatchContext(super::PatchContextAction),
    }
}

#[derive(Clone, PartialEq, prost::Message)]
pub(crate) struct AddKeyAction {
    #[prost(message, optional, tag = "1")]
    pub key: Option<PublicKey>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub(crate) struct RemoveKeyAction {
    #[prost(string, tag = "1")]
    pub key_id: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub(crate) struct AddServiceAction {
    #[prost(message, optional, tag = "1")]
    pub service: Option<Service>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub(crate) struct RemoveServiceAction {
    #[prost(string, tag = "1")]
    pub service_id: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub(crate) struct UpdateServiceAction {
    #[prost(string, tag = "1")]
    pub service_id: String,
    #[prost(string, tag = "2")]
    pub r#type: String,
    #[prost(string, tag = "3")]
    pub service_endpoints: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub(crate) struct PatchContextAction {
    #[prost(string, repeated, tag = "1")]
    pub context: Vec<String>,
}
