# DID consumer compatibility evidence

Issue #65 turns the initial Rust consumer boundary into executable evidence in
`crates/did/tests/did_consumer_compatibility.rs`. The suite is intentionally a
public-API integration test rather than a new conformance API.

| Consumer evidence | Immutable revision | Generic SDK intersection | Remains downstream |
| --- | --- | --- | --- |
| NeoPRISM `lib/did-core` | `8becb225132efb1d9302b2c5f6ed4d87b84e8685` | DID documents, embedded/reference relationships, resolution envelopes, explicit legacy-error migration | `did:prism`, Cardano/VDR, HTTP and historical-operation policy |
| midnight-identity `midnight-did-domain` | `427f8571950c42967a18726cbcbefecc19ef8d79` | Rich public DID document, relationships, JWK maps, service endpoints and extensions | `did:midnight` network grammar, Compact/runtime, ledger and cryptosuite policy |
| Lace ID Portal `core` | `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` | Object-safe resolver injection and open result/document consumption | HTTP status, service orchestration, retries and trust policy |
| Oxid `identity/domain` | `685f9670af4846d52697a4cfeb94779758ae1075` | Generic document projection, public-key rejection boundary and multi-method composition | Wallet profile, custody, persistence, consent and method validation |

The cases are independently authored from standards-defined fields and public
API observations. They do not copy donor source, fixtures or production data.
Lace ID Portal remains evidence-only because its repository license evidence
was unresolved at the pinned revision.

Passing this suite means the pinned generic shapes remain representable under
the SDK's current bounds. It does not mean a downstream repository was built,
that a DID method or ledger is correct, that cryptographic proofs were checked,
or that transport, storage, wallet or regulatory behavior is certified.
