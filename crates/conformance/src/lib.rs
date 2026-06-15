//! Specification conformance catalog and test boundaries.
//!
//! This crate is intentionally metadata-first. It creates an executable
//! contract: every specification planned for `sdk-rust` must have an owner,
//! source reference, conformance mode, and backlog task before production
//! implementation starts.

/// Component metadata.
pub const COMPONENT: identus_core::Component = identus_core::Component {
    name: "identus-conformance",
    summary: "Specification conformance catalog and test boundaries.",
};

/// Functional area covered by a specification.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum SpecArea {
    /// Cryptographic keys and signing representations.
    Crypto,
    /// Decentralized identifiers and DID resolution.
    Did,
    /// Verifiable credentials, credential formats, and schemas.
    Credential,
    /// Presentation request, selection, disclosure, and verification.
    Presentation,
    /// `DIDComm` messaging and protocols.
    Messaging,
    /// Mediator routing and pickup.
    Mediation,
    /// `OpenID4VC`, `SIOPv2`, and related trust protocols.
    OpenId,
    /// Trust anchors, attestations, and status mechanisms.
    Trust,
    /// Mobile, browser, and proximity exchange APIs.
    Platform,
}

/// Crate family responsible for conformance.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum OwnerCrate {
    /// `identus-crypto`.
    Crypto,
    /// `identus-did`.
    Did,
    /// `identus-credentials`.
    Credentials,
    /// `identus-presentations`.
    Presentations,
    /// `identus-messaging`.
    Messaging,
    /// `identus-openid4vc`.
    OpenId4Vc,
    /// `identus-trust`.
    Trust,
    /// `identus-adapters`.
    Adapters,
    /// `identus-bindings`.
    Bindings,
}

/// Conformance strategy for a specification.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ConformanceMode {
    /// Static model/API tests can prove basic conformance without network
    /// services.
    StaticModel,
    /// Deterministic vectors should be stored and executed in Rust.
    Vector,
    /// Protocol transcript tests should be executed without external services.
    Transcript,
    /// Cross-implementation interoperability tests are required.
    Interop,
    /// Optional infrastructure is needed for full coverage.
    Infrastructure,
}

/// Planned support maturity.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum SupportStage {
    /// Required for feature parity with existing Identus behavior.
    Parity,
    /// Required for the target `sdk-rust` architecture beyond parity.
    Roadmap,
    /// Required only behind an optional feature or infrastructure gate.
    Optional,
}

/// One specification planned for `sdk-rust`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Specification {
    /// Stable machine-readable id.
    pub id: &'static str,
    /// Human-readable name.
    pub name: &'static str,
    /// Functional area.
    pub area: SpecArea,
    /// Owner crate family.
    pub owner: OwnerCrate,
    /// Primary specification or source `URI`.
    pub source_uri: &'static str,
    /// Conformance mode.
    pub mode: ConformanceMode,
    /// Planned support stage.
    pub stage: SupportStage,
    /// Backlog task that must add or deepen executable tests.
    pub backlog_task: &'static str,
}

/// Specifications that `sdk-rust` plans to support.
#[rustfmt::skip]
pub const ALL_SPECIFICATIONS: &[Specification] = &[
    spec("bip-39", "BIP-39 mnemonic seed derivation", SpecArea::Crypto, OwnerCrate::Crypto, "https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki", ConformanceMode::Vector, SupportStage::Parity, "T032"),
    spec("jwk", "JSON Web Key", SpecArea::Crypto, OwnerCrate::Crypto, "https://www.rfc-editor.org/rfc/rfc7517", ConformanceMode::Vector, SupportStage::Parity, "T037"),
    spec("jose", "JOSE for credentials", SpecArea::Crypto, OwnerCrate::Crypto, "https://www.rfc-editor.org/rfc/rfc7515", ConformanceMode::Vector, SupportStage::Parity, "T037"),
    spec("cose", "COSE for credentials", SpecArea::Crypto, OwnerCrate::Crypto, "https://www.rfc-editor.org/rfc/rfc9052", ConformanceMode::Vector, SupportStage::Roadmap, "T037"),
    spec("did-core", "W3C DID Core", SpecArea::Did, OwnerCrate::Did, "https://www.w3.org/TR/did-core/", ConformanceMode::StaticModel, SupportStage::Parity, "T038"),
    spec("did-url", "DID URL parsing and dereferencing", SpecArea::Did, OwnerCrate::Did, "https://www.w3.org/TR/did-core/#did-url-syntax", ConformanceMode::Vector, SupportStage::Parity, "T038"),
    spec("did-prism", "PRISM DID Method", SpecArea::Did, OwnerCrate::Did, "https://github.com/hyperledger-identus/prism-did-method-spec", ConformanceMode::Vector, SupportStage::Parity, "T032"),
    spec("did-peer-1", "Peer DID 1.0", SpecArea::Did, OwnerCrate::Did, "https://identity.foundation/peer-did-method-spec/", ConformanceMode::Vector, SupportStage::Parity, "T038"),
    spec("did-web", "Web DID Method", SpecArea::Did, OwnerCrate::Did, "https://w3c-ccg.github.io/did-method-web/", ConformanceMode::Vector, SupportStage::Roadmap, "T059"),
    spec("did-key", "Key DID Method", SpecArea::Did, OwnerCrate::Did, "https://w3c-ccg.github.io/did-key-spec/", ConformanceMode::Vector, SupportStage::Roadmap, "T059"),
    spec("did-jwk", "JWK DID Method", SpecArea::Did, OwnerCrate::Did, "https://github.com/quartzjer/did-jwk/blob/main/spec.md", ConformanceMode::Vector, SupportStage::Roadmap, "T059"),
    spec("did-pkh", "Public Key Hash DID Method", SpecArea::Did, OwnerCrate::Did, "https://github.com/w3c-ccg/did-pkh", ConformanceMode::Vector, SupportStage::Roadmap, "T059"),
    spec("did-example", "Example DID Method", SpecArea::Did, OwnerCrate::Did, "https://www.w3.org/TR/did-core/", ConformanceMode::StaticModel, SupportStage::Optional, "T059"),
    spec("vc-data-model", "W3C Verifiable Credentials Data Model", SpecArea::Credential, OwnerCrate::Credentials, "https://www.w3.org/TR/vc-data-model/", ConformanceMode::StaticModel, SupportStage::Parity, "T040"),
    spec("vc-json-schema", "W3C VC JSON Schema", SpecArea::Credential, OwnerCrate::Credentials, "https://www.w3.org/TR/vc-json-schema/", ConformanceMode::Vector, SupportStage::Parity, "T033"),
    spec("jwt-vc", "JWT VC and VP", SpecArea::Credential, OwnerCrate::Credentials, "https://www.w3.org/TR/vc-data-model/#json-web-token", ConformanceMode::Vector, SupportStage::Parity, "T033"),
    spec("sd-jwt", "Selective Disclosure JWT", SpecArea::Credential, OwnerCrate::Credentials, "https://www.ietf.org/archive/id/draft-ietf-oauth-selective-disclosure-jwt.html", ConformanceMode::Vector, SupportStage::Parity, "T033"),
    spec("sd-jwt-vc", "SD-JWT VC", SpecArea::Credential, OwnerCrate::Credentials, "https://www.ietf.org/archive/id/draft-ietf-oauth-sd-jwt-vc.html", ConformanceMode::Vector, SupportStage::Parity, "T033"),
    spec("anoncreds-v1", "AnonCreds v1.0", SpecArea::Credential, OwnerCrate::Credentials, "https://hyperledger.github.io/anoncreds-spec/", ConformanceMode::Interop, SupportStage::Parity, "T040"),
    spec("anoncreds-did-prism", "DID:PRISM AnonCreds method", SpecArea::Credential, OwnerCrate::Credentials, "https://hyperledger.github.io/anoncreds-spec/#did-methods", ConformanceMode::Interop, SupportStage::Parity, "T040"),
    spec("anoncreds-http", "HTTP AnonCreds method", SpecArea::Credential, OwnerCrate::Credentials, "https://hyperledger.github.io/anoncreds-spec/#http-method", ConformanceMode::Interop, SupportStage::Parity, "T040"),
    spec("openbadges-3", "Open Badges 3.0", SpecArea::Credential, OwnerCrate::Credentials, "https://www.imsglobal.org/spec/ob/v3p0", ConformanceMode::StaticModel, SupportStage::Roadmap, "T043"),
    spec("iso-mdoc", "ISO mdoc", SpecArea::Credential, OwnerCrate::Credentials, "https://www.iso.org/standard/69084.html", ConformanceMode::Interop, SupportStage::Roadmap, "T043"),
    spec("presentation-exchange", "DIF Presentation Exchange", SpecArea::Presentation, OwnerCrate::Presentations, "https://identity.foundation/presentation-exchange/", ConformanceMode::Vector, SupportStage::Parity, "T042"),
    spec("dcql", "Digital Credentials Query Language", SpecArea::Presentation, OwnerCrate::Presentations, "https://openid.net/specs/openid-4-verifiable-presentations-1_0.html#name-digital-credentials-query-l", ConformanceMode::Vector, SupportStage::Roadmap, "T042"),
    spec("didcomm-v2", "DIDComm Messaging v2.x", SpecArea::Messaging, OwnerCrate::Messaging, "https://identity.foundation/didcomm-messaging/spec/v2.1/", ConformanceMode::Transcript, SupportStage::Parity, "T044"),
    spec("didcomm-oob-2", "DIDComm Out-of-Band 2.0", SpecArea::Messaging, OwnerCrate::Messaging, "https://identity.foundation/didcomm-messaging/spec/v2.1/#out-of-band-messages", ConformanceMode::Transcript, SupportStage::Parity, "T013"),
    spec("didcomm-basicmessage-2", "DIDComm BasicMessage 2.0", SpecArea::Messaging, OwnerCrate::Messaging, "https://identity.foundation/didcomm-messaging/spec/v2.1/", ConformanceMode::Transcript, SupportStage::Parity, "T044"),
    spec("didcomm-discover-features-2", "DIDComm Discover Features 2.0", SpecArea::Messaging, OwnerCrate::Messaging, "https://didcomm.org/discover-features/2.0/", ConformanceMode::Transcript, SupportStage::Parity, "T064"),
    spec("didcomm-issue-credential-3", "DIDComm Issue Credential 3.0", SpecArea::Messaging, OwnerCrate::Messaging, "https://didcomm.org/issue-credential/3.0/", ConformanceMode::Transcript, SupportStage::Parity, "T044"),
    spec("didcomm-present-proof-3", "DIDComm Present Proof 3.0", SpecArea::Messaging, OwnerCrate::Messaging, "https://didcomm.org/present-proof/3.0/", ConformanceMode::Transcript, SupportStage::Parity, "T044"),
    spec("didcomm-report-problem-2", "DIDComm Report Problem 2.0", SpecArea::Messaging, OwnerCrate::Messaging, "https://didcomm.org/report-problem/2.0/", ConformanceMode::Transcript, SupportStage::Parity, "T044"),
    spec("didcomm-revocation-notification", "Identus Revocation Notification 1.0", SpecArea::Messaging, OwnerCrate::Messaging, "https://github.com/hyperledger-identus/cloud-agent", ConformanceMode::Transcript, SupportStage::Parity, "T041"),
    spec("coordinate-mediation-2", "Coordinate Mediation 2.0", SpecArea::Mediation, OwnerCrate::Messaging, "https://didcomm.org/coordinate-mediation/2.0/", ConformanceMode::Transcript, SupportStage::Parity, "T045"),
    spec("coordinate-mediation-3", "Coordinate Mediation 3.0", SpecArea::Mediation, OwnerCrate::Messaging, "https://didcomm.org/coordinate-mediation/3.0/", ConformanceMode::Transcript, SupportStage::Roadmap, "T045"),
    spec("message-pickup-3", "Message Pickup 3.0", SpecArea::Mediation, OwnerCrate::Messaging, "https://didcomm.org/messagepickup/3.0/", ConformanceMode::Transcript, SupportStage::Parity, "T045"),
    spec("trust-ping-2", "Trust Ping 2.0", SpecArea::Messaging, OwnerCrate::Messaging, "https://didcomm.org/trust-ping/2.0/", ConformanceMode::Transcript, SupportStage::Parity, "T044"),
    spec("routing-2", "Routing 2.0", SpecArea::Mediation, OwnerCrate::Messaging, "https://didcomm.org/routing/2.0/", ConformanceMode::Transcript, SupportStage::Parity, "T045"),
    spec("oid4vci-1", "OpenID for Verifiable Credential Issuance", SpecArea::OpenId, OwnerCrate::OpenId4Vc, "https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0.html", ConformanceMode::Interop, SupportStage::Parity, "T046"),
    spec("oid4vp-1", "OpenID for Verifiable Presentations", SpecArea::OpenId, OwnerCrate::OpenId4Vc, "https://openid.net/specs/openid-4-verifiable-presentations-1_0-final.html", ConformanceMode::Interop, SupportStage::Parity, "T047"),
    spec("siopv2", "Self-Issued OpenID Provider v2", SpecArea::OpenId, OwnerCrate::OpenId4Vc, "https://openid.net/specs/openid-connect-self-issued-v2-1_0.html", ConformanceMode::Interop, SupportStage::Roadmap, "T047"),
    spec("openid-federation-1", "OpenID Federation 1.0", SpecArea::OpenId, OwnerCrate::Trust, "https://openid.net/specs/openid-federation-1_0.html", ConformanceMode::Interop, SupportStage::Roadmap, "T039"),
    spec("openid4vc-haip", "OpenID4VC High Assurance Interoperability Profile", SpecArea::OpenId, OwnerCrate::OpenId4Vc, "https://openid.net/specs/openid4vc-high-assurance-interoperability-profile-1_0.html", ConformanceMode::Interop, SupportStage::Roadmap, "T046"),
    spec("bitstring-status-list", "W3C Bitstring Status List", SpecArea::Trust, OwnerCrate::Trust, "https://www.w3.org/TR/vc-bitstring-status-list/", ConformanceMode::Vector, SupportStage::Parity, "T041"),
    spec("ietf-token-status-list", "IETF Token Status List", SpecArea::Trust, OwnerCrate::Trust, "https://datatracker.ietf.org/doc/draft-ietf-oauth-status-list/", ConformanceMode::Vector, SupportStage::Roadmap, "T041"),
    spec("x509-iaca", "X.509 and IACA trust anchors", SpecArea::Trust, OwnerCrate::Trust, "https://www.itu.int/rec/T-REC-X.509", ConformanceMode::Interop, SupportStage::Roadmap, "T039"),
    spec("digital-credentials-api", "Digital Credentials API", SpecArea::Platform, OwnerCrate::Bindings, "https://wicg.github.io/digital-credentials/", ConformanceMode::Interop, SupportStage::Roadmap, "T050"),
    spec("proximity-ble-nfc-qr", "BLE, NFC, and QR proximity handoff", SpecArea::Platform, OwnerCrate::Adapters, "https://www.iso.org/standard/69084.html", ConformanceMode::Infrastructure, SupportStage::Optional, "T048"),
];

/// Return all planned specification conformance entries.
#[must_use]
pub const fn all_specifications() -> &'static [Specification] {
    ALL_SPECIFICATIONS
}

/// Find a specification by id.
#[must_use]
pub fn find_specification(id: &str) -> Option<&'static Specification> {
    ALL_SPECIFICATIONS
        .iter()
        .find(|specification| specification.id == id)
}

