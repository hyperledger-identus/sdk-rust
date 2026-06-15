//! Cross-language binding boundaries.
//!
//! This crate will own stable DTOs and binding-facing errors for
//! `WASM`, `Node`/`N-API`, `UniFFI`, Swift, Kotlin, TypeScript, React, and
//! React Native packages.

/// Component metadata.
pub const COMPONENT: identus_core::Component = identus_core::Component {
    name: "identus-bindings",
    summary: "Stable DTO and error surfaces for language bindings.",
};

/// Binding target that must consume Rust-owned semantics instead of
/// reimplementing SDK behavior.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BindingTarget {
    /// Browser and web-worker target through `WASM`.
    Wasm,
    /// Server-side JavaScript and TypeScript target through `N-API`.
    NodeNapi,
    /// Shared foreign-function bridge for Swift and Kotlin.
    UniFfi,
    /// Apple platform package over the shared mobile bridge.
    Swift,
    /// Android and JVM package over the shared mobile bridge.
    Kotlin,
    /// Browser and Node package facade.
    TypeScript,
    /// Web application package facade.
    React,
    /// Mobile application package facade.
    ReactNative,
}

impl BindingTarget {
    /// Stable target identifier used by manifests and conformance output.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::Wasm => "wasm",
            Self::NodeNapi => "node-napi",
            Self::UniFfi => "uniffi",
            Self::Swift => "swift",
            Self::Kotlin => "kotlin",
            Self::TypeScript => "typescript",
            Self::React => "react",
            Self::ReactNative => "react-native",
        }
    }

    /// Bridge technology expected for the target.
    #[must_use]
    pub const fn bridge(self) -> &'static str {
        match self {
            Self::Wasm => "wasm-bindgen",
            Self::NodeNapi => "napi-rs",
            Self::UniFfi | Self::Swift | Self::Kotlin => "uniffi",
            Self::TypeScript | Self::React => "wasm-or-napi-facade",
            Self::ReactNative => "native-module-facade",
        }
    }

    /// Package family that will expose the target.
    #[must_use]
    pub const fn package_family(self) -> &'static str {
        match self {
            Self::Wasm => "browser-wasm",
            Self::NodeNapi => "node",
            Self::UniFfi => "mobile-ffi",
            Self::Swift => "swiftpm",
            Self::Kotlin => "gradle-kmp",
            Self::TypeScript => "typescript",
            Self::React => "react",
            Self::ReactNative => "react-native",
        }
    }
}

/// Stable facade surface exposed through binding targets.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BindingSurface {
    /// Stable surface identifier.
    pub id: &'static str,
    /// Rust owner crate for the semantics.
    pub owner_crate: &'static str,
    /// Binding responsibility at the facade boundary.
    pub responsibility: &'static str,
}

/// Binding targets required for parity and wrapper migration.
pub const ALL_BINDING_TARGETS: &[BindingTarget] = &[
    BindingTarget::Wasm,
    BindingTarget::NodeNapi,
    BindingTarget::UniFfi,
    BindingTarget::Swift,
    BindingTarget::Kotlin,
    BindingTarget::TypeScript,
    BindingTarget::React,
    BindingTarget::ReactNative,
];

