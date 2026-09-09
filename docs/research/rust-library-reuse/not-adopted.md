# Dependencies not adopted now

This is the durable negative-decision ledger for ADR 0061. “Not adopted” means
the assessed release is unsuitable for production use in `sdk-rust` today. It
is not a permanent ban. A new issue must satisfy the reconsideration trigger
and update the ADR before integration.

| Candidate | Assessed version/revision | Reason codes | Current alternative | Reconsider when |
| --- | --- | --- | --- | --- |
| `bip32` | 0.5.3 / `240679a2` | `semantic-mismatch`, `dependency-cone`, `coupling`, `low-payoff` | Keep narrow BIP-32 HMAC orchestration and reuse existing `k256` exact scalar parsing/arithmetic behind `HDKey` ([ADR 0080](../../adr/0080-retain-bip32-mechanics-over-existing-k256.md)). | A stable narrow API accepts zero `IL`, supports every 16..=64-byte seed and feature-slices xprv/xpub serialization dependencies. |
| `slip10` | 0.4.3 / `08faabf3` | `maintenance`, `dependency-cone`, `low-payoff` | Keep the small vector-tested Ed25519 SLIP-0010 implementation. | A maintained, target-tested release materially reduces code or proves broader curve demand. |
| `identity_did` | 1.5.1 / `7dd52708` | `dependency-cone`, `coupling` | Identus DID facade; use `identity.rs` for differential tests. | A narrow subcrate fits the cone budget without leaking framework types, or the SDK explicitly chooses model convergence. |
| `identity_document` | 1.5.1 / `7dd52708` | `dependency-cone`, `coupling` | Identus document types and staged validation. | Its validator can be isolated behind Identus types with bounded input and compatible error semantics. |
| `identity_credential` | 1.5.1 / `7dd52708` | `dependency-cone`, `draft-version`, `coupling` | Identus credential facade and pinned format modules. | Current RFC/profile support is published independently and passes SDK conformance without public type leakage. |
| `identity_jose` | 1.5.1 / `7dd52708` | `dependency-cone`, `coupling` | Identus JOSE facade over RustCrypto primitives. | A narrow operation removes local risky code while preserving duplicate rejection, algorithm policy and parsed/verified states. |
| `url` | 2.5.8 / `00a6ce58` | `semantic-mismatch`, `coupling` | `fluent-uri` grammar under exact-preserving Identus types. | A concrete HTTP adapter needs WHATWG URL behavior and keeps its types private. |
| `http` | 1.5.0 / `b6161d8f` | `coupling`, `low-payoff` | Transport-neutral Identus request/response ports. | Multiple adapters require shared HTTP vocabulary without exposing it in domain APIs. |
| `mime` | 0.3.17 / `1ef137c7` | `strictness-mismatch`, `low-payoff` | Existing bounded media-type check. | It can prove duplicate-parameter and fail-closed parity while deleting meaningful local complexity. |
| `headers` | 0.4.1 / `c3e009e8` | `coupling`, `semantic-mismatch`, `dependency-cone` | Existing bounded `no-store` parser. | A newer narrow parser implements current HTTP semantics and exact fail-closed behavior on SDK targets. |
| `form_urlencoded` | 1.2.2 / crates.io checksum `cb4cb245038516f5f85277875cdaa4f7d2c9a0fa0468de06ed190163b1581fcf`; VCS `91377f48` | `semantic-mismatch`, `strictness-mismatch`, `low-payoff` | Keep the strict bounded OID4VCI form codec ([ADR 0083](../../adr/0083-retain-strict-form-codec.md)); use the candidate only as a source-level oracle. | A maintained release provides strict non-lossy parsing with full negative parity, or multiple consumers create measured serializer payoff after sizing, ordering and secret ownership remain local. |
| `multibase` | 0.9.3 / `cdeda567` | `dependency-cone`, `coupling`, `low-payoff` | Compose exact `bs58 0.5.1` and the existing `base64 0.22` engine behind the private `z`/`u` policy ([ADR 0085](../../adr/0085-compose-narrow-multibase-codecs.md)). | Upstream feature-slices the required bases, or named consumers require enough additional registry encodings to justify the nine-name incremental cone. |
| Spruce `ssi` umbrella and broad modules | 0.16.0 / `89630368` | `MSRV`, `dependency-cone`, `coupling` | Use selected modules as conformance oracles. | Evaluate one narrow module at a time after the rolling MSRV lands; never adopt the umbrella solely because it compiles. |
| Spruce `openid4vp` | `e5f29b85` | `publication-readiness`, `coupling`, `dependency-cone` | Identus OID4VP spec plus differential oracle. | A crates.io release removes git pins, isolates core wire behavior and fits the target/cone gates. |
| impierce `openid4vc` production crates | `e9d99d21`; crates.io 0.1 placeholders | `publication-readiness`, `coupling` | Treat upstream as a read-only final-spec oracle. | Non-placeholder crates are published from a pinned release without git patches and expose a narrow core surface. |
| OWF `sd-jwt-rs` | 0.7.1 / `146546cc` | `draft-version` | Implement RFC 9901 behind Identus credential/presentation states. | A published release explicitly implements RFC 9901 and passes the RFC vectors and SDK negative cases. |
| `didcomm` | 0.4.1 / `4388350d` | `maintenance`, `draft-version`, `dependency-cone` | No DIDComm runtime dependency; keep IDR-041 conditional. | An actively maintained release targets DIDComm 2.1, passes current vectors and removes stale git/native dependencies. |
| Procivis ONE Core | `e66ec8c5` | `coupling`, `dependency-cone`, `publication-readiness` | Identus-owned components; use ONE Core as a capability oracle. | A cohesive published component is isolated from application, transport, database, runtime and product policy. |
| DIF `did-key.rs` | 0.2.1 / `eb00da60` | `maintenance`, `dependency-cone` | Identus-owned method adapter; use behavior and vectors as an oracle. | A maintained release modernizes cryptography and proves exact key-family plus SDK target parity. |
| AnonCreds v2 Rust | `691297a7` | `publication-readiness`, `draft-version`, `native-unsafe` | Keep the capability uncommitted; monitor research and profiles. | A stable interoperability profile, reviewed release, vectors, declared Rust floor and mobile/WASM evidence exist. |
| OWF VCX | 0.68.0 / `5fa3cd50` | `coupling`, `dependency-cone`, `publication-readiness` | Use Aries/AATH state-machine behavior as an oracle. | One unique cohesive component is isolated from the unpublished wallet/runtime/VDR graph. |
| Spruce DIDKit | `57a3b451` | `maintenance`, `coupling` | Evaluate maintained underlying Spruce SSI crates directly. | The repository is maintained and unarchived with unique value absent from its underlying libraries. |
| RustCrypto `jose-jwk` | 0.1.2 / `0a9a98a9` | `publication-readiness`, `coupling`, `low-payoff` | Identus JWK facade; use as a differential oracle. | API maturity and operation coverage remove local risky mechanics without exporting crate types. |
| `did_url_parser` | 0.3.0 / `cdde0daf` | `semantic-mismatch`, `strictness-mismatch`, `native-unsafe`, `low-payoff` | Retain ADR 0008's bounded local parser; preserve only the isolated parity harness. | A new release passes the 30-case and 17,284-case corpora, pre-allocation bounds, allocation reuse, immutable facade, strict lint, unsafe and target gates in #159's successor. |

