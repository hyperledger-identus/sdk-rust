#![no_main]

use identus_prism_did::{
    LongFormPrismDid, OperationKind, PrismDid, PrismDidState, RawPrismOperation,
    project_did_document_for,
};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let data = data
        .strip_prefix(b"text:")
        .map(|value| value.strip_suffix(b"\n").unwrap_or(value))
        .unwrap_or(data);

    if let Ok(input) = std::str::from_utf8(data)
        && let Ok(did) = PrismDid::parse(input)
    {
        assert_eq!(did.to_string(), input, "PRISM DID spelling changed");
        assert_eq!(did.as_did().as_str(), input);
        assert_eq!(did.canonical().as_did().method(), "prism");

        if let PrismDid::LongForm(long) = did {
            let reparsed =
                LongFormPrismDid::parse(input).expect("accepted long-form DID must reparse");
            assert_eq!(
                reparsed.initial_state().as_bytes(),
                long.initial_state().as_bytes()
            );
            let state = PrismDidState::from_initial_state(long.initial_state())
                .expect("accepted long-form DID must contain valid create state");
            let document = project_did_document_for(&state, long.as_did().clone())
                .expect("validated long-form state must project");
            assert_eq!(
                document.as_ref().map(identus_did::DidDocument::id),
                Some(long.as_did())
            );
        }
    }

    if let Ok(raw) = RawPrismOperation::parse(data)
        && raw.operation_kind() == OperationKind::CreateDid
        && PrismDidState::from_initial_state(&raw).is_ok()
        && let Ok(long) = LongFormPrismDid::from_initial_state(raw)
    {
        let reparsed =
            LongFormPrismDid::parse(&long.to_string()).expect("constructed long DID must reparse");
        assert_eq!(reparsed, long);
    }
});