/// Facade surfaces required before wrapper packages can claim parity.
pub const ALL_BINDING_SURFACES: &[BindingSurface] = &[
    BindingSurface {
        id: "agent",
        owner_crate: "identus-agent",
        responsibility: "issuer, holder, verifier, peer, embedded mediator workflows, and high-level events",
    },
    BindingSurface {
        id: "wallet",
        owner_crate: "identus-wallet",
        responsibility: "wallet records, backup/restore, secure-store handles, credential inventory, and DID inventory",
    },
    BindingSurface {
        id: "did",
        owner_crate: "identus-did",
        responsibility: "DID parsing, DID URL parsing, method support, and resolver handles",
    },
    BindingSurface {
        id: "credential",
        owner_crate: "identus-credentials",
        responsibility: "credential parsing, verification, status checks, and typed verification errors",
    },
    BindingSurface {
        id: "presentation",
        owner_crate: "identus-presentations",
        responsibility: "presentation request parsing, selection, disclosure, and verifier result DTOs",
    },
    BindingSurface {
        id: "messaging",
        owner_crate: "identus-messaging",
        responsibility: "DIDComm message parsing, protocol state transitions, and transport-independent events",
    },
    BindingSurface {
        id: "openid4vc",
        owner_crate: "identus-openid4vc",
        responsibility: "OID4VCI, OID4VP, SIOPv2, HAIP, and federation-backed request and response DTOs",
    },
    BindingSurface {
        id: "trust",
        owner_crate: "identus-trust",
        responsibility: "trust anchors, trust-chain result DTOs, status-list result DTOs, and policy decisions",
    },
    BindingSurface {
        id: "adapters",
        owner_crate: "identus-adapters",
        responsibility: "platform-specific storage, signer, resolver, transport, VDR, and proximity adapter handles",
    },
];

/// Capability contract metadata consumed by binding manifests.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CapabilityContract {
    /// Stable capability contract identifier.
    pub id: &'static str,
    /// Rust owner crate for the capability semantics.
    pub owner_crate: &'static str,
    /// Required hexagonal ports for the capability boundary.
    pub required_ports: &'static [&'static str],
    /// Role or operation inputs accepted by the contract.
    pub role_inputs: &'static [&'static str],
    /// Outputs or events produced by the contract.
    pub outputs: &'static [&'static str],
    /// Stable typed error families surfaced through bindings.
    pub typed_errors: &'static [&'static str],
    /// Fixture families required before parity can be claimed.
    pub fixture_families: &'static [&'static str],
    /// Binding targets that must expose or consume the capability.
    pub wrapper_targets: &'static [&'static str],
    /// Acceptance tests proving the first behavior contract.
    pub acceptance_tests: &'static [&'static str],
}

impl CapabilityContract {
    /// Whether this capability is exposed for `target`.
    #[must_use]
    pub fn supports_target(self, target: BindingTarget) -> bool {
        let target_id = target.id();
        self.wrapper_targets.contains(&target_id)
    }
}

/// Binding manifest view for a target package family.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BindingManifest {
    /// Binding target identifier.
    pub target_id: &'static str,
    /// Package family for the target.
    pub package_family: &'static str,
    /// Capability contract ids exposed by the target.
    pub contract_ids: &'static [&'static str],
}

const ISSUER_PORTS: &[&str] = &[
    "SignerPort",
    "DidResolverPort",
    "CredentialFormatRegistry",
    "TransportPort",
    "StoragePort",
];
const HOLDER_PORTS: &[&str] = &[
    "WalletStore",
    "SecretResolverPort",
    "DidResolverPort",
    "TransportPort",
];
const VERIFIER_PORTS: &[&str] = &[
    "DidResolverPort",
    "CredentialVerifier",
    "PresentationVerifier",
    "TrustPolicyResolver",
];
const PEER_PORTS: &[&str] = &[
    "DidResolverPort",
    "TransportPort",
    "MessageStore",
    "ClockPort",
];
const WALLET_PORTS: &[&str] = &["SecureStore", "KeyStore", "BackupStore", "EntropySource"];
const MEDIATOR_PORTS: &[&str] = &[
    "MessageQueuePort",
    "DidResolverPort",
    "ClockPort",
    "TransportPort",
];
const DID_PORTS: &[&str] = &["DidResolverPort", "DidDereferencerPort", "VdrPort"];
const CREDENTIAL_PORTS: &[&str] = &[
    "CredentialFormatRegistry",
    "SignerPort",
    "DidResolverPort",
    "StatusVerifier",
];
const PRESENTATION_PORTS: &[&str] = &[
    "CredentialStore",
    "CredentialVerifier",
    "TrustPolicyResolver",
];
const DIDCOMM_PORTS: &[&str] = &[
    "DidResolverPort",
    "SecretResolverPort",
    "TransportPort",
    "MessageStore",
];
const OPENID4VC_PORTS: &[&str] = &[
    "HttpClientPort",
    "WalletStore",
    "CredentialVerifier",
    "PresentationVerifier",
    "TrustPolicyResolver",
];
const TRUST_STATUS_PORTS: &[&str] = &[
    "StatusResolver",
    "TrustAnchorResolver",
    "TrustEvidenceStore",
    "ClockPort",
];
const STORAGE_PORTS: &[&str] = &[
    "SecureStore",
    "KeyStore",
    "SecretResolver",
    "BackupStore",
    "EntropySource",
];
const BINDING_FACADE_PORTS: &[&str] =
    &["DtoCodec", "OpaqueHandleStore", "EventSink", "ErrorMapper"];

