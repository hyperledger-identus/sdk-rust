#![allow(dead_code)]

use std::{borrow::Cow, sync::Once};

use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use identus_jose::{CAPABILITY, JoseError, JwsLimits, JwsSigningInput, UnverifiedCompactJws};

static DEFAULT_BOUNDARY_PROBE: Once = Once::new();

pub(crate) fn fuzz_jws_compact(data: &[u8]) {
    assert_default_resource_boundary();

    let normalized = decode_text_seed(data);
    if let Ok(compact) = std::str::from_utf8(&normalized) {
        check(compact, JwsLimits::default());
    }

    if data.len() >= 10 {
        let limits = derived_limits(&data[..10]);
        let suffix = decode_limit_seed(data);
        if let Ok(compact) = std::str::from_utf8(&suffix) {
            check(compact, limits);
        }
    }
}

fn decode_text_seed(data: &[u8]) -> Cow<'_, [u8]> {
    let Some(text) = data.strip_prefix(b"text:") else {
        return Cow::Borrowed(data);
    };
    let text = text.strip_suffix(b"\n").unwrap_or(text);
    let text = text.strip_suffix(b"\r").unwrap_or(text);
    Cow::Borrowed(text)
}

fn decode_limit_seed(data: &[u8]) -> Cow<'_, [u8]> {
    let suffix = &data[10..];
    if !data.starts_with(b"limits:") {
        return Cow::Borrowed(suffix);
    }
    let suffix = suffix.strip_suffix(b"\n").unwrap_or(suffix);
    let suffix = suffix.strip_suffix(b"\r").unwrap_or(suffix);
    Cow::Borrowed(suffix)
}

fn derived_limits(prefix: &[u8]) -> JwsLimits {
    let selected = |offset: usize, cap: usize| {
        usize::from(u16::from_le_bytes([prefix[offset], prefix[offset + 1]])) % cap + 1
    };
    JwsLimits::new(
        selected(0, 8_192),
        selected(2, 4_096),
        selected(4, 4_096),
        selected(6, 1_024),
        selected(8, 2_048),
    )
    .unwrap_or_else(|_| panic!("positive capped JWS limits must be valid"))
}

fn check(compact: &str, limits: JwsLimits) {
    match UnverifiedCompactJws::parse(compact, limits) {
        Ok(parsed) => assert_accepted(compact, limits, &parsed),
        Err(error) => assert_static_codec_error(error),
    }
}

fn assert_accepted(compact: &str, limits: JwsLimits, parsed: &UnverifiedCompactJws) {
    assert!(parsed.compact() == compact, "accepted compact text changed");

    let second_dot = compact
        .rfind('.')
        .unwrap_or_else(|| panic!("accepted compact text must contain two separators"));
    assert!(
        parsed.signing_input() == &compact.as_bytes()[..second_dot],
        "accepted signing input changed"
    );

    let mut segments = compact.split('.');
    for _ in 0..3 {
        let segment = segments
            .next()
            .unwrap_or_else(|| panic!("accepted compact text must have three segments"));
        let decoded = URL_SAFE_NO_PAD
            .decode(segment)
            .unwrap_or_else(|_| panic!("accepted segment must decode as base64url"));
        assert!(
            URL_SAFE_NO_PAD.encode(decoded) == segment,
            "accepted segment was not canonical unpadded base64url"
        );
    }
    assert!(
        segments.next().is_none(),
        "accepted compact text had extra segments"
    );

    let reparsed = UnverifiedCompactJws::parse(compact, limits)
        .unwrap_or_else(|_| panic!("accepted compact text must reparse under identical limits"));
    assert!(
        reparsed == *parsed,
        "same-limit parse was not deterministic"
    );

    let rebuild_limits = canonical_rebuild_limits(parsed, limits);
    let rebuilt = JwsSigningInput::new(
        parsed.protected_header().clone(),
        parsed.payload().to_vec(),
        rebuild_limits,
    )
    .unwrap_or_else(|_| panic!("accepted JWS semantics must rebuild"))
    .attach_signature(parsed.signature().to_vec())
    .unwrap_or_else(|_| panic!("accepted JWS signature must reattach"));
    let rebuilt = UnverifiedCompactJws::parse(rebuilt.compact(), rebuild_limits)
        .unwrap_or_else(|_| panic!("rebuilt JWS must parse under canonical output limits"));
    assert!(
        rebuilt.protected_header() == parsed.protected_header()
            && rebuilt.payload() == parsed.payload()
            && rebuilt.signature() == parsed.signature(),
        "staged JWS rebuild changed accepted semantics"
    );
}

