#![no_main]

use libfuzzer_sys::fuzz_target;

mod crypto_support;

fuzz_target!(|data: &[u8]| {
    crypto_support::fuzz_public_cose(data);
});