const TRANSCRIPT_INTEROP: &[&str] = &["transcript", "interop"];
const VECTOR_TRANSCRIPT_INTEROP: &[&str] = &["vector", "transcript", "interop"];
const STATIC_VECTOR: &[&str] = &["static-model", "vector"];
const STATIC_VECTOR_INFRA: &[&str] = &["static-model", "vector", "infrastructure"];
const STATIC_VECTOR_TRANSCRIPT_INFRA: &[&str] =
    &["static-model", "vector", "transcript", "infrastructure"];
const STATIC_INTEROP: &[&str] = &["static-model", "interop"];
const VECTOR_INTEROP: &[&str] = &["vector", "interop"];

const TS_SWIFT_KOTLIN_NODE_WASM: &[&str] = &["typescript", "swift", "kotlin", "node-napi", "wasm"];
const TS_SWIFT_KOTLIN_RN_NODE_WASM: &[&str] = &[
    "typescript",
    "swift",
    "kotlin",
    "react-native",
    "node-napi",
    "wasm",
];
const TS_SWIFT_KOTLIN_REACT_RN_WASM: &[&str] = &[
    "typescript",
    "swift",
    "kotlin",
    "react",
    "react-native",
    "wasm",
];
const TS_SWIFT_KOTLIN_NODE: &[&str] = &["typescript", "swift", "kotlin", "node-napi"];
const TS_SWIFT_KOTLIN_RN_NODE: &[&str] =
    &["typescript", "swift", "kotlin", "react-native", "node-napi"];
const TS_SWIFT_KOTLIN_REACT_RN_NODE_WASM: &[&str] = &[
    "typescript",
    "swift",
    "kotlin",
    "react",
    "react-native",
    "node-napi",
    "wasm",
];

