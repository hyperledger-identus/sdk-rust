#![forbid(unsafe_code)]

#[cfg(test)]
mod tests {
    use std::{convert::Infallible, panic::AssertUnwindSafe};

    use oauth2::{
        AuthUrl, AuthorizationCode, ClientId, CsrfToken, HttpRequest, PkceCodeChallenge,
        PkceCodeVerifier, RedirectUrl, Scope, TokenResponse, TokenUrl,
        basic::{BasicClient, BasicTokenType},
        http::{Response, StatusCode, header::CONTENT_TYPE},
    };

    const VERIFIER: &str = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk";
    const CHALLENGE: &str = "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM";

    fn client() -> BasicClient<
        oauth2::EndpointSet,
        oauth2::EndpointNotSet,
        oauth2::EndpointNotSet,
        oauth2::EndpointNotSet,
        oauth2::EndpointSet,
    > {
        BasicClient::new(ClientId::new("wallet-client".to_owned()))
            .set_auth_uri(
                AuthUrl::new("https://issuer.example/authorize?existing=1".to_owned())
                    .expect("fixed authorization URL is valid"),
            )
            .set_token_uri(
                TokenUrl::new("https://issuer.example/token".to_owned())
                    .expect("fixed token URL is valid"),
            )
            .set_redirect_uri(
                RedirectUrl::new("com.example.wallet:/callback".to_owned())
                    .expect("fixed redirect URL is valid"),
            )
    }

    #[test]
    fn deterministic_authorization_and_pkce() {
        let verifier = PkceCodeVerifier::new(VERIFIER.to_owned());
        let challenge = PkceCodeChallenge::from_code_verifier_sha256(&verifier);
        assert_eq!(challenge.as_str(), CHALLENGE);
        assert_eq!(challenge.method().as_str(), "S256");

        let (url, csrf) = client()
            .authorize_url(|| CsrfToken::new("fixed-csrf".to_owned()))
            .set_pkce_challenge(challenge)
            .add_scope(Scope::new("openid".to_owned()))
            .add_extra_param("issuer_state", "issuer-state")
            .url();
        assert_eq!(csrf.secret(), "fixed-csrf");
        assert_eq!(
            url.as_str(),
            concat!(
                "https://issuer.example/authorize?existing=1&response_type=code",
                "&client_id=wallet-client&state=fixed-csrf",
                "&code_challenge=E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM",
                "&code_challenge_method=S256",
                "&redirect_uri=com.example.wallet%3A%2Fcallback",
                "&scope=openid&issuer_state=issuer-state",
            )
        );
    }

    #[test]
    fn recording_transport_observes_exact_token_request() {
        let response = client()
            .exchange_code(AuthorizationCode::new("authorization-code".to_owned()))
            .set_pkce_verifier(PkceCodeVerifier::new(VERIFIER.to_owned()))
            .request(&|request: HttpRequest| -> Result<_, Infallible> {
                assert_eq!(request.method(), "POST");
                assert_eq!(request.uri(), "https://issuer.example/token");
                assert_eq!(request.headers()["accept"], "application/json");
                assert_eq!(
                    request.headers()["content-type"],
                    "application/x-www-form-urlencoded"
                );
                assert_eq!(
                    request.body(),
                    concat!(
                        "grant_type=authorization_code&code=authorization-code",
                        "&code_verifier=dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk",
                        "&client_id=wallet-client",
                        "&redirect_uri=com.example.wallet%3A%2Fcallback",
                    )
                    .as_bytes()
                );
                Ok(Response::builder()
                    .status(StatusCode::OK)
                    .header(CONTENT_TYPE, "application/json")
                    .body(
                        br#"{"access_token":"access-secret","token_type":"bearer","expires_in":300}"#
                            .to_vec(),
                    )
                    .expect("fixed response is valid"))
            })
            .expect("fixed token response parses");

        assert_eq!(response.access_token().secret(), "access-secret");
        assert_eq!(response.token_type(), &BasicTokenType::Bearer);
        assert_eq!(
            response
                .expires_in()
                .expect("fixed expiry is present")
                .as_secs(),
            300
        );
    }

    #[test]
    fn mismatch_contract_is_executable() {
        assert!(AuthUrl::new("http://user:password@example.com/a#fragment".to_owned()).is_ok());
        let (url, _) = client()
            .authorize_url(|| CsrfToken::new("fixed".to_owned()))
            .add_scope(Scope::new("bad scope".to_owned()))
            .add_extra_param("client_id", "duplicate")
            .url();
        assert_eq!(
            url.query_pairs()
                .filter(|(key, _)| key == "client_id")
                .count(),
            2
        );

        let short = PkceCodeVerifier::new("x".repeat(42));
        assert!(
            std::panic::catch_unwind(AssertUnwindSafe(|| {
                PkceCodeChallenge::from_code_verifier_sha256(&short)
            }))
            .is_err()
        );
        assert_eq!(format!("{short:?}"), "PkceCodeVerifier([redacted])");

        let invalid_characters = PkceCodeVerifier::new("!".repeat(43));
        assert_eq!(
            PkceCodeChallenge::from_code_verifier_sha256(&invalid_characters)
                .as_str()
                .len(),
            43
        );
    }

    #[test]
    fn response_boundary_mismatch_is_executable() {
        let canary = "ACCESS-TOKEN-CANARY";
        let error = client()
            .exchange_code(AuthorizationCode::new("code".to_owned()))
            .request(&|_: HttpRequest| -> Result<_, Infallible> {
                Ok(Response::builder()
                    .status(StatusCode::OK)
                    .header(CONTENT_TYPE, "application/json")
                    .body(format!(r#"{{"access_token":"{canary}","token_type":7}}"#).into_bytes())
                    .expect("fixed response is valid"))
            })
            .expect_err("invalid token_type must fail");
        match error {
            oauth2::RequestTokenError::Parse(_, body) => {
                assert!(
                    String::from_utf8(body)
                        .expect("fixture body is UTF-8")
                        .contains(canary)
                );
            }
            other => panic!("unexpected error: {other}"),
        }

        let oversized_unknown = "x".repeat(1_048_576);
        let response = client()
            .exchange_code(AuthorizationCode::new("code".to_owned()))
            .request(&|_: HttpRequest| -> Result<_, Infallible> {
                Ok(Response::builder()
                    .status(StatusCode::OK)
                    .header(CONTENT_TYPE, "application/json")
                    .body(
                        format!(
                            r#"{{"access_token":"secret","token_type":"bearer","authorization_details":"{oversized_unknown}"}}"#
                        )
                        .into_bytes(),
                    )
                    .expect("fixed response is valid"))
            })
            .expect("unknown oversized extension is ignored");
        assert_eq!(response.access_token().secret(), "secret");
    }
}
