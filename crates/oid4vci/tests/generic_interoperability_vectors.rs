use std::{
    collections::BTreeSet,
    fs,
    path::{Component, Path, PathBuf},
};

use identus_crypto::sha256;
use identus_jose::{
    JwsAlgorithm, JwsKeyReference, JwsSigner, Oid4vciProofJwt, Oid4vciProofJwtBuilder,
    Oid4vciProofJwtClaims, Oid4vciProofJwtClient, Oid4vciProofJwtLimits, SignerFailure,
};
use identus_oid4vci::{
    AuthorizationServerMetadataCore, AuthorizationServerMetadataLimits, CredentialIssuerMetadata,
    CredentialIssuerMetadataLimits, CredentialOffer, CredentialOfferError,
    CredentialOfferGrantLimits, CredentialOfferLimits, CredentialOfferRequest,
    CredentialOfferSemanticLimits, CredentialOfferWithMetadata, EmbeddedCredentialOffer,
    ImmediateCredentialHttpResponseLimits, JwtCredentialRequest, JwtCredentialRequestLimits,
    PreAuthorizedTokenHttpResponseLimits, PreAuthorizedTokenRequestLimits,
    PreAuthorizedTokenResponseOutcome, TokenResponseCore, TokenResponseLimits,
    TransactionCodeInputLimits,
};
use serde_json::{Map, Value};

const ISSUER: &str = "https://issuer.interop.example";
const SERVER: &str = "https://authorization.interop.example";
const CONFIGURATION: &str = "UniversityDegreeCredential";
const ACCESS_TOKEN: &str = "SYNTHETIC_ACCESS_TOKEN";
const TRANSACTION_CODE: &str = "123456";
const PLANNING_REVISION: &str = "f8e77570dfbb5c84a1c2434193dadfe5638b5feb";

struct FixedSigner;

impl JwsSigner for FixedSigner {
    fn algorithm(&self) -> JwsAlgorithm {
        JwsAlgorithm::Ed25519
    }

    fn sign(&self, _: &[u8]) -> Result<[u8; 64], SignerFailure> {
        Ok([0x42; 64])
    }
}

fn fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/interop-v1")
}

fn exact_keys(object: &Map<String, Value>, expected: &[&str]) {
    let actual = object.keys().map(String::as_str).collect::<BTreeSet<_>>();
    let expected = expected.iter().copied().collect::<BTreeSet<_>>();
    assert_eq!(actual, expected);
}

fn required_string<'a>(object: &'a Map<String, Value>, name: &str) -> &'a str {
    let value = object
        .get(name)
        .and_then(Value::as_str)
        .unwrap_or_else(|| panic!("manifest field {name} must be a string"));
    assert!(!value.is_empty(), "manifest field {name} must not be empty");
    value
}

fn exact_lower_hex(value: &str, length: usize) -> bool {
    value.len() == length
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
}