fn canonical_rebuild_limits(parsed: &UnverifiedCompactJws, limits: JwsLimits) -> JwsLimits {
    let header = serde_json::to_vec(parsed.protected_header())
        .unwrap_or_else(|_| panic!("accepted protected header must serialize"));
    let compact = encoded_len(header.len())
        .checked_add(encoded_len(parsed.payload().len()))
        .and_then(|length| length.checked_add(encoded_len(parsed.signature().len())))
        .and_then(|length| length.checked_add(2))
        .unwrap_or_else(|| panic!("bounded canonical JWS size must be representable"));
    JwsLimits::new(
        limits.max_compact_bytes().max(compact),
        limits.max_protected_header_bytes().max(header.len()),
        limits.max_payload_bytes().max(parsed.payload().len()),
        limits.max_signature_bytes().max(parsed.signature().len()),
        limits.max_header_string_bytes(),
    )
    .unwrap_or_else(|_| panic!("canonical JWS rebuild limits must be valid"))
}

fn encoded_len(length: usize) -> usize {
    length / 3 * 4
        + match length % 3 {
            0 => 0,
            1 => 2,
            _ => 3,
        }
}

fn assert_static_codec_error(error: JoseError) {
    let expected_code = match error {
        JoseError::CompactTooLarge => "jose.compact_too_large",
        JoseError::InvalidCompactStructure => "jose.invalid_compact_structure",
        JoseError::NonCanonicalBase64Url => "jose.non_canonical_base64url",
        JoseError::ProtectedHeaderTooLarge => "jose.protected_header_too_large",
        JoseError::PayloadTooLarge => "jose.payload_too_large",
        JoseError::SignatureTooLarge => "jose.signature_too_large",
        JoseError::InvalidProtectedHeader => "jose.invalid_protected_header",
        JoseError::DuplicateProtectedHeader => "jose.duplicate_protected_header",
        JoseError::UnknownProtectedHeader => "jose.unknown_protected_header",
        JoseError::MissingAlgorithm => "jose.missing_algorithm",
        JoseError::InvalidHeaderValue => "jose.invalid_header_value",
        JoseError::AmbiguousKeyReference => "jose.ambiguous_key_reference",
        JoseError::EmptySignature => "jose.empty_signature",
        JoseError::SizeOverflow => "jose.size_overflow",
        JoseError::UnsupportedAlgorithm => "jose.unsupported_algorithm",
        _ => panic!("compact parser returned a non-codec error variant"),
    };
    let bridged = error.to_identus_error();
    assert!(
        bridged.code().as_str() == expected_code,
        "compact parser error-code bridge drifted"
    );
    assert!(
        bridged.capability() == Some(CAPABILITY),
        "compact parser error capability drifted"
    );
    let _ = format!("{error}");
    let _ = format!("{error:?}");
    let _ = format!("{bridged}");
}

fn assert_default_resource_boundary() {
    DEFAULT_BOUNDARY_PROBE.call_once(|| {
        let limits = JwsLimits::default();
        let exact = "a".repeat(limits.max_compact_bytes());
        let over = "a".repeat(limits.max_compact_bytes() + 1);
        assert!(
            !matches!(
                UnverifiedCompactJws::parse(&exact, limits),
                Err(JoseError::CompactTooLarge)
            ),
            "exact-limit compact input was rejected as oversized"
        );
        assert!(
            matches!(
                UnverifiedCompactJws::parse(&over, limits),
                Err(JoseError::CompactTooLarge)
            ),
            "over-limit compact input was not rejected at the resource boundary"
        );

        let raw_header = br#"{"alg":"Ed25519","jwk":{"kty":"OKP","crv":"Ed25519","x":"BwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwc","ext":1e1}}"#;
        let tight_compact = format!(
            "{}..{}",
            URL_SAFE_NO_PAD.encode(raw_header),
            URL_SAFE_NO_PAD.encode(b"x")
        );
        let tight_limits = JwsLimits::new(
            tight_compact.len(),
            raw_header.len(),
            1,
            1,
            64,
        )
        .unwrap_or_else(|_| panic!("tight canonical-growth limits must be valid"));
        let parsed = UnverifiedCompactJws::parse(&tight_compact, tight_limits)
            .unwrap_or_else(|_| panic!("tight representation-dependent JWS must parse"));
        let canonical_header = serde_json::to_vec(parsed.protected_header())
            .unwrap_or_else(|_| panic!("accepted protected header must serialize"));
        assert!(
            canonical_header.len() > raw_header.len(),
            "canonical-growth probe no longer exercises a larger encoding"
        );
        assert_accepted(&tight_compact, tight_limits, &parsed);
    });
}
