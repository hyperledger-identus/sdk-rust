use std::hint::black_box;

use identus_core::ErrorKind;
use identus_did::{Did, DidSyntaxError, DidUrl, Error, MAX_DID_BYTES, MAX_DID_URL_BYTES};

#[test]
fn accepts_w3c_and_consumer_shaped_dids() {
    let cases = [
        (
            "did:example:123456789abcdefghi",
            "example",
            "123456789abcdefghi",
        ),
        (
            "did:prism:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
            "prism",
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        ),
        (
            "did:midnight:undeployed:network:holder_1",
            "midnight",
            "undeployed:network:holder_1",
        ),
        (
            "did:web:example.com:users:alice",
            "web",
            "example.com:users:alice",
        ),
        ("did:key:z6MkiTBz1ym", "key", "z6MkiTBz1ym"),
        ("did:method1:a:b::c%2Fd", "method1", "a:b::c%2Fd"),
    ];

    for (input, method, method_specific_id) in cases {
        let did = Did::parse(input).unwrap();
        assert_eq!(did.as_str(), input);
        assert_eq!(did.method(), method);
        assert_eq!(did.method_specific_id(), method_specific_id);
        assert_eq!(did.to_string(), input);
        assert_eq!(did.as_ref(), input);
    }
}

#[test]
fn rejects_malformed_bare_dids_without_reflecting_input() {
    let invalid = [
        "",
        "DID:example:123",
        "did:",
        "did::123",
        "did:Example:123",
        "did:exam-ple:123",
        "did:example:",
        "did:example:123:",
        "did:example:%",
        "did:example:%2",
        "did:example:%GG",
        "did:example:with space",
        "did:example:ümlaut",
        "did:example:123/path",
        "did:example:123?query",
        "did:example:123#fragment",
    ];

    for input in invalid {
        let error = Did::parse(input).unwrap_err();
        assert!(matches!(error, Error::InvalidDid(_)));

        let public = error.to_identus_error();
        assert_eq!(public.code().as_str(), "did.invalid_did");
        assert_eq!(public.capability().map(|value| value.as_str()), Some("did"));
        assert_eq!(public.kind(), ErrorKind::InvalidInput);
    }

    let secret = "private-caller-value";
    let error = Did::parse(&format!("did:example:{secret} space")).unwrap_err();
    assert!(!error.to_string().contains(secret));
    assert!(!error.to_identus_error().to_string().contains(secret));
}

#[test]
fn did_url_exposes_borrowed_components() {
    let value = "did:example:123/a:b/@c/%2F?version=1/2?ok#key-1/path?part";
    let url = DidUrl::parse(value).unwrap();

    assert_eq!(url.as_str(), value);
    assert_eq!(url.as_did_str(), "did:example:123");
    assert_eq!(url.method(), "example");
    assert_eq!(url.method_specific_id(), "123");
    assert_eq!(url.path(), "/a:b/@c/%2F");
    assert_eq!(url.query(), Some("version=1/2?ok"));
    assert_eq!(url.fragment(), Some("key-1/path?part"));
    assert_eq!(url.to_did().as_str(), "did:example:123");

    let base = url.as_str().as_ptr() as usize;
    let end = base + url.as_str().len();
    for component in [
        url.as_did_str(),
        url.method(),
        url.method_specific_id(),
        url.path(),
        url.query().unwrap(),
        url.fragment().unwrap(),
    ] {
        let pointer = component.as_ptr() as usize;
        assert!((base..=end).contains(&pointer));
    }
}

