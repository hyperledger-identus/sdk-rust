use identus_core::{ErrorKind, IdentusError};
use identus_oid4vci::{
    CAPABILITY, CredentialIssuerMetadata, CredentialIssuerMetadataLimits,
    CredentialNonceHttpResponseLimits, CredentialNonceRequest, CredentialNonceResponseCore,
    CredentialNonceResponseLimits, CredentialOfferError, error_code,
};

const ISSUER: &str = "https://credential-issuer.example.com";
const ENDPOINT: &str = "https://credential-issuer.example.com/nonce";
const BODY: &str = r#"{"c_nonce":"fresh-opaque"}"#;

fn request() -> CredentialNonceRequest {
    let json = format!(
        r#"{{"credential_issuer":"{ISSUER}","credential_endpoint":"{ISSUER}/credential","nonce_endpoint":"{ENDPOINT}","credential_configurations_supported":{{"degree":{{"format":"dc+sd-jwt"}}}}}}"#,
    );
    CredentialIssuerMetadata::parse(&json, ISSUER, CredentialIssuerMetadataLimits::default())
        .expect("metadata")
        .try_nonce_request()
        .expect("nonce request")
}

fn validate(
    request: &CredentialNonceRequest,
    status: u16,
    content_type: &str,
    cache_control: &str,
    body: &str,
) -> Result<CredentialNonceResponseCore, CredentialOfferError> {
    request.validate_response(
        status,
        content_type,
        cache_control,
        body,
        CredentialNonceHttpResponseLimits::default(),
    )
}

#[test]
fn defaults_and_positive_limits_are_explicit() {
    let defaults = CredentialNonceHttpResponseLimits::default();
    assert_eq!(
        defaults.response_limits(),
        CredentialNonceResponseLimits::default()
    );
    assert_eq!(defaults.max_content_type_bytes(), 1_024);
    assert_eq!(defaults.max_cache_control_bytes(), 4_096);

    for result in [
        CredentialNonceHttpResponseLimits::new(CredentialNonceResponseLimits::default(), 0, 1),
        CredentialNonceHttpResponseLimits::new(CredentialNonceResponseLimits::default(), 1, 0),
    ] {
        assert_eq!(
            result,
            Err(CredentialOfferError::InvalidCredentialNonceHttpResponseLimits)
        );
    }
}

#[test]
fn success_status_boundaries_and_rfc_shaped_fields_reach_the_nonce() {
    let request = request();
    for status in [200, 204, 299] {
        let response = validate(
            &request,
            status,
            " Application/JSON ; charset=utf-8; profile=\"wallet\\\"profile\"\t",
            ", max-age=0, private=\"field,a; no-store\", No-StOrE, ext=token,,",
            BODY,
        )
        .expect("valid Final response envelope");
        assert_eq!(response.response_len(), BODY.len());
        assert_eq!(response.nonce().expose_sensitive_nonce(), "fresh-opaque");
    }
}

#[test]
fn non_success_and_non_http_statuses_fail_before_body_parsing() {
    let request = request();
    for status in [0, 99, 199, 300, 599, 600, u16::MAX] {
        assert_eq!(
            validate(
                &request,
                status,
                "application/json",
                "no-store",
                "BODY_CANARY_not-json"
            )
            .expect_err("invalid status"),
            CredentialOfferError::InvalidCredentialNonceHttpStatus
        );
    }

    validate(&request, 200, "application/json", "no-store", BODY)
        .expect("request remains reusable after failure");
}

#[test]
fn json_media_type_casing_and_valid_parameters_interoperate() {
    let request = request();
    for content_type in [
        "application/json",
        "APPLICATION/JSON",
        "\tapplication/json \t",
        "application/json;charset=utf-8",
        "application/json ; charset=\"utf-8\" ;profile=wallet",
        "application/json;; ; charset=utf-8;",
        "application/json;x=\"a,b;c\\\"d\"",
        "application/json;x=\"opaque-€\"",
    ] {
        validate(&request, 200, content_type, "no-store", BODY)
            .unwrap_or_else(|error| panic!("rejected {content_type:?}: {error:?}"));
    }
}

