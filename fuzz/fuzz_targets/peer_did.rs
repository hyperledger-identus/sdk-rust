#![no_main]

use identus_did_peer::{
    numalgo2_compat::PeerDid,
    numalgo4::{InputDocument, LongForm, ShortForm},
};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(text) = std::str::from_utf8(data) {
        if let Ok(long) = LongForm::parse(text) {
            assert_eq!(long.did().as_str(), text);
            assert_eq!(
                LongForm::parse(long.did().as_str()).expect("accepted long form must reparse"),
                long
            );
            long.resolve().expect("accepted long form must resolve");
        }
        if let Ok(short) = ShortForm::parse(text) {
            assert_eq!(short.did().as_str(), text);
        }
        if let Ok(compatibility) = PeerDid::parse(text) {
            assert_eq!(compatibility.did().as_str(), text);
            compatibility
                .resolve()
                .expect("accepted compatibility DID must resolve");
        }
    }

    if let Ok(input) = InputDocument::from_json_slice(data) {
        if let Ok(long) = LongForm::create(input) {
            LongForm::parse(long.did().as_str())
                .expect("created long form must parse")
                .resolve()
                .expect("created long form must resolve");
        }
    }
});
