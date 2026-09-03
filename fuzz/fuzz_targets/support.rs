// The two fuzz binaries intentionally compile different subsets of this shared
// invariant module.
#![allow(dead_code)]

use std::{fmt::Display, str::FromStr, sync::Once};

use identus_did::{Did, DidUrl};
use serde_json::{from_str, to_string};

static DID_BOUNDARY_PROBE: Once = Once::new();
static DID_URL_BOUNDARY_PROBE: Once = Once::new();

pub(crate) fn assert_did_resource_boundaries() {
    DID_BOUNDARY_PROBE.call_once(|| {
        let exact = format!("did:example:{}", "a".repeat(2_036));
        let over = format!("did:example:{}", "a".repeat(2_037));
        assert!(exact.len() == 2_048, "DID exact-limit probe drifted");
        assert!(Did::parse(&exact).is_ok(), "DID exact limit became invalid");
        assert!(
            Did::parse(&over).is_err(),
            "DID over-limit value was accepted"
        );
    });
}

pub(crate) fn assert_did_url_resource_boundaries() {
    DID_URL_BOUNDARY_PROBE.call_once(|| {
        let exact = format!("did:example:a/{}", "b".repeat(4_082));
        let over = format!("did:example:a/{}", "b".repeat(4_083));
        assert!(exact.len() == 4_096, "DID URL exact-limit probe drifted");
        assert!(
            DidUrl::parse(&exact).is_ok(),
            "DID URL exact limit became invalid"
        );
        assert!(
            DidUrl::parse(&over).is_err(),
            "DID URL over-limit value was accepted"
        );
    });
}

pub(crate) fn assert_did_invariants(input: &str, did: &Did) {
    assert!(did.as_str() == input, "DID spelling changed");
    assert!(did.to_string() == input, "DID display changed");
    assert!(!did.method().is_empty(), "DID method became empty");
    assert!(
        !did.method_specific_id().is_empty(),
        "DID method-specific identifier became empty"
    );
    assert!(
        is_utf8_subslice(did.as_str(), did.method()),
        "DID method range escaped"
    );
    assert!(
        is_utf8_subslice(did.as_str(), did.method_specific_id()),
        "DID method-specific identifier range escaped"
    );

    let mut rebuilt = String::from("did:");
    rebuilt.push_str(did.method());
    rebuilt.push(':');
    rebuilt.push_str(did.method_specific_id());
    assert!(rebuilt == input, "DID components do not reconstruct input");

    let owned = Did::try_new(input.to_owned()).expect("accepted DID must pass owned validation");
    assert!(owned == *did, "owned DID validation disagrees");
    let parsed = Did::from_str(input).expect("accepted DID must pass FromStr validation");
    assert!(parsed == *did, "DID FromStr validation disagrees");
    assert_json_round_trip(did, "DID serde round trip disagrees");

    let as_url = DidUrl::from(did.clone());
    assert!(
        as_url.as_str() == input,
        "DID-to-URL conversion changed spelling"
    );
    assert!(
        as_url.to_did() == *did,
        "DID-to-URL conversion changed value"
    );
}

pub(crate) fn assert_did_url_invariants(input: &str, did_url: &DidUrl) {
    assert!(did_url.as_str() == input, "DID URL spelling changed");
    assert!(did_url.to_string() == input, "DID URL display changed");
    assert!(!did_url.method().is_empty(), "DID URL method became empty");
    assert!(
        !did_url.method_specific_id().is_empty(),
        "DID URL method-specific identifier became empty"
    );

    for component in [
        did_url.as_did_str(),
        did_url.method(),
        did_url.method_specific_id(),
        did_url.path(),
    ] {
        assert!(
            is_utf8_subslice(did_url.as_str(), component),
            "DID URL component range escaped"
        );
    }
    for component in [did_url.query(), did_url.fragment()].into_iter().flatten() {
        assert!(
            is_utf8_subslice(did_url.as_str(), component),
            "optional DID URL component range escaped"
        );
    }

    let did = did_url.to_did();
    assert!(
        did.as_str() == did_url.as_did_str(),
        "DID URL prefix changed"
    );
    assert_did_invariants(did_url.as_did_str(), &did);

    let mut rebuilt = did_url.as_did_str().to_owned();
    rebuilt.push_str(did_url.path());
    if let Some(query) = did_url.query() {
        rebuilt.push('?');
        rebuilt.push_str(query);
    }
    if let Some(fragment) = did_url.fragment() {
        rebuilt.push('#');
        rebuilt.push_str(fragment);
    }
    assert!(
        rebuilt == input,
        "DID URL components do not reconstruct input"
    );

    let owned =
        DidUrl::try_new(input.to_owned()).expect("accepted DID URL must pass owned validation");
    assert!(owned == *did_url, "owned DID URL validation disagrees");
    let parsed = DidUrl::from_str(input).expect("accepted DID URL must pass FromStr validation");
    assert!(parsed == *did_url, "DID URL FromStr validation disagrees");
    assert_json_round_trip(did_url, "DID URL serde round trip disagrees");
}

fn assert_json_round_trip<T>(value: &T, message: &str)
where
    T: serde::Serialize + serde::de::DeserializeOwned + PartialEq + Display,
{
    let encoded = to_string(value).expect("validated value must serialize");
    let decoded = from_str::<T>(&encoded).expect("serialized value must deserialize");
    assert!(&decoded == value, "{message}");
}

fn is_utf8_subslice(whole: &str, part: &str) -> bool {
    let whole_start = whole.as_ptr() as usize;
    let part_start = part.as_ptr() as usize;
    let Some(offset) = part_start.checked_sub(whole_start) else {
        return false;
    };
    let Some(end) = offset.checked_add(part.len()) else {
        return false;
    };

    end <= whole.len() && whole.is_char_boundary(offset) && whole.is_char_boundary(end)
}
