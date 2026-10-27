use identus_openid4vc::{
    FORM_MEDIA_TYPE, REQUEST_OBJECT_MEDIA_TYPE, RequestRetrievalError, RequestRetrievalInput,
    RequestRetrievalLimits, RequestUriMethod, RequestUriReference,
};

#[test]
fn get_retrieval_binds_one_exact_compact_response() {
    let request = RequestUriReference::new(
        "https://rp.example",
        "https://rp.example/request.jwt",
        RequestUriMethod::Get,
        RequestRetrievalLimits::default(),
    )
    .unwrap()
    .prepare(
        RequestRetrievalInput::get(),
        RequestRetrievalLimits::default(),
    )
    .unwrap();
    assert_eq!(request.method(), RequestUriMethod::Get);
    assert_eq!(request.accept(), REQUEST_OBJECT_MEDIA_TYPE);
    assert_eq!(request.content_type(), None);
    assert!(request.expose_sensitive_body().is_empty());
    let retrieved = request
        .bind_response(200, Some(REQUEST_OBJECT_MEDIA_TYPE), b"a.b.c")
        .unwrap();
    assert_eq!(retrieved.expose_sensitive_compact(), "a.b.c");
    assert_eq!(retrieved.expose_sensitive_client_id(), "https://rp.example");
}

#[test]
fn post_retrieval_is_bounded_and_percent_encoded() {
    let request = RequestUriReference::new(
        "client",
        "https://rp.example/request.jwt",
        RequestUriMethod::Post,
        RequestRetrievalLimits::default(),
    )
    .unwrap()
    .prepare(
        RequestRetrievalInput::post(
            Some(r#"{"authorization_endpoint":"https://wallet.example"}"#),
            Some("n 1"),
        ),
        RequestRetrievalLimits::default(),
    )
    .unwrap();
    assert_eq!(request.content_type(), Some(FORM_MEDIA_TYPE));
    let body = std::str::from_utf8(request.expose_sensitive_body()).unwrap();
    assert!(body.contains(
        "wallet_metadata=%7B%22authorization_endpoint%22%3A%22https%3A%2F%2Fwallet.example%22%7D"
    ));
    assert!(body.ends_with("wallet_nonce=n%201"));
}

#[test]
fn unsafe_endpoints_and_ambiguous_responses_fail_closed() {
    for endpoint in [
        "http://rp.example/request",
        "https://user@rp.example/request",
        "https://rp.example/request#fragment",
    ] {
        assert_eq!(
            RequestUriReference::new(
                "client",
                endpoint,
                RequestUriMethod::Get,
                RequestRetrievalLimits::default()
            )
            .err(),
            Some(RequestRetrievalError::UnsafeEndpoint),
        );
    }
    for (body, error) in [
        (&b""[..], RequestRetrievalError::EmptyResponse),
        (&b"a.b"[..], RequestRetrievalError::InvalidCompact),
        (
            &b"a.b.c.d.e"[..],
            RequestRetrievalError::UnsupportedEncryptedObject,
        ),
    ] {
        let request = RequestUriReference::new(
            "client",
            "https://rp.example/request",
            RequestUriMethod::Get,
            RequestRetrievalLimits::default(),
        )
        .unwrap()
        .prepare(
            RequestRetrievalInput::get(),
            RequestRetrievalLimits::default(),
        )
        .unwrap();
        assert_eq!(
            request
                .bind_response(200, Some(REQUEST_OBJECT_MEDIA_TYPE), body)
                .err(),
            Some(error)
        );
    }
}
