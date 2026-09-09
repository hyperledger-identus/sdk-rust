# Rust SSI repository portfolio

**Decision date:** 2026-09-08

**Issue:** [#175](https://github.com/hyperledger-identus/sdk-rust/issues/175)

**Discussion:** [#174](https://github.com/hyperledger-identus/sdk-rust/discussions/174)

**Baseline:** `develop@ff924db61452d4a1fd2a8bd4a1bfd3d702519604`

## Executive decision

The SDK does not standardize on a third-party SSI framework. It owns public
types, ports, limits, redacted errors, lifecycle states, and product-neutral
policy boundaries. Selected upstream components may be private dependencies or
optional adapters only after a focused issue proves the conditions below.

| Disposition | Runtime meaning |
| --- | --- |
| `conditional-adopt` | A focused issue may evaluate or integrate one narrow private crate/adapter after its trigger passes. |
| `spike` | A bounded adapter evaluation is approved, but production adoption remains undecided. |
| `oracle` | Read-only source, provenance-pinned fixtures, or dev-only differential evidence; absent from production artifacts. |
| `not-adopt` | Do not depend on or fork the assessed repository under this decision. |

## Decision matrix

| Repository | Assessed revision | License evidence | Rust/publication evidence | Coupling and target evidence | Decision |
| --- | --- | --- | --- | --- | --- |
| Spruce [`ssi`](https://github.com/spruceid/ssi/tree/89630368438c81b55335362b21621d3aadd48d93) | `89630368` | Apache-2.0 | [Facade 0.16.0](https://crates.io/crates/ssi/0.16.0); release declares Rust 1.87 and assessed workspace 1.89; MSRV and WASM CI | Modular but the facade enables a broad crypto/JSON-LD/RDF graph; no explicit mobile lane | `conditional-adopt` for named narrow crates; umbrella remains forbidden |
| Procivis [`one-core`](https://github.com/procivis/one-core/tree/e66ec8c533acb919611df75a38c7d23c2ccf658a) | `e66ec8c5` | Apache-2.0 | [Release v1.85.2](https://github.com/procivis/one-core/releases/tag/v1.85.2); workspace Rust 1.95/edition 2024; useful core packages are unpublished | Application/backend workspace with Tokio, Reqwest, SQL/provider configuration, build scripts, and UniFFI; no WASM lane | `oracle` |
| Impierce [`openid4vc`](https://github.com/impierce/openid4vc/tree/e9d99d211036f61d46f2c5c56e26197f74290929) | `e9d99d21` | Apache-2.0 | 0.1 workspace, no declared MSRV or tagged release; rolling stable Linux CI | Git-pinned IOTA dependencies, forked SD-JWT patch, Tokio/Reqwest and IOTA types; mobile/WASM unproved | `oracle` |
| IOTA [`identity`](https://github.com/iotaledger/identity/tree/7dd527087b96bf26ea97d486225eff8278c9a89c) | `7dd52708` | Apache-2.0 | [Beta v1.9.13](https://github.com/iotaledger/identity/releases/tag/v1.9.13-beta.1); workspace MSRV 1.65; host and WASM CI | Mature generic crates coexist with IOTA network/MoveVM and exact internal/Git coupling; second public DID/VC model | `oracle` |
| OWF [`askar`](https://github.com/openwallet-foundation/askar/tree/48a495920e2166771c6be1aca2fd057a8b0c8831) | `48a49592` | MIT OR Apache-2.0 | [Repository release v0.5.0](https://github.com/openwallet-foundation/askar/releases/tag/v0.5.0); crates.io `aries-askar` remains 0.4.6; MSRV 1.81 | The #162 fixture proves exact-record adaptability on host/iOS/Android compile lanes, but 192 host graph lines, broad unsliceable KMS crypto, native SQLite and missing row revisions outweigh reuse; no WASM | `not-adopt`; reference fixture only per ADR 0103 |
| DIF [`did-key.rs`](https://github.com/decentralized-identity/did-key.rs/tree/eb00da6074d8bc61e5d4c8129fbdd9dc05735cbf) | `eb00da60` | Apache-2.0 | 0.2.1, no declared MSRV, nightly/Ubuntu CI | Old crypto dependency generations; mobile/WASM and maintenance unproved | `oracle` |
| SICPA [`didcomm-rust`](https://github.com/sicpa-dlab/didcomm-rust/tree/4388350def84b6d7f6b65cf4a451607200035d8d) | `4388350d` | Apache-2.0 | [Release 0.4.1](https://github.com/sicpa-dlab/didcomm-rust/releases/tag/v0.4.1) from 2023; no declared MSRV | Old Node CI and Git-pinned 2021 Askar crypto; advertised WASM/UniFFI but no current mobile matrix | `not-adopt`; reference use only |
| AnonCreds [`anoncreds-rs`](https://github.com/anoncreds/anoncreds-rs/tree/08317a7428afe81f7b710669dc64878f98a6447b) | `08317a74` | Apache-2.0 | [Release v0.2.3](https://github.com/anoncreds/anoncreds-rs/releases/tag/v0.2.3); edition 2024/MSRV 1.85; Rust crate not published on crates.io | Broad native/mobile matrix; OpenSSL, optional C FFI, unsafe and VDR/secret boundaries; no WASM evidence | `conditional-adopt` when IDR-050 activates |
| AnonCreds [`anoncreds-v2-rs`](https://github.com/anoncreds/anoncreds-v2-rs/tree/691297a7f9ffcc1f51a5d30741086402d64544c9) | `691297a7` | Apache-2.0 | No stable repository release or declared MSRV; floating stable CI | Pre-release cryptography dependency and native `blst`; no mobile/WASM evidence or settled interoperability profile | `not-adopt` |
| OWF [`vcx`](https://github.com/openwallet-foundation/vcx/tree/5fa3cd5023ed4a0dc42a36f48bb1834961c3812d) | `5fa3cd50` | Apache-2.0 | [Release 0.68.0](https://github.com/openwallet-foundation/vcx/releases/tag/0.68.0); MSRV 1.81/development Rust 1.95; workspace crates unpublished | Large Tokio/Reqwest/server workspace with OpenSSL/ZMQ and Git dependencies; Ubuntu-only CI; DIDComm v1/Aries focus | `oracle` |
| OWF Labs [`sd-jwt-rust`](https://github.com/openwallet-foundation-labs/sd-jwt-rust/tree/146546cc20682b6ddee0fcc270211aac9b0bc83b) | `146546cc` | MIT OR Apache-2.0 | [Release/crate 0.7.1](https://github.com/openwallet-foundation-labs/sd-jwt-rust/releases/tag/v0.7.1); MSRV 1.67; Linux CI | Implements SD-JWT draft-07, not RFC 9901; mobile/WASM unproved; `jsonwebtoken`/`ring` coupling | `not-adopt`; legacy reference only |
| Spruce [`didkit`](https://github.com/spruceid/didkit/tree/57a3b45111f8b1003ef1a50cf7cf21c81cd59cb1) | `57a3b451` | Apache-2.0 | Archived/deprecated; old SSI 0.7 facade, edition 2018, no MSRV | Historical CLI/C/JNI/mobile bindings with an obsolete dependency graph | `not-adopt` |

## Repository decisions

The controlling ADRs are:

- [ADR 0066](../../adr/0066-conditionally-adopt-narrow-spruce-ssi-crates.md)
- [ADR 0067](../../adr/0067-use-procivis-one-core-as-a-capability-oracle.md)
- [ADR 0068](../../adr/0068-use-impierce-openid4vc-as-a-protocol-oracle.md)
- [ADR 0069](../../adr/0069-use-iota-identity-as-a-did-vc-oracle.md)
- [ADR 0070](../../adr/0070-evaluate-askar-as-a-storage-adapter.md)
- [ADR 0103](../../adr/0103-spike-aries-askar-as-an-isolated-storage-adapter.md)
- [ADR 0071](../../adr/0071-use-dif-did-key-as-a-method-oracle.md)
- [ADR 0072](../../adr/0072-use-didcomm-rust-as-a-reference-only.md)
- [ADR 0073](../../adr/0073-conditionally-adopt-anoncreds-rs.md)
- [ADR 0074](../../adr/0074-do-not-adopt-anoncreds-v2-rs.md)
- [ADR 0075](../../adr/0075-use-vcx-as-an-aries-reference-only.md)
- [ADR 0076](../../adr/0076-use-owf-sd-jwt-rust-as-a-legacy-oracle.md)
- [ADR 0077](../../adr/0077-do-not-adopt-the-didkit-facade.md)

## Source ledger

| Claim group | Primary evidence |
| --- | --- |
| Spruce SSI toolchain, modules, defaults and CI | [manifest](https://github.com/spruceid/ssi/blob/89630368438c81b55335362b21621d3aadd48d93/Cargo.toml), [README](https://github.com/spruceid/ssi/blob/89630368438c81b55335362b21621d3aadd48d93/README.md), [CI](https://github.com/spruceid/ssi/blob/89630368438c81b55335362b21621d3aadd48d93/.github/workflows/build.yml) |
| Procivis packaging, dependencies and standards | [workspace manifest](https://github.com/procivis/one-core/blob/e66ec8c533acb919611df75a38c7d23c2ccf658a/Cargo.toml), [core manifest](https://github.com/procivis/one-core/blob/e66ec8c533acb919611df75a38c7d23c2ccf658a/lib/one-core/Cargo.toml), [release v1.85.2](https://github.com/procivis/one-core/releases/tag/v1.85.2); the [mutable standards matrix](https://docs.procivis.ch/standards) was retrieved 2026-09-08 |
| Impierce versions, Git patches and CI | [manifest](https://github.com/impierce/openid4vc/blob/e9d99d211036f61d46f2c5c56e26197f74290929/Cargo.toml), [README](https://github.com/impierce/openid4vc/blob/e9d99d211036f61d46f2c5c56e26197f74290929/README.md), [CI](https://github.com/impierce/openid4vc/blob/e9d99d211036f61d46f2c5c56e26197f74290929/.github/workflows/format-lint-test.yaml) |
| IOTA workspace, credential defaults and CI | [manifest](https://github.com/iotaledger/identity/blob/7dd527087b96bf26ea97d486225eff8278c9a89c/Cargo.toml), [credential manifest](https://github.com/iotaledger/identity/blob/7dd527087b96bf26ea97d486225eff8278c9a89c/identity_credential/Cargo.toml), [CI](https://github.com/iotaledger/identity/blob/7dd527087b96bf26ea97d486225eff8278c9a89c/.github/workflows/build-and-test.yml) |
| Askar toolchain, features, crypto and CI | [manifest](https://github.com/openwallet-foundation/askar/blob/48a495920e2166771c6be1aca2fd057a8b0c8831/Cargo.toml), [README](https://github.com/openwallet-foundation/askar/blob/48a495920e2166771c6be1aca2fd057a8b0c8831/README.md), [CI](https://github.com/openwallet-foundation/askar/blob/48a495920e2166771c6be1aca2fd057a8b0c8831/.github/workflows/build.yml) |
| DIF did:key behavior, dependencies and CI | [README](https://github.com/decentralized-identity/did-key.rs/blob/eb00da6074d8bc61e5d4c8129fbdd9dc05735cbf/README.md), [manifest](https://github.com/decentralized-identity/did-key.rs/blob/eb00da6074d8bc61e5d4c8129fbdd9dc05735cbf/Cargo.toml), [CI](https://github.com/decentralized-identity/did-key.rs/blob/eb00da6074d8bc61e5d4c8129fbdd9dc05735cbf/.github/workflows/rust.yml) |
| DIDComm behavior, limitations, dependencies and CI | [README](https://github.com/sicpa-dlab/didcomm-rust/blob/4388350def84b6d7f6b65cf4a451607200035d8d/README.md), [manifest](https://github.com/sicpa-dlab/didcomm-rust/blob/4388350def84b6d7f6b65cf4a451607200035d8d/Cargo.toml), [CI](https://github.com/sicpa-dlab/didcomm-rust/blob/4388350def84b6d7f6b65cf4a451607200035d8d/.github/workflows/verify.yml) |
| AnonCreds v1 status, dependencies and platforms | [README](https://github.com/anoncreds/anoncreds-rs/blob/08317a7428afe81f7b710669dc64878f98a6447b/README.md), [manifest](https://github.com/anoncreds/anoncreds-rs/blob/08317a7428afe81f7b710669dc64878f98a6447b/Cargo.toml), [CI](https://github.com/anoncreds/anoncreds-rs/blob/08317a7428afe81f7b710669dc64878f98a6447b/.github/workflows/build.yml) |
| AnonCreds v2 maturity, dependencies and CI | [README](https://github.com/anoncreds/anoncreds-v2-rs/blob/691297a7f9ffcc1f51a5d30741086402d64544c9/README.md), [manifest](https://github.com/anoncreds/anoncreds-v2-rs/blob/691297a7f9ffcc1f51a5d30741086402d64544c9/Cargo.toml), [CI](https://github.com/anoncreds/anoncreds-v2-rs/blob/691297a7f9ffcc1f51a5d30741086402d64544c9/.github/workflows/build.yml) |
| VCX architecture, dependencies and CI | [README](https://github.com/openwallet-foundation/vcx/blob/5fa3cd5023ed4a0dc42a36f48bb1834961c3812d/README.md), [manifest](https://github.com/openwallet-foundation/vcx/blob/5fa3cd5023ed4a0dc42a36f48bb1834961c3812d/Cargo.toml), [CI](https://github.com/openwallet-foundation/vcx/blob/5fa3cd5023ed4a0dc42a36f48bb1834961c3812d/.github/workflows/main.yml) |
| OWF SD-JWT draft, toolchain and CI | [README](https://github.com/openwallet-foundation-labs/sd-jwt-rust/blob/146546cc20682b6ddee0fcc270211aac9b0bc83b/README.md), [manifest](https://github.com/openwallet-foundation-labs/sd-jwt-rust/blob/146546cc20682b6ddee0fcc270211aac9b0bc83b/Cargo.toml), [CI](https://github.com/openwallet-foundation-labs/sd-jwt-rust/blob/146546cc20682b6ddee0fcc270211aac9b0bc83b/.github/workflows/rust.yml), [RFC 9901](https://www.rfc-editor.org/rfc/rfc9901.html) |
| DIDKit deprecation and old facade | [archive notice](https://github.com/spruceid/didkit/blob/57a3b45111f8b1003ef1a50cf7cf21c81cd59cb1/README.md), [library manifest](https://github.com/spruceid/didkit/blob/57a3b45111f8b1003ef1a50cf7cf21c81cd59cb1/lib/Cargo.toml) |

## Evidence limits

- Upstream CI and manifest declarations are observations, not SDK target proof.
- Dependency cones were inspected structurally where prior research exists;
  every future adoption must measure its exact selected feature graph again.
- Unsafe-token and native-dependency observations are triage, not a soundness audit.
- Repository activity and releases do not establish standards conformance.
- No consumer repository was changed and no third-party code or fixture was copied.
