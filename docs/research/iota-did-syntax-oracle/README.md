# IOTA Identity DID syntax oracle

This isolated unpublished Cargo workspace compares the SDK's bounded DID and
DID URL parser with exact `identity_did 1.5.1`. It is oracle evidence only and
is not part of the SDK workspace, dependency graph or public model.

Run from the repository root with Rust 1.98.1:

```sh
./scripts/check-iota-did-syntax-oracle.sh
```

The corpus pins its sources, revisions, normative expectation, SDK observation,
IOTA observation and mismatch class. Candidate errors and caller inputs are not
printed. W3C DID Core 1.0 and RFC 3986 remain authoritative.

This is a manual reference fixture, not a CI gate. ADR 0104 records why its
large dependency graph, denied advisories, unsafe reach and WASM failure make
automatic or production adoption unacceptable at the pinned version.