#[allow(clippy::too_many_arguments)]
const fn spec(
    id: &'static str,
    name: &'static str,
    area: SpecArea,
    owner: OwnerCrate,
    source_uri: &'static str,
    mode: ConformanceMode,
    stage: SupportStage,
    backlog_task: &'static str,
) -> Specification {
    Specification {
        id,
        name,
        area,
        owner,
        source_uri,
        mode,
        stage,
        backlog_task,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ALL_SPECIFICATIONS, ConformanceMode, OwnerCrate, SpecArea, SupportStage, find_specification,
    };
    use std::collections::BTreeSet;
    use std::path::Path;
    use std::process::Command;

    const TASKS: &str = include_str!("../../../specs/001-sdk-rust-platform-core/tasks.md");
    const CONSTITUTION: &str = include_str!("../../../.specify/memory/constitution.md");
    const WORKSPACE_MANIFEST: &str = include_str!("../../../Cargo.toml");
    const PLAN: &str = include_str!("../../../specs/001-sdk-rust-platform-core/plan.md");
    const COMPONENTS: &str = include_str!("../../../docs/architecture/components.md");
    const CURRENT_WORKSPACE_ARCHITECTURE: &str =
        include_str!("../../../docs/architecture/current-workspace.md");
    const CORE_ERROR_CONVENTIONS: &str =
        include_str!("../../../docs/architecture/core-error-conventions.md");
    const NAMING_POLICY: &str = include_str!("../../../docs/architecture/naming-policy.md");
    const CRATE_LAYOUT_ADR: &str = include_str!("../../../docs/architecture/adr-crate-layout.md");
    const LEGACY_MIGRATION_MAP: &str = include_str!("../../../docs/migration/legacy-sdk-map.md");
    const DID_METHODS: &str = include_str!("../../../docs/architecture/did-methods.md");
    const DIDCOMM_PROTOCOLS: &str = include_str!("../../../docs/architecture/didcomm-protocols.md");
    const NEOPRISM_CONVERGENCE_ADR: &str =
        include_str!("../../../docs/architecture/adr-neoprism-convergence.md");
    const BINDINGS_CORE_BOUNDARY_ADR: &str =
        include_str!("../../../docs/architecture/adr-bindings-core-boundary.md");
    const PLUGIN_EXTENSION_ADR: &str =
        include_str!("../../../docs/architecture/adr-plugin-extension-compatibility.md");
    const CAPABILITY_API_ADR: &str =
        include_str!("../../../docs/architecture/adr-capability-driven-api.md");
    const DIDCOMM_PACK_UNPACK_ADR: &str =
        include_str!("../../../docs/architecture/adr-didcomm-pack-unpack-dependency.md");
    const OPENID4VC_CONFORMANCE_ADR: &str =
        include_str!("../../../docs/architecture/adr-openid4vc-conformance.md");
    const SECURE_STORAGE_ADR: &str =
        include_str!("../../../docs/architecture/adr-secure-storage.md");
    const TRUST_STATUS_ADR: &str =
        include_str!("../../../docs/architecture/adr-trust-status-policy.md");
    const FIXTURE_SCHEMA_ADR: &str =
        include_str!("../../../docs/architecture/adr-fixture-schema-policy.md");
    const INTEGRATION_RUNNER_CONTRACT: &str =
        include_str!("../../../docs/testing/integration-runner-contract.md");
    const FIXTURE_POLICY_DOC: &str = include_str!("../../../docs/testing/fixtures.md");
    const FIXTURE_ROOT_README: &str = include_str!("../../../fixtures/conformance/README.md");
    const FIXTURE_SCHEMA_README: &str = include_str!("../../../fixtures/schema/README.md");
    const STATIC_MODEL_SCHEMA: &str =
        include_str!("../../../fixtures/schema/static-model-fixture.schema.json");
    const VECTOR_SCHEMA: &str = include_str!("../../../fixtures/schema/vector-fixture.schema.json");
    const TRANSCRIPT_SCHEMA: &str =
        include_str!("../../../fixtures/schema/transcript-fixture.schema.json");
    const INTEROP_SCHEMA: &str =
        include_str!("../../../fixtures/schema/interop-fixture.schema.json");
    const INFRASTRUCTURE_SCHEMA: &str =
        include_str!("../../../fixtures/schema/infrastructure-fixture.schema.json");
    const STATIC_MODEL_README: &str =
        include_str!("../../../fixtures/conformance/static-model/README.md");
    const CAPABILITY_CONTRACTS: &str =
        include_str!("../../../fixtures/conformance/static-model/capability-contracts.json");
    const WORKSPACE_DEPENDENCY_GRAPH: &str =
        include_str!("../../../fixtures/conformance/static-model/workspace-dependency-graph.json");
    const TYPED_ERROR_CATALOG: &str =
        include_str!("../../../fixtures/conformance/static-model/typed-error-catalog.json");
    const TRUST_STATUS_POLICY: &str =
        include_str!("../../../fixtures/conformance/static-model/trust-status-policy.json");
    const VECTOR_README: &str = include_str!("../../../fixtures/conformance/vector/README.md");
    const DID_PRISM_VECTOR_README: &str =
        include_str!("../../../fixtures/conformance/vector/did-prism/README.md");
    const DID_PRISM_VECTOR: &str = include_str!(
        "../../../fixtures/conformance/vector/did-prism/deterministic-master-key.json"
    );
    const DID_PRISM_OPERATION_VDR_VECTOR: &str = include_str!(
        "../../../fixtures/conformance/vector/did-prism/prism-operation-vdr-lifecycle.json"
    );
    const CREDENTIAL_NEGATIVE_VECTOR_README: &str =
        include_str!("../../../fixtures/conformance/vector/credential-verification/README.md");
    const CREDENTIAL_NEGATIVE_VECTOR: &str = include_str!(
        "../../../fixtures/conformance/vector/credential-verification/negative-cases.json"
    );
    const TRANSCRIPT_README: &str =
        include_str!("../../../fixtures/conformance/transcript/README.md");
    const DIDCOMM_TRANSCRIPT_README: &str =
        include_str!("../../../fixtures/conformance/transcript/didcomm/README.md");
    const DIDCOMM_MEDIATION_PICKUP_TRANSCRIPT: &str = include_str!(
        "../../../fixtures/conformance/transcript/didcomm/edge-mediation-and-pickup.json"
    );
    const DIDCOMM_CREDENTIAL_PRESENTATION_TRANSCRIPT: &str = include_str!(
        "../../../fixtures/conformance/transcript/didcomm/credential-presentation-revocation.json"
    );
    const DIDCOMM_CONNECTIONLESS_OOB_TRANSCRIPT: &str = include_str!(
        "../../../fixtures/conformance/transcript/didcomm/connectionless-oob-credential-proof.json"
    );
    const DIDCOMM_COMPATIBILITY_TRANSCRIPT: &str =
        include_str!("../../../fixtures/conformance/transcript/didcomm/compatibility-aliases.json");
    const OPENID4VC_TRANSCRIPT_README: &str =
        include_str!("../../../fixtures/conformance/transcript/openid4vc/README.md");
    const OPENID4VC_CORE_TRANSCRIPT: &str =
        include_str!("../../../fixtures/conformance/transcript/openid4vc/core-flows.json");
    const INTEROP_README: &str = include_str!("../../../fixtures/conformance/interop/README.md");
    const WRAPPER_API_PARITY: &str =
        include_str!("../../../fixtures/conformance/interop/wrapper-api-parity.json");
    const WRAPPER_API_PARITY_GENERATOR: &str =
        include_str!("../../../tools/generate-wrapper-api-parity.mjs");
    const AGENTS_DOC: &str = include_str!("../../../AGENTS.md");
    const AGENT_DOC: &str = include_str!("../../../AGENT.md");
    const AGENTIC_SDLC_DOC: &str = include_str!("../../../docs/maintenance/agentic-sdlc.md");
    const AGENTIC_SDLC_HARNESS: &str = include_str!("../../../tools/check-agent-sdlc.mjs");
    const GITHUB_LABELS: &str = include_str!("../../../.github/labels.yml");
    const ISSUE_TEMPLATE_CONFIG: &str = include_str!("../../../.github/ISSUE_TEMPLATE/config.yml");
    const ISSUE_TEMPLATE_CAPABILITY: &str =
        include_str!("../../../.github/ISSUE_TEMPLATE/ssi-capability.yml");
    const ISSUE_TEMPLATE_DEFECT: &str = include_str!("../../../.github/ISSUE_TEMPLATE/defect.yml");
    const ISSUE_TEMPLATE_MAINTENANCE: &str =
        include_str!("../../../.github/ISSUE_TEMPLATE/maintenance.yml");
    const DISCUSSION_TEMPLATE_ARCHITECTURE: &str =
        include_str!("../../../.github/DISCUSSION_TEMPLATE/architecture.md");
    const DISCUSSION_TEMPLATE_RESEARCH: &str =
        include_str!("../../../.github/DISCUSSION_TEMPLATE/research.md");
    const PULL_REQUEST_TEMPLATE: &str = include_str!("../../../.github/pull_request_template.md");
    const CONFORMANCE_WORKFLOW: &str = include_str!("../../../.github/workflows/conformance.yml");
    const CONFORMANCE_DOC: &str = include_str!("../../../docs/testing/conformance.md");
    const INFRASTRUCTURE_README: &str =
        include_str!("../../../fixtures/conformance/infrastructure/README.md");
    const NEOPRISM_VDR_INFRASTRUCTURE: &str =
        include_str!("../../../fixtures/conformance/infrastructure/neoprism-vdr-adapters.json");
    const CRATE_SOURCES: &[(&str, &str)] = &[
        (
            "crates/adapters/src/lib.rs",
            include_str!("../../../crates/adapters/src/lib.rs"),
        ),
        (
            "crates/agent/src/lib.rs",
            include_str!("../../../crates/agent/src/lib.rs"),
        ),
        (
            "crates/bindings/src/lib.rs",
            include_str!("../../../crates/bindings/src/lib.rs"),
        ),
        (
            "crates/core/src/lib.rs",
            include_str!("../../../crates/core/src/lib.rs"),
        ),
        (
            "crates/credentials/src/lib.rs",
            include_str!("../../../crates/credentials/src/lib.rs"),
        ),
        (
            "crates/crypto/src/lib.rs",
            include_str!("../../../crates/crypto/src/lib.rs"),
        ),
        (
            "crates/did/src/lib.rs",
            include_str!("../../../crates/did/src/lib.rs"),
        ),
        (
            "crates/messaging/src/lib.rs",
            include_str!("../../../crates/messaging/src/lib.rs"),
        ),
        (
            "crates/openid4vc/src/lib.rs",
            include_str!("../../../crates/openid4vc/src/lib.rs"),
        ),
        (
            "crates/presentations/src/lib.rs",
            include_str!("../../../crates/presentations/src/lib.rs"),
        ),
        (
            "crates/trust/src/lib.rs",
            include_str!("../../../crates/trust/src/lib.rs"),
        ),
        (
            "crates/wallet/src/lib.rs",
            include_str!("../../../crates/wallet/src/lib.rs"),
        ),
    ];

    const REQUIRED_IDS: &[&str] = &[
        "bip-39",
        "jwk",
        "jose",
        "cose",
        "did-core",
        "did-url",
        "did-prism",
        "did-peer-1",
        "did-web",
        "did-key",
        "did-jwk",
        "did-pkh",
        "did-example",
        "vc-data-model",
        "vc-json-schema",
        "jwt-vc",
        "sd-jwt",
        "sd-jwt-vc",
        "anoncreds-v1",
        "anoncreds-did-prism",
        "anoncreds-http",
        "openbadges-3",
        "iso-mdoc",
        "presentation-exchange",
        "dcql",
        "didcomm-v2",
        "didcomm-oob-2",
        "didcomm-basicmessage-2",
        "didcomm-discover-features-2",
        "didcomm-issue-credential-3",
        "didcomm-present-proof-3",
        "didcomm-report-problem-2",
        "didcomm-revocation-notification",
        "coordinate-mediation-2",
        "coordinate-mediation-3",
        "message-pickup-3",
        "trust-ping-2",
        "routing-2",
        "oid4vci-1",
        "oid4vp-1",
        "siopv2",
        "openid-federation-1",
        "openid4vc-haip",
        "bitstring-status-list",
        "ietf-token-status-list",
        "x509-iaca",
        "digital-credentials-api",
        "proximity-ble-nfc-qr",
    ];

    #[test]
    fn required_specifications_have_conformance_entries() {
        for required_id in REQUIRED_IDS {
            assert!(
                find_specification(required_id).is_some(),
                "missing conformance entry for {required_id}"
            );
        }
    }

    #[test]
    fn conformance_entries_are_unique_and_complete() {
        let mut ids = BTreeSet::new();

        for specification in ALL_SPECIFICATIONS {
            assert!(!specification.id.is_empty(), "spec id is empty");
            assert!(
                ids.insert(specification.id),
                "duplicate spec id {}",
                specification.id
            );
            assert!(
                !specification.name.is_empty(),
                "{} has no name",
                specification.id
            );
            assert!(
                specification.source_uri.starts_with("https://"),
                "{} source is not an https URI",
                specification.id
            );
            assert!(
                specification.backlog_task.starts_with('T'),
                "{} has no backlog task",
                specification.id
            );
        }
    }

    #[test]
    fn public_surfaces_use_ssi_domain_names() {
        let audited_documents = [
            ("constitution", CONSTITUTION),
            ("workspace manifest", WORKSPACE_MANIFEST),
            ("spec plan", PLAN),
            ("spec tasks", TASKS),
            ("component architecture", COMPONENTS),
            (
                "current workspace architecture",
                CURRENT_WORKSPACE_ARCHITECTURE,
            ),
            ("core error conventions", CORE_ERROR_CONVENTIONS),
            ("binding boundary ADR", BINDINGS_CORE_BOUNDARY_ADR),
            ("crate layout ADR", CRATE_LAYOUT_ADR),
            ("naming policy", NAMING_POLICY),
        ];

        for (document_name, text) in audited_documents {
            assert_no_legacy_codename(document_name, text);
        }

        for (source_name, text) in CRATE_SOURCES {
            assert_no_legacy_codename(source_name, text);
        }

        for expected in [
            "SSI Domain Naming Policy",
            "historical source evidence only",
            "workspace and crate manifests",
            "crate source files",
            "Migration maps and source evidence documents are intentionally excluded",
            "T058",
        ] {
            assert!(
                NAMING_POLICY.contains(expected) || TASKS.contains(expected),
                "naming policy or tasks missing {expected}"
            );
        }

        for expected in [
            "SSI Domain Naming Is Mandatory",
            "conformance naming audit",
            "Version**: 1.3.1",
        ] {
            assert!(
                CONSTITUTION.contains(expected),
                "constitution missing naming audit rule {expected}"
            );
        }
    }

    #[test]
    fn backlog_tasks_are_defined() {
        for specification in ALL_SPECIFICATIONS {
            assert!(
                TASKS.contains(specification.backlog_task),
                "{} references missing backlog task {}",
                specification.id,
                specification.backlog_task
            );
        }
    }

    #[test]
    fn current_workspace_architecture_tracks_crates_and_dependencies() {
        for expected in [
            "Current Workspace Architecture",
            "Production Dependency Graph",
            "Dependency Table",
            "Hexagonal Boundary View",
            "Current Gaps",
            "T090",
        ] {
            assert!(
                CURRENT_WORKSPACE_ARCHITECTURE.contains(expected) || TASKS.contains(expected),
                "current workspace architecture missing {expected}"
            );
        }

        for crate_name in [
            "identus-core",
            "identus-crypto",
            "identus-did",
            "identus-trust",
            "identus-credentials",
            "identus-presentations",
            "identus-messaging",
            "identus-openid4vc",
            "identus-wallet",
            "identus-agent",
            "identus-adapters",
            "identus-bindings",
            "identus-conformance",
        ] {
            assert!(
                CURRENT_WORKSPACE_ARCHITECTURE.contains(crate_name),
                "current workspace architecture missing crate {crate_name}"
            );
        }

        for edge in [
            "Adapters --> Core",
            "Adapters --> Did",
            "Adapters --> Messaging",
            "Adapters --> OpenId4Vc",
            "Adapters --> Trust",
            "Adapters --> Wallet",
            "Agent --> Core",
            "Bindings --> Core",
            "Bindings --> Wallet",
            "Conformance --> Core",
            "Credentials --> Core",
            "Credentials --> Crypto",
            "Credentials --> Did",
            "Credentials --> Trust",
            "Crypto --> Core",
            "Did --> Core",
            "Did --> Crypto",
            "Messaging --> Core",
            "Messaging --> Crypto",
            "Messaging --> Did",
            "OpenId4Vc --> Core",
            "OpenId4Vc --> Credentials",
            "OpenId4Vc --> Presentations",
            "OpenId4Vc --> Trust",
            "Presentations --> Core",
            "Presentations --> Credentials",
            "Presentations --> Trust",
            "Trust --> Core",
            "Trust --> Crypto",
            "Trust --> Did",
            "Wallet --> Core",
            "Wallet --> Credentials",
            "Wallet --> Crypto",
            "Wallet --> Did",
            "Wallet --> Messaging",
            "Wallet --> OpenId4Vc",
            "Wallet --> Presentations",
            "Wallet --> Trust",
        ] {
            assert!(
                CURRENT_WORKSPACE_ARCHITECTURE.contains(edge),
                "current workspace architecture missing production edge {edge}"
            );
        }

        for dev_dependency in [
            "`identus-bindings`, `identus-credentials`, `identus-messaging`, `identus-openid4vc`, `identus-presentations`, `identus-trust`",
            "dev-dependencies",
        ] {
            assert!(
                CURRENT_WORKSPACE_ARCHITECTURE.contains(dev_dependency),
                "current workspace architecture missing test-only dependency marker {dev_dependency}"
            );
        }
    }

    #[test]
    fn workspace_dependency_graph_fixture_matches_cargo_metadata() {
        let fixture = parse_json("workspace dependency graph", WORKSPACE_DEPENDENCY_GRAPH);
        assert_eq!(
            fixture
                .get("architecture_style")
                .and_then(serde_json::Value::as_str),
            Some("hexagonal")
        );
        assert_eq!(
            fixture
                .get("arrow_semantics")
                .and_then(serde_json::Value::as_str),
            Some("depends_on")
        );
        assert_eq!(
            fixture
                .get("source_command")
                .and_then(serde_json::Value::as_str),
            Some("cargo metadata --format-version 1 --no-deps")
        );

        let (metadata_crates, metadata_production_edges, metadata_dev_edges) =
            workspace_dependency_graph_from_cargo_metadata();
        assert_eq!(
            workspace_dependency_crates(&fixture),
            metadata_crates,
            "workspace dependency graph fixture crate list drifted"
        );
        assert_eq!(
            json_string_set(&fixture, "production_edges"),
            metadata_production_edges,
            "workspace dependency graph fixture production edges drifted"
        );
        assert_eq!(
            json_string_set(&fixture, "dev_edges"),
            metadata_dev_edges,
            "workspace dependency graph fixture dev edges drifted"
        );

        assert_hex_boundary_constraints(&fixture);
        assert_workspace_graph_has_no_policy_violations(&fixture);
        assert!(
            TASKS.contains("T091")
                && TASKS.contains("[x] T091")
                && TASKS.contains("T092")
                && TASKS.contains("[x] T092"),
            "T091 and T092 must be marked complete when workspace graph evidence is enforced"
        );
    }

    #[test]
    fn core_error_conventions_define_redaction_safe_binding_surface() {
        let core_source = CRATE_SOURCES
            .iter()
            .find_map(|(path, source)| (*path == "crates/core/src/lib.rs").then_some(*source))
            .expect("core source is not included in conformance audit");

        for expected in [
            "Core Error And Result Conventions",
            "`CapabilityId`",
            "`ErrorCode`",
            "`ErrorKind`",
            "`RedactionPolicy`",
            "`IdentusError`",
            "`IdentusResult<T>`",
            "`ResultEnvelope<T>`",
            "`ErrorEnvelope`",
            "Display",
            "redaction-safe",
            "Bindings should expose `ErrorEnvelope`",
            "`identus-did` is the first domain crate",
            "`parse_with_core_error`",
            "`DidParseError::to_identus_error`",
            "`DidParseError::to_error_envelope`",
            "`identus-messaging` bridges local DIDComm parser",
            "`addressed_with_core_error`",
            "`DidCommParseError::to_identus_error`",
            "`DidCommParseError::to_error_envelope`",
        ] {
            assert!(
                CORE_ERROR_CONVENTIONS.contains(expected),
                "core error conventions doc missing {expected}"
            );
        }

        for expected in [
            "pub struct CapabilityId",
            "pub struct ErrorCode",
            "pub enum ErrorKind",
            "pub enum RedactionPolicy",
            "pub struct IdentusError",
            "pub type IdentusResult<T>",
            "pub enum ResultEnvelope<T>",
            "pub struct ErrorEnvelope",
            "impl Display for IdentusError",
            "ErrorEnvelope::from_error",
            "error_display_is_redaction_safe",
            "result_envelope_maps_typed_error_fields",
        ] {
            assert!(
                core_source.contains(expected),
                "core source missing error convention surface {expected}"
            );
        }

        assert!(
            TASKS.contains("T093") && TASKS.contains("[x] T093"),
            "T093 must be marked complete when core error conventions are enforced"
        );
    }

    #[test]
    fn did_parser_errors_map_to_core_typed_errors() {
        let did_source = CRATE_SOURCES
            .iter()
            .find_map(|(path, source)| (*path == "crates/did/src/lib.rs").then_some(*source))
            .expect("DID source is not included in conformance audit");

        for expected in [
            "parse_with_core_error",
            "pub const fn code(self) -> ErrorCode",
            "pub const fn to_identus_error(self) -> IdentusError",
            "pub fn to_error_envelope(self) -> ErrorEnvelope",
            "impl From<DidParseError> for IdentusError",
            "did_parse_errors_map_to_core_error_surface",
            "did_parse_with_core_error_preserves_typed_codes",
        ] {
            assert!(
                did_source.contains(expected),
                "DID source missing core error bridge {expected}"
            );
        }

        for expected_code in [
            "missing_did_scheme",
            "invalid_did_method",
            "invalid_did_method_specific_id",
            "did_url_components_not_allowed",
            "invalid_did_url_component",
        ] {
            assert!(
                did_source.contains(expected_code)
                    && CORE_ERROR_CONVENTIONS.contains(expected_code),
                "DID parser typed error code {expected_code} must be in source and docs"
            );
        }

        assert!(
            TASKS.contains("T094") && TASKS.contains("[x] T094"),
            "T094 must be marked complete when DID parser errors map to core typed errors"
        );
    }

    #[test]
    fn didcomm_parser_errors_map_to_core_typed_errors() {
        let messaging_source = CRATE_SOURCES
            .iter()
            .find_map(|(path, source)| (*path == "crates/messaging/src/lib.rs").then_some(*source))
            .expect("messaging source is not included in conformance audit");

        for expected in [
            "parse_with_core_error",
            "addressed_with_core_error",
            "pub const fn code(&self) -> ErrorCode",
            "pub const fn to_identus_error(&self) -> IdentusError",
            "pub fn to_error_envelope(&self) -> ErrorEnvelope",
            "impl From<DidCommParseError> for IdentusError",
            "didcomm_parse_errors_map_to_core_error_surface",
            "didcomm_parse_with_core_error_preserves_typed_codes",
        ] {
            assert!(
                messaging_source.contains(expected),
                "messaging source missing core error bridge {expected}"
            );
        }

        for expected_code in [
            "missing_didcomm_prefix",
            "missing_didcomm_path_segment",
            "invalid_didcomm_path_segment",
            "invalid_didcomm_version",
            "unexpected_didcomm_path_segment",
            "invalid_didcomm_identifier",
            "missing_didcomm_recipient",
        ] {
            assert!(
                messaging_source.contains(expected_code)
                    && CORE_ERROR_CONVENTIONS.contains(expected_code),
                "DIDComm parser typed error code {expected_code} must be in source and docs"
            );
        }

        assert!(
            TASKS.contains("T095") && TASKS.contains("[x] T095"),
            "T095 must be marked complete when DIDComm parser errors map to core typed errors"
        );
    }

    #[test]
    fn typed_error_catalog_tracks_core_error_adopters() {
        assert_fixture_satisfies_schema("static-model", STATIC_MODEL_SCHEMA, TYPED_ERROR_CATALOG);
        assert!(STATIC_MODEL_README.contains("typed-error-catalog.json"));
        assert!(CORE_ERROR_CONVENTIONS.contains("typed-error-catalog.json"));

        let did_source = included_crate_source("crates/did/src/lib.rs");
        let messaging_source = included_crate_source("crates/messaging/src/lib.rs");
        let catalog = parse_json("typed error catalog", TYPED_ERROR_CATALOG);
        let expected = catalog
            .get("expected")
            .expect("typed error catalog has no expected object");

        assert_eq!(
            expected
                .get("catalog_owner")
                .and_then(serde_json::Value::as_str),
            Some("identus-core")
        );
        assert_eq!(
            expected
                .get("binding_dto")
                .and_then(serde_json::Value::as_str),
            Some("ErrorEnvelope")
        );
        assert_eq!(
            expected
                .get("redaction")
                .and_then(serde_json::Value::as_str),
            Some("public_message_only")
        );

        let capabilities = expected
            .get("capabilities")
            .and_then(serde_json::Value::as_array)
            .expect("typed error catalog has no capabilities array");
        let code_count = capabilities
            .iter()
            .map(|capability| typed_error_codes(capability).len())
            .sum::<usize>();
        let minimum_codes = expected
            .get("minimum_codes")
            .and_then(serde_json::Value::as_u64)
            .expect("typed error catalog has no minimum_codes");
        let minimum_codes = usize::try_from(minimum_codes)
            .expect("typed error catalog minimum_codes does not fit usize");
        assert!(
            code_count >= minimum_codes,
            "typed error catalog has {code_count} codes, expected at least {minimum_codes}"
        );

        assert_typed_error_capability(
            typed_error_capability(capabilities, "did"),
            "DidParseError",
            "identus-did",
            did_source,
        );
        assert_typed_error_capability(
            typed_error_capability(capabilities, "didcomm"),
            "DidCommParseError",
            "identus-messaging",
            messaging_source,
        );

        assert!(
            TASKS.contains("T096") && TASKS.contains("[x] T096"),
            "T096 must be marked complete when typed error catalog is enforced"
        );
    }

    #[test]
    fn conformance_fixture_layout_is_checked_in() {
        for (name, readme) in [
            ("root", FIXTURE_ROOT_README),
            ("schema", FIXTURE_SCHEMA_README),
            ("static-model", STATIC_MODEL_README),
            ("vector", VECTOR_README),
            ("transcript", TRANSCRIPT_README),
            ("interop", INTEROP_README),
            ("infrastructure", INFRASTRUCTURE_README),
        ] {
            assert!(
                readme.contains("Fixture") || readme.contains("Fixtures"),
                "{name} fixture README is missing fixture policy text"
            );
        }

        assert!(
            STATIC_MODEL_README.contains("workspace-dependency-graph.json"),
            "static-model README must list workspace dependency graph fixture"
        );
        assert!(
            STATIC_MODEL_README.contains("typed-error-catalog.json"),
            "static-model README must list typed error catalog fixture"
        );
        assert!(
            STATIC_MODEL_README.contains("trust-status-policy.json"),
            "static-model README must list trust/status policy fixture"
        );

        for expected in [
            "adr-fixture-schema-policy.md",
            "schema versioning",
            "source evidence",
            "owner crate",
            "redaction policy",
            "fixtures/schema/",
        ] {
            assert!(
                FIXTURE_POLICY_DOC.contains(expected) || FIXTURE_ROOT_README.contains(expected),
                "fixture policy docs missing {expected}"
            );
        }
    }

    #[test]
    fn fixture_schema_files_define_required_policy_fields() {
        assert!(FIXTURE_SCHEMA_README.contains("JSON Schema"));
        assert!(FIXTURE_SCHEMA_README.contains("cargo test --workspace"));

        for (family, schema_text) in [
            ("static-model", STATIC_MODEL_SCHEMA),
            ("vector", VECTOR_SCHEMA),
            ("transcript", TRANSCRIPT_SCHEMA),
            ("interop", INTEROP_SCHEMA),
            ("infrastructure", INFRASTRUCTURE_SCHEMA),
        ] {
            let schema = parse_json(family, schema_text);
            assert_schema_declares_required_field(&schema, "schema_version");
            assert_schema_declares_required_field(&schema, "source");
            assert_schema_declares_required_field(&schema, "owner_crate");
            assert_schema_declares_required_field(&schema, "redaction_policy");
            assert!(
                schema
                    .get("$schema")
                    .and_then(serde_json::Value::as_str)
                    .is_some_and(|uri| uri.contains("2020-12")),
                "{family} schema must declare JSON Schema 2020-12"
            );
        }

        assert_schema_declares_required_field(&parse_json("vector", VECTOR_SCHEMA), "expected");
        assert_schema_declares_required_field(
            &parse_json("transcript", TRANSCRIPT_SCHEMA),
            "expected_states",
        );
        assert_schema_declares_required_field(
            &parse_json("infrastructure", INFRASTRUCTURE_SCHEMA),
            "docker_free_alternative",
        );
    }

    #[test]
    fn checked_in_json_fixtures_satisfy_schema_required_fields() {
        assert_fixture_satisfies_schema("static-model", STATIC_MODEL_SCHEMA, CAPABILITY_CONTRACTS);
        assert_fixture_satisfies_schema("static-model", STATIC_MODEL_SCHEMA, TYPED_ERROR_CATALOG);
        assert_fixture_satisfies_schema("static-model", STATIC_MODEL_SCHEMA, TRUST_STATUS_POLICY);
        assert_fixture_satisfies_schema("vector", VECTOR_SCHEMA, DID_PRISM_VECTOR);
        assert_fixture_satisfies_schema("vector", VECTOR_SCHEMA, DID_PRISM_OPERATION_VDR_VECTOR);
        assert_fixture_satisfies_schema("vector", VECTOR_SCHEMA, CREDENTIAL_NEGATIVE_VECTOR);

        for transcript in [
            DIDCOMM_MEDIATION_PICKUP_TRANSCRIPT,
            DIDCOMM_CREDENTIAL_PRESENTATION_TRANSCRIPT,
            DIDCOMM_CONNECTIONLESS_OOB_TRANSCRIPT,
            DIDCOMM_COMPATIBILITY_TRANSCRIPT,
            OPENID4VC_CORE_TRANSCRIPT,
        ] {
            assert_fixture_satisfies_schema("transcript", TRANSCRIPT_SCHEMA, transcript);
        }

        assert_fixture_satisfies_schema(
            "infrastructure",
            INFRASTRUCTURE_SCHEMA,
            NEOPRISM_VDR_INFRASTRUCTURE,
        );
        assert_fixture_satisfies_schema("interop", INTEROP_SCHEMA, WRAPPER_API_PARITY);
    }

    #[test]
    fn fixture_schema_adr_covers_required_domains_and_redaction() {
        for expected in [
            "Conformance Fixture Schema Policy",
            "schema_version",
            "specification_ids",
            "source",
            "owner_crate",
            "expected_states",
            "redaction_policy",
            "DID And Key Vector Fixtures",
            "Credential And Presentation Fixtures",
            "DIDComm Transcript Fixtures",
            "OpenID4VC Transcript Fixtures",
            "Negative fixtures must cover expired credentials",
            "OOB invitation and connectionless attachments",
            "Issue Credential 3.0",
            "Present Proof 3.0",
            "pre-authorized",
            "authorization code",
            "wrong nonce/state",
            "trust-chain rejection",
            "identus-conformance",
            "Binding crates must reuse the same fixtures",
            "Behavior fixtures must reference at least one backlog task with acceptance",
            "cargo test --workspace",
        ] {
            assert!(
                FIXTURE_SCHEMA_ADR.contains(expected),
                "fixture schema ADR missing {expected}"
            );
        }
    }

    #[test]
    fn deterministic_prism_did_vector_is_pinned() {
        for expected in [
            "\"specification_id\": \"did-prism\"",
            "\"derivation_path\": \"m/29'/29'/0'/1'/0'\"",
            "\"public_key_id\": \"master\"",
            "\"curve\": \"secp256k1\"",
            "\"protobuf_key_encoding\": \"CompressedECKeyData\"",
            "3b32a5049f2b4e3af31ec5c1ae75fada1ad2eb8be5accf56ada343ad89eeb083208e538b3b97836e3bd7048c131421bf5bea9e3a1d25812a2d831e2bab89e058",
            "158bf13202ccafe551b5b4e60ed516efe0fe190e5c1421c3387f0f9fef2a6111",
            "023f7c75c9e5fba08fea1640d6faa3f8dc0151261d2b56026d46ddcbe1fc5a5bbb",
        ] {
            assert!(
                DID_PRISM_VECTOR.contains(expected),
                "deterministic PRISM DID vector missing {expected}"
            );
        }
    }

    #[test]
    fn prism_operation_vdr_fixtures_cover_neoprism_lifecycle() {
        assert!(DID_PRISM_VECTOR_README.contains("prism-operation-vdr-lifecycle.json"));
        assert!(INFRASTRUCTURE_README.contains("neoprism-vdr-adapters.json"));

        let operation_fixture =
            parse_json("PRISM operation VDR vector", DID_PRISM_OPERATION_VDR_VECTOR);
        let infrastructure_fixture =
            parse_json("NeoPRISM VDR infrastructure", NEOPRISM_VDR_INFRASTRUCTURE);

        let operations = operation_fixture
            .get("operations")
            .and_then(serde_json::Value::as_array)
            .expect("PRISM operation fixture has no operations array");
        assert_eq!(
            operations.len(),
            4,
            "expected create/update/deactivate/resolve operations"
        );

        for expected_operation in ["create", "update", "deactivate", "resolve"] {
            assert!(
                operations.iter().any(|operation| {
                    operation
                        .get("operation")
                        .and_then(serde_json::Value::as_str)
                        .is_some_and(|operation| operation == expected_operation)
                }),
                "PRISM operation fixture missing {expected_operation}"
            );
        }

        for expected in [
            "AtalaOperation",
            "CreateDIDOperation",
            "UpdateDIDOperation",
            "DeactivateDIDOperation",
            "DidResolutionRequest",
            "sha256(serialized AtalaOperation)",
            "secure_depth",
            "112",
            "PrismOperationStore",
            "PrismLedgerReader",
            "PrismLedgerSubmitter",
            "DidResolverPort",
            "TrustPolicyPort",
            "operation_hash",
            "transaction_id",
            "block_height",
            "source_adapter",
            "docker_required\": false",
        ] {
            assert!(
                DID_PRISM_OPERATION_VDR_VECTOR.contains(expected),
                "PRISM operation fixture missing {expected}"
            );
        }

        let adapter_gates = infrastructure_fixture
            .get("adapter_gates")
            .and_then(serde_json::Value::as_array)
            .expect("NeoPRISM infrastructure fixture has no adapter_gates array");
        assert!(
            adapter_gates.len() >= 7,
            "expected NeoPRISM infrastructure adapters to be enumerated"
        );

        for expected in [
            "Oura",
            "DBSync",
            "Blockfrost",
            "PostgreSQL",
            "SQLite",
            "cardano-wallet",
            "embedded-wallet",
            "NeoPRISM HTTP resolver",
            "SDK_RUST_NEOPRISM_INFRASTRUCTURE=1",
            "docker_free",
            "opt_in",
            "in_memory_prism_operation_store",
        ] {
            assert!(
                NEOPRISM_VDR_INFRASTRUCTURE.contains(expected),
                "NeoPRISM infrastructure fixture missing {expected}"
            );
        }
    }

    #[test]
    fn credential_negative_vector_covers_required_typed_errors() {
        assert!(
            CREDENTIAL_NEGATIVE_VECTOR_README.contains("Credential Verification Vector Fixtures")
        );
        assert!(CREDENTIAL_NEGATIVE_VECTOR_README.contains("stable `case_id`"));

        let fixture = parse_json("credential negative vector", CREDENTIAL_NEGATIVE_VECTOR);
        let cases = fixture
            .get("cases")
            .and_then(serde_json::Value::as_array)
            .expect("credential negative vector has no cases array");

        assert!(cases.len() >= 12, "expected broad negative-case coverage");

        for expected_case in [
            ("credential-expired", "credential_expired"),
            ("credential-not-before", "credential_not_yet_valid"),
            ("presentation-wrong-audience", "audience_mismatch"),
            ("presentation-wrong-domain", "domain_mismatch"),
            ("presentation-wrong-challenge", "challenge_mismatch"),
            ("unsupported-key-purpose", "unsupported_key_purpose"),
            ("status-revoked", "credential_revoked"),
            ("tampered-signature", "signature_verification_failed"),
            ("malformed-schema", "schema_validation_failed"),
            ("missing-holder-binding", "holder_binding_missing"),
            ("unsupported-format", "unsupported_credential_format"),
            (
                "selective-disclosure-claim-missing",
                "required_disclosure_missing",
            ),
        ] {
            assert_case_with_error(cases, expected_case.0, expected_case.1);
        }

        for expected_spec in [
            "vc-json-schema",
            "jwt-vc",
            "sd-jwt",
            "sd-jwt-vc",
            "presentation-exchange",
            "dcql",
            "bitstring-status-list",
            "ietf-token-status-list",
        ] {
            assert!(
                CREDENTIAL_NEGATIVE_VECTOR.contains(expected_spec),
                "credential negative vector missing spec {expected_spec}"
            );
        }

        for expected_format in [
            "jwt_vc_json",
            "sd_jwt_vc",
            "w3c_vc_json",
            "anoncreds",
            "openbadges_3",
            "iso_mdoc",
        ] {
            assert!(
                CREDENTIAL_NEGATIVE_VECTOR.contains(expected_format),
                "credential negative vector missing format {expected_format}"
            );
        }
    }

    #[test]
    fn credential_format_registry_defines_format_neutral_model_boundary() {
        let credentials_source = included_crate_source("crates/credentials/src/lib.rs");

        for expected in [
            "pub struct CredentialFormatId",
            "pub enum CredentialFormatFamily",
            "pub enum CredentialFormatStage",
            "pub struct CredentialFormatProfile",
            "pub struct CredentialDescriptor",
            "pub enum CredentialFormatCapability",
            "pub trait CredentialFormatRegistry",
            "pub struct StaticCredentialFormatRegistry",
            "pub const CREDENTIAL_FORMAT_PROFILES",
            "credential_format_registry_covers_parity_and_roadmap_formats",
            "credential_descriptor_keeps_model_metadata_format_neutral",
        ] {
            assert!(
                credentials_source.contains(expected),
                "credentials source missing format-neutral boundary {expected}"
            );
        }

        let registry = identus_credentials::StaticCredentialFormatRegistry;
        for (format_id, specification_id) in [
            ("jwt_vc_json", "jwt-vc"),
            ("sd_jwt_vc", "sd-jwt-vc"),
            ("w3c_vc_json", "vc-data-model"),
            ("anoncreds", "anoncreds-v1"),
            ("openbadges_3", "openbadges-3"),
            ("iso_mdoc", "iso-mdoc"),
        ] {
            let profile = identus_credentials::CredentialFormatRegistry::find(
                &registry,
                identus_credentials::CredentialFormatId::new(format_id),
            )
            .unwrap_or_else(|| panic!("missing credential format profile {format_id}"));
            assert_eq!(profile.specification_id, specification_id);
            assert!(profile.wallet_supported);
        }

        for didcomm_format in ["jwt_vc_json", "sd_jwt_vc", "w3c_vc_json", "anoncreds"] {
            assert!(
                identus_credentials::CredentialFormatRegistry::supports(
                    &registry,
                    identus_credentials::CredentialFormatId::new(didcomm_format),
                    identus_credentials::CredentialFormatCapability::DidCommIssueCredential,
                ),
                "{didcomm_format} must support DIDComm issue-credential"
            );
        }

        for openid_format in ["jwt_vc_json", "sd_jwt_vc", "openbadges_3", "iso_mdoc"] {
            assert!(
                identus_credentials::CredentialFormatRegistry::supports(
                    &registry,
                    identus_credentials::CredentialFormatId::new(openid_format),
                    identus_credentials::CredentialFormatCapability::OpenId4VcIssuance,
                ),
                "{openid_format} must support OpenID4VCI issuance"
            );
        }

        let descriptor = identus_credentials::CredentialDescriptor {
            id: "fixture-credential-1",
            format: identus_credentials::CredentialFormatId::new("jwt_vc_json"),
            issuer: "did:example:issuer",
            subject: "did:example:holder",
            schema: Some("fixture-schema"),
        };
        assert_eq!(descriptor.format.as_str(), "jwt_vc_json");
        assert_eq!(descriptor.schema, Some("fixture-schema"));

        assert!(
            TASKS.contains("T040") && TASKS.contains("[x] T040"),
            "T040 must be marked complete when credential model registry is enforced"
        );
    }

    #[test]
    fn credential_negative_vector_replays_through_policy_ports() {
        use identus_credentials::CredentialVerificationReplay as _;
        use identus_presentations::PresentationVerificationReplay as _;

        let fixture = parse_json("credential negative vector", CREDENTIAL_NEGATIVE_VECTOR);
        let cases = fixture
            .get("cases")
            .and_then(serde_json::Value::as_array)
            .expect("credential negative vector has no cases array");
        let credential_policy = identus_credentials::FixtureCredentialVerificationPolicy;
        let presentation_policy = identus_presentations::FixturePresentationVerificationPolicy;
        let mut replayed = BTreeSet::new();

        for case in cases {
            let case_id = json_str(case, "case_id");
            let format = json_str(case, "format");
            let expected_error = json_str(case, "expected_error");

            let error = if identus_credentials::expected_credential_negative_error(case_id)
                .is_some()
            {
                credential_policy
                    .replay_credential_negative_case(
                        &identus_credentials::CredentialVerificationCase {
                            case_id,
                            format,
                            expected_error,
                        },
                    )
                    .expect_err("credential negative case must reject")
            } else if identus_presentations::expected_presentation_negative_error(case_id).is_some()
            {
                presentation_policy
                    .replay_presentation_negative_case(
                        &identus_presentations::PresentationVerificationCase {
                            case_id,
                            format,
                            expected_error,
                        },
                    )
                    .expect_err("presentation negative case must reject")
            } else {
                panic!("negative fixture case {case_id} is not covered by a replay port");
            };

            assert_eq!(
                error.code().as_str(),
                expected_error,
                "{case_id} replayed wrong typed error"
            );
            assert!(
                !error.to_string().contains(json_str(case, "input_ref")),
                "{case_id} leaked fixture input reference in error display"
            );
            replayed.insert(case_id);
        }

        assert_eq!(
            replayed.len(),
            cases.len(),
            "each negative fixture case must replay exactly once"
        );
        assert!(
            TASKS.contains("T072") && TASKS.contains("[x] T072"),
            "T072 must be marked complete when negative fixture replay is enforced"
        );
    }

    #[test]
    fn legacy_migration_map_names_all_replacement_sources() {
        for expected in [
            "sdk-ts",
            "sdk-swift",
            "sdk-kmp",
            "neoprism",
            "identus-crypto",
            "identus-did",
            "identus-messaging",
            "identus-openid4vc",
            "identus-bindings",
            "T053",
            "T054",
            "T055",
            "T056",
            "T057",
        ] {
            assert!(
                LEGACY_MIGRATION_MAP.contains(expected),
                "legacy migration map missing {expected}"
            );
        }
    }

    #[test]
    fn wrapper_api_parity_inventory_maps_public_exports() {
        assert!(INTEROP_README.contains("wrapper-api-parity.json"));

        let inventory = parse_json("wrapper API parity inventory", WRAPPER_API_PARITY);
        let languages = inventory
            .get("languages")
            .and_then(serde_json::Value::as_array)
            .expect("wrapper parity inventory has no languages array");

        assert_eq!(
            languages.len(),
            3,
            "expected TS, Swift, and Kotlin inventories"
        );

        let mut entry_count = 0usize;
        for expected_language in ["typescript", "swift", "kotlin"] {
            let language = languages
                .iter()
                .find(|language| {
                    language
                        .get("language")
                        .and_then(serde_json::Value::as_str)
                        .is_some_and(|actual| actual == expected_language)
                })
                .unwrap_or_else(|| panic!("missing {expected_language} wrapper inventory"));

            let exports = language
                .get("exports")
                .and_then(serde_json::Value::as_array)
                .unwrap_or_else(|| panic!("{expected_language} inventory has no exports"));
            assert!(
                !exports.is_empty(),
                "{expected_language} inventory must include public exports"
            );

            for export in exports {
                entry_count += 1;
                for required_field in [
                    "source_name",
                    "source_kind",
                    "source_path",
                    "owner_crate",
                    "binding_surface",
                    "migration_disposition",
                    "fixture_coverage",
                ] {
                    assert!(
                        export.get(required_field).is_some(),
                        "{expected_language} export missing {required_field}: {export:?}"
                    );
                }

                assert!(
                    export
                        .get("owner_crate")
                        .and_then(serde_json::Value::as_str)
                        .is_some_and(|owner| owner.starts_with("identus-")),
                    "{expected_language} export owner_crate must name an Identus crate"
                );
                assert!(
                    export
                        .get("fixture_coverage")
                        .and_then(serde_json::Value::as_array)
                        .is_some_and(|coverage| !coverage.is_empty()),
                    "{expected_language} export needs fixture coverage or backlog reference"
                );
            }
        }

        assert!(entry_count >= 20, "expected broad wrapper parity inventory");

        for expected in [
            "repos/sdk-ts/packages/lib/sdk/src/index.ts",
            "repos/sdk-swift/EdgeAgentSDK/Domain/Sources/BBs/Castor.swift",
            "repos/sdk-kmp/sdk/src/commonMain/kotlin/org/hyperledger/identus/walletsdk/edgeagent/EdgeAgent.kt",
            "PluginManager",
            "requires_plugin_compatibility_adr",
            "node tools/generate-wrapper-api-parity.mjs --check",
            "identus-bindings",
            "identus-agent",
            "identus-wallet",
            "identus-did",
            "identus-crypto",
            "identus-messaging",
            "identus-credentials",
            "T057",
            "T083",
        ] {
            assert!(
                WRAPPER_API_PARITY.contains(expected) || TASKS.contains(expected),
                "wrapper parity inventory or tasks missing {expected}"
            );
        }

        assert_wrapper_parity_generator_contract();
    }

    #[test]
    fn conformance_workflow_gates_wrapper_parity_drift() {
        for expected in [
            "name: Conformance",
            "uses: actions/checkout@34e114876b0b11c390a56381ad16ebd13914f8d5",
            "path: repos/sdk-rust",
            "path: repos/sdk-ts",
            "path: repos/sdk-swift",
            "path: repos/sdk-kmp",
            ".specify/scripts/bash/check-prerequisites.sh --json --include-tasks",
            "node tools/generate-wrapper-api-parity.mjs --check",
            "cargo fmt --all -- --check",
            "cargo clippy --workspace --all-targets -- -D warnings",
            "cargo test --workspace",
        ] {
            assert!(
                CONFORMANCE_WORKFLOW.contains(expected),
                "conformance workflow missing {expected}"
            );
        }

        for expected in [
            "wrapper API parity drift",
            "repos/sdk-rust",
            "repos/sdk-ts",
            "repos/sdk-swift",
            "repos/sdk-kmp",
            "Third-party GitHub Actions must be pinned to",
            "node tools/generate-wrapper-api-parity.mjs --check",
            "fixtures/conformance/interop/wrapper-api-parity.json",
        ] {
            assert!(
                CONFORMANCE_DOC.contains(expected),
                "conformance docs missing CI drift contract {expected}"
            );
        }

        for expected in [
            "T083",
            "CI runs `node tools/generate-wrapper-api-parity.mjs --check`",
            "expected sibling repository checkout layout",
            "fails with an actionable message",
        ] {
            assert!(
                TASKS.contains(expected),
                "tasks missing wrapper parity CI acceptance item {expected}"
            );
        }

        assert!(
            !CONFORMANCE_WORKFLOW.contains("uses: actions/checkout@v4"),
            "conformance workflow must not use floating checkout tags"
        );
    }

    #[test]
    fn agentic_sdlc_harness_defines_multi_agent_github_workflow() {
        assert_agent_instruction_files_define_sdlc();
        assert_agentic_sdlc_doc_and_labels_are_aligned();
        assert_github_templates_capture_agent_handoff();
        assert_agentic_sdlc_harness_is_wired_to_ci();
        assert!(
            CONFORMANCE_WORKFLOW.contains("node tools/check-agent-sdlc.mjs --check"),
            "conformance workflow must run the agentic SDLC harness"
        );
        assert_agentic_sdlc_task_is_complete();
    }

    fn assert_agent_instruction_files_define_sdlc() {
        assert_contains_all(
            "AGENTS.md",
            AGENTS_DOC,
            &[
                "Agentic SDLC Contract",
                "Agent Roles",
                "GitHub Status Flow",
                "GitHub Discussions",
                "Spec Kit task",
                "GPG-signed",
                "DCO-signed",
                "node tools/check-agent-sdlc.mjs --check",
            ],
        );
        assert_contains_all(
            "AGENT.md",
            AGENT_DOC,
            &[
                "`AGENTS.md` is the canonical repository instruction file",
                "docs/maintenance/agentic-sdlc.md",
                "signed DCO commits",
            ],
        );
    }

    fn assert_agentic_sdlc_doc_and_labels_are_aligned() {
        assert_contains_all(
            "agentic SDLC doc",
            AGENTIC_SDLC_DOC,
            &[
                "Issue Statuses",
                "GitHub Project Fields",
                "Discussion Workflow",
                "Pull Request Workflow",
                "Harness",
                "status:triage",
                "status:ready-to-merge",
                "Fixture Impact",
            ],
        );

        for expected_status in [
            "status:triage",
            "status:needs-spec",
            "status:ready",
            "status:in-progress",
            "status:blocked",
            "status:review",
            "status:security-review",
            "status:docs-review",
            "status:conformance",
            "status:ready-to-merge",
            "status:released",
        ] {
            assert!(
                GITHUB_LABELS.contains(expected_status),
                "GitHub labels missing {expected_status}"
            );
            assert!(
                AGENTIC_SDLC_DOC.contains(expected_status),
                "agentic SDLC doc missing {expected_status}"
            );
        }

        assert_contains_all(
            "GitHub labels",
            GITHUB_LABELS,
            &[
                "agent:manager",
                "agent:planner",
                "agent:engineer",
                "agent:conformance",
                "agent:reviewer",
                "agent:security",
                "agent:docs",
                "agent:release",
                "agent:maintenance",
            ],
        );
    }

    fn assert_github_templates_capture_agent_handoff() {
        for (template_name, template) in [
            ("capability issue template", ISSUE_TEMPLATE_CAPABILITY),
            ("defect issue template", ISSUE_TEMPLATE_DEFECT),
            ("maintenance issue template", ISSUE_TEMPLATE_MAINTENANCE),
        ] {
            assert_contains_all(
                template_name,
                template,
                &[
                    "Agent",
                    "Capability",
                    "Acceptance Criteria",
                    "Conformance Impact",
                    "Docs Impact",
                    "Discussion Link",
                ],
            );
        }

        for expected in [
            "Architecture discussion",
            "GitHub Discussion",
            "Linked Work",
            "Validation",
            "Security Impact",
            "Docs Impact",
            "Conformance Impact",
            "GPG-signed",
            "DCO-signed",
            "node tools/check-agent-sdlc.mjs --check",
        ] {
            assert!(
                ISSUE_TEMPLATE_CONFIG.contains(expected)
                    || DISCUSSION_TEMPLATE_ARCHITECTURE.contains(expected)
                    || DISCUSSION_TEMPLATE_RESEARCH.contains(expected)
                    || PULL_REQUEST_TEMPLATE.contains(expected),
                "GitHub workflow templates missing {expected}"
            );
        }
    }

    fn assert_agentic_sdlc_harness_is_wired_to_ci() {
        assert_contains_all(
            "agentic SDLC harness",
            AGENTIC_SDLC_HARNESS,
            &[
                "requiredFiles",
                ".github/labels.yml",
                ".github/ISSUE_TEMPLATE/ssi-capability.yml",
                ".github/DISCUSSION_TEMPLATE/architecture.md",
                ".github/pull_request_template.md",
                "node tools/check-agent-sdlc.mjs --check",
                "T097",
            ],
        );
    }

    fn assert_agentic_sdlc_task_is_complete() {
        for expected in [
            "T097",
            "[x] T097",
            "Agentic SDLC harness",
            "issue status labels",
            "GitHub Discussion",
            "node tools/check-agent-sdlc.mjs --check",
        ] {
            assert!(TASKS.contains(expected), "tasks missing {expected}");
        }
    }

    fn assert_contains_all(name: &str, text: &str, expected_terms: &[&str]) {
        for expected in expected_terms {
            assert!(text.contains(expected), "{name} missing {expected}");
        }
    }

    #[test]
    fn did_method_inventory_tracks_current_support_tiers() {
        for expected in [
            "did:prism",
            "did:peer",
            "did:web",
            "did:key",
            "did:jwk",
            "did:pkh",
            "did:example",
            "did-peer",
            "ssi-dids",
            "did-key",
        ] {
            assert!(
                DID_METHODS.contains(expected),
                "DID method inventory missing {expected}"
            );
        }
    }

    #[test]
    fn didcomm_protocol_inventory_tracks_current_and_planned_protocols() {
        for expected in [
            "Out-of-Band 2.0",
            "BasicMessage 2.0",
            "Trust Ping 2.0",
            "Discover Features 2.0",
            "Routing 2.0",
            "Coordinate Mediation 2.0",
            "Coordinate Mediation 3.0",
            "Message Pickup 3.0",
            "Issue Credential 3.0",
            "Present Proof 3.0",
            "Report Problem 2.0",
            "Revocation Notification 1.0",
            "DID Rotate 2.0",
            "didcomm-rs",
            "affinidi-messaging-didcomm",
            "didcomm",
        ] {
            assert!(
                DIDCOMM_PROTOCOLS.contains(expected),
                "DIDComm protocol inventory missing {expected}"
            );
        }
    }

    #[test]
    fn didcomm_pack_unpack_adr_blocks_core_dependency_lock_in() {
        for expected in [
            "Do not add a DIDComm pack/unpack crate to `identus-messaging` yet.",
            "DidCommPacker",
            "DidCommUnpacker",
            "DidResolverPort",
            "KeyAgreementPort",
            "SignerPort",
            "SecretResolverPort",
            "`didcomm` 0.4.1",
            "`didcomm-rs` 0.7.2",
            "`affinidi-messaging-didcomm` 0.15.1",
            "WASM",
            "Android",
            "UniFFI",
            "T068",
            "T069",
        ] {
            assert!(
                DIDCOMM_PACK_UNPACK_ADR.contains(expected),
                "DIDComm pack/unpack ADR missing {expected}"
            );
        }
    }

    #[test]
    fn neoprism_convergence_adr_maps_crates_and_quality_gates() {
        for expected in [
            "NeoPRISM Convergence",
            "`lib/apollo`",
            "`identus-apollo`",
            "`lib/did-core`",
            "`identus-did-core`",
            "`lib/did-prism`",
            "`identus-did-prism`",
            "`lib/did-prism-ledger`",
            "`identus-did-prism-ledger`",
            "`lib/did-prism-indexer`",
            "`identus-did-prism-indexer`",
            "`lib/did-prism-submitter`",
            "`identus-did-prism-submitter`",
            "`lib/did-resolver-http`",
            "`identus-did-resolver-http`",
            "`lib/node-storage`",
            "`bin/neoprism-node`",
            "`identus-crypto`",
            "`identus-did`",
            "`identus-adapters`",
            "`identus-wallet`",
            "`identus-trust`",
            "AtalaOperation",
            "Cardano Oura data source",
            "Cardano DBSync data source",
            "Blockfrost data source",
            "Cardano-wallet submitter",
            "Embedded-wallet submitter",
            "Universal-resolver HTTP adapter",
            "PostgreSQL and SQLite infrastructure adapters",
            "VDR Ownership",
            "Rust 2024 workspace",
            "unsafe_code = \"forbid\"",
            "cargo clippy --workspace --all-targets -- -D warnings",
            "Specification conformance tests",
            "Coverage reporting",
            "SonarCloud",
            "OpenSSF Scorecard",
            "OpenSSF Best Practices",
            "Release automation",
            "Docker, Nix, and deployment integration hooks",
            "Phase A: Fixture And Port Alignment",
            "Phase B: Core Module Port",
            "Phase C: Adapter Port",
            "Phase D: Service Thinning",
            "T079",
        ] {
            assert!(
                NEOPRISM_CONVERGENCE_ADR.contains(expected) || TASKS.contains(expected),
                "NeoPRISM convergence ADR or tasks missing {expected}"
            );
        }
    }

    #[test]
    fn bindings_core_boundary_adr_defines_wrapper_rules() {
        for expected in [
            "Binding Core Boundary",
            "Rust core owns product semantics",
            "not legacy SDK codenames",
            "Secrets and raw private keys must not cross public Rust, UniFFI, WASM, Node",
            "identus-bindings",
            "Browser/WASM",
            "wasm-bindgen",
            "Node/N-API",
            "napi-rs",
            "UniFFI",
            "SwiftPM",
            "Gradle/KMP",
            "TypeScript",
            "React",
            "React Native",
            "Agent",
            "Wallet",
            "DID",
            "Credential",
            "Presentation",
            "Messaging",
            "OpenID4VC",
            "Trust",
            "Adapters",
            "JSON-compatible DTOs",
            "typed error codes",
            "opaque handle",
            "No-secret-leak tests",
            "Target-build checks",
            "Wrapper API parity inventories",
            "T080",
        ] {
            assert!(
                BINDINGS_CORE_BOUNDARY_ADR.contains(expected) || TASKS.contains(expected),
                "binding core boundary ADR or tasks missing {expected}"
            );
        }
    }

    #[test]
    fn plugin_extension_adr_maps_ts_surface_to_typed_registry() {
        for expected in [
            "Plugin And Extension Compatibility",
            "PluginManager",
            "Plugins.Task",
            "addModule(key, module)",
            "Protocol task dispatch",
            "DIDComm",
            "AnonCreds",
            "DIF",
            "OIDC",
            "OEA compatibility",
            "identus-bindings",
            "ExtensionRegistry",
            "ExtensionModuleDescriptor",
            "ProtocolTaskDescriptor",
            "ExtensionContext",
            "ExtensionResult",
            "DIDComm message type",
            "credential format",
            "presentation format",
            "OpenID4VC flow",
            "trust/status mechanism",
            "opaque host handle",
            "capabilities and permissions",
            "No-secret-leak tests",
            "generated inventory of TypeScript plugin exports",
            "Wrapper tests proving TypeScript compatibility names dispatch through",
            "T057",
        ] {
            assert!(
                PLUGIN_EXTENSION_ADR.contains(expected) || TASKS.contains(expected),
                "plugin extension ADR or tasks missing {expected}"
            );
        }
    }

    #[test]
    fn capability_driven_api_adr_blocks_lowest_common_denominator_design() {
        for expected in [
            "Capability-Driven API Strategy",
            "valuable source evidence",
            "not define a universal cross-SDK public API",
            "capability contracts",
            "Domain primitives",
            "Ports",
            "Protocol state machines",
            "Capability services",
            "Binding facades",
            "API Selection Criteria",
            "issuer, holder, verifier, peer",
            "typed result envelopes",
            "redaction-safe diagnostics",
            "DIDComm",
            "OID4VCI",
            "OID4VP",
            "SIOPv2",
            "HAIP",
            "Wrapper API inventories are compatibility gates, not API design inputs",
            "Public API stabilization requires capability coverage",
            "T087",
        ] {
            assert!(
                CAPABILITY_API_ADR.contains(expected) || TASKS.contains(expected),
                "capability-driven API ADR or tasks missing {expected}"
            );
        }
    }

    #[test]
    fn capability_contract_fixture_covers_first_public_surfaces() {
        assert!(STATIC_MODEL_README.contains("capability-contracts.json"));
        assert_fixture_satisfies_schema("static-model", STATIC_MODEL_SCHEMA, CAPABILITY_CONTRACTS);

        let fixture = parse_json("capability contracts", CAPABILITY_CONTRACTS);
        assert_eq!(
            fixture
                .get("expected")
                .and_then(|expected| expected.get("api_design_source"))
                .and_then(serde_json::Value::as_str),
            Some("capability_contracts"),
            "capability fixture must identify capability contracts as API design source"
        );
        assert_eq!(
            fixture
                .get("expected")
                .and_then(|expected| expected.get("wrapper_api_inventories_role"))
                .and_then(serde_json::Value::as_str),
            Some("compatibility_gate"),
            "wrapper API inventories must be compatibility gates"
        );

        let contracts = fixture
            .get("expected")
            .and_then(|expected| expected.get("contracts"))
            .and_then(serde_json::Value::as_array)
            .expect("capability fixture has no contracts array");

        for required_contract in [
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
            let contract = contracts
                .iter()
                .find(|contract| {
                    contract
                        .get("contract_id")
                        .and_then(serde_json::Value::as_str)
                        .is_some_and(|contract_id| contract_id == required_contract)
                })
                .unwrap_or_else(|| panic!("missing capability contract {required_contract}"));

            assert!(
                contract
                    .get("owner_crate")
                    .and_then(serde_json::Value::as_str)
                    .is_some_and(|owner| owner.starts_with("identus-")),
                "{required_contract} must name an Identus owner crate"
            );

            for required_array in [
                "required_ports",
                "role_inputs",
                "outputs",
                "typed_errors",
                "fixture_families",
                "wrapper_targets",
                "acceptance_tests",
            ] {
                assert!(
                    contract
                        .get(required_array)
                        .and_then(serde_json::Value::as_array)
                        .is_some_and(|values| !values.is_empty()),
                    "{required_contract} must define non-empty {required_array}"
                );
            }
        }

        for expected in [
            "legacy_sdk_union",
            "legacy_sdk_intersection",
            "legacy_sdk_codename_modules",
            "secret_leak_blocked",
            "typed_error",
            "redaction_policy",
            "node-napi",
            "wasm",
            "react-native",
            "untrusted_anchor",
            "trust_chain_rejected",
            "oid4vci_authorization_code",
            "connectionless_credential_offer",
            "prism_operation_lifecycle",
            "T088",
        ] {
            assert!(
                CAPABILITY_CONTRACTS.contains(expected) || TASKS.contains(expected),
                "capability contracts fixture or tasks missing {expected}"
            );
        }
    }

    fn capability_contract_ids(contracts: &[serde_json::Value]) -> BTreeSet<&str> {
        contracts
            .iter()
            .map(|contract| {
                contract
                    .get("contract_id")
                    .and_then(serde_json::Value::as_str)
                    .expect("contract has no id")
            })
            .collect()
    }

    fn assert_registry_contract_matches_fixture(fixture_contract: &serde_json::Value) {
        let contract_id = fixture_contract
            .get("contract_id")
            .and_then(serde_json::Value::as_str)
            .expect("contract has no id");
        let registry_contract = identus_bindings::all_capability_contracts()
            .iter()
            .find(|contract| contract.id == contract_id)
            .unwrap_or_else(|| panic!("registry missing contract {contract_id}"));

        assert_eq!(
            fixture_contract
                .get("owner_crate")
                .and_then(serde_json::Value::as_str),
            Some(registry_contract.owner_crate),
            "{contract_id} owner crate drifted"
        );

        for (field_name, registry_values) in [
            ("required_ports", registry_contract.required_ports),
            ("role_inputs", registry_contract.role_inputs),
            ("outputs", registry_contract.outputs),
            ("typed_errors", registry_contract.typed_errors),
            ("fixture_families", registry_contract.fixture_families),
            ("wrapper_targets", registry_contract.wrapper_targets),
            ("acceptance_tests", registry_contract.acceptance_tests),
        ] {
            let fixture_values = fixture_contract
                .get(field_name)
                .and_then(serde_json::Value::as_array)
                .unwrap_or_else(|| panic!("{contract_id} missing {field_name}"))
                .iter()
                .map(|value| value.as_str().expect("contract value must be a string"))
                .collect::<BTreeSet<_>>();
            let registry_values = registry_values.iter().copied().collect::<BTreeSet<_>>();

            assert_eq!(
                fixture_values, registry_values,
                "{contract_id} {field_name} drifted between fixture and registry"
            );
        }
    }

    fn assert_binding_manifests_reference_supported_contracts() {
        for manifest in identus_bindings::all_binding_manifests() {
            assert!(
                [
                    "typescript",
                    "swift",
                    "kotlin",
                    "react",
                    "react-native",
                    "node-napi",
                    "wasm"
                ]
                .contains(&manifest.target_id),
                "unexpected binding manifest target {}",
                manifest.target_id
            );
            assert!(
                !manifest.contract_ids.is_empty(),
                "{} manifest must expose at least one contract",
                manifest.target_id
            );

            for contract_id in manifest.contract_ids {
                let contract = identus_bindings::all_capability_contracts()
                    .iter()
                    .find(|contract| contract.id == *contract_id)
                    .unwrap_or_else(|| {
                        panic!("{} manifest references {contract_id}", manifest.target_id)
                    });
                assert!(
                    contract.wrapper_targets.contains(&manifest.target_id),
                    "{} manifest exposes unsupported contract {contract_id}",
                    manifest.target_id
                );
            }
        }
    }

    #[test]
    fn bindings_registry_matches_capability_contract_fixture() {
        let fixture = parse_json("capability contracts", CAPABILITY_CONTRACTS);
        let contracts = fixture
            .get("expected")
            .and_then(|expected| expected.get("contracts"))
            .and_then(serde_json::Value::as_array)
            .expect("capability fixture has no contracts array");

        let fixture_ids = capability_contract_ids(contracts);
        let registry_ids = identus_bindings::all_capability_contracts()
            .iter()
            .map(|contract| contract.id)
            .collect::<BTreeSet<_>>();

        assert_eq!(
            fixture_ids, registry_ids,
            "binding registry contract ids must match capability fixture"
        );

        for fixture_contract in contracts {
            assert_registry_contract_matches_fixture(fixture_contract);
        }

        assert_binding_manifests_reference_supported_contracts();

        assert!(
            TASKS.contains("T089") && TASKS.contains("[x] T089"),
            "T089 must be marked complete when binding registry matches fixture"
        );
    }

    #[test]
    fn crate_layout_adr_defines_dependency_rings() {
        for expected in [
            "Crate Layout And Dependency Rings",
            "Foundation",
            "`identus-core`",
            "Cryptography",
            "`identus-crypto`",
            "Identity",
            "`identus-did`",
            "`identus-trust`",
            "Credential And Presentation",
            "`identus-credentials`",
            "`identus-presentations`",
            "Protocol",
            "`identus-messaging`",
            "`identus-openid4vc`",
            "Wallet And Agent",
            "`identus-wallet`",
            "`identus-agent`",
            "Adapters",
            "`identus-adapters`",
            "Bindings",
            "`identus-bindings`",
            "Conformance",
            "`identus-conformance`",
            "Domain crates must not depend on `identus-adapters`",
            "Binding crates must expose DTOs, opaque handles, typed errors",
            "Conformance fixtures must be added before claiming support",
            "Optional infrastructure must have a Docker-free vector",
            "`crates/<name>/src/lib.rs`",
            "`fixtures/conformance/`",
            "`fixtures/schema/`",
            "`docs/architecture/`",
            "`docs/testing/`",
            "`docs/migration/`",
            "`tools/`",
            "Crates should not split only to mirror historical SDK module names.",
            "T084",
        ] {
            assert!(
                CRATE_LAYOUT_ADR.contains(expected) || TASKS.contains(expected),
                "crate layout ADR or tasks missing {expected}"
            );
        }
    }

    #[test]
    fn secure_storage_adr_defines_platform_adapter_boundary() {
        let wallet_source = included_crate_source("crates/wallet/src/lib.rs");
        let adapters_source = included_crate_source("crates/adapters/src/lib.rs");

        for expected in [
            "Platform Secure Storage Boundary",
            "adapter boundary",
            "identus-wallet",
            "identus-adapters",
            "PublicMetadata",
            "PrivateMetadata",
            "CredentialPayload",
            "KeyMaterial",
            "RecoveryMaterial",
            "SecureStore",
            "KeyStore",
            "SecretResolver",
            "BackupStore",
            "EntropySource",
            "Browser/WASM",
            "WebCrypto",
            "IndexedDB",
            "Node",
            "iOS",
            "Keychain",
            "Android",
            "Android Keystore",
            "JVM",
            "Server",
            "HSM/KMS",
            "Raw private keys must not cross public Rust, UniFFI, WASM, Node, Swift, or",
            "redact secrets",
            "Default tests must use deterministic in-memory adapters only.",
            "Infrastructure tests may exercise SQLite",
            "Backup and restore must be verified without production secrets",
            "redaction-safe errors",
            "backup manifest",
            "T073",
        ] {
            assert!(
                SECURE_STORAGE_ADR.contains(expected) || TASKS.contains(expected),
                "secure storage ADR or tasks missing {expected}"
            );
        }

        for expected in [
            "pub enum SecretClass",
            "pub struct StorageRecordMetadata",
            "pub struct StorageRecord",
            "pub struct KeyHandle",
            "pub struct BackupSnapshot",
            "pub trait SecureStore",
            "pub trait KeyStore",
            "pub trait SecretResolver",
            "pub trait BackupStore",
            "pub trait EntropySource",
            "record_not_found",
            "storage_version_conflict",
            "secret_unavailable",
            "invalid_storage_input",
        ] {
            assert!(
                wallet_source.contains(expected),
                "wallet secure-storage source missing {expected}"
            );
        }

        for expected in [
            "pub struct InMemorySecureStorageAdapter",
            "impl SecureStore for InMemorySecureStorageAdapter",
            "impl KeyStore for InMemorySecureStorageAdapter",
            "impl SecretResolver for InMemorySecureStorageAdapter",
            "impl BackupStore for InMemorySecureStorageAdapter",
            "pub struct DeterministicEntropy",
            "in_memory_backup_restore_keeps_records_and_redacts_missing_secrets",
            "in_memory_key_store_returns_handles_not_key_material",
            "in_memory_secure_store_supports_cas_and_listing",
        ] {
            assert!(
                adapters_source.contains(expected),
                "adapter secure-storage source missing {expected}"
            );
        }

        assert!(
            TASKS.contains("T073") && TASKS.contains("[x] T073"),
            "T073 must be marked complete when secure-storage ports and adapter are enforced"
        );
    }

    #[test]
    fn trust_status_adr_defines_policy_boundary() {
        let trust_source = included_crate_source("crates/trust/src/lib.rs");
        let credentials_source = included_crate_source("crates/credentials/src/lib.rs");
        let presentations_source = included_crate_source("crates/presentations/src/lib.rs");
        let openid4vc_source = included_crate_source("crates/openid4vc/src/lib.rs");

        for expected in [
            "Trust And Status Policy Boundary",
            "Verifiable Credentials Data Model v2.0",
            "W3C Recommendation, 2025-05-15",
            "Bitstring Status List v1.0",
            "Token Status List",
            "draft-ietf-oauth-status-list-20",
            "Standards Track intent",
            "OpenID Federation 1.0",
            "X.509",
            "identus-trust",
            "Credential status",
            "Status purpose",
            "Status mechanisms",
            "Trust anchors",
            "Trust chains",
            "Verification policy",
            "StatusResolver",
            "StatusVerifier",
            "TrustAnchorResolver",
            "TrustChainVerifier",
            "TrustPolicyEngine",
            "TrustEvidenceStore",
            "BitstringStatusListEntry",
            "BitstringStatusListCredential",
            "IETF Token Status List",
            "AnonCreds Revocation",
            "OpenID Federation And Trust Marks",
            "Entity statements and entity configurations",
            "Trust marks and trust mark status",
            "X.509 And IACA",
            "IACA trust anchors for mdoc",
            "Default tests must not call issuer, verifier, wallet, federation, OCSP, CRL",
            "Draft-based mechanisms must be versioned and feature-gated until stable.",
            "Static-model fixtures",
            "Vector fixtures",
            "Transcript fixtures",
            "Default `cargo test --workspace`",
            "redaction-safe",
            "T078",
        ] {
            assert!(
                TRUST_STATUS_ADR.contains(expected) || TASKS.contains(expected),
                "trust/status ADR or tasks missing {expected}"
            );
        }

        assert_trust_status_source_boundaries(
            trust_source,
            credentials_source,
            presentations_source,
            openid4vc_source,
        );

        assert_trust_status_policy_fixture_tracks_sources(
            trust_source,
            credentials_source,
            presentations_source,
            openid4vc_source,
        );

        assert!(
            TASKS.contains("T078") && TASKS.contains("[x] T078"),
            "T078 must be marked complete when trust/status ports and fixtures are enforced"
        );
    }

    #[test]
    fn openid4vc_conformance_adr_defines_protocol_boundary() {
        for expected in [
            "OpenID4VC Conformance Boundary",
            "OpenID for Verifiable Credential Issuance 1.0",
            "OpenID for Verifiable Presentations 1.0",
            "Self-Issued OpenID Provider v2",
            "OpenID4VC High Assurance Interoperability Profile 1.0",
            "OpenID Federation 1.0",
            "Final, published 2025-09-16",
            "Final, published 2025-07-09",
            "Draft 13, published 2023-11-28",
            "Final, published 2025-12-24",
            "Final, published 2026-02-17",
            "identus-openid4vc",
            "Credential issuer",
            "Wallet",
            "Verifier",
            "OAuth authorization server",
            "Trust anchor",
            "Credential issuer metadata",
            "Credential offer by value and by reference",
            "Authorization code flow",
            "Pre-authorized code flow",
            "Deferred credential endpoint",
            "Notification endpoint",
            "wrong nonce",
            "wrong state",
            "Same-device flow",
            "Cross-device flow",
            "direct_post",
            "direct_post.jwt",
            "DCQL credential",
            "SIOPv2 self-issued ID token flow",
            "OID4VP Digital Credentials API requirements",
            "Wallet attestation",
            "Key attestation",
            "Entity statements and entity configurations",
            "Trust chain resolution and validation",
            "Static-model fixtures",
            "Transcript fixtures",
            "Default `cargo test --workspace`",
            "CredentialIssuanceFlow",
            "PresentationFlow",
            "SelfIssuedOpenIdProviderFlow",
            "HighAssuranceProfileFlow",
            "FederationTrustFlow",
            "CredentialFormatRegistry",
            "PresentationQueryEngine",
            "TrustPolicyResolver",
            "SecureStore",
            "DidResolverPort",
            "SignerPort",
            "T075",
            "T076",
        ] {
            assert!(
                OPENID4VC_CONFORMANCE_ADR.contains(expected) || TASKS.contains(expected),
                "OpenID4VC conformance ADR or tasks missing {expected}"
            );
        }
    }

    #[test]
    fn openid4vc_transcript_fixture_covers_core_flows() {
        assert!(OPENID4VC_TRANSCRIPT_README.contains("OpenID4VC Transcript Fixtures"));
        assert!(OPENID4VC_TRANSCRIPT_README.contains("identus-openid4vc"));

        let transcript = parse_json("openid4vc core transcript", OPENID4VC_CORE_TRANSCRIPT);
        let messages = transcript
            .get("messages")
            .and_then(serde_json::Value::as_array)
            .expect("OpenID4VC transcript has no messages array");
        let expected_states = transcript
            .get("expected_states")
            .and_then(serde_json::Value::as_array)
            .expect("OpenID4VC transcript has no expected_states array");

        assert!(
            messages.len() >= 20,
            "expected broad OpenID4VC flow coverage"
        );

        for expected_spec in [
            "oid4vci-1",
            "oid4vp-1",
            "siopv2",
            "openid4vc-haip",
            "openid-federation-1",
        ] {
            assert!(
                OPENID4VC_CORE_TRANSCRIPT.contains(expected_spec),
                "OpenID4VC transcript missing spec {expected_spec}"
            );
        }

        for expected_flow in [
            "oid4vci_authorization_code",
            "oid4vci_pre_authorized_code",
            "oid4vci_deferred_credential",
            "oid4vp_same_device_direct_post",
            "oid4vp_cross_device_direct_post",
            "siopv2_self_issued",
            "haip_wallet_attestation",
            "haip_digital_credentials_api",
            "federation_backed_trust",
            "negative_wrong_nonce",
            "negative_trust_chain_rejection",
        ] {
            assert_message_flow(messages, expected_flow);
        }

        for expected_state in [
            "oid4vci_authorization_code_credential_issued",
            "oid4vci_pre_authorized_code_credential_issued",
            "oid4vci_deferred_credential_ready",
            "oid4vp_same_device_direct_post_response_submitted",
            "oid4vp_cross_device_direct_post_encrypted_response_submitted",
            "siopv2_self_issued_id_token_validated",
            "haip_wallet_attestation_validated",
            "haip_digital_credentials_api_request_classified",
            "federation_trust_chain_validated",
            "negative_wrong_nonce_rejected",
            "negative_trust_chain_rejected",
        ] {
            assert!(
                expected_states
                    .iter()
                    .filter_map(serde_json::Value::as_str)
                    .any(|state| state == expected_state),
                "OpenID4VC transcript missing expected state {expected_state}"
            );
        }

        for prohibited in ["access_token\":", "authorization_code\":", "private_key"] {
            assert!(
                !OPENID4VC_CORE_TRANSCRIPT.contains(prohibited),
                "OpenID4VC transcript contains prohibited token {prohibited}"
            );
        }
    }

    #[test]
    fn openid4vc_transcript_replays_through_typed_state_machines() {
        let transcript = parse_json("openid4vc core transcript", OPENID4VC_CORE_TRANSCRIPT);
        let messages = transcript
            .get("messages")
            .and_then(serde_json::Value::as_array)
            .expect("OpenID4VC transcript has no messages array");
        let expected_states = transcript
            .get("expected_states")
            .and_then(serde_json::Value::as_array)
            .expect("OpenID4VC transcript has no expected_states array")
            .iter()
            .map(|state| state.as_str().expect("expected state must be a string"))
            .collect::<BTreeSet<_>>();
        let replay_messages = messages
            .iter()
            .map(openid4vc_transcript_message)
            .collect::<Vec<_>>();

        let events = identus_openid4vc::replay_openid4vc_transcript(&replay_messages);
        let replayed_states = events
            .iter()
            .map(|event| event.state.as_str())
            .collect::<BTreeSet<_>>();

        assert_eq!(
            replayed_states, expected_states,
            "OpenID4VC replayed states must match transcript expected states"
        );

        let replayed_errors = events
            .iter()
            .filter_map(|event| event.error.as_ref().map(|error| error.code().as_str()))
            .collect::<BTreeSet<_>>();
        assert!(replayed_errors.contains("nonce_mismatch"));
        assert!(replayed_errors.contains("trust_chain_rejected"));

        for event in events {
            if let Some(error) = event.error {
                assert!(
                    !error.to_string().contains("redacted"),
                    "OpenID4VC replay error leaked fixture text"
                );
            }
        }

        assert!(
            TASKS.contains("T076") && TASKS.contains("[x] T076"),
            "T076 must be marked complete when OpenID4VC transcript replay is enforced"
        );
    }

    #[test]
    fn openid4vci_transcript_drives_credential_issuance_state_machine() {
        let openid4vc_source = included_crate_source("crates/openid4vc/src/lib.rs");
        let transcript = parse_json("openid4vc core transcript", OPENID4VC_CORE_TRANSCRIPT);
        let messages = transcript
            .get("messages")
            .and_then(serde_json::Value::as_array)
            .expect("OpenID4VC transcript has no messages array");

        for expected in [
            "pub enum CredentialIssuanceFlowKind",
            "pub enum CredentialIssuanceState",
            "pub enum CredentialIssuanceEvent",
            "pub struct CredentialIssuanceTransition",
            "pub struct CredentialIssuanceStateMachine",
            "pub fn apply(",
            "pub const fn invalid_openid4vc_transition()",
            "invalid_openid4vc_transition",
            "credential_issuance_state_machine_accepts_oid4vci_sequences",
            "credential_issuance_state_machine_rejects_invalid_transition",
        ] {
            assert!(
                openid4vc_source.contains(expected),
                "OpenID4VC source missing OID4VCI state-machine boundary {expected}"
            );
        }

        let mut authorization_code = identus_openid4vc::CredentialIssuanceStateMachine::new(
            identus_openid4vc::CredentialIssuanceFlowKind::AuthorizationCode,
        );
        apply_oid4vci_transcript_flow(
            messages,
            "oid4vci_authorization_code",
            &mut authorization_code,
        );
        assert_eq!(
            authorization_code.state(),
            identus_openid4vc::CredentialIssuanceState::CredentialIssued
        );

        let mut pre_authorized = identus_openid4vc::CredentialIssuanceStateMachine::new(
            identus_openid4vc::CredentialIssuanceFlowKind::PreAuthorizedCode,
        );
        apply_oid4vci_transcript_flow(messages, "oid4vci_pre_authorized_code", &mut pre_authorized);
        assert_eq!(
            pre_authorized.state(),
            identus_openid4vc::CredentialIssuanceState::CredentialIssued
        );

        let mut deferred = identus_openid4vc::CredentialIssuanceStateMachine::new(
            identus_openid4vc::CredentialIssuanceFlowKind::DeferredCredential,
        );
        apply_oid4vci_transcript_flow(messages, "oid4vci_deferred_credential", &mut deferred);
        assert_eq!(
            deferred.state(),
            identus_openid4vc::CredentialIssuanceState::DeferredCredentialReady
        );

        let mut invalid = identus_openid4vc::CredentialIssuanceStateMachine::new(
            identus_openid4vc::CredentialIssuanceFlowKind::AuthorizationCode,
        );
        let error = invalid
            .apply(identus_openid4vc::CredentialIssuanceEvent::CredentialResponse)
            .expect_err("credential response cannot be accepted first");
        assert_eq!(error.code().as_str(), "invalid_openid4vc_transition");
        assert!(!error.to_string().contains("redacted"));

        assert!(
            TASKS.contains("T046") && TASKS.contains("[x] T046"),
            "T046 must be marked complete when OID4VCI state-machine transitions are enforced"
        );
    }

    #[test]
    fn openid4vp_and_siopv2_transcripts_drive_state_machines() {
        let openid4vc_source = included_crate_source("crates/openid4vc/src/lib.rs");
        let transcript = parse_json("openid4vc core transcript", OPENID4VC_CORE_TRANSCRIPT);
        let messages = transcript
            .get("messages")
            .and_then(serde_json::Value::as_array)
            .expect("OpenID4VC transcript has no messages array");

        for expected in [
            "pub enum PresentationFlowKind",
            "pub enum PresentationState",
            "pub enum PresentationEvent",
            "pub struct PresentationTransition",
            "pub struct PresentationStateMachine",
            "pub enum SelfIssuedOpenIdProviderState",
            "pub enum SelfIssuedOpenIdProviderEvent",
            "pub struct SelfIssuedOpenIdProviderTransition",
            "pub struct SelfIssuedOpenIdProviderStateMachine",
            "presentation_state_machine_accepts_oid4vp_sequences",
            "presentation_state_machine_rejects_wrong_nonce_without_state_change",
            "self_issued_openid_provider_state_machine_accepts_siopv2_sequence",
            "self_issued_openid_provider_state_machine_rejects_invalid_transition",
        ] {
            assert!(
                openid4vc_source.contains(expected),
                "OpenID4VC source missing OID4VP/SIOPv2 state-machine boundary {expected}"
            );
        }

        let mut same_device = identus_openid4vc::PresentationStateMachine::new(
            identus_openid4vc::PresentationFlowKind::SameDeviceDirectPost,
        );
        apply_oid4vp_transcript_flow(messages, "oid4vp_same_device_direct_post", &mut same_device);
        assert_eq!(
            same_device.state(),
            identus_openid4vc::PresentationState::ResponseSubmitted
        );

        let mut cross_device = identus_openid4vc::PresentationStateMachine::new(
            identus_openid4vc::PresentationFlowKind::CrossDeviceDirectPost,
        );
        apply_oid4vp_transcript_flow(
            messages,
            "oid4vp_cross_device_direct_post",
            &mut cross_device,
        );
        assert_eq!(
            cross_device.state(),
            identus_openid4vc::PresentationState::EncryptedResponseSubmitted
        );

        let mut siopv2 = identus_openid4vc::SelfIssuedOpenIdProviderStateMachine::new();
        apply_siopv2_transcript_flow(messages, &mut siopv2);
        assert_eq!(
            siopv2.state(),
            identus_openid4vc::SelfIssuedOpenIdProviderState::IdTokenValidated
        );

        let mut negative = identus_openid4vc::PresentationStateMachine::new(
            identus_openid4vc::PresentationFlowKind::SameDeviceDirectPost,
        );
        negative
            .apply(identus_openid4vc::PresentationEvent::AuthorizationRequestObject)
            .expect("same-device request should be accepted before negative response");
        let error = negative
            .apply(identus_openid4vc::PresentationEvent::AuthorizationResponseRejected)
            .expect_err("wrong nonce response must be rejected");
        assert_eq!(error.code().as_str(), "nonce_mismatch");
        assert!(!error.to_string().contains("redacted"));

        assert!(
            TASKS.contains("T047") && TASKS.contains("[x] T047"),
            "T047 must be marked complete when OID4VP/SIOPv2 state-machine transitions are enforced"
        );
    }

    #[test]
    fn didcomm_transcript_fixtures_parse_with_messaging_primitives() {
        assert!(DIDCOMM_TRANSCRIPT_README.contains("DIDComm Transcript Fixtures"));

        let transcripts = [
            DIDCOMM_MEDIATION_PICKUP_TRANSCRIPT,
            DIDCOMM_CREDENTIAL_PRESENTATION_TRANSCRIPT,
            DIDCOMM_CONNECTIONLESS_OOB_TRANSCRIPT,
            DIDCOMM_COMPATIBILITY_TRANSCRIPT,
        ];
        let message_types = transcripts
            .iter()
            .flat_map(|transcript| extract_didcomm_message_types(transcript))
            .collect::<Vec<_>>();

        assert!(
            message_types.len() >= 20,
            "expected broad DIDComm fixture coverage"
        );

        for message_type in &message_types {
            identus_messaging::DidCommMessageType::parse(message_type)
                .unwrap_or_else(|error| panic!("{message_type} did not parse: {error}"));
        }

        for required in [
            "https://didcomm.org/out-of-band/2.0/invitation",
            "https://didcomm.org/basicmessage/2.0/message",
            "https://didcomm.org/trust-ping/2.0/ping",
            "https://didcomm.org/discover-features/2.0/queries",
            "https://didcomm.org/routing/2.0/forward",
            "https://didcomm.org/coordinate-mediation/2.0/mediate-request",
            "https://didcomm.org/messagepickup/3.0/status-request",
            "https://didcomm.org/issue-credential/3.0/offer-credential",
            "https://didcomm.org/present-proof/3.0/request-presentation",
            "https://didcomm.org/report-problem/2.0/problem-report",
            "https://didcomm.org/revocation-notification/1.0/revoke",
            "https://didcomm.org/present-proof/3.0/presentation",
            "https://didcomm.org/didexchange/1.0/request",
            "https://didcomm.org/connections/1.0/invitation",
            "https://didcomm.org/mediator-coordination/2.0/mediate-request",
            "https://didcomm.org/pickup/3.0/status-request",
        ] {
            assert!(
                message_types.contains(&required),
                "DIDComm transcript fixtures missing {required}"
            );
        }
    }

    #[test]
    fn didcomm_mediation_pickup_transcript_drives_state_machines() {
        let messaging_source = included_crate_source("crates/messaging/src/lib.rs");

        for expected in [
            "pub enum MediationCoordinationState",
            "pub enum MediationCoordinationEvent",
            "pub struct MediationCoordinationTransition",
            "pub struct MediationCoordinationStateMachine",
            "pub enum MessagePickupState",
            "pub enum MessagePickupEvent",
            "pub struct MessagePickupTransition",
            "pub struct MessagePickupStateMachine",
            "pub enum MediatorRoutingState",
            "pub enum MediatorRoutingEvent",
            "pub struct MediatorRoutingTransition",
            "pub struct MediatorRoutingStateMachine",
            "pub const fn invalid_didcomm_transition()",
            "mediation_coordination_state_machine_accepts_grant_and_keylist_update",
            "message_pickup_state_machine_accepts_status_delivery_and_ack",
            "mediator_routing_state_machine_accepts_forward_once",
        ] {
            assert!(
                messaging_source.contains(expected),
                "messaging source missing mediation/pickup boundary {expected}"
            );
        }

        let transcript = parse_json(
            "DIDComm mediation and pickup transcript",
            DIDCOMM_MEDIATION_PICKUP_TRANSCRIPT,
        );
        let expected_states = transcript
            .get("expected_states")
            .and_then(serde_json::Value::as_array)
            .expect("DIDComm mediation transcript has no expected_states array")
            .iter()
            .map(|state| state.as_str().expect("expected state must be a string"))
            .collect::<BTreeSet<_>>();
        let messages = transcript
            .get("messages")
            .and_then(serde_json::Value::as_array)
            .expect("DIDComm mediation transcript has no messages array");

        let replayed_states = replay_didcomm_mediation_pickup_states(messages);
        for required_state in [
            "mediation_granted",
            "recipient_key_registered",
            "message_forwarded",
            "pickup_completed",
        ] {
            assert!(
                replayed_states.contains(required_state),
                "DIDComm mediation replay missing state {required_state}"
            );
            assert!(
                expected_states.contains(required_state),
                "DIDComm mediation fixture missing state {required_state}"
            );
        }

        let compatibility = parse_json(
            "DIDComm compatibility transcript",
            DIDCOMM_COMPATIBILITY_TRANSCRIPT,
        );
        let compatibility_messages = compatibility
            .get("messages")
            .and_then(serde_json::Value::as_array)
            .expect("DIDComm compatibility transcript has no messages array");
        let compatibility_states =
            classify_didcomm_mediation_pickup_aliases(compatibility_messages);
        assert!(compatibility_states.contains("mediation_alias_classified"));
        assert!(compatibility_states.contains("pickup_alias_classified"));

        let mut invalid = identus_messaging::MediationCoordinationStateMachine::new();
        let error = invalid
            .apply(identus_messaging::MediationCoordinationEvent::MediateGrant)
            .expect_err("mediate-grant cannot be first");
        assert_eq!(error.code().as_str(), "invalid_didcomm_transition");
        assert!(!error.to_string().contains("redacted"));

        assert!(
            TASKS.contains("T045") && TASKS.contains("[x] T045"),
            "T045 must be marked complete when mediation and pickup state machines are enforced"
        );
    }

    #[test]
    fn didcomm_credential_presentation_transcript_drives_state_machines() {
        let messaging_source = included_crate_source("crates/messaging/src/lib.rs");

        for expected in [
            "pub enum IssueCredentialState",
            "pub enum IssueCredentialEvent",
            "pub struct IssueCredentialTransition",
            "pub struct IssueCredentialStateMachine",
            "pub enum PresentProofState",
            "pub enum PresentProofEvent",
            "pub struct PresentProofTransition",
            "pub struct PresentProofStateMachine",
            "pub enum ReportProblemState",
            "pub enum ReportProblemEvent",
            "pub struct ReportProblemTransition",
            "pub struct ReportProblemStateMachine",
            "pub enum TrustPingState",
            "pub enum TrustPingEvent",
            "pub struct TrustPingTransition",
            "pub struct TrustPingStateMachine",
            "pub enum RevocationNotificationState",
            "pub enum RevocationNotificationEvent",
            "pub struct RevocationNotificationTransition",
            "pub struct RevocationNotificationStateMachine",
            "issue_credential_state_machine_accepts_offer_request_issue",
            "present_proof_state_machine_accepts_request_presentation",
            "report_problem_revocation_and_trust_ping_state_machines_accept_protocol_events",
        ] {
            assert!(
                messaging_source.contains(expected),
                "messaging source missing DIDComm protocol boundary {expected}"
            );
        }

        let transcript = parse_json(
            "DIDComm credential presentation transcript",
            DIDCOMM_CREDENTIAL_PRESENTATION_TRANSCRIPT,
        );
        let expected_states = transcript
            .get("expected_states")
            .and_then(serde_json::Value::as_array)
            .expect("DIDComm credential transcript has no expected_states array")
            .iter()
            .map(|state| state.as_str().expect("expected state must be a string"))
            .collect::<BTreeSet<_>>();
        let messages = transcript
            .get("messages")
            .and_then(serde_json::Value::as_array)
            .expect("DIDComm credential transcript has no messages array");

        let replayed_states = replay_didcomm_credential_presentation_states(messages);
        for required_state in [
            "credential_offer_received",
            "credential_requested",
            "credential_issued",
            "presentation_requested",
            "presentation_submitted",
            "problem_report_recorded",
            "credential_revoked",
        ] {
            assert!(
                replayed_states.contains(required_state),
                "DIDComm credential/presentation replay missing state {required_state}"
            );
            assert!(
                expected_states.contains(required_state),
                "DIDComm credential/presentation fixture missing state {required_state}"
            );
        }

        let mediation_transcript = parse_json(
            "DIDComm mediation and pickup transcript",
            DIDCOMM_MEDIATION_PICKUP_TRANSCRIPT,
        );
        let mediation_messages = mediation_transcript
            .get("messages")
            .and_then(serde_json::Value::as_array)
            .expect("DIDComm mediation transcript has no messages array");
        let trust_ping_states = replay_didcomm_trust_ping_states(mediation_messages);
        assert!(trust_ping_states.contains("trust_ping_acknowledged"));

        let mut invalid = identus_messaging::IssueCredentialStateMachine::new();
        let error = invalid
            .apply(identus_messaging::IssueCredentialEvent::IssueCredential)
            .expect_err("issue-credential cannot be first");
        assert_eq!(error.code().as_str(), "invalid_didcomm_transition");
        assert!(!error.to_string().contains("redacted"));

        assert!(
            TASKS.contains("T044") && TASKS.contains("[x] T044"),
            "T044 must be marked complete when DIDComm protocol state machines are enforced"
        );
    }

    #[test]
    fn connectionless_oob_fixture_records_no_persistent_connection_expectation() {
        for expected in [
            "connectionless_credential_offer_accepted_without_connection",
            "connectionless_credential_issued_without_connection",
            "connectionless_presentation_submitted_without_connection",
            "\"pthid\": \"connectionless-offer-oob-1\"",
            "\"pthid\": \"connectionless-proof-oob-1\"",
            "\"goal_code\": \"issue-vc\"",
            "\"goal_code\": \"verify-credential\"",
        ] {
            assert!(
                DIDCOMM_CONNECTIONLESS_OOB_TRANSCRIPT.contains(expected),
                "connectionless OOB transcript missing {expected}"
            );
        }
    }

    #[test]
    fn integration_runner_contract_tracks_sdk_rust_execution_shape() {
        for expected in [
            "Runner key | `sdk-rust`",
            "hyperledger-identus/sdk-rust",
            "Embedded agent and mediator mode",
            "External Cloud Agent and Mediator services",
            "tmp/sdk-rust",
            "target/sdk-rust/allure-results",
            "cargo test --workspace",
            "SDK_RUST_MODE",
            "AGENT_URL",
            "MEDIATOR_OOB_URL",
            "TEST_RUNNER_PRISM_AGENT_URL",
            "TEST_RUNNER_MEDIATOR_OOB_URL",
            "SDK_RUST_ALLURE_DIR",
            "SDK_RUST_SELECTED_FLOWS",
            "backup_restore",
            "receive_oob_jwt_credential",
            "provide_oob_jwt_proof",
            "env.runners[\"sdk-rust\"].version",
        ] {
            assert!(
                INTEGRATION_RUNNER_CONTRACT.contains(expected),
                "integration runner contract missing {expected}"
            );
        }
    }

    #[test]
    fn each_functional_area_has_a_conformance_entry() {
        let areas = ALL_SPECIFICATIONS
            .iter()
            .map(|specification| specification.area)
            .collect::<BTreeSet<_>>();

        for required_area in [
            SpecArea::Crypto,
            SpecArea::Did,
            SpecArea::Credential,
            SpecArea::Presentation,
            SpecArea::Messaging,
            SpecArea::Mediation,
            SpecArea::OpenId,
            SpecArea::Trust,
            SpecArea::Platform,
        ] {
            assert!(
                areas.contains(&required_area),
                "missing area {required_area:?}"
            );
        }
    }

    #[test]
    fn each_owner_crate_has_a_conformance_entry() {
        let owners = ALL_SPECIFICATIONS
            .iter()
            .map(|specification| specification.owner)
            .collect::<BTreeSet<_>>();

        for required_owner in [
            OwnerCrate::Crypto,
            OwnerCrate::Did,
            OwnerCrate::Credentials,
            OwnerCrate::Presentations,
            OwnerCrate::Messaging,
            OwnerCrate::OpenId4Vc,
            OwnerCrate::Trust,
            OwnerCrate::Adapters,
            OwnerCrate::Bindings,
        ] {
            assert!(
                owners.contains(&required_owner),
                "missing owner {required_owner:?}"
            );
        }
    }

    #[test]
    fn conformance_modes_cover_catalog_depth() {
        let modes = ALL_SPECIFICATIONS
            .iter()
            .map(|specification| specification.mode)
            .collect::<BTreeSet<_>>();

        for required_mode in [
            ConformanceMode::StaticModel,
            ConformanceMode::Vector,
            ConformanceMode::Transcript,
            ConformanceMode::Interop,
            ConformanceMode::Infrastructure,
        ] {
            assert!(
                modes.contains(&required_mode),
                "missing conformance mode {required_mode:?}"
            );
        }
    }

    #[test]
    fn parity_specifications_are_the_default_commitment() {
        let parity_count = ALL_SPECIFICATIONS
            .iter()
            .filter(|specification| specification.stage == SupportStage::Parity)
            .count();
        let roadmap_count = ALL_SPECIFICATIONS
            .iter()
            .filter(|specification| specification.stage == SupportStage::Roadmap)
            .count();

        assert!(parity_count > roadmap_count);
    }

    fn extract_didcomm_message_types(transcript: &'static str) -> Vec<&'static str> {
        transcript
            .lines()
            .filter_map(|line| {
                let trimmed = line.trim();
                let value = trimmed.strip_prefix("\"type\": \"")?;
                let (message_type, _) = value.split_once('"')?;
                Some(message_type)
            })
            .collect()
    }

    fn included_crate_source(path: &str) -> &'static str {
        CRATE_SOURCES
            .iter()
            .find_map(|(included_path, source)| (*included_path == path).then_some(*source))
            .unwrap_or_else(|| panic!("{path} source is not included in conformance audit"))
    }

    fn typed_error_capability<'a>(
        capabilities: &'a [serde_json::Value],
        capability_id: &str,
    ) -> &'a serde_json::Value {
        capabilities
            .iter()
            .find(|capability| {
                capability
                    .get("capability")
                    .and_then(serde_json::Value::as_str)
                    .is_some_and(|actual| actual == capability_id)
            })
            .unwrap_or_else(|| panic!("typed error catalog missing {capability_id} capability"))
    }

    fn typed_error_codes(capability: &serde_json::Value) -> &[serde_json::Value] {
        capability
            .get("codes")
            .and_then(serde_json::Value::as_array)
            .map(Vec::as_slice)
            .expect("typed error capability has no codes array")
    }

    fn assert_typed_error_capability(
        capability: &serde_json::Value,
        local_error: &str,
        owner_crate: &str,
        source: &str,
    ) {
        assert_eq!(
            json_str(capability, "owner_crate"),
            owner_crate,
            "{owner_crate} typed error catalog owner drifted"
        );
        assert_eq!(
            json_str(capability, "local_error"),
            local_error,
            "{owner_crate} typed error catalog local error drifted"
        );
        assert_eq!(json_str(capability, "core_error"), "IdentusError");
        assert_eq!(json_str(capability, "binding_envelope"), "ErrorEnvelope");
        assert_eq!(json_str(capability, "error_kind"), "InvalidInput");

        for entry in typed_error_codes(capability) {
            let code = json_str(entry, "code");
            let message = json_str(entry, "message");
            let source_variant = json_str(entry, "source_variant");

            assert!(
                source.contains(code),
                "{owner_crate} source missing typed error code {code}"
            );
            assert!(
                CORE_ERROR_CONVENTIONS.contains(code),
                "core error conventions missing typed error code {code}"
            );
            assert!(
                source.contains(message),
                "{owner_crate} source missing public message for {code}"
            );
            assert!(
                source.contains(source_variant),
                "{owner_crate} source missing variant {source_variant} for {code}"
            );

            let binding_targets = entry
                .get("binding_targets")
                .and_then(serde_json::Value::as_array)
                .unwrap_or_else(|| panic!("{code} has no binding_targets array"));
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
                    binding_targets.iter().any(|target| {
                        target
                            .as_str()
                            .is_some_and(|actual| actual == expected_target)
                    }),
                    "{code} missing binding target {expected_target}"
                );
            }
        }
    }

    fn assert_trust_status_source_boundaries(
        trust_source: &str,
        credentials_source: &str,
        presentations_source: &str,
        openid4vc_source: &str,
    ) {
        for expected in [
            "pub enum StatusState",
            "pub enum StatusPurpose",
            "pub enum StatusMechanism",
            "pub struct StatusReference",
            "pub struct StatusDecision",
            "pub struct TrustAnchor",
            "pub struct TrustChain",
            "pub struct TrustPolicyInput",
            "pub struct TrustPolicyDecision",
            "pub trait StatusResolver",
            "pub trait StatusVerifier",
            "pub trait TrustAnchorResolver",
            "pub trait TrustChainVerifier",
            "pub trait TrustPolicyEngine",
            "pub trait TrustEvidenceStore",
            "pub struct InMemoryTrustRegistry",
            "with_default_fixtures",
            "default_trust_fixtures_cover_status_mechanisms",
            "revoked_status_returns_redaction_safe_typed_error",
            "trust_chain_requires_trusted_anchor",
        ] {
            assert!(
                trust_source.contains(expected),
                "trust source missing {expected}"
            );
        }

        for (consumer_name, consumer_source, expected_function) in [
            (
                "credentials",
                credentials_source,
                "evaluate_credential_trust",
            ),
            (
                "presentations",
                presentations_source,
                "evaluate_presentation_trust",
            ),
            ("openid4vc", openid4vc_source, "evaluate_openid4vc_trust"),
        ] {
            assert!(
                consumer_source.contains("TrustPolicyEngine")
                    && consumer_source.contains(expected_function),
                "{consumer_name} source must delegate trust policy through identus-trust"
            );
        }
    }

    fn assert_trust_status_policy_fixture_tracks_sources(
        trust_source: &str,
        credentials_source: &str,
        presentations_source: &str,
        openid4vc_source: &str,
    ) {
        assert_fixture_satisfies_schema("static-model", STATIC_MODEL_SCHEMA, TRUST_STATUS_POLICY);
        let fixture = parse_json("trust/status policy", TRUST_STATUS_POLICY);
        let expected = fixture
            .get("expected")
            .expect("trust/status policy fixture has no expected object");

        let mechanisms = expected
            .get("mechanisms")
            .and_then(serde_json::Value::as_array)
            .expect("trust/status policy fixture has no mechanisms array");
        let minimum_mechanism_count = expected
            .get("minimum_mechanism_count")
            .and_then(serde_json::Value::as_u64)
            .expect("trust/status policy fixture has no minimum_mechanism_count");
        let minimum_mechanism_count = usize::try_from(minimum_mechanism_count)
            .expect("trust/status policy minimum_mechanism_count does not fit usize");
        assert!(
            mechanisms.len() >= minimum_mechanism_count,
            "trust/status fixture has too few mechanisms"
        );

        for port in json_string_values(expected, "ports") {
            assert!(
                trust_source.contains(&format!("pub trait {port}")),
                "trust source missing port {port}"
            );
        }

        for state in json_string_values(expected, "status_states") {
            assert!(
                trust_source.contains(&state),
                "trust source missing status state {state}"
            );
        }

        for mechanism in mechanisms {
            let rust_variant = json_str(mechanism, "rust_variant");
            let fixture_reference = json_str(mechanism, "fixture_reference");
            let anchor_reference = json_str(mechanism, "anchor_reference");
            assert!(
                trust_source.contains(rust_variant),
                "trust source missing mechanism variant {rust_variant}"
            );
            assert!(
                fixture_reference.starts_with("fixture-"),
                "trust fixture reference must be synthetic: {fixture_reference}"
            );
            assert!(
                anchor_reference.starts_with("fixture-anchor-"),
                "trust fixture anchor reference must be synthetic: {anchor_reference}"
            );
            assert_eq!(
                mechanism
                    .get("docker_required")
                    .and_then(serde_json::Value::as_bool),
                Some(false),
                "{rust_variant} must be Docker-free in the default fixture"
            );
        }

        for typed_error in json_string_values(expected, "typed_errors") {
            assert!(
                trust_source.contains(&typed_error),
                "trust source missing typed error {typed_error}"
            );
        }

        let consumers = expected
            .get("consumers")
            .and_then(serde_json::Value::as_array)
            .expect("trust/status policy fixture has no consumers array");
        for consumer in consumers {
            let function = json_str(consumer, "function");
            let source = match json_str(consumer, "crate") {
                "identus-credentials" => credentials_source,
                "identus-presentations" => presentations_source,
                "identus-openid4vc" => openid4vc_source,
                crate_name => panic!("unexpected trust/status consumer {crate_name}"),
            };
            assert!(
                source.contains(function) && source.contains("TrustPolicyEngine"),
                "trust/status consumer source missing {function}"
            );
        }
    }

    fn openid4vc_transcript_message(
        message: &serde_json::Value,
    ) -> identus_openid4vc::OpenId4VcTranscriptMessage<'_> {
        identus_openid4vc::OpenId4VcTranscriptMessage {
            flow: json_str(message, "flow"),
            message_type: json_str(message, "type"),
            expected_error: message
                .get("body")
                .and_then(|body| body.get("expected_error"))
                .and_then(serde_json::Value::as_str),
        }
    }

    fn apply_oid4vci_transcript_flow(
        messages: &[serde_json::Value],
        flow: &str,
        state_machine: &mut identus_openid4vc::CredentialIssuanceStateMachine,
    ) {
        for message in messages
            .iter()
            .filter(|message| json_str(message, "flow") == flow)
        {
            let transcript_message = openid4vc_transcript_message(message);
            if let Some(event) = identus_openid4vc::CredentialIssuanceEvent::from_transcript_message(
                &transcript_message,
            ) {
                state_machine
                    .apply(event)
                    .unwrap_or_else(|error| panic!("{flow} transition failed: {error}"));
            }
        }
    }

    fn apply_oid4vp_transcript_flow(
        messages: &[serde_json::Value],
        flow: &str,
        state_machine: &mut identus_openid4vc::PresentationStateMachine,
    ) {
        for message in messages
            .iter()
            .filter(|message| json_str(message, "flow") == flow)
        {
            let transcript_message = openid4vc_transcript_message(message);
            if let Some(event) =
                identus_openid4vc::PresentationEvent::from_transcript_message(&transcript_message)
            {
                state_machine
                    .apply(event)
                    .unwrap_or_else(|error| panic!("{flow} transition failed: {error}"));
            }
        }
    }

    fn apply_siopv2_transcript_flow(
        messages: &[serde_json::Value],
        state_machine: &mut identus_openid4vc::SelfIssuedOpenIdProviderStateMachine,
    ) {
        for message in messages
            .iter()
            .filter(|message| json_str(message, "flow") == "siopv2_self_issued")
        {
            let transcript_message = openid4vc_transcript_message(message);
            if let Some(event) =
                identus_openid4vc::SelfIssuedOpenIdProviderEvent::from_transcript_message(
                    &transcript_message,
                )
            {
                state_machine
                    .apply(event)
                    .unwrap_or_else(|error| panic!("siopv2 transition failed: {error}"));
            }
        }
    }

    fn replay_didcomm_mediation_pickup_states(
        messages: &[serde_json::Value],
    ) -> BTreeSet<&'static str> {
        let mut states = BTreeSet::new();
        let mut mediation = identus_messaging::MediationCoordinationStateMachine::new();
        let mut pickup = identus_messaging::MessagePickupStateMachine::new();
        let mut routing = identus_messaging::MediatorRoutingStateMachine::new();

        for message in messages {
            let message_type =
                identus_messaging::DidCommMessageType::parse(json_str(message, "type"))
                    .expect("DIDComm mediation transcript message type should parse");

            if let Some(event) =
                identus_messaging::MediationCoordinationEvent::from_message_type(&message_type)
            {
                let transition = mediation
                    .apply(event)
                    .unwrap_or_else(|error| panic!("mediation transition failed: {error}"));
                states.insert(transition.to.as_str());
            }

            if let Some(event) =
                identus_messaging::MessagePickupEvent::from_message_type(&message_type)
            {
                let transition = pickup
                    .apply(event)
                    .unwrap_or_else(|error| panic!("pickup transition failed: {error}"));
                states.insert(transition.to.as_str());
            }

            if let Some(event) =
                identus_messaging::MediatorRoutingEvent::from_message_type(&message_type)
            {
                let transition = routing
                    .apply(event)
                    .unwrap_or_else(|error| panic!("routing transition failed: {error}"));
                states.insert(transition.to.as_str());
            }
        }

        states
    }

    fn classify_didcomm_mediation_pickup_aliases(
        messages: &[serde_json::Value],
    ) -> BTreeSet<&'static str> {
        let mut states = BTreeSet::new();

        for message in messages {
            let message_type =
                identus_messaging::DidCommMessageType::parse(json_str(message, "type"))
                    .expect("DIDComm compatibility transcript message type should parse");
            if identus_messaging::MediationCoordinationEvent::from_message_type(&message_type)
                .is_some()
                && message_type.family() == "mediator-coordination"
            {
                states.insert("mediation_alias_classified");
            }
            if identus_messaging::MessagePickupEvent::from_message_type(&message_type).is_some()
                && message_type.family() == "pickup"
            {
                states.insert("pickup_alias_classified");
            }
        }

        states
    }

    fn replay_didcomm_credential_presentation_states(
        messages: &[serde_json::Value],
    ) -> BTreeSet<&'static str> {
        let mut states = BTreeSet::new();
        let mut issue = identus_messaging::IssueCredentialStateMachine::new();
        let mut proof = identus_messaging::PresentProofStateMachine::new();
        let mut report_problem = identus_messaging::ReportProblemStateMachine::new();
        let mut revocation = identus_messaging::RevocationNotificationStateMachine::new();

        for message in messages {
            let message_type =
                identus_messaging::DidCommMessageType::parse(json_str(message, "type"))
                    .expect("DIDComm credential transcript message type should parse");

            if let Some(event) =
                identus_messaging::IssueCredentialEvent::from_message_type(&message_type)
            {
                let transition = issue
                    .apply(event)
                    .unwrap_or_else(|error| panic!("issue-credential transition failed: {error}"));
                states.insert(transition.to.as_str());
            }

            if let Some(event) =
                identus_messaging::PresentProofEvent::from_message_type(&message_type)
            {
                let transition = proof
                    .apply(event)
                    .unwrap_or_else(|error| panic!("present-proof transition failed: {error}"));
                states.insert(transition.to.as_str());
            }

            if let Some(event) =
                identus_messaging::ReportProblemEvent::from_message_type(&message_type)
            {
                let transition = report_problem
                    .apply(event)
                    .unwrap_or_else(|error| panic!("report-problem transition failed: {error}"));
                states.insert(transition.to.as_str());
            }

            if let Some(event) =
                identus_messaging::RevocationNotificationEvent::from_message_type(&message_type)
            {
                let transition = revocation
                    .apply(event)
                    .unwrap_or_else(|error| panic!("revocation transition failed: {error}"));
                states.insert(transition.to.as_str());
            }
        }

        states
    }

    fn replay_didcomm_trust_ping_states(messages: &[serde_json::Value]) -> BTreeSet<&'static str> {
        let mut states = BTreeSet::new();
        let mut trust_ping = identus_messaging::TrustPingStateMachine::new();

        for message in messages {
            let message_type =
                identus_messaging::DidCommMessageType::parse(json_str(message, "type"))
                    .expect("DIDComm trust-ping transcript message type should parse");
            if let Some(event) = identus_messaging::TrustPingEvent::from_message_type(&message_type)
            {
                let transition = trust_ping
                    .apply(event)
                    .unwrap_or_else(|error| panic!("trust-ping transition failed: {error}"));
                states.insert(transition.to.as_str());
            }
        }

        states
    }

    fn json_string_values(value: &serde_json::Value, field: &str) -> Vec<String> {
        value
            .get(field)
            .and_then(serde_json::Value::as_array)
            .unwrap_or_else(|| panic!("JSON object has no {field} array"))
            .iter()
            .map(|entry| {
                entry
                    .as_str()
                    .unwrap_or_else(|| panic!("{field} entry is not a string"))
                    .to_owned()
            })
            .collect()
    }

    fn json_str<'a>(value: &'a serde_json::Value, field: &str) -> &'a str {
        value
            .get(field)
            .and_then(serde_json::Value::as_str)
            .unwrap_or_else(|| panic!("JSON object missing non-empty string field {field}"))
    }

    fn parse_json(label: &str, text: &str) -> serde_json::Value {
        serde_json::from_str(text)
            .unwrap_or_else(|error| panic!("{label} is invalid JSON: {error}"))
    }

    fn workspace_dependency_graph_from_cargo_metadata()
    -> (BTreeSet<String>, BTreeSet<String>, BTreeSet<String>) {
        let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let output = Command::new("cargo")
            .args(["metadata", "--format-version", "1", "--no-deps"])
            .current_dir(repo_root)
            .output()
            .expect("failed to run cargo metadata");

        assert!(
            output.status.success(),
            "cargo metadata failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );

        let metadata_text =
            String::from_utf8(output.stdout).expect("cargo metadata output is not UTF-8");
        let metadata = parse_json("cargo metadata", &metadata_text);
        let packages = metadata
            .get("packages")
            .and_then(serde_json::Value::as_array)
            .expect("cargo metadata has no packages array");
        let crate_names = packages
            .iter()
            .map(|package| package_name(package).to_owned())
            .collect::<BTreeSet<_>>();

        let mut production_edges = BTreeSet::new();
        let mut dev_edges = BTreeSet::new();
        for package in packages {
            let source = package_name(package);
            let dependencies = package
                .get("dependencies")
                .and_then(serde_json::Value::as_array)
                .expect("cargo metadata package has no dependencies array");

            for dependency in dependencies {
                let target = dependency
                    .get("name")
                    .and_then(serde_json::Value::as_str)
                    .expect("cargo metadata dependency has no name");
                if !crate_names.contains(target) || !dependency_source_is_workspace(dependency) {
                    continue;
                }

                let edge = format!("{source}->{target}");
                match dependency.get("kind") {
                    Some(kind) if kind.is_null() => {
                        production_edges.insert(edge);
                    }
                    Some(kind) if kind.as_str() == Some("dev") => {
                        dev_edges.insert(edge);
                    }
                    _ => {}
                }
            }
        }

        (crate_names, production_edges, dev_edges)
    }

    fn package_name(package: &serde_json::Value) -> &str {
        package
            .get("name")
            .and_then(serde_json::Value::as_str)
            .expect("cargo metadata package has no name")
    }

    fn dependency_source_is_workspace(dependency: &serde_json::Value) -> bool {
        dependency
            .get("source")
            .is_some_and(serde_json::Value::is_null)
    }

    fn workspace_dependency_crates(fixture: &serde_json::Value) -> BTreeSet<String> {
        fixture
            .get("crates")
            .and_then(serde_json::Value::as_array)
            .expect("workspace dependency graph fixture has no crates array")
            .iter()
            .map(|entry| {
                entry
                    .get("crate")
                    .and_then(serde_json::Value::as_str)
                    .expect("workspace dependency graph crate entry has no crate")
                    .to_owned()
            })
            .collect()
    }

    fn json_string_set(fixture: &serde_json::Value, field: &str) -> BTreeSet<String> {
        fixture
            .get(field)
            .and_then(serde_json::Value::as_array)
            .unwrap_or_else(|| panic!("workspace dependency graph fixture has no {field} array"))
            .iter()
            .map(|value| {
                value
                    .as_str()
                    .unwrap_or_else(|| panic!("{field} entry is not a string"))
                    .to_owned()
            })
            .collect()
    }

    fn assert_hex_boundary_constraints(fixture: &serde_json::Value) {
        let production_edges = json_string_set(fixture, "production_edges");
        let core_edges = production_edges
            .iter()
            .filter(|edge| edge.starts_with("identus-core->"))
            .collect::<Vec<_>>();
        assert!(
            core_edges.is_empty(),
            "identus-core has dependencies: {core_edges:?}"
        );

        let forbidden_outer_targets = [
            "identus-adapters",
            "identus-bindings",
            "identus-conformance",
        ];
        for edge in &production_edges {
            let (source, target) = edge
                .split_once("->")
                .unwrap_or_else(|| panic!("invalid dependency edge {edge}"));
            let is_domain_or_protocol = matches!(
                source,
                "identus-core"
                    | "identus-crypto"
                    | "identus-did"
                    | "identus-trust"
                    | "identus-credentials"
                    | "identus-presentations"
                    | "identus-messaging"
                    | "identus-openid4vc"
            );
            assert!(
                !(is_domain_or_protocol && forbidden_outer_targets.contains(&target)),
                "domain/protocol crate has invalid outer-boundary dependency {edge}"
            );
        }

        assert_eq!(
            fixture
                .get("constraints")
                .and_then(|constraints| constraints.get("core_has_no_workspace_dependencies"))
                .and_then(serde_json::Value::as_bool),
            Some(true)
        );
        assert_eq!(
            fixture
                .get("constraints")
                .and_then(|constraints| {
                    constraints.get("production_crates_must_not_depend_on_verification")
                })
                .and_then(serde_json::Value::as_bool),
            Some(true)
        );
    }

    fn assert_workspace_graph_has_no_policy_violations(fixture: &serde_json::Value) {
        for expected_policy_field in [
            "production_dependency_policy",
            "allowed_target_layers_by_source_layer",
            "expected_policy_violations",
            "expected_cycle_violations",
        ] {
            assert!(
                WORKSPACE_DEPENDENCY_GRAPH.contains(expected_policy_field),
                "workspace dependency graph missing policy field {expected_policy_field}"
            );
        }

        assert_json_array_empty(fixture, "production_policy_violations");
        assert_json_array_empty(fixture, "production_cycle_violations");
        assert_eq!(
            fixture
                .get("production_dependency_policy")
                .and_then(|policy| policy.get("expected_policy_violations"))
                .and_then(serde_json::Value::as_u64),
            Some(0)
        );
        assert_eq!(
            fixture
                .get("production_dependency_policy")
                .and_then(|policy| policy.get("expected_cycle_violations"))
                .and_then(serde_json::Value::as_u64),
            Some(0)
        );
    }

    fn assert_json_array_empty(fixture: &serde_json::Value, field: &str) {
        let values = fixture
            .get(field)
            .and_then(serde_json::Value::as_array)
            .unwrap_or_else(|| panic!("workspace dependency graph fixture has no {field} array"));
        assert!(values.is_empty(), "{field} must be empty: {values:?}");
    }

    fn assert_schema_declares_required_field(schema: &serde_json::Value, field: &str) {
        let Some(required) = schema.get("required").and_then(serde_json::Value::as_array) else {
            panic!("schema has no required array");
        };

        assert!(
            required
                .iter()
                .filter_map(serde_json::Value::as_str)
                .any(|required_field| required_field == field),
            "schema required array is missing {field}"
        );
    }

    fn assert_fixture_satisfies_schema(family: &str, schema_text: &str, fixture_text: &str) {
        let schema = parse_json(family, schema_text);
        let fixture = parse_json(family, fixture_text);

        let Some(required) = schema.get("required").and_then(serde_json::Value::as_array) else {
            panic!("{family} schema has no required array");
        };

        for required_field in required.iter().filter_map(serde_json::Value::as_str) {
            assert!(
                fixture.get(required_field).is_some(),
                "{family} fixture is missing required field {required_field}"
            );
        }

        if family == "vector" {
            assert!(
                fixture.get("specification_id").is_some()
                    || fixture.get("specification_ids").is_some(),
                "vector fixture must identify at least one specification"
            );
        }

        assert_non_empty_string_field(family, &fixture, "owner_crate");
        assert_non_empty_string_field(family, &fixture, "redaction_policy");
        assert!(
            fixture
                .get("owner_crate")
                .and_then(serde_json::Value::as_str)
                .is_some_and(|owner| owner.starts_with("identus-")),
            "{family} fixture owner_crate must name an Identus crate"
        );
    }

    fn assert_non_empty_string_field(family: &str, fixture: &serde_json::Value, field: &str) {
        assert!(
            fixture
                .get(field)
                .and_then(serde_json::Value::as_str)
                .is_some_and(|value| !value.is_empty()),
            "{family} fixture field {field} must be a non-empty string"
        );
    }

    fn assert_wrapper_parity_generator_contract() {
        for expected in [
            "generate-wrapper-api-parity",
            "--check",
            "--write",
            "assertSourceExport",
            "source file does not exist",
            "does not contain expected export",
            "wrapper API parity fixture is stale",
            "repos/sdk-ts/packages/lib/sdk/src/index.ts",
            "repos/sdk-swift/EdgeAgentSDK/Domain/Sources/BBs",
            "repos/sdk-kmp/sdk/src/commonMain/kotlin",
        ] {
            assert!(
                WRAPPER_API_PARITY_GENERATOR.contains(expected),
                "wrapper parity generator missing {expected}"
            );
        }
    }

    fn assert_no_legacy_codename(document_name: &str, text: &str) {
        for legacy_codename in [
            "Apollo",
            "Castor",
            "Mercury",
            "Pollux",
            "Pluto",
            "EdgeAgent",
            "edge-agent",
            "edgeagent",
        ] {
            assert!(
                !text.contains(legacy_codename),
                "{document_name} contains legacy SDK codename {legacy_codename}"
            );
        }
    }

    fn assert_case_with_error(cases: &[serde_json::Value], case_id: &str, expected_error: &str) {
        let case = cases
            .iter()
            .find(|case| {
                case.get("case_id")
                    .and_then(serde_json::Value::as_str)
                    .is_some_and(|actual| actual == case_id)
            })
            .unwrap_or_else(|| panic!("missing negative case {case_id}"));

        assert_eq!(
            case.get("expected_error")
                .and_then(serde_json::Value::as_str),
            Some(expected_error),
            "{case_id} has wrong typed error"
        );

        for required_field in ["capability", "input_ref", "format", "policy"] {
            assert!(
                case.get(required_field).is_some(),
                "{case_id} missing {required_field}"
            );
        }
    }

    fn assert_message_flow(messages: &[serde_json::Value], expected_flow: &str) {
        assert!(
            messages.iter().any(|message| {
                message
                    .get("flow")
                    .and_then(serde_json::Value::as_str)
                    .is_some_and(|flow| flow == expected_flow)
            }),
            "OpenID4VC transcript missing flow {expected_flow}"
        );
    }
}
