#![no_main]

use identus_did::Did;
use libfuzzer_sys::fuzz_target;

mod support;

fuzz_target!(|data: &[u8]| {
    support::assert_did_resource_boundaries();

    let Ok(input) = std::str::from_utf8(data) else {
        return;
    };

    if let Ok(did) = Did::parse(input) {
        support::assert_did_invariants(input, &did);
    }
});