#[test]
fn did_url_preserves_absent_and_empty_components() {
    let bare = DidUrl::parse("did:example:123").unwrap();
    assert_eq!(bare.path(), "");
    assert_eq!(bare.query(), None);
    assert_eq!(bare.fragment(), None);

    let empty_query = DidUrl::parse("did:example:123?").unwrap();
    assert_eq!(empty_query.path(), "");
    assert_eq!(empty_query.query(), Some(""));
    assert_eq!(empty_query.fragment(), None);

    let empty_fragment = DidUrl::parse("did:example:123#").unwrap();
    assert_eq!(empty_fragment.query(), None);
    assert_eq!(empty_fragment.fragment(), Some(""));

    let both = DidUrl::parse("did:example:123?#").unwrap();
    assert_eq!(both.query(), Some(""));
    assert_eq!(both.fragment(), Some(""));
}

#[test]
fn rejects_malformed_did_urls_with_redacted_reasons() {
    let invalid = [
        "did:example:123/%",
        "did:example:123/%0",
        "did:example:123/%XZ",
        "did:example:123/a b",
        "did:example:123?query value",
        "did:example:123#fragment#again",
        "did:example:123/ümlaut",
        "did:example:123?x=%Q0",
    ];

    for input in invalid {
        let error = DidUrl::parse(input).unwrap_err();
        assert!(matches!(error, Error::InvalidDidUrl(_)));
        assert!(!error.to_string().contains(input));

        let public = error.to_identus_error();
        assert_eq!(public.code().as_str(), "did.invalid_did_url");
        assert_eq!(public.capability().map(|value| value.as_str()), Some("did"));
        assert_eq!(public.kind(), ErrorKind::InvalidInput);
        assert!(!public.to_string().contains(input));
    }
}

#[test]
fn exact_byte_limits_are_enforced() {
    let prefix = "did:example:";
    let at_did_limit = format!("{prefix}{}", "a".repeat(MAX_DID_BYTES - prefix.len()));
    assert_eq!(at_did_limit.len(), MAX_DID_BYTES);
    assert!(Did::parse(&at_did_limit).is_ok());
    assert!(matches!(
        Did::parse(&format!("{at_did_limit}a")),
        Err(Error::InvalidDid(DidSyntaxError::TooLong))
    ));

    let path_len = MAX_DID_URL_BYTES - at_did_limit.len() - 1;
    let at_url_limit = format!("{at_did_limit}/{}", "a".repeat(path_len));
    assert_eq!(at_url_limit.len(), MAX_DID_URL_BYTES);
    assert!(DidUrl::parse(&at_url_limit).is_ok());
    assert!(matches!(
        DidUrl::parse(&format!("{at_url_limit}a")),
        Err(Error::InvalidDidUrl(DidSyntaxError::TooLong))
    ));

    let oversized_embedded_did = format!("{prefix}{}", "a".repeat(MAX_DID_BYTES));
    assert!(matches!(
        DidUrl::parse(&oversized_embedded_did),
        Err(Error::InvalidDidUrl(DidSyntaxError::TooLong))
    ));
}

#[test]
fn owned_construction_and_did_conversion_reuse_allocations() {
    let mut source = String::with_capacity(128);
    source.push_str("did:example:allocation-test");
    let original_pointer = source.as_ptr();
    let did = Did::try_from(source).unwrap();
    assert_eq!(did.as_str().as_ptr(), original_pointer);

    let did_pointer = did.as_str().as_ptr();
    let url = DidUrl::from(did);
    assert_eq!(url.as_str().as_ptr(), did_pointer);
    assert_eq!(url.path(), "");
}

#[test]
fn serde_and_native_boundaries_are_equivalent() {
    let valid = "did:example:a:b%2Fc/path?query#fragment";
    let native = DidUrl::parse(valid).unwrap();
    let json = serde_json::to_string(&native).unwrap();
    assert_eq!(json, format!("\"{valid}\""));
    assert_eq!(serde_json::from_str::<DidUrl>(&json).unwrap(), native);

    let did = Did::parse("did:example:a:b%2Fc").unwrap();
    assert_eq!(
        serde_json::from_str::<Did>(&serde_json::to_string(&did).unwrap()).unwrap(),
        did
    );

    for invalid in [
        "did:Example:123",
        "did:example:%GG",
        "did:example:123 space",
    ] {
        assert!(Did::parse(invalid).is_err());
        assert!(serde_json::from_str::<Did>(&format!("\"{invalid}\"")).is_err());
    }
    for invalid in ["did:example:123/a b", "did:example:123#one#two"] {
        assert!(DidUrl::parse(invalid).is_err());
        assert!(serde_json::from_str::<DidUrl>(&format!("\"{invalid}\"")).is_err());
    }
}

