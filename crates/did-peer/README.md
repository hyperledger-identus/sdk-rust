# identus-did-peer

Bounded, effect-free peer DID profiles for SDK-Rust.

- `numalgo4` is the current pinned short/long-form profile.
- `numalgo2_compat` is an explicit legacy Identus compatibility profile.

The crate accepts public material only. Key generation and custody, DIDComm,
relationship storage, networking, persistence and agent orchestration belong to
callers.
