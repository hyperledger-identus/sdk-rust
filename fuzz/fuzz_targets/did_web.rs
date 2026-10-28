#![no_main]

use identus_did_web::DidWeb;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let fixed = DidWeb::parse("did:web:example.com").unwrap();
    let request = fixed.request();
    if let Ok(response) = request.response(200, Some("application/did+json"), data.to_vec()) {
        let _ = fixed.bind_response(response);
    }

    if let Ok(text) = std::str::from_utf8(data)
        && let Ok(parsed) = DidWeb::parse(text)
    {
        assert_eq!(parsed.did().as_str(), text);
        assert_eq!(DidWeb::parse(parsed.did().as_str()).unwrap(), parsed);
        assert!(parsed.url().starts_with("https://"));
        assert!(parsed.url().ends_with("/did.json"));
    }
});
