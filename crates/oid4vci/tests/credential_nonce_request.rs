use identus_core::{ErrorKind, IdentusError};
use identus_oid4vci::{
    CAPABILITY, CredentialIssuerMetadata, CredentialIssuerMetadataLimits, CredentialOfferError,
    NONCE_REQUEST_BODY, NONCE_REQUEST_HTTP_METHOD, error_code,
};

const ISSUER: &str = "https://credential-issuer.example.com/tenant";
const NONCE_ENDPOINT: &str = "https://issuer.example:8443/nonce?tenant=wallet";

fn metadata(nonce_member: &str) -> CredentialIssuerMetadata {
    let json = format!(
        r#"{{"credential_issuer":"{ISSUER}","credential_endpoint":"https://credential-issuer.example.com/credential"{nonce_member},"credential_configurations_supported":{{"degree":{{"format":"dc+sd-jwt"}}}}}}"#,
    );
    CredentialIssuerMetadata::parse(&json, ISSUER, CredentialIssuerMetadataLimits::default())
        .expect("valid issuer metadata")
}

#[test]
fn advertised_endpoint_produces_an_owned_exact_final_request() {
    let metadata = metadata(&format!(r#", "nonce_endpoint":"{NONCE_ENDPOINT}""#));
    let request = metadata.try_nonce_request().expect("Nonce Request");
    drop(metadata);

    assert_eq!(request.nonce_endpoint().as_str(), NONCE_ENDPOINT);
    assert_eq!(request.http_method(), "POST");
    assert_eq!(request.http_method(), NONCE_REQUEST_HTTP_METHOD);
    assert!(request.body().is_empty());
    assert_eq!(request.body(), NONCE_REQUEST_BODY);
    assert!(!request.access_token_required());
}

#[test]
fn omitted_endpoint_fails_statically_without_invalidating_metadata() {
    let metadata = metadata("");
    let error = metadata
        .try_nonce_request()
        .expect_err("missing Nonce Endpoint");

    assert_eq!(error, CredentialOfferError::NonceEndpointRequired);
    assert_eq!(metadata.credential_issuer().as_str(), ISSUER);
    assert!(metadata.nonce_endpoint().is_none());

    let core: IdentusError = error.into();
    assert_eq!(core.code(), error_code::NONCE_ENDPOINT_REQUIRED);
    assert_eq!(core.kind(), ErrorKind::InvalidInput);
    assert_eq!(core.capability(), Some(CAPABILITY));
    assert_eq!(core.public_message(), "OID4VCI Nonce Endpoint is required");
}

#[test]
fn request_and_error_diagnostics_redact_remote_content() {
    let canary = "ENDPOINT_CANARY_8a2e";
    let endpoint = format!("https://issuer.example/{canary}");
    let metadata = metadata(&format!(r#", "nonce_endpoint":"{endpoint}""#));
    let request = metadata.try_nonce_request().expect("Nonce Request");

    for diagnostic in [
        format!("{request:?}"),
        format!("{:?}", request.nonce_endpoint()),
    ] {
        assert!(!diagnostic.contains(canary));
        assert!(!diagnostic.contains(&endpoint));
    }

    let error = CredentialOfferError::NonceEndpointRequired;
    let core: IdentusError = error.into();
    for diagnostic in [
        format!("{error}"),
        format!("{error:?}"),
        format!("{core}"),
        format!("{core:?}"),
    ] {
        assert!(!diagnostic.contains(canary));
        assert!(!diagnostic.contains(&endpoint));
    }
}

#[test]
fn consumer_shaped_adapter_inputs_are_post_empty_and_unprotected() {
    let metadata = metadata(&format!(r#", "nonce_endpoint":"{NONCE_ENDPOINT}""#));
    let request = metadata.try_nonce_request().expect("Nonce Request");
    let adapter_input = (
        request.http_method(),
        request.nonce_endpoint().as_str(),
        request.body(),
        request.access_token_required(),
    );

    assert_eq!(adapter_input.0, "POST");
    assert_eq!(adapter_input.1, NONCE_ENDPOINT);
    assert_eq!(adapter_input.2.len(), 0);
    assert!(!adapter_input.3);
}