/// Capability contracts promoted from
/// `fixtures/conformance/static-model/capability-contracts.json`.
pub const ALL_CAPABILITY_CONTRACTS: &[CapabilityContract] = &[
    CapabilityContract {
        id: "issuer",
        owner_crate: "identus-agent",
        required_ports: ISSUER_PORTS,
        role_inputs: &[
            "issuer_did",
            "subject_did",
            "credential_claims",
            "credential_format",
            "issuance_policy",
        ],
        outputs: &["credential_offer", "issued_credential", "issuance_event"],
        typed_errors: &[
            "unsupported_credential_format",
            "issuer_key_unavailable",
            "schema_validation_failed",
            "transport_failed",
        ],
        fixture_families: VECTOR_TRANSCRIPT_INTEROP,
        wrapper_targets: TS_SWIFT_KOTLIN_RN_NODE_WASM,
        acceptance_tests: &[
            "connectionless_credential_offer",
            "didcomm_issue_credential",
            "oid4vci_issuance",
        ],
    },
    CapabilityContract {
        id: "holder",
        owner_crate: "identus-agent",
        required_ports: HOLDER_PORTS,
        role_inputs: &["credential_offer", "presentation_request", "holder_policy"],
        outputs: &[
            "stored_credential",
            "credential_request",
            "proof_presentation",
            "holder_event",
        ],
        typed_errors: &[
            "credential_rejected",
            "holder_binding_missing",
            "required_disclosure_missing",
            "wallet_locked",
        ],
        fixture_families: VECTOR_TRANSCRIPT_INTEROP,
        wrapper_targets: TS_SWIFT_KOTLIN_REACT_RN_WASM,
        acceptance_tests: &[
            "receive_credentials_for_portable_formats",
            "connectionless_proof_request",
            "backup_restore",
        ],
    },
    CapabilityContract {
        id: "verifier",
        owner_crate: "identus-agent",
        required_ports: VERIFIER_PORTS,
        role_inputs: &[
            "presentation_request",
            "proof_presentation",
            "verification_policy",
        ],
        outputs: &["verification_decision", "verification_event"],
        typed_errors: &[
            "audience_mismatch",
            "challenge_mismatch",
            "domain_mismatch",
            "credential_revoked",
        ],
        fixture_families: VECTOR_TRANSCRIPT_INTEROP,
        wrapper_targets: TS_SWIFT_KOTLIN_NODE_WASM,
        acceptance_tests: &[
            "holder_presents_proof_to_verifier",
            "wrong_claim_request_is_rejected",
        ],
    },
    CapabilityContract {
        id: "peer",
        owner_crate: "identus-agent",
        required_ports: PEER_PORTS,
        role_inputs: &["peer_did", "message", "routing_policy"],
        outputs: &["sent_message", "received_message", "peer_event"],
        typed_errors: &[
            "unknown_did_method",
            "recipient_unreachable",
            "message_parse_failed",
            "routing_failed",
        ],
        fixture_families: TRANSCRIPT_INTEROP,
        wrapper_targets: TS_SWIFT_KOTLIN_RN_NODE,
        acceptance_tests: &["basic_message", "trust_ping", "mediated_delivery"],
    },
    CapabilityContract {
        id: "wallet",
        owner_crate: "identus-wallet",
        required_ports: WALLET_PORTS,
        role_inputs: &[
            "wallet_profile",
            "record_query",
            "backup_request",
            "restore_request",
        ],
        outputs: &["wallet_record", "backup_snapshot", "restore_report"],
        typed_errors: &[
            "wallet_locked",
            "record_not_found",
            "backup_invalid",
            "secret_unavailable",
        ],
        fixture_families: STATIC_VECTOR,
        wrapper_targets: TS_SWIFT_KOTLIN_RN_NODE_WASM,
        acceptance_tests: &["backup_restore", "secure_storage_no_secret_leak"],
    },
    CapabilityContract {
        id: "mediator",
        owner_crate: "identus-messaging",
        required_ports: MEDIATOR_PORTS,
        role_inputs: &["mediate_request", "pickup_request", "routing_message"],
        outputs: &["mediate_grant", "pickup_delivery", "queued_message"],
        typed_errors: &[
            "mediation_denied",
            "pickup_batch_empty",
            "recipient_unknown",
            "queue_unavailable",
        ],
        fixture_families: &["transcript", "infrastructure"],
        wrapper_targets: TS_SWIFT_KOTLIN_NODE,
        acceptance_tests: &[
            "embedded_mediator_pickup",
            "coordinate_mediation",
            "message_pickup",
        ],
    },
    CapabilityContract {
        id: "did",
        owner_crate: "identus-did",
        required_ports: DID_PORTS,
        role_inputs: &["did", "did_url", "method_operation"],
        outputs: &[
            "did_document",
            "dereferenced_resource",
            "method_operation_result",
        ],
        typed_errors: &[
            "invalid_did",
            "invalid_did_url",
            "unknown_did_method",
            "resolution_failed",
        ],
        fixture_families: STATIC_VECTOR_INFRA,
        wrapper_targets: TS_SWIFT_KOTLIN_NODE_WASM,
        acceptance_tests: &["did_parse", "did_url_parse", "prism_operation_lifecycle"],
    },
    CapabilityContract {
        id: "credential",
        owner_crate: "identus-credentials",
        required_ports: CREDENTIAL_PORTS,
        role_inputs: &["credential", "credential_claims", "credential_policy"],
        outputs: &[
            "credential_model",
            "credential_verification_decision",
            "issued_credential",
        ],
        typed_errors: &[
            "credential_expired",
            "signature_verification_failed",
            "unsupported_credential_format",
            "schema_validation_failed",
        ],
        fixture_families: VECTOR_INTEROP,
        wrapper_targets: TS_SWIFT_KOTLIN_NODE_WASM,
        acceptance_tests: &[
            "credential_negative_cases",
            "jwt_vc",
            "sd_jwt_vc",
            "anoncreds",
        ],
    },
    CapabilityContract {
        id: "presentation",
        owner_crate: "identus-presentations",
        required_ports: PRESENTATION_PORTS,
        role_inputs: &[
            "presentation_request",
            "available_credentials",
            "disclosure_policy",
        ],
        outputs: &[
            "presentation_submission",
            "presentation_verification_decision",
        ],
        typed_errors: &[
            "required_disclosure_missing",
            "presentation_definition_invalid",
            "holder_binding_missing",
            "unsupported_presentation_format",
        ],
        fixture_families: VECTOR_TRANSCRIPT_INTEROP,
        wrapper_targets: TS_SWIFT_KOTLIN_REACT_RN_WASM,
        acceptance_tests: &["presentation_exchange", "dcql", "connectionless_proof"],
    },
    CapabilityContract {
        id: "didcomm",
        owner_crate: "identus-messaging",
        required_ports: DIDCOMM_PORTS,
        role_inputs: &["plaintext_message", "packed_message", "protocol_state"],
        outputs: &["parsed_message", "packed_envelope", "protocol_event"],
        typed_errors: &[
            "message_parse_failed",
            "unsupported_algorithm",
            "wrong_recipient",
            "missing_secret",
        ],
        fixture_families: TRANSCRIPT_INTEROP,
        wrapper_targets: TS_SWIFT_KOTLIN_NODE_WASM,
        acceptance_tests: &[
            "oob",
            "basic_message",
            "issue_credential",
            "present_proof",
            "report_problem",
        ],
    },
    CapabilityContract {
        id: "openid4vc",
        owner_crate: "identus-openid4vc",
        required_ports: OPENID4VC_PORTS,
        role_inputs: &[
            "credential_offer",
            "authorization_request",
            "presentation_request",
            "client_metadata",
        ],
        outputs: &[
            "credential_request",
            "credential_response",
            "presentation_response",
            "openid4vc_event",
        ],
        typed_errors: &[
            "invalid_nonce",
            "invalid_state",
            "issuer_metadata_invalid",
            "trust_chain_rejected",
        ],
        fixture_families: TRANSCRIPT_INTEROP,
        wrapper_targets: TS_SWIFT_KOTLIN_REACT_RN_NODE_WASM,
        acceptance_tests: &[
            "oid4vci_authorization_code",
            "oid4vci_pre_authorized",
            "oid4vp_direct_post",
            "siopv2",
            "haip",
        ],
    },
    CapabilityContract {
        id: "trust-status",
        owner_crate: "identus-trust",
        required_ports: TRUST_STATUS_PORTS,
        role_inputs: &["status_reference", "trust_chain", "trust_policy"],
        outputs: &["status_decision", "trust_decision", "evidence_record"],
        typed_errors: &[
            "status_unavailable",
            "credential_revoked",
            "untrusted_anchor",
            "trust_chain_rejected",
        ],
        fixture_families: STATIC_VECTOR_TRANSCRIPT_INFRA,
        wrapper_targets: TS_SWIFT_KOTLIN_NODE_WASM,
        acceptance_tests: &[
            "bitstring_status_list",
            "token_status_list",
            "openid_federation_trust",
            "x509_iaca",
        ],
    },
    CapabilityContract {
        id: "storage",
        owner_crate: "identus-adapters",
        required_ports: STORAGE_PORTS,
        role_inputs: &[
            "storage_record",
            "secret_handle",
            "backup_request",
            "restore_request",
        ],
        outputs: &[
            "stored_record",
            "secret_handle",
            "backup_snapshot",
            "restore_report",
        ],
        typed_errors: &[
            "secret_unavailable",
            "storage_unavailable",
            "record_not_found",
            "backup_invalid",
        ],
        fixture_families: STATIC_VECTOR_INFRA,
        wrapper_targets: TS_SWIFT_KOTLIN_RN_NODE_WASM,
        acceptance_tests: &[
            "secure_storage_no_secret_leak",
            "backup_restore",
            "in_memory_adapter",
        ],
    },
    CapabilityContract {
        id: "binding-facade",
        owner_crate: "identus-bindings",
        required_ports: BINDING_FACADE_PORTS,
        role_inputs: &[
            "json_dto",
            "opaque_handle",
            "target_event",
            "binding_options",
        ],
        outputs: &[
            "json_result",
            "typed_error",
            "binding_event",
            "target_manifest",
        ],
        typed_errors: &[
            "dto_decode_failed",
            "handle_not_found",
            "target_unsupported",
            "secret_leak_blocked",
        ],
        fixture_families: STATIC_INTEROP,
        wrapper_targets: TS_SWIFT_KOTLIN_REACT_RN_NODE_WASM,
        acceptance_tests: &[
            "dto_round_trip",
            "wrapper_error_parity",
            "no_secret_leak",
            "target_build",
        ],
    },
];

