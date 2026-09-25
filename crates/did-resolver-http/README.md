# identus-did-resolver-http

Chain-neutral HTTP bindings for DID resolution in the Hyperledger Identus Rust
SDK.

This crate maps the generic `identus-did` resolver contract to HTTP request,
response, content-negotiation, and optional OpenAPI types. It contains no DID
method, ledger, network, deployment, or wallet policy.

The crate is experimental. The `0.1.0-rc.1` candidate is assembled and tested
without changing the unpublished workspace manifest; publication requires a
separate reviewed activation change.