#[test]
fn generic_did_ascii_grammar_is_exhaustive() {
    for byte in 0_u8..=127 {
        let character = char::from(byte);
        if byte != b':' {
            let method = format!("did:a{character}b:value");
            let method_expected = byte.is_ascii_lowercase() || byte.is_ascii_digit();
            assert_eq!(
                Did::parse(&method).is_ok(),
                method_expected,
                "method byte 0x{byte:02x}"
            );
        }

        let method_specific_id = format!("did:example:a{character}b");
        let id_expected = byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-' | b'_' | b':');
        assert_eq!(
            Did::parse(&method_specific_id).is_ok(),
            id_expected,
            "method-specific-id byte 0x{byte:02x}"
        );
    }

    for escape in ["%00", "%2f", "%2F", "%aB", "%FF"] {
        assert!(Did::parse(&format!("did:example:a{escape}b")).is_ok());
    }
}

#[test]
fn did_url_ascii_component_grammar_is_exhaustive() {
    fn is_pchar(byte: u8) -> bool {
        byte.is_ascii_alphanumeric()
            || matches!(
                byte,
                b'-' | b'.'
                    | b'_'
                    | b'~'
                    | b'!'
                    | b'$'
                    | b'&'
                    | b'\''
                    | b'('
                    | b')'
                    | b'*'
                    | b'+'
                    | b','
                    | b';'
                    | b'='
                    | b':'
                    | b'@'
            )
    }

    for byte in 0_u8..=127 {
        let character = char::from(byte);

        if !matches!(byte, b'?' | b'#') {
            let path = format!("did:example:123/a{character}b");
            assert_eq!(
                DidUrl::parse(&path).is_ok(),
                is_pchar(byte) || byte == b'/',
                "path byte 0x{byte:02x}"
            );
        }

        if byte != b'#' {
            let query = format!("did:example:123?a{character}b");
            assert_eq!(
                DidUrl::parse(&query).is_ok(),
                is_pchar(byte) || matches!(byte, b'/' | b'?'),
                "query byte 0x{byte:02x}"
            );
        }

        let fragment = format!("did:example:123#a{character}b");
        assert_eq!(
            DidUrl::parse(&fragment).is_ok(),
            is_pchar(byte) || matches!(byte, b'/' | b'?'),
            "fragment byte 0x{byte:02x}"
        );
    }

    for escape in ["%00", "%2f", "%2F", "%aB", "%FF"] {
        assert!(DidUrl::parse(&format!("did:example:123/a{escape}b")).is_ok());
        assert!(DidUrl::parse(&format!("did:example:123?a{escape}b")).is_ok());
        assert!(DidUrl::parse(&format!("did:example:123#a{escape}b")).is_ok());
    }
}

#[test]
#[ignore = "manual release-mode throughput diagnostic"]
fn parser_throughput_diagnostic() {
    const ITERATIONS: u64 = 500_000;
    const VALUE: &str =
        "did:midnight:undeployed:network:holder_1/credentials/active?limit=20#key-1";

    let started = std::time::Instant::now();
    for _ in 0..ITERATIONS {
        black_box(DidUrl::parse(black_box(VALUE)).unwrap());
    }
    let elapsed = started.elapsed();
    let parses_per_second = ITERATIONS as f64 / elapsed.as_secs_f64();
    eprintln!(
        "DID URL parser diagnostic: {ITERATIONS} parses in {elapsed:?} ({parses_per_second:.0} parses/s)"
    );
}