const TYPESCRIPT_CONTRACTS: &[&str] = &[
    "issuer",
    "holder",
    "verifier",
    "peer",
    "wallet",
    "mediator",
    "did",
    "credential",
    "presentation",
    "didcomm",
    "openid4vc",
    "trust-status",
    "storage",
    "binding-facade",
];
const SWIFT_CONTRACTS: &[&str] = TYPESCRIPT_CONTRACTS;
const KOTLIN_CONTRACTS: &[&str] = TYPESCRIPT_CONTRACTS;
const REACT_CONTRACTS: &[&str] = &["holder", "presentation", "openid4vc", "binding-facade"];
const REACT_NATIVE_CONTRACTS: &[&str] = &[
    "issuer",
    "holder",
    "peer",
    "wallet",
    "presentation",
    "openid4vc",
    "storage",
    "binding-facade",
];
const NODE_NAPI_CONTRACTS: &[&str] = &[
    "issuer",
    "verifier",
    "peer",
    "wallet",
    "mediator",
    "did",
    "credential",
    "didcomm",
    "openid4vc",
    "trust-status",
    "storage",
    "binding-facade",
];
const WASM_CONTRACTS: &[&str] = &[
    "issuer",
    "holder",
    "verifier",
    "wallet",
    "did",
    "credential",
    "presentation",
    "didcomm",
    "openid4vc",
    "trust-status",
    "storage",
    "binding-facade",
];

