# Dependencies not adopted now

This is the durable negative-decision ledger for ADR 0061. “Not adopted” means
the assessed release is unsuitable for production use in `sdk-rust` today. It
is not a permanent ban. A new issue must satisfy the reconsideration trigger
and update the ADR before integration.

| Candidate | Assessed version/revision | Reason codes | Current alternative | Reconsider when |
| --- | --- | --- | --- | --- |
| `slip10` | 0.4.3 / `08faabf3` | maintenance, cone, duplication | Keep the small vector-tested Ed25519 SLIP-0010 implementation. | A maintained, target-tested release materially reduces code or proves broader curve demand. |
| `identity_did` | 1.5.1 / `7dd52708` | dependency-cone, domain coupling | Identus DID facade; use `identity.rs` for differential tests. | A narrow subcrate fits the cone budget without leaking framework types, or the SDK explicitly chooses model convergence. |
| `identity_document` | 1.5.1 / `7dd52708` | dependency-cone, validation-policy coupling | Identus document types and staged validation. | Its validator can be isolated behind Identus types with bounded input and compatible error semantics. |
| `identity_credential` | 1.5.1 / `7dd52708` | dependency-cone, draft-version, domain coupling | Identus credential facade and pinned format modules. | Current RFC/profile support is published independently and passes SDK conformance without public type leakage. |
| `identity_jose` | 1.5.1 / `7dd52708` | dependency-cone, security-state coupling | Identus JOSE facade over RustCrypto primitives. | A narrow operation removes local risky code while preserving duplicate rejection, algorithm policy and parsed/verified states. |
| `url` | 2.5.8 / `00a6ce58` | semantic mismatch, normalization coupling | `fluent-uri` grammar under exact-preserving Identus types. | A concrete HTTP adapter needs WHATWG URL behavior and keeps its types private. |
| `http` | 1.5.0 / `b6161d8f` | transport coupling, no current payoff | Transport-neutral Identus request/response ports. | Multiple adapters require shared HTTP vocabulary without exposing it in domain APIs. |
| `mime` | 0.3.17 / `1ef137c7` | strictness mismatch, low payoff | Existing bounded media-type check. | It can prove duplicate-parameter and fail-closed parity while deleting meaningful local complexity. |
| `headers` | 0.4.1 / `c3e009e8` | `std` coupling, permissive semantics, cone | Existing bounded `no-store` parser. | A newer narrow parser implements current HTTP semantics and exact fail-closed behavior on SDK targets. |
| Spruce `ssi` umbrella and broad modules | 0.16.0 / `89630368` | MSRV 1.89 under current policy, dependency-cone, domain coupling | Use selected modules as conformance oracles. | Evaluate one narrow module at a time after the rolling MSRV lands; never adopt the umbrella solely because it compiles. |
| Spruce `openid4vp` | `e5f29b85` | git dependency, transport/runtime coupling, cone | Identus OID4VP spec plus differential oracle. | A crates.io release removes git pins, isolates core wire behavior and fits the target/cone gates. |
| impierce `openid4vc` production crates | `e9d99d21`; crates.io 0.1 placeholders | publication readiness, git patches, coupling | Treat upstream as a read-only final-spec oracle. | Non-placeholder crates are published from a pinned release without git patches and expose a narrow core surface. |
| OWF `sd-jwt-rs` | 0.7.1 / `146546cc` | obsolete draft version | Implement RFC 9901 behind Identus credential/presentation states. | A published release explicitly implements RFC 9901 and passes the RFC vectors and SDK negative cases. |
| `didcomm` | 0.4.1 / `4388350d` | maintenance, protocol-version drift, old dependencies | No DIDComm runtime dependency; keep IDR-041 conditional. | An actively maintained release targets DIDComm 2.1, passes current vectors and removes stale git/native dependencies. |
| RustCrypto `jose-jwk` | 0.1.2 / `0a9a98a9` | API maturity, model coupling, low payoff | Identus JWK facade; use as a differential oracle. | API maturity and operation coverage remove local risky mechanics without exporting crate types. |

## Conditional rather than rejected

These candidates remain in the positive portfolio but cannot be production
dependencies until their named prerequisite is satisfied:

| Candidate | Condition |
| --- | --- |
| `multibase 0.9.3` | [#156](https://github.com/hyperledger-identus/sdk-rust/issues/156): the SDK MSRV is at least Rust 1.88, or upstream publishes accurate MSRV-compatible transitive resolution. No indefinite transitive pin. |
| `did_url_parser 0.3.0` | [#159](https://github.com/hyperledger-identus/sdk-rust/issues/159): a differential DID/DID-URL corpus proves accepted/rejected syntax, exact serialization, error mapping and target parity before ADR 0008 is superseded. |
| `isomdl 0.2.0` | [#161](https://github.com/hyperledger-identus/sdk-rust/issues/161): a spike isolates holder/verifier core from CLI, RNG, X.509 and transport choices and measures all target/cone costs. |
| `oauth2 5.0.0` | [#160](https://github.com/hyperledger-identus/sdk-rust/issues/160): a spike proves authorization-code/PKCE reuse without importing HTTP clients, clock, URL or token policy into protocol domain crates. |
| `aries-askar` | [#162](https://github.com/hyperledger-identus/sdk-rust/issues/162): an optional adapter proves storage-port fit, cancellation/concurrency, migrations, secret redaction, platform linking and a no-database core boundary. |
| `uniffi 0.32.0` | [#163](https://github.com/hyperledger-identus/sdk-rust/issues/163): binding value/opaque-handle/error contracts stabilize and native runtime tests exist; UniFFI stays out of generic core crates. |

Reason codes are deliberately finite: `MSRV`, `dependency-cone`,
`draft-version`, `coupling`, `maintenance`, `publication readiness`, `semantic
mismatch`, `strictness mismatch`, `native/unsafe`, `license/provenance`, and
`low payoff`.