fn hex_digest(bytes: &[u8]) -> String {
    sha256(bytes)
        .as_bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn collect_regular_files(root: &Path, current: &Path, files: &mut BTreeSet<String>) {
    for entry in fs::read_dir(current).expect("fixture directory must be readable") {
        let entry = entry.expect("fixture entry must be readable");
        let path = entry.path();
        let metadata = fs::symlink_metadata(&path).expect("fixture metadata must be readable");
        assert!(
            !metadata.file_type().is_symlink(),
            "fixture symlink: {path:?}"
        );
        if metadata.is_dir() {
            collect_regular_files(root, &path, files);
        } else {
            assert!(
                metadata.is_file(),
                "fixture must be a regular file: {path:?}"
            );
            let relative = path
                .strip_prefix(root)
                .expect("fixture path must remain below root")
                .to_str()
                .expect("fixture path must be UTF-8")
                .replace('\\', "/");
            if relative != "manifest.json" {
                assert!(files.insert(relative), "duplicate fixture path");
            }
        }
    }
}

fn validate_manifest() -> Value {
    let root = fixture_root();
    let manifest_path = root.join("manifest.json");
    let metadata = fs::symlink_metadata(&manifest_path).expect("manifest metadata");
    assert!(metadata.is_file());
    assert!(!metadata.file_type().is_symlink());
    let manifest: Value =
        serde_json::from_slice(&fs::read(&manifest_path).expect("manifest bytes"))
            .expect("valid manifest JSON");
    let object = manifest.as_object().expect("manifest object");
    exact_keys(
        object,
        &[
            "schemaVersion",
            "suite",
            "authorship",
            "normativeSource",
            "consumerReferences",
            "vectors",
        ],
    );
    assert_eq!(object["schemaVersion"], 1);
    assert_eq!(object["suite"], "identus-oid4vci-generic-interop-v1");

    let authorship = object["authorship"].as_object().expect("authorship object");
    exact_keys(
        authorship,
        &[
            "kind",
            "repository",
            "planningRevision",
            "license",
            "copiedConsumerBytes",
        ],
    );
    assert_eq!(authorship["kind"], "repository-authored");
    assert_eq!(
        authorship["repository"],
        "https://github.com/hyperledger-identus/sdk-rust"
    );
    assert_eq!(authorship["planningRevision"], PLANNING_REVISION);
    assert_eq!(authorship["license"], "Apache-2.0");
    assert_eq!(authorship["copiedConsumerBytes"], false);

    let source = object["normativeSource"]
        .as_object()
        .expect("normative source object");
    exact_keys(source, &["uri", "version", "retrievedAt"]);
    assert_eq!(source["version"], "OpenID4VCI 1.0 Final");
    assert_eq!(source["retrievedAt"], "2026-09-25");
    assert_eq!(
        source["uri"],
        "https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0-final.html"
    );

    let references = object["consumerReferences"]
        .as_array()
        .expect("consumer reference array");
    assert_eq!(references.len(), 2);
    for reference in references {
        let reference = reference.as_object().expect("consumer reference object");
        exact_keys(
            reference,
            &[
                "repository",
                "revision",
                "relatedRevisions",
                "paths",
                "licenseDisposition",
                "classification",
                "importedBytes",
                "role",
            ],
        );
        assert_eq!(reference["classification"], "reference-only");
        assert_eq!(reference["importedBytes"], false);
        assert!(exact_lower_hex(required_string(reference, "revision"), 40));
        assert!(!required_string(reference, "repository").is_empty());
        assert!(!required_string(reference, "licenseDisposition").is_empty());
        assert!(!required_string(reference, "role").is_empty());
        let paths = reference["paths"].as_array().expect("reference paths");
        assert!(!paths.is_empty());
        assert!(
            paths
                .iter()
                .all(|path| path.as_str().is_some_and(|path| !path.is_empty()))
        );
        for revision in reference["relatedRevisions"]
            .as_array()
            .expect("related revisions")
        {
            assert!(
                revision
                    .as_str()
                    .is_some_and(|revision| exact_lower_hex(revision, 40))
            );
        }
    }

    let vectors = object["vectors"].as_array().expect("vector array");
    assert_eq!(vectors.len(), 9);
    let expected_ids = [
        "positive-credential-offer",
        "positive-credential-issuer-metadata",
        "positive-authorization-server-metadata",
        "positive-token-request",
        "positive-token-response",
        "positive-credential-response",
        "negative-extra-offer-query",
        "negative-null-transaction-code",
        "negative-singular-credential-response",
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();
    let mut ids = BTreeSet::new();
    let mut paths = BTreeSet::new();
    for vector in vectors {
        let vector = vector.as_object().expect("vector object");
        exact_keys(
            vector,
            &[
                "id",
                "path",
                "sha256",
                "section",
                "transformation",
                "expectedResult",
                "publicApi",
            ],
        );
        let id = required_string(vector, "id");
        assert!(ids.insert(id), "duplicate vector ID: {id}");
        let relative = required_string(vector, "path");
        assert!(paths.insert(relative.to_owned()), "duplicate vector path");
        let relative_path = Path::new(relative);
        assert!(!relative_path.is_absolute());
        assert!(
            relative_path
                .components()
                .all(|component| matches!(component, Component::Normal(_)))
        );
        let path = root.join(relative_path);
        let metadata = fs::symlink_metadata(&path).expect("vector metadata");
        assert!(metadata.is_file());
        assert!(!metadata.file_type().is_symlink());
        let bytes = fs::read(path).expect("vector bytes");
        let expected_digest = required_string(vector, "sha256");
        assert!(exact_lower_hex(expected_digest, 64));
        assert_eq!(hex_digest(&bytes), expected_digest, "vector drift: {id}");
        assert!(required_string(vector, "transformation").contains("independently authored"));
        required_string(vector, "section");
        required_string(vector, "expectedResult");
        required_string(vector, "publicApi");
    }
    assert_eq!(ids, expected_ids);
    let mut actual_files = BTreeSet::new();
    collect_regular_files(&root, &root, &mut actual_files);
    assert_eq!(paths, actual_files);
    manifest
}

fn fixture_bytes(id: &str) -> Vec<u8> {
    let manifest = validate_manifest();
    let vector = manifest["vectors"]
        .as_array()
        .expect("vectors")
        .iter()
        .find(|vector| vector["id"] == id)
        .unwrap_or_else(|| panic!("missing vector: {id}"));
    fs::read(fixture_root().join(vector["path"].as_str().expect("vector path")))
        .expect("fixture bytes")
}

fn fixture_text(id: &str) -> String {
    String::from_utf8(fixture_bytes(id)).expect("fixture must be UTF-8")
}

fn encode_form(value: &str) -> String {
    let mut encoded = String::new();
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~') {
            encoded.push(char::from(byte));
        } else if byte == b' ' {
            encoded.push('+');
        } else {
            encoded.push_str(&format!("%{byte:02X}"));
        }
    }
    encoded
}