/// Target manifests derived from capability contract wrapper targets.
pub const ALL_BINDING_MANIFESTS: &[BindingManifest] = &[
    BindingManifest {
        target_id: "typescript",
        package_family: "typescript",
        contract_ids: TYPESCRIPT_CONTRACTS,
    },
    BindingManifest {
        target_id: "swift",
        package_family: "swiftpm",
        contract_ids: SWIFT_CONTRACTS,
    },
    BindingManifest {
        target_id: "kotlin",
        package_family: "gradle-kmp",
        contract_ids: KOTLIN_CONTRACTS,
    },
    BindingManifest {
        target_id: "react",
        package_family: "react",
        contract_ids: REACT_CONTRACTS,
    },
    BindingManifest {
        target_id: "react-native",
        package_family: "react-native",
        contract_ids: REACT_NATIVE_CONTRACTS,
    },
    BindingManifest {
        target_id: "node-napi",
        package_family: "node",
        contract_ids: NODE_NAPI_CONTRACTS,
    },
    BindingManifest {
        target_id: "wasm",
        package_family: "browser-wasm",
        contract_ids: WASM_CONTRACTS,
    },
];

/// Return all binding targets.
#[must_use]
pub const fn all_binding_targets() -> &'static [BindingTarget] {
    ALL_BINDING_TARGETS
}

/// Return all binding facade surfaces.
#[must_use]
pub const fn all_binding_surfaces() -> &'static [BindingSurface] {
    ALL_BINDING_SURFACES
}

