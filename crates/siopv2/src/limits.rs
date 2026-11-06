use identus_jose::JwsLimits;

use crate::Siopv2Error;

/// Finite allocation and time policy for the draft-13 profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Siopv2Limits {
    jws: JwsLimits,
    max_json_depth: usize,
    max_json_nodes: usize,
    max_object_members: usize,
    max_live_key_bytes: usize,
    max_string_bytes: usize,
    max_response_body_bytes: usize,
    max_event_id_bytes: usize,
    max_request_lifetime_seconds: i64,
    max_clock_skew_seconds: i64,
}

impl Siopv2Limits {
    /// Construct one strictly positive finite limit policy.
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        jws: JwsLimits,
        max_json_depth: usize,
        max_json_nodes: usize,
        max_object_members: usize,
        max_live_key_bytes: usize,
        max_string_bytes: usize,
        max_response_body_bytes: usize,
        max_event_id_bytes: usize,
        max_request_lifetime_seconds: i64,
        max_clock_skew_seconds: i64,
    ) -> Result<Self, Siopv2Error> {
        if max_json_depth == 0
            || max_json_nodes == 0
            || max_object_members == 0
            || max_live_key_bytes == 0
            || max_string_bytes == 0
            || max_response_body_bytes == 0
            || max_event_id_bytes == 0
            || max_request_lifetime_seconds <= 0
            || max_clock_skew_seconds < 0
        {
            return Err(Siopv2Error::InvalidLimits);
        }
        Ok(Self {
            jws,
            max_json_depth,
            max_json_nodes,
            max_object_members,
            max_live_key_bytes,
            max_string_bytes,
            max_response_body_bytes,
            max_event_id_bytes,
            max_request_lifetime_seconds,
            max_clock_skew_seconds,
        })
    }

    /// Nested compact-JWS limits.
    pub const fn jws(self) -> JwsLimits {
        self.jws
    }
    /// Maximum JSON nesting depth.
    pub const fn max_json_depth(self) -> usize {
        self.max_json_depth
    }
    /// Maximum JSON node count.
    pub const fn max_json_nodes(self) -> usize {
        self.max_json_nodes
    }
    /// Maximum members in one JSON object.
    pub const fn max_object_members(self) -> usize {
        self.max_object_members
    }
    /// Maximum aggregate live JSON object-key bytes.
    pub const fn max_live_key_bytes(self) -> usize {
        self.max_live_key_bytes
    }
    /// Maximum accepted protocol string size in bytes.
    pub const fn max_string_bytes(self) -> usize {
        self.max_string_bytes
    }
    /// Maximum encoded direct-post body size in bytes.
    pub const fn max_response_body_bytes(self) -> usize {
        self.max_response_body_bytes
    }
    /// Maximum caller event-identity size in bytes.
    pub const fn max_event_id_bytes(self) -> usize {
        self.max_event_id_bytes
    }
    /// Maximum request and response lifetime in seconds.
    pub const fn max_request_lifetime_seconds(self) -> i64 {
        self.max_request_lifetime_seconds
    }
    /// Accepted request clock skew in seconds.
    pub const fn max_clock_skew_seconds(self) -> i64 {
        self.max_clock_skew_seconds
    }
}

impl Default for Siopv2Limits {
    fn default() -> Self {
        Self {
            jws: JwsLimits::default(),
            max_json_depth: 16,
            max_json_nodes: 256,
            max_object_members: 32,
            max_live_key_bytes: 4_096,
            max_string_bytes: 4_096,
            max_response_body_bytes: 65_536,
            max_event_id_bytes: 128,
            max_request_lifetime_seconds: 600,
            max_clock_skew_seconds: 60,
        }
    }
}
