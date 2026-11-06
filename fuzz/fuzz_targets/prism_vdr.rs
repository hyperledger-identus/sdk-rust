#![no_main]

use identus_prism_did::SignedPrismOperation;
use identus_prism_vdr::VdrOperation;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(signed) = SignedPrismOperation::parse(data) else {
        return;
    };
    let first = VdrOperation::parse(&signed);
    let second = VdrOperation::parse(&signed);
    match (first, second) {
        (Ok(left), Ok(right)) => {
            assert_eq!(left.operation_hash(), right.operation_hash());
            assert_eq!(left.predecessor(), right.predecessor());
        }
        (Err(left), Err(right)) => assert_eq!(left.kind(), right.kind()),
        _ => panic!("VDR parsing changed across identical inputs"),
    }
});