fn offer_invocation(offer_json: &str) -> String {
    format!(
        "openid-credential-offer://?credential_offer={}",
        encode_form(offer_json.trim_end())
    )
}

fn parsed_offer(id: &str) -> CredentialOffer {
    let json = fixture_text(id);
    CredentialOffer::try_from_embedded(
        EmbeddedCredentialOffer::try_from_json(json.trim_end(), CredentialOfferLimits::default())
            .expect("embedded offer"),
        CredentialOfferSemanticLimits::default(),
    )
    .expect("offer semantics")
}

fn matched_state() -> CredentialOfferWithMetadata {
    let offer = parsed_offer("positive-credential-offer")
        .try_into_grants(CredentialOfferGrantLimits::default())
        .expect("offer grants");
    let metadata = CredentialIssuerMetadata::parse(
        fixture_text("positive-credential-issuer-metadata").trim_end(),
        ISSUER,
        CredentialIssuerMetadataLimits::default(),
    )
    .expect("issuer metadata");
    offer.try_with_metadata(metadata).expect("matched metadata")
}

fn proof() -> Oid4vciProofJwt {
    let limits = Oid4vciProofJwtLimits::default();
    let claims = Oid4vciProofJwtClaims::new(
        Oid4vciProofJwtClient::AnonymousPreAuthorized,
        ISSUER,
        1_700_000_000,
        Some("SYNTHETIC_NONCE".to_owned()),
        limits,
    )
    .expect("proof claims");
    Oid4vciProofJwtBuilder::new(limits)
        .prepare(
            JwsAlgorithm::Ed25519,
            JwsKeyReference::key_id("did:example:interop-holder#key-1", limits.jws())
                .expect("proof key reference"),
            claims,
        )
        .expect("proof input")
        .sign_with(&FixedSigner)
        .expect("synthetic proof")
}

fn credential_request() -> JwtCredentialRequest {
    let token = TokenResponseCore::parse(
        fixture_text("positive-token-response").trim_end(),
        TokenResponseLimits::default(),
    )
    .expect("Token Response");
    matched_state()
        .try_create_jwt_credential_request(
            &token,
            0,
            &[proof()],
            JwtCredentialRequestLimits::default(),
        )
        .expect("Credential Request")
}

#[test]
fn manifest_is_closed_provenance_and_drift_evidence() {
    validate_manifest();
}

