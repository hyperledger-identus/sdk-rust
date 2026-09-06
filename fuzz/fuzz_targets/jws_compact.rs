#![no_main]

use libfuzzer_sys::fuzz_target;

mod jws_support;

fuzz_target!(|data: &[u8]| {
    jws_support::fuzz_jws_compact(data);
});
