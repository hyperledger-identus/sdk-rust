#![no_main]

use identus_did_key::DidKey;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(text) = std::str::from_utf8(data)
        && let Ok(parsed) = DidKey::parse(text)
    {
        assert_eq!(parsed.did().as_str(), text);
        assert_eq!(DidKey::parse(parsed.did().as_str()).unwrap(), parsed);
        let document = parsed.resolve().expect("accepted did:key must resolve");
        assert_eq!(document.id(), parsed.did());
    }
});
