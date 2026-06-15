# DID Method Support

`identus-did` owns the safe parsing boundary for DIDs and DID URLs. Method
creation and resolution are intentionally separate adapter concerns, but every
crate must use validated `Did` and `DidUrl` values instead of unchecked strings.

## Current Inventory

| Method | Support tier | Source evidence | Owner | Candidate crates |
|---|---|---|---|---|
| `did:prism` | Product | Cloud Agent, SDKs, neoprism, PRISM VDR | `identus-did` with neoprism convergence | Internal neoprism crates |
| `did:peer` | Product | Mediator, DIDComm, SDK peer connection flows | `identus-did`, `identus-messaging` | `did-peer`, `ssi-dids` |
| `did:web` | Extension | AnonCreds interop fixtures and common SSI deployments | resolver adapter | `ssi-dids`, `affinidi-did-web` |
| `did:key` | Extension | DIDComm and VC ecosystem fixtures | resolver adapter | `did-key`, `did-method-key`, `ssi-dids`, `affinidi-did-key` |
| `did:jwk` | Extension | key-material DID ecosystem candidate | resolver adapter | `did-jwk`, `ssi-dids` |
| `did:pkh` | Extension | account-bound DID ecosystem candidate and `ssi` support | resolver adapter | `ssi-dids` |
| `did:example` | Fixture only | DIDComm test vectors and W3C examples | `identus-conformance` | `ssi-dids` |

`did:prism` and `did:peer` are the only first-class product methods found in
the SDK, Mediator, Cloud Agent, and neoprism product paths during this slice.
Other methods are parsed safely so fixtures, wrappers, and future adapters can
carry them without stringly typed fallbacks.

## Dependency Policy

No external DID dependency is added to the core parser in this phase. Candidate
crates are recorded for adapter evaluation:

- `did-peer` for method-specific peer DID generation and resolution.
- `did-key`, `did-method-key`, and `did-jwk` for key-material DID adapters.
- `ssi` and `ssi-dids` for broader DID method and VC ecosystem interop.
- Affinidi DID crates for DID method adapter comparison.

The next resolver implementation must compare maintenance, feature flags,
`unsafe` usage, transitive dependencies, WASM support, and conformance coverage
before adding any of these dependencies to production crates.