/// Return all capability contracts.
#[must_use]
pub const fn all_capability_contracts() -> &'static [CapabilityContract] {
    ALL_CAPABILITY_CONTRACTS
}

/// Return all binding target manifests.
#[must_use]
pub const fn all_binding_manifests() -> &'static [BindingManifest] {
    ALL_BINDING_MANIFESTS
}

#[cfg(test)]
mod tests {
    use super::{
        ALL_BINDING_MANIFESTS, ALL_BINDING_SURFACES, ALL_BINDING_TARGETS, ALL_CAPABILITY_CONTRACTS,
    };

    #[test]
    fn binding_targets_cover_web_mobile_and_server_wrappers() {
        for expected in [
            "wasm",
            "node-napi",
            "uniffi",
            "swift",
            "kotlin",
            "typescript",
            "react",
            "react-native",
        ] {
            assert!(
                ALL_BINDING_TARGETS
                    .iter()
                    .any(|target| target.id() == expected),
                "missing binding target {expected}"
            );
        }

        for target in ALL_BINDING_TARGETS {
            assert!(!target.bridge().is_empty());
            assert!(!target.package_family().is_empty());
        }
    }

    #[test]
    fn binding_surfaces_have_identus_owner_crates() {
        for expected in [
            "agent",
            "wallet",
            "did",
            "credential",
            "presentation",
            "messaging",
            "openid4vc",
            "trust",
            "adapters",
        ] {
            assert!(
                ALL_BINDING_SURFACES
                    .iter()
                    .any(|surface| surface.id == expected),
                "missing binding surface {expected}"
            );
        }

        for surface in ALL_BINDING_SURFACES {
            assert!(surface.owner_crate.starts_with("identus-"));
            assert!(!surface.responsibility.is_empty());
        }
    }

    #[test]
    fn capability_contracts_cover_binding_surfaces() {
        for expected in [
            "issuer",
            "holder",
            "verifier",
            "peer",
            "wallet",
            "mediator",
            "did",
            "credential",
            "presentation",
            "didcomm",
            "openid4vc",
            "trust-status",
            "storage",
            "binding-facade",
        ] {
            assert!(
                ALL_CAPABILITY_CONTRACTS
                    .iter()
                    .any(|contract| contract.id == expected),
                "missing capability contract {expected}"
            );
        }

        for contract in ALL_CAPABILITY_CONTRACTS {
            assert!(contract.owner_crate.starts_with("identus-"));
            assert!(!contract.required_ports.is_empty());
            assert!(!contract.role_inputs.is_empty());
            assert!(!contract.outputs.is_empty());
            assert!(!contract.typed_errors.is_empty());
            assert!(!contract.fixture_families.is_empty());
            assert!(!contract.wrapper_targets.is_empty());
            assert!(!contract.acceptance_tests.is_empty());
        }
    }

    #[test]
    fn binding_manifests_reference_supported_contracts() {
        for expected_target in [
            "typescript",
            "swift",
            "kotlin",
            "react",
            "react-native",
            "node-napi",
            "wasm",
        ] {
            assert!(
                ALL_BINDING_MANIFESTS
                    .iter()
                    .any(|manifest| manifest.target_id == expected_target),
                "missing binding manifest {expected_target}"
            );
        }

        for manifest in ALL_BINDING_MANIFESTS {
            assert!(!manifest.package_family.is_empty());
            assert!(!manifest.contract_ids.is_empty());

            for contract_id in manifest.contract_ids {
                let contract = ALL_CAPABILITY_CONTRACTS
                    .iter()
                    .find(|contract| contract.id == *contract_id)
                    .unwrap_or_else(|| {
                        panic!(
                            "manifest {} references unknown contract {contract_id}",
                            manifest.target_id
                        )
                    });
                assert!(
                    contract.wrapper_targets.contains(&manifest.target_id),
                    "{} manifest includes unsupported contract {contract_id}",
                    manifest.target_id
                );
            }
        }
    }
}
