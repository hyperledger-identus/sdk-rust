#![no_main]

use std::sync::OnceLock;

use identus_did::{
    Did, DidResolutionResult, DidUrlDereferencingResult, MAX_DID_RESOLUTION_RESULT_BYTES,
};
use libfuzzer_sys::fuzz_target;

const REQUEST_DID: &str = "did:example:123";

fuzz_target!(|data: &[u8]| {
    let resolution = DidResolutionResult::from_json_slice_for(data, request_did());
    let dereferencing = DidUrlDereferencingResult::from_json_slice(data);

    if data.len() > MAX_DID_RESOLUTION_RESULT_BYTES {
        assert!(resolution.is_err());
        assert!(dereferencing.is_err());
        return;
    }

    if let Ok(result) = resolution {
        let serialized = serde_json::to_vec(&result).expect("validated result must serialize");
        let reparsed = DidResolutionResult::from_json_slice_for(&serialized, request_did())
            .expect("serialized validated result must reparse for the same DID");
        assert_eq!(result, reparsed);
        assert_eq!(
            serialized,
            serde_json::to_vec(&reparsed).expect("reparsed result must serialize")
        );
    }

    if let Ok(result) = dereferencing {
        let serialized = serde_json::to_vec(&result).expect("validated result must serialize");
        let reparsed = DidUrlDereferencingResult::from_json_slice(&serialized)
            .expect("serialized validated result must reparse");
        assert_eq!(result, reparsed);
        assert_eq!(
            serialized,
            serde_json::to_vec(&reparsed).expect("reparsed result must serialize")
        );
    }
});

fn request_did() -> &'static Did {
    static DID: OnceLock<Did> = OnceLock::new();
    DID.get_or_init(|| Did::parse(REQUEST_DID).expect("fixed request DID is valid"))
}
