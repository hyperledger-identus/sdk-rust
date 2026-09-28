use crate::{
    Error,
    error::ResolutionError,
    wire_json::{JsonWireError, JsonWireLimits, validate_unique_object_names},
};

/// Maximum raw JSON size accepted by resolution result entry points.
pub const MAX_DID_RESOLUTION_RESULT_BYTES: usize = 512 * 1_024;
/// Maximum containers nested in raw resolution result JSON during preflight.
pub const MAX_DID_RESOLUTION_WIRE_DEPTH: usize = 64;
/// Maximum JSON values visited during raw resolution result preflight.
pub const MAX_DID_RESOLUTION_WIRE_NODES: usize = 16_384;
/// Maximum members permitted in one raw resolution result JSON object.
pub const MAX_DID_RESOLUTION_WIRE_OBJECT_MEMBERS: usize = 128;
/// Maximum decoded object-name bytes retained simultaneously during preflight.
pub const MAX_DID_RESOLUTION_WIRE_LIVE_KEY_BYTES: usize = 128 * 1_024;
pub(super) fn validate_resolution_wire(input: &[u8]) -> Result<(), Error> {
    validate_unique_object_names(
        input,
        JsonWireLimits {
            max_depth: MAX_DID_RESOLUTION_WIRE_DEPTH,
            max_nodes: MAX_DID_RESOLUTION_WIRE_NODES,
            max_object_members: MAX_DID_RESOLUTION_WIRE_OBJECT_MEMBERS,
            max_live_key_bytes: MAX_DID_RESOLUTION_WIRE_LIVE_KEY_BYTES,
        },
    )
    .map_err(|reason| Error::InvalidResolution(map_wire_error(reason)))
}

const fn map_wire_error(reason: JsonWireError) -> ResolutionError {
    match reason {
        JsonWireError::DuplicateName => ResolutionError::DuplicateJsonProperty,
        JsonWireError::TooDeep => ResolutionError::WireTooDeep,
        JsonWireError::TooManyNodes | JsonWireError::TooManyLiveKeyBytes => {
            ResolutionError::WireTooLarge
        }
        JsonWireError::TooManyMembers => ResolutionError::WireTooManyProperties,
        JsonWireError::Malformed => ResolutionError::MalformedJson,
    }
}