#[test]
fn positive_vectors_execute_a_coherent_public_wallet_flow() {
    let offer_json = fixture_text("positive-credential-offer");
    let invocation = offer_invocation(&offer_json);
    let CredentialOfferRequest::Embedded(embedded) =
        CredentialOfferRequest::parse(&invocation, CredentialOfferLimits::default())
            .expect("offer transport")
    else {
        panic!("expected embedded offer")
    };
    let offer =
        CredentialOffer::try_from_embedded(embedded, CredentialOfferSemanticLimits::default())
            .expect("offer semantics")
            .try_into_grants(CredentialOfferGrantLimits::default())
            .expect("offer grants");
    let metadata = CredentialIssuerMetadata::parse(
        fixture_text("positive-credential-issuer-metadata").trim_end(),
        ISSUER,
        CredentialIssuerMetadataLimits::default(),
    )
    .expect("issuer metadata");
    let matched = offer.try_with_metadata(metadata).expect("matched metadata");
    let server = AuthorizationServerMetadataCore::parse(
        fixture_text("positive-authorization-server-metadata").trim_end(),
        SERVER,
        AuthorizationServerMetadataLimits::default(),
    )
    .expect("authorization-server metadata");
    let request = matched
        .try_with_pre_authorized_server(server)
        .expect("server binding")
        .try_with_transaction_code_input(
            Some(TRANSACTION_CODE.to_owned()),
            TransactionCodeInputLimits::default(),
        )
        .expect("Transaction Code input")
        .try_into_pre_authorized_token_request(PreAuthorizedTokenRequestLimits::default())
        .expect("Token Request");
    assert_eq!(
        request.expose_sensitive_form_body(),
        fixture_text("positive-token-request").trim_end()
    );
    let outcome = request
        .try_bind_response(
            200,
            "application/json",
            "no-store",
            "no-cache",
            fixture_text("positive-token-response").trim_end(),
            PreAuthorizedTokenHttpResponseLimits::default(),
        )
        .expect("request-bound Token Response");
    let PreAuthorizedTokenResponseOutcome::Success(bound) = outcome else {
        panic!("expected successful Token Response")
    };
    assert_eq!(
        bound.response().expose_sensitive_access_token(),
        ACCESS_TOKEN
    );
    assert_eq!(
        bound
            .lineage()
            .credential_issuer_metadata()
            .credential_issuer()
            .as_str(),
        ISSUER
    );
    assert_eq!(
        bound
            .lineage()
            .authorization_server_metadata()
            .issuer()
            .as_str(),
        SERVER
    );
    assert_eq!(
        bound.lineage().offered_credential_configurations()[0].as_str(),
        CONFIGURATION
    );
    assert!(bound.lineage().transaction_code_present());

    let request = matched_state()
        .try_create_jwt_credential_request(
            bound.response(),
            0,
            &[proof()],
            JwtCredentialRequestLimits::default(),
        )
        .expect("Credential Request");
    let response = request
        .validate_immediate_response(
            200,
            "application/json",
            fixture_text("positive-credential-response").trim_end(),
            ImmediateCredentialHttpResponseLimits::default(),
        )
        .expect("immediate Credential Response");
    assert_eq!(response.request_proof_count(), 1);
    assert_eq!(response.response().credentials().len(), 1);
    assert_eq!(
        response.response().credentials()[0].expose_sensitive_string(),
        Some("SYNTHETIC_CREDENTIAL")
    );
}

#[test]
fn negative_vectors_reject_legacy_wire_classes_at_public_boundaries() {
    let offer_json = fixture_text("positive-credential-offer");
    let invocation = format!(
        "{}&{}",
        offer_invocation(&offer_json),
        fixture_text("negative-extra-offer-query").trim_end()
    );
    assert_eq!(
        CredentialOfferRequest::parse(&invocation, CredentialOfferLimits::default())
            .expect_err("extra offer query must fail"),
        CredentialOfferError::UnsupportedTransport
    );

    assert_eq!(
        parsed_offer("negative-null-transaction-code")
            .try_into_grants(CredentialOfferGrantLimits::default())
            .expect_err("null Transaction Code must fail"),
        CredentialOfferError::InvalidTransactionCode
    );

    assert_eq!(
        credential_request()
            .validate_immediate_response(
                200,
                "application/json",
                fixture_text("negative-singular-credential-response").trim_end(),
                ImmediateCredentialHttpResponseLimits::default(),
            )
            .expect_err("singular Credential Response must fail"),
        CredentialOfferError::InvalidImmediateCredentialResponse
    );
}