#[test]
fn missing_different_ambiguous_and_malformed_media_types_fail_closed() {
    let request = request();
    let invalid = [
        "",
        " \t ",
        "text/json",
        "application/problem+json",
        "application/json, text/plain",
        "application//json",
        "application/json/extra",
        "application/json;=value",
        "application/json;charset =utf-8",
        "application/json;charset= utf-8",
        "application/json;charset=utf-8;CHARSET=iso-8859-1",
        "application/json;charset=\"unterminated",
        "application/json;charset=\"bad\\\nvalue\"",
        "application/json\r\nX-Injected: value",
    ];
    for content_type in invalid {
        assert_eq!(
            validate(&request, 200, content_type, "no-store", "BODY_CANARY")
                .expect_err("invalid media type"),
            CredentialOfferError::InvalidCredentialNonceContentType,
            "accepted {content_type:?}"
        );
    }
}

#[test]
fn cache_control_requires_a_parsed_bare_no_store_directive() {
    let request = request();
    let valid = [
        "no-store",
        "NO-STORE",
        "max-age=0,no-store",
        "\t private=\"field,a\" , No-StOrE , ext=token \t",
        ",,no-store,,",
        "ext=\"comma,semicolon;quoted no-store\", no-store",
    ];
    for cache_control in valid {
        validate(&request, 200, "application/json", cache_control, BODY)
            .unwrap_or_else(|error| panic!("rejected {cache_control:?}: {error:?}"));
    }

    let invalid = [
        "",
        " , , ",
        "max-age=0",
        "x-no-store",
        "ext=no-store",
        "ext=\"no-store, private\"",
        "no-store=value",
        "no-store=\"value\"",
        "no-store; private",
        "no-store, invalid@name",
        "no-store, ext =value",
        "no-store, ext=\"unterminated",
        "no-store, ext=\"bad\\\nvalue\"",
        "no-store\r\nX-Injected: value",
    ];
    for cache_control in invalid {
        assert_eq!(
            validate(
                &request,
                200,
                "application/json",
                cache_control,
                "BODY_CANARY"
            )
            .expect_err("invalid cache control"),
            CredentialOfferError::InvalidCredentialNonceCacheControl,
            "accepted {cache_control:?}"
        );
    }
}

#[test]
fn exact_header_bounds_pass_and_oversize_values_are_specific() {
    let request = request();
    let exact = CredentialNonceHttpResponseLimits::new(
        CredentialNonceResponseLimits::default(),
        "application/json".len(),
        "no-store".len(),
    )
    .expect("exact field limits");
    request
        .validate_response(200, "application/json", "no-store", BODY, exact)
        .expect("exact bounds");

    let short_content_type = CredentialNonceHttpResponseLimits::new(
        CredentialNonceResponseLimits::default(),
        "application/json".len() - 1,
        64,
    )
    .expect("small content type bound");
    assert_eq!(
        request
            .validate_response(
                200,
                "application/json",
                "no-store",
                BODY,
                short_content_type,
            )
            .expect_err("oversize content type"),
        CredentialOfferError::CredentialNonceContentTypeTooLarge
    );

    let short_cache_control = CredentialNonceHttpResponseLimits::new(
        CredentialNonceResponseLimits::default(),
        64,
        "no-store".len() - 1,
    )
    .expect("small cache bound");
    assert_eq!(
        request
            .validate_response(
                200,
                "application/json",
                "no-store",
                BODY,
                short_cache_control,
            )
            .expect_err("oversize cache control"),
        CredentialOfferError::CredentialNonceCacheControlTooLarge
    );
}

