#![no_main]

use identus_did::DidUrl;
use libfuzzer_sys::fuzz_target;

mod support;

fuzz_target!(|data: &[u8]| {
    support::assert_did_url_resource_boundaries();

    let Ok(input) = std::str::from_utf8(data) else {
        return;
    };

    if let Ok(did_url) = DidUrl::parse(input) {
        support::assert_did_url_invariants(input, &did_url);
    }
});
