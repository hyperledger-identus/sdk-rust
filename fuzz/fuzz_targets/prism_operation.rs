#![no_main]

use identus_prism_did::{
    OperationKind, PrismDidState, RawPrismOperation, SignedPrismOperation, project_did_document,
};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let decoded;
    let data = if let Some(encoded) = data.strip_prefix(b"hex:") {
        let encoded = encoded.strip_suffix(b"\n").unwrap_or(encoded);
        let Ok(encoded) = std::str::from_utf8(encoded) else {
            return;
        };
        let Ok(bytes) = hex::decode(encoded) else {
            return;
        };
        decoded = bytes;
        decoded.as_slice()
    } else {
        data
    };

    if let Ok(raw) = RawPrismOperation::parse(data) {
        assert_eq!(raw.as_bytes(), data, "raw operation bytes changed");
        assert!(raw.operation_payload().len() <= data.len());
        assert_eq!(raw.operation_hash(), raw.operation_hash());

        if raw.operation_kind() == OperationKind::CreateDid
            && let Ok(state) = PrismDidState::from_initial_state(&raw)
        {
            assert_eq!(state.last_operation_hash(), raw.operation_hash());
            let first = project_did_document(&state).expect("validated state must project");
            let second = project_did_document(&state).expect("validated state must reproject");
            assert_eq!(first, second, "projection changed across identical calls");
        }
    }

    if let Ok(signed) = SignedPrismOperation::parse(data) {
        assert_eq!(signed.as_bytes(), data, "signed envelope bytes changed");
        assert!(signed.operation_bytes().len() <= data.len());
        assert!(signed.operation_payload().len() <= signed.operation_bytes().len());
        let raw = RawPrismOperation::parse(signed.operation_bytes())
            .expect("accepted signed operation must contain an accepted operation");
        assert_eq!(raw.operation_kind(), signed.operation_kind());
        assert_eq!(raw.operation_hash(), signed.operation_hash());
    }
});