#[test]
fn existing_body_limits_and_errors_are_preserved_after_transport_validation() {
    let request = request();
    assert_eq!(
        validate(&request, 200, "application/json", "no-store", "not-json")
            .expect_err("malformed body"),
        CredentialOfferError::InvalidCredentialNonceResponse
    );

    let body_limits =
        CredentialNonceResponseLimits::new(1, 1, 1, 1).expect("positive constrained body limits");
    let limits =
        CredentialNonceHttpResponseLimits::new(body_limits, 64, 64).expect("positive HTTP limits");
    assert_eq!(
        request
            .validate_response(200, "application/json", "no-store", BODY, limits)
            .expect_err("oversize body"),
        CredentialOfferError::CredentialNonceResponseTooLarge
    );
}

#[test]
fn transport_errors_and_debug_are_static_and_redacted() {
    let request = request();
    let canaries = [
        "STATUS_CANARY_87c1",
        "CONTENT_TYPE_CANARY_24ab",
        "CACHE_CANARY_59de",
        "BODY_CANARY_4d30",
        ENDPOINT,
    ];
    let cases = [
        (
            CredentialOfferError::InvalidCredentialNonceHttpResponseLimits,
            error_code::INVALID_CREDENTIAL_NONCE_HTTP_RESPONSE_LIMITS,
        ),
        (
            CredentialOfferError::InvalidCredentialNonceHttpStatus,
            error_code::INVALID_CREDENTIAL_NONCE_HTTP_STATUS,
        ),
        (
            CredentialOfferError::CredentialNonceContentTypeTooLarge,
            error_code::CREDENTIAL_NONCE_CONTENT_TYPE_TOO_LARGE,
        ),
        (
            CredentialOfferError::InvalidCredentialNonceContentType,
            error_code::INVALID_CREDENTIAL_NONCE_CONTENT_TYPE,
        ),
        (
            CredentialOfferError::CredentialNonceCacheControlTooLarge,
            error_code::CREDENTIAL_NONCE_CACHE_CONTROL_TOO_LARGE,
        ),
        (
            CredentialOfferError::InvalidCredentialNonceCacheControl,
            error_code::INVALID_CREDENTIAL_NONCE_CACHE_CONTROL,
        ),
    ];

    for (error, code) in cases {
        let core: IdentusError = error.into();
        assert_eq!(core.code(), code);
        assert_eq!(core.kind(), ErrorKind::InvalidInput);
        assert_eq!(core.capability(), Some(CAPABILITY));
        for value in [
            format!("{error}"),
            format!("{error:?}"),
            format!("{core}"),
            format!("{core:?}"),
            format!("{request:?}"),
        ] {
            for canary in canaries {
                assert!(!value.contains(canary));
            }
        }
    }

    assert_eq!(
        validate(
            &request,
            500,
            "CONTENT_TYPE_CANARY_24ab",
            "CACHE_CANARY_59de",
            "BODY_CANARY_4d30",
        )
        .expect_err("status precedes remote values"),
        CredentialOfferError::InvalidCredentialNonceHttpStatus
    );
}

#[test]
fn consumer_shaped_adapter_supplies_only_effective_transport_inputs() {
    struct AdapterResponse<'a> {
        status: u16,
        content_type: &'a str,
        cache_control: &'a str,
        decoded_body: &'a str,
    }

    fn accept(
        request: &CredentialNonceRequest,
        response: AdapterResponse<'_>,
    ) -> Result<CredentialNonceResponseCore, CredentialOfferError> {
        request.validate_response(
            response.status,
            response.content_type,
            response.cache_control,
            response.decoded_body,
            CredentialNonceHttpResponseLimits::default(),
        )
    }

    let response = accept(
        &request(),
        AdapterResponse {
            status: 200,
            content_type: "application/json; charset=utf-8",
            cache_control: "private, no-store",
            decoded_body: BODY,
        },
    )
    .expect("consumer-shaped response");
    assert_eq!(response.nonce().expose_sensitive_nonce(), "fresh-opaque");
}
