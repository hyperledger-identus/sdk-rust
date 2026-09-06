use identus_core::{ErrorKind, IdentusError};
use identus_oid4vci::{
    AuthorizationServerMetadataCore, AuthorizationServerMetadataLimits, CAPABILITY,
    CredentialIssuerMetadata, CredentialIssuerMetadataLimits, CredentialOffer,
    CredentialOfferError, CredentialOfferGrantLimits, CredentialOfferLimits,
    CredentialOfferSemanticLimits, EmbeddedCredentialOffer, PRE_AUTHORIZED_CODE_GRANT_TYPE,
    TransactionCodeInputLimits, error_code,
};

const ISSUER: &str = "https://credential-issuer.example";

fn bound_offer(
    transaction_code: Option<&str>,
    pre_authorized_code: &str,
    endpoint_suffix: &str,
) -> identus_oid4vci::CredentialOfferWithPreAuthorizedServer {
    let transaction_code =
        transaction_code.map_or_else(String::new, |value| format!(r#", "tx_code":{value}"#));
    let offer_json = format!(
        r#"{{"credential_issuer":"{ISSUER}","credential_configuration_ids":["degree"],"grants":{{"{PRE_AUTHORIZED_CODE_GRANT_TYPE}":{{"pre-authorized_code":"{pre_authorized_code}"{transaction_code}}}}}}}"#,
    );
    let offer = CredentialOffer::try_from_embedded(
        EmbeddedCredentialOffer::try_from_json(&offer_json, CredentialOfferLimits::default())
            .expect("offer transport"),
        CredentialOfferSemanticLimits::default(),
    )
    .expect("offer semantics")
    .try_into_grants(CredentialOfferGrantLimits::default())
    .expect("offer grants");
    let issuer_metadata = CredentialIssuerMetadata::parse(
        &format!(
            r#"{{"credential_issuer":"{ISSUER}","credential_endpoint":"{ISSUER}/credential","credential_configurations_supported":{{"degree":{{"format":"dc+sd-jwt"}}}}}}"#,
        ),
        ISSUER,
        CredentialIssuerMetadataLimits::default(),
    )
    .expect("issuer metadata");
    let matched = offer
        .try_with_metadata(issuer_metadata)
        .expect("matched metadata");
    let authorization_metadata = AuthorizationServerMetadataCore::parse(
        &format!(
            r#"{{"issuer":"{ISSUER}","token_endpoint":"{ISSUER}/{endpoint_suffix}","grant_types_supported":["{PRE_AUTHORIZED_CODE_GRANT_TYPE}"]}}"#,
        ),
        ISSUER,
        AuthorizationServerMetadataLimits::default(),
    )
    .expect("Authorization Server Metadata core");
    matched
        .try_with_pre_authorized_server(authorization_metadata)
        .expect("bound server")
}

#[test]
fn defaults_and_positive_limits_are_explicit() {
    let defaults = TransactionCodeInputLimits::default();
    assert_eq!(defaults.max_transaction_code_bytes(), 256);
    assert_eq!(
        TransactionCodeInputLimits::new(0),
        Err(CredentialOfferError::InvalidTransactionCodeInputLimits)
    );
    assert_eq!(
        TransactionCodeInputLimits::new(17)
            .expect("positive limit")
            .max_transaction_code_bytes(),
        17
    );
}

#[test]
fn required_input_advances_and_preserves_the_owned_predecessor() {
    let prepared = bound_offer(Some("{}"), "secret-code", "token")
        .try_with_transaction_code_input(
            Some("493536".to_owned()),
            TransactionCodeInputLimits::default(),
        )
        .expect("required input");

    assert!(prepared.transaction_code_present());
    assert_eq!(
        prepared
            .credential_offer_with_pre_authorized_server()
            .authorization_server_metadata()
            .token_endpoint()
            .expect("Token Endpoint")
            .as_str(),
        "https://credential-issuer.example/token"
    );
}

#[test]
fn absent_requirement_accepts_only_absent_input() {
    let prepared = bound_offer(None, "secret-code", "token")
        .try_with_transaction_code_input(None, TransactionCodeInputLimits::default())
        .expect("no input requested");

    assert!(!prepared.transaction_code_present());
}

#[test]
fn presence_disagreement_fails_closed() {
    assert!(matches!(
        bound_offer(Some("{}"), "secret-code", "token")
            .try_with_transaction_code_input(None, TransactionCodeInputLimits::default()),
        Err(CredentialOfferError::TransactionCodeInputRequired)
    ));
    assert!(matches!(
        bound_offer(None, "secret-code", "token").try_with_transaction_code_input(
            Some("unexpected".to_owned()),
            TransactionCodeInputLimits::default(),
        ),
        Err(CredentialOfferError::TransactionCodeInputUnexpected)
    ));
}

#[test]
fn empty_and_oversized_multibyte_input_fail_distinctly() {
    assert!(matches!(
        bound_offer(Some("{}"), "secret-code", "token").try_with_transaction_code_input(
            Some(String::new()),
            TransactionCodeInputLimits::default(),
        ),
        Err(CredentialOfferError::TransactionCodeInputEmpty)
    ));
    assert!(matches!(
        bound_offer(Some("{}"), "secret-code", "token").try_with_transaction_code_input(
            Some("é".to_owned()),
            TransactionCodeInputLimits::new(1).expect("one-byte limit"),
        ),
        Err(CredentialOfferError::TransactionCodeInputTooLarge)
    ));
}

#[test]
fn exact_multibyte_boundary_is_accepted() {
    let prepared = bound_offer(Some("{}"), "secret-code", "token")
        .try_with_transaction_code_input(
            Some("éé".to_owned()),
            TransactionCodeInputLimits::new(4).expect("four-byte limit"),
        )
        .expect("exact byte boundary");

    assert!(prepared.transaction_code_present());
}

#[test]
fn advertised_mode_and_length_remain_ui_guidance() {
    let prepared = bound_offer(
        Some(r#"{"input_mode":"numeric","length":4}"#),
        "secret-code",
        "token",
    )
    .try_with_transaction_code_input(
        Some("server-validates-this-text".to_owned()),
        TransactionCodeInputLimits::default(),
    )
    .expect("opaque input");

    assert!(prepared.transaction_code_present());
    let requirements = prepared
        .credential_offer_with_pre_authorized_server()
        .credential_offer_with_metadata()
        .credential_offer()
        .pre_authorized_code()
        .expect("grant")
        .transaction_code()
        .expect("requirements");
    assert_eq!(requirements.length(), Some(4));
}

#[test]
fn prepared_state_and_new_errors_have_static_redacted_diagnostics() {
    let input_canary = "TRANSACTION_CODE_INPUT_CANARY_4dc1";
    let predecessor_canary = "PRE_AUTHORIZED_CANARY_5be2";
    let prepared = bound_offer(Some("{}"), predecessor_canary, "TOKEN_ENDPOINT_CANARY_6cf3")
        .try_with_transaction_code_input(
            Some(input_canary.to_owned()),
            TransactionCodeInputLimits::default(),
        )
        .expect("prepared state");
    let rendered = format!("{prepared:?}");
    assert!(!rendered.contains(input_canary));
    assert!(!rendered.contains(predecessor_canary));

    let cases = [
        (
            CredentialOfferError::InvalidTransactionCodeInputLimits,
            error_code::INVALID_TRANSACTION_CODE_INPUT_LIMITS,
        ),
        (
            CredentialOfferError::TransactionCodeInputRequired,
            error_code::TRANSACTION_CODE_INPUT_REQUIRED,
        ),
        (
            CredentialOfferError::TransactionCodeInputUnexpected,
            error_code::TRANSACTION_CODE_INPUT_UNEXPECTED,
        ),
        (
            CredentialOfferError::TransactionCodeInputEmpty,
            error_code::TRANSACTION_CODE_INPUT_EMPTY,
        ),
        (
            CredentialOfferError::TransactionCodeInputTooLarge,
            error_code::TRANSACTION_CODE_INPUT_TOO_LARGE,
        ),
    ];
    for (error, code) in cases {
        let core: IdentusError = error.into();
        assert_eq!(core.code(), code);
        assert_eq!(core.kind(), ErrorKind::InvalidInput);
        assert_eq!(core.capability(), Some(CAPABILITY));
        for rendered in [
            format!("{error}"),
            format!("{error:?}"),
            format!("{core}"),
            format!("{core:?}"),
        ] {
            assert!(!rendered.contains(input_canary));
            assert!(!rendered.contains(predecessor_canary));
        }
    }
}
