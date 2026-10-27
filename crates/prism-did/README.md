# identus-prism-did

Portable, chain-neutral PRISM DID method primitives for SDK-Rust.

This crate owns bounded PRISM identifier syntax, exact protobuf operation
bytes, create/update/deactivate semantics, deterministic state reduction and
DID document projection. Cardano ingestion, persistence, VDR payload semantics,
HTTP and node composition are deliberately outside this crate.

Typed constructors validate keys, services, contexts and ordered actions before
emitting deterministic bounded protobuf bytes. Callers sign
`RawPrismOperation::as_bytes()` with their own custody implementation, then use
`SignedPrismOperation::from_signature` to assemble the exact envelope consumed
by `PrismDidState`. The crate never owns private keys or randomness.

The 96,000-byte operation ceiling derives a 128,075-byte maximum long-form DID.
Native projection has a separate 32 MiB preflight budget for repeated
controller identifiers; it does not widen generic untrusted JSON ingress.
