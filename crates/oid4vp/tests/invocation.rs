use identus_oid4vp::{
    AuthorizationRequestInvocation, AuthorizationRequestInvocationLimits, Oid4vpError,
    RequestUriMethod,
};

const REFERENCE: &str = "openid4vp:?client_id=https%3A%2F%2Fverifier.example%2Fclient&request_uri=https%3A%2F%2Fverifier.example%2Frequest.jwt%3Fx%3D1";

fn referenced(input: &str) -> identus_oid4vp::ReferencedAuthorizationRequest {
    let AuthorizationRequestInvocation::Referenced(request) =
        AuthorizationRequestInvocation::parse(
            input,
            AuthorizationRequestInvocationLimits::default(),
        )
        .expect("valid reference invocation");
    request
}

#[test]
fn final_reference_defaults_to_get_and_retains_only_bounded_values() {
    let request = referenced(REFERENCE);
    assert_eq!(
        request.expose_sensitive_client_id(),
        "https://verifier.example/client"
    );
    assert_eq!(
        request.expose_sensitive_request_uri(),
        "https://verifier.example/request.jwt?x=1"
    );
    assert_eq!(request.request_uri_method(), RequestUriMethod::Get);
    assert_eq!(request.client_id_len(), 31);
    assert_eq!(request.request_uri_len(), 40);
}

#[test]
fn explicit_post_and_bounded_unknown_extensions_are_supported() {
    let input = format!("{REFERENCE}&request_uri_method=post&future=value");
    let request = referenced(&input);
    assert_eq!(request.request_uri_method(), RequestUriMethod::Post);
}

#[test]
fn decoded_duplicate_names_fail_closed() {
    let input = format!("{REFERENCE}&client%5fid=shadow");
    assert_eq!(
        AuthorizationRequestInvocation::parse(
            &input,
            AuthorizationRequestInvocationLimits::default()
        )
        .unwrap_err(),
        Oid4vpError::DuplicateParameter
    );
}

#[test]
fn malformed_or_ambiguous_invocations_are_rejected() {
    let cases = [
        (
            "openid4vp:?client_id=x&request_uri=%",
            Oid4vpError::InvalidFormEncoding,
        ),
        (
            "openid4vp:?client_id=x&request_uri=https%3A%2F%2Fv.example&client_id=y",
            Oid4vpError::DuplicateParameter,
        ),
        (
            "openid4vp:?client_id=x&request=jwt",
            Oid4vpError::UnsupportedTransport,
        ),
        (
            "openid4vp:?client_id=x&dcql_query=%7B%7D",
            Oid4vpError::UnsupportedTransport,
        ),
        (
            "openid4vp:?client_id=x&request_uri=https%3A%2F%2Fv.example&transaction_data=%5B%5D",
            Oid4vpError::UnsupportedParameter,
        ),
        (
            "openid4vp:?client_id=x&request_uri=https%3A%2F%2Fv.example&request_uri_method=get",
            Oid4vpError::UnsupportedRequestUriMethod,
        ),
        (
            "openid4vp:?client_id=x",
            Oid4vpError::MissingRequiredParameter,
        ),
        (
            "openid4vp:?request_uri=https%3A%2F%2Fv.example",
            Oid4vpError::MissingRequiredParameter,
        ),
        (
            "openid4vp:?client_id=&request_uri=https%3A%2F%2Fv.example",
            Oid4vpError::MissingRequiredParameter,
        ),
        (
            "openid4vp:?client_id=x&request_uri=http%3A%2F%2Fv.example",
            Oid4vpError::UnsafeRequestUri,
        ),
        (
            "openid4vp:?client_id=x&request_uri=https%3A%2F%2Fuser%40v.example",
            Oid4vpError::UnsafeRequestUri,
        ),
        (
            "openid4vp:?client_id=x&request_uri=https%3A%2F%2Fv.example%2Fr%23fragment",
            Oid4vpError::UnsafeRequestUri,
        ),
        (
            "openid4vp://authorize?client_id=x&request_uri=https%3A%2F%2Fv.example",
            Oid4vpError::InvalidInvocation,
        ),
        (
            "openid4vp:/path?client_id=x&request_uri=https%3A%2F%2Fv.example",
            Oid4vpError::InvalidInvocation,
        ),
        (
            "other:?client_id=x&request_uri=https%3A%2F%2Fv.example",
            Oid4vpError::InvalidInvocation,
        ),
    ];

    for (input, expected) in cases {
        assert_eq!(
            AuthorizationRequestInvocation::parse(
                input,
                AuthorizationRequestInvocationLimits::default()
            )
            .unwrap_err(),
            expected,
            "input: {input}"
        );
    }
}

#[test]
fn every_configurable_resource_boundary_is_enforced() {
    let limits = AuthorizationRequestInvocationLimits::new(512, 2, 11, 25, 4, 25).unwrap();
    let exact = "openid4vp:?client_id=abcd&request_uri=https%3A%2F%2Fv.example%2Fr";
    assert!(AuthorizationRequestInvocation::parse(exact, limits).is_ok());

    let cases = [
        (
            exact,
            AuthorizationRequestInvocationLimits::new(exact.len() - 1, 2, 11, 25, 4, 25).unwrap(),
            Oid4vpError::InvocationTooLarge,
        ),
        (
            exact,
            AuthorizationRequestInvocationLimits::new(512, 1, 11, 25, 4, 25).unwrap(),
            Oid4vpError::TooManyParameters,
        ),
        (
            exact,
            AuthorizationRequestInvocationLimits::new(512, 2, 10, 25, 4, 25).unwrap(),
            Oid4vpError::ParameterNameTooLarge,
        ),
        (
            exact,
            AuthorizationRequestInvocationLimits::new(512, 2, 11, 18, 4, 25).unwrap(),
            Oid4vpError::ParameterValueTooLarge,
        ),
        (
            exact,
            AuthorizationRequestInvocationLimits::new(512, 2, 11, 25, 3, 25).unwrap(),
            Oid4vpError::ClientIdTooLarge,
        ),
        (
            exact,
            AuthorizationRequestInvocationLimits::new(512, 2, 11, 25, 4, 18).unwrap(),
            Oid4vpError::RequestUriTooLarge,
        ),
    ];
    for (input, limits, expected) in cases {
        assert_eq!(
            AuthorizationRequestInvocation::parse(input, limits).unwrap_err(),
            expected
        );
    }
}

#[test]
fn zero_limits_are_invalid() {
    assert_eq!(
        AuthorizationRequestInvocationLimits::new(0, 1, 1, 1, 1, 1),
        Err(Oid4vpError::InvalidLimits)
    );
}

#[test]
fn successful_and_failed_diagnostics_do_not_leak_verifier_values() {
    const CANARY: &str = "VERIFIER_SECRET_CANARY";
    let input =
        format!("openid4vp:?client_id={CANARY}&request_uri=https%3A%2F%2Fv.example%2F{CANARY}");
    let parsed = AuthorizationRequestInvocation::parse(
        &input,
        AuthorizationRequestInvocationLimits::default(),
    )
    .unwrap();
    assert!(!format!("{parsed:?}").contains(CANARY));

    let malformed = format!("openid4vp:?client_id={CANARY}&request_uri=%");
    let error = AuthorizationRequestInvocation::parse(
        &malformed,
        AuthorizationRequestInvocationLimits::default(),
    )
    .unwrap_err();
    assert!(!format!("{error}").contains(CANARY));
    assert!(!format!("{error:?}").contains(CANARY));
    assert!(!format!("{}", error.to_identus_error()).contains(CANARY));
}