## Conditional rather than rejected

These candidates remain in the positive portfolio but cannot be production
dependencies until their named prerequisite is satisfied:

| Candidate | Condition |
| --- | --- |
| `multihash 0.19.5` | [ADR 0082](../../adr/0082-defer-multihash-until-a-method-consumes-it.md) and [#155](https://github.com/hyperledger-identus/sdk-rust/issues/155): a named DID-method consumer pins a normative multihash profile and defines code/digest allow-lists, resource limits, canonical representation and migration behavior. Technical fit alone is insufficient. |
| `isomdl 0.2.0` | [#161](https://github.com/hyperledger-identus/sdk-rust/issues/161): a spike isolates holder/verifier core from CLI, RNG, X.509 and transport choices and measures all target/cone costs. |
| `oauth2 5.0.0` | [#160](https://github.com/hyperledger-identus/sdk-rust/issues/160): a spike proves authorization-code/PKCE reuse without importing HTTP clients, clock, URL or token policy into protocol domain crates. |
| `aries-askar` | [#162](https://github.com/hyperledger-identus/sdk-rust/issues/162): an optional adapter proves storage-port fit, cancellation/concurrency, migrations, secret redaction, platform linking and a no-database core boundary. |
| selected Spruce SSI crates | [ADR 0066](../../adr/0066-conditionally-adopt-narrow-spruce-ssi-crates.md): a named crate passes exact-profile, effective Rust/toolchain, dependency-cone, target, security and facade-isolation gates; the umbrella remains prohibited. |
| `anoncreds-rs` | [ADR 0073](../../adr/0073-conditionally-adopt-anoncreds-rs.md): IDR-050 is activated and a focused optional adapter proves native/mobile, OpenSSL, unsafe, secret, VDR and conformance boundaries. |
| `uniffi 0.32.0` | [#163](https://github.com/hyperledger-identus/sdk-rust/issues/163): binding value/opaque-handle/error contracts stabilize and native runtime tests exist; UniFFI stays out of generic core crates. |
| `uniffi-bindgen-react-native 0.31.0-5` | Current release is pinned to UniFFI 0.31; reconsider through [#223](https://github.com/hyperledger-identus/sdk-rust/issues/223) after an exact 0.32-compatible release proves iOS/Android TurboModule runtime and packaging. |

Reason codes are deliberately finite: `MSRV`, `dependency-cone`,
`draft-version`, `coupling`, `maintenance`, `publication-readiness`,
`semantic-mismatch`, `strictness-mismatch`, `native-unsafe`,
`license-provenance`, and `low-payoff`.
