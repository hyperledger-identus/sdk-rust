# Rust library reuse research

**Decision date:** 2026-09-07

**Issue:** [#151](https://github.com/hyperledger-identus/sdk-rust/issues/151)

**Baseline:** `develop@516d91a7c2a783e86f19f2172168a6d716212303`

**Scope:** dependency selection for chain-neutral `sdk-rust` components
**Consumers inspected read-only:** NeoPRISM, midnight-identity, Lace ID Portal,
and Oxid

## Executive decision

The SDK should reuse narrow crates for closed, standards-defined mechanics and
keep Identus-owned public facades for policy, lifecycle, bounded parsing,
redacted errors, secret ownership, verification states and injected ports.
Whole SSI frameworks are not suitable as the SDK's domain model.

Two near-term dependency changes have a positive evidence balance:

1. replace the mnemonic implementation with `bip39 2.2.2` behind the existing
   `MnemonicHelper` facade;
2. use `fluent-uri 0.4.1` as the RFC 3986 grammar engine without re-exporting
   its types or normalizing caller input.

Focused integration research confirms the second change for `identus-did::Uri`
and four OID4VCI seams. [ADR 0084](../../adr/0084-adopt-fluent-uri-as-private-grammar-engine.md)
records the exact package, five-name incremental cone, MIT-0 allowlist addition,
transitive `ref-cast` unsafe boundary and exclusion of unused
`identus-core::Url`.

Focused issue-level research supersedes the preliminary form-codec
disposition. `form_urlencoded 1.2.2` preserves malformed percent escapes and
decodes invalid UTF-8 lossily, while the SDK parser rejects both. Its compatible
serializer alone would replace too little code to justify a new production
dependency. [ADR 0083](../../adr/0083-retain-strict-form-codec.md) and GitHub
issue #158 therefore retain the bounded local codec and classify the candidate
as `not-adopt` for this boundary.

Focused issue-level research also supersedes the preliminary multihash
disposition. `multihash 0.19.5` is technically cohesive, but no current SDK
capability consumes multihash semantics; did:key uses multicodec-prefixed key
bytes instead. [ADR 0082](../../adr/0082-defer-multihash-until-a-method-consumes-it.md)
and issue #155 retain it as `conditional-adopt` until a named DID method
defines the consumer, policy and migration boundary. Focused issue #156 also
supersedes the preliminary multibase disposition: Rust 1.98.1 removes the old
compiler blocker, but `multibase 0.9.3` still imports unused encodings and
build machinery. [ADR 0085](../../adr/0085-compose-narrow-multibase-codecs.md)
instead composes exact `bs58 0.5.1` with the already-locked Base64 engine for
the current `z`/`u` verification-material boundary.

Focused issue-level research supersedes the preliminary BIP-32 disposition:
retain the narrow HMAC orchestration and reuse the existing `k256` scalar
primitive. `bip32 0.5.3` rejects a standards-valid zero `IL` and brings unused
extended-key serialization dependencies; [ADR 0080](../../adr/0080-retain-bip32-mechanics-over-existing-k256.md)
and issue #153 record the correction.

`did_url_parser`, `isomdl`, `oauth2`, Askar and UniFFI justify bounded
integration spikes. `identity.rs`, Spruce SSI and current OpenID4VC Rust
implementations are valuable differential oracles, but importing their public
models would increase coupling and weaken the SDK's explicit security states.
[#164](https://github.com/hyperledger-identus/sdk-rust/issues/164) owns the
first dev-only oracle harness.
Draft-era or stale packages are recorded in the
[not-adopted ledger](not-adopted.md) with objective reconsideration triggers.

## Decision method

Each candidate was assessed against the same questions:

- Is the implemented standard/profile the exact version in the SDK roadmap?
- Is the crate maintained, published, licensed and attributable?
- Does its declared and observed Rust requirement fit the SDK's MSRV policy?
- Does the minimal feature set compile for the relevant host, WASM and mobile
  targets without an unowned native runtime?
- What is the standalone normal dependency cone, and how much of it overlaps
  the SDK?
- Does it add unsafe or FFI code to a security-sensitive path?
- Can the crate remain private behind an Identus facade with deterministic
  errors, resource bounds and a cheap rollback?
- Does the behavior benefit at least two independent SDK consumers?
- Is adoption safer than retaining the local implementation?

The dispositions are:

| Disposition | Meaning |
| --- | --- |
| `adopt` | Open an implementation issue for a private dependency behind an Identus facade. |
| `conditional-adopt` | Open an issue, but a named prerequisite must pass first. |
| `spike` | Produce compatibility evidence before any production dependency. |
| `oracle` | Use the implementation only in differential/conformance evidence. |
| `retain-local` | Keep the bounded local implementation and monitor a candidate. |
| `not-adopt` | Do not integrate now; reconsider only on the recorded trigger. |

Popularity, download counts and a successful latest-stable build were not
treated as conformance or security evidence.

## Current implementation findings

### Mnemonics are not currently BIP-39 validating

`crates/crypto/src/derivation/mnemonic.rs` currently accepts any sequence whose
individual words occur in the English word list. It does not validate the
allowed word counts or checksum. Entropy conversion accepts any non-empty byte
length divisible by four rather than BIP-39's 128–256 bit range, and seed
derivation does not NFKD-normalize the mnemonic or passphrase.

The canonical BIP specifies the entropy range, checksum and NFKD inputs.
`bip39 2.2.2` implements those checks and normalization and supports an
allocation-capable, non-`std` feature surface. Reusing it removes security-
relevant bespoke code while preserving the SDK's injected randomness and
zeroizing ownership contracts.

### BIP-32 currently reduces invalid scalars

`crates/crypto/src/derivation/hdkey.rs` reduces the left HMAC half modulo the
secp256k1 order. BIP-32 instead declares a child invalid when `IL >= n` or the
resulting key is zero. Master-key construction also accepts the HMAC output
without validating it as a non-zero scalar. Focused source review found that
`bip32 0.5.3` parses `IL` as a `NonZeroScalar`, incorrectly rejecting zero
before adding the parent, while BIP-32 permits zero if the resulting child is
nonzero. The candidate also retains Base58Check/RIPEMD extended-key coupling.
The SDK will therefore reuse its existing `k256` exact scalar parser and
arithmetic behind the hardened-only facade without adding the candidate.

### URI and multiformat syntax is intentionally under-validated

`identus-core::Url` validates only a scheme, `://` and non-empty authority.
`identus-did` owns a larger RFC 3986/DID parser, while OID4VCI also depends on
`uriparse`. The three engines can drift. `fluent-uri` implements RFC 3986/3987,
has an allocation-only feature surface and can be used only as an internal
grammar validator so exact caller bytes and Identus errors remain stable.

`identus-did::Multihash` currently accepts arbitrary bytes and serializes them
as hex. `multihash` provides a small, `no_std` structural codec but no hash
policy. That is strong technical fit, but no current method consumes the
structure, so adopting it would create policy without replacing reachable
behavior. `did:key` is not such a consumer: it uses multibase around a
multicodec key type and raw public-key bytes. `multibase` is evaluated
separately under #156.

### The existing HTTP grammar is small and deliberately stricter

OID4VCI owns bounded media-type and `Cache-Control: no-store` parsing.
`headers 0.4.1` is `std`-only at this layer, documents the superseded RFC 7234,
and deliberately ignores unknown cache directives. That is not a drop-in
replacement for the SDK's fail-closed check. `mime` does not by itself preserve
the SDK's duplicate-parameter rejection. `http` types would also couple the
transport-neutral port surface to an HTTP ecosystem type.

The form URL encoding algorithm is closed, but the candidate and SDK acceptance
boundaries differ. `form_urlencoded 1.2.2` preserves malformed percent escapes
and uses lossy UTF-8 decoding. The SDK rejects malformed escapes, raw non-ASCII,
decoded invalid UTF-8, decoded NUL and resource-limit overflow. The candidate
serializer matches valid output, but serializer-only adoption would retain the
checked-size pass, deterministic ordering, zeroizing allocation and strict
decoder. The local bounded codec therefore remains the lower-risk, higher-
cohesion choice.

## Candidate matrix

Standalone cone counts below are unique normal packages measured with
`cargo tree` for the stated minimal feature surface. They are comparison data,
not promised workspace deltas; shared RustCrypto packages make the incremental
SDK cone smaller for some candidates.

| Candidate | Version | Declared MSRV | Standalone cone | Decision | Cohesion and coupling result |
| --- | --- | ---: | ---: | --- | --- |
| `bip39` | 2.2.2 | not declared | 13 | `adopt` | Closed mnemonic/checksum/normalization mechanics; keep randomness, errors and zeroization at facade. |
| `bip32` | 0.5.3 | 1.65 | 29 with `secp256k1`; 3 incremental names | `not-adopt` | Its k256 backend rejects standards-valid zero `IL`; xprv/xpub dependencies are mandatory even though the facade does not use them. Reuse existing `k256`. |
| `slip10` | 0.4.3 | not declared | 25 | `retain-local` | Existing Ed25519 SLIP-0010 core is small and vector-tested; candidate maintenance and cone do not reduce risk. |
| `multihash` | 0.19.5 | 1.81 | 2 | `conditional-adopt` | Bare structural codec with high cohesion, but no current method consumes multihash; require a named normative consumer before production adoption. |
| `multibase` | 0.9.3 | not declared | 14; 9 incremental names | `not-adopt` for current boundary | Compiler-compatible under Rust 1.98.1, but unused Base45/Base256Emoji and build macros make the cone disproportionate; reconsider when features are sliced or named consumers need more bases. |
| `bs58` plus existing `base64` | 0.5.1 plus 0.22.1 | not declared; 1.48 | 1 new package | `adopt` | Reuses the two required base algorithms; Identus owns only the private `z`/`u` dispatch, canonicality, bounds and errors. |
| `did_url_parser` | 0.3.0 | not declared | 3 | `spike` | Narrow and `no_std`; parity must be proven before superseding ADR 0008 and changing accepted syntax/errors. |
| `identity_did` | 1.5.1 | not declared | 138 | `oracle` | Mature generic API and used by NeoPRISM, but broad model/cone would replace Identus ownership. |
| `identity_document` | 1.5.1 | not declared | 140 | `oracle` | Useful document and validation differential source; too coupled for the core domain facade. |
| `identity_credential` | 1.5.1 | not declared | 146 | `oracle` | Useful VC evidence, but current SD-JWT code references old drafts and imports framework policy. |
| `identity_jose` | 1.5.1 | not declared | 136 | `oracle` | Useful JOSE comparison; would bypass bounded parsed/verified states and duplicate checks. |
| `fluent-uri` | 0.4.1 | 1.68 | 8 standalone; 5 incremental | `adopt` | Exact RFC 3986 grammar behind owned string types in DID/OID4VCI; no IRI mode, normalization or public re-export. |
| `url` | 2.5.8 | 1.63 | 13 minimal | `not-adopt` | WHATWG URL semantics and canonicalization are not the generic DID/URI contract; reconsider only in a concrete HTTP adapter. |
| `http` | 1.5.0 | 1.57 | 3 | `not-adopt` | Good ecosystem type, but no current need outweighs transport-port coupling. |
| `mime` | 0.3.17 | not declared | 1 | `retain-local` | Does not enforce the SDK's complete duplicate/strictness policy; local bounded parser is small. |
| `headers` | 0.4.1 | 1.56 | 18 | `not-adopt` | `std`-coupled and permissive unknown-directive behavior conflicts with fail-closed protocol checks. |
| `form_urlencoded` | 1.2.2 | 1.51 | 2 | `not-adopt` | Browser-tolerant malformed-percent and lossy UTF-8 behavior conflicts with the strict SDK parser; serializer-only reuse has low payoff. |
| Spruce `ssi` umbrella | 0.16.0 | 1.89 | broad; selected modules 163–263 | `not-adopt` | MSRV can be solved, but domain, JSON-LD, crypto and protocol coupling cannot. Use selected modules as oracles. |
| Spruce `openid4vp` | upstream at assessed SHA | inherits 1.89+ | broad | `oracle` | OID4VP 1.0 reference, but carries transport/runtime dependencies and a git-pinned JOSE fork. |
| Spruce `isomdl` | 0.2.0 | not declared | 251 | `spike` | Strong behavior and state-machine evidence; monolithic library/CLI dependencies and target cost need isolation first. |
| impierce `openid4vc` | upstream at assessed SHA | not declared | broad | `oracle` | Tracks OID4VCI/OID4VP finals, but published `oid4vci`/`openid4vp` 0.1 crates are empty placeholders and upstream uses git patches. |
| OWF `sd-jwt-rs` | 0.7.1 | 1.67 | 54 | `not-adopt` | Explicitly implements draft version 7, not RFC 9901. |
| `didcomm` | 0.4.1 | not declared | about 84 | `not-adopt` | Last release is from 2023 and dependencies/API predate the roadmap's DIDComm 2.1 target. |
| `aries-askar` | 0.4.6 | 1.81 | 136 minimal | `spike` | Viable optional storage adapter, but even minimal mode brings crypto/storage policy and substantial unsafe/FFI-adjacent code. |
| `uniffi` | 0.32.0 | not declared | 37 core package | `conditional-adopt` | Correct native binding tool once value/handle/error contracts stabilize; keep it in binding crates only. |
| RustCrypto `jose-jwk` | 0.1.2 | 1.65 | 13 | `oracle` | Narrow and pure Rust, but 0.1 API maturity does not justify replacing the existing constrained JWK facade. |
| `oauth2` | 5.0.0 | 1.65 | 43 minimal | `spike` | Strong OAuth mechanics and PKCE, but `url`/clock/model assumptions must remain outside OID4VC domain types. |

## Compatibility probes

An ephemeral library was resolved with exact versions and minimal features for
`bip39`, `bip32`, `multihash`, `multibase`, `fluent-uri`,
`form_urlencoded` and `did_url_parser`.

| Probe | Result |
| --- | --- |
| Rust 1.85 host, each candidate independently except `multibase` | Passed. |
| Rust 1.85 host, combined candidate set | Failed in `base45 3.2.0`, reached from `multibase`, because `slice::as_chunks` stabilized in Rust 1.88. |
| Rust 1.90 host, combined candidate set | Passed. |
| Rust 1.95, `wasm32-unknown-unknown` | Passed. |
| Rust 1.95, `aarch64-apple-ios` | Passed. |
| Rust 1.95, `aarch64-linux-android` | Passed as a compile check. |

These probes compile dependency code only. They do not prove SDK integration,
linking, runtime behavior, conformance or release fitness. Every adoption issue
must run the repository's Nix, MSRV, feature and target gates after integration.

The current locked SDK graph has 94 external packages: 73 declare a
`rust-version` and 21 do not. The absence of a declaration is therefore not
evidence of compatibility. ADR 0064 supersedes ADR 0062 and uses these probes
to select Rust 1.89 as the next evidence-gated candidate while leaving Rust
1.85 effective, rather than calculating an unreliable ecosystem “average.”

## Security and maintenance evidence

An OSV query for each exact crate version on 2026-09-07 returned no known
advisories for the assessed versions. Historical advisories exist for earlier
`multihash` and `http` releases, which confirms that version-qualified queries
are required. A zero-result snapshot is not an audit and does not replace
`cargo audit`, `cargo deny`, provenance review or cryptographic review.

The locally installed `cargo-audit 0.20.1` could not parse a newer CVSS 4.0
RustSec advisory. No “audit passed” claim is made from that tool. The normal
pinned repository security gate remains mandatory for each dependency PR.

Direct unsafe-token counts were inspected only as triage. `bip39`, `bip32`,
`fluent-uri`, `isomdl` and `oauth2` contained no direct unsafe declarations in
their packaged Rust sources; `multihash`, `did_url_parser`, `http`, `mime`,
`form_urlencoded`, `headers` and Askar did. Counts alone do not establish
soundness: each production adoption must attribute and review reachable unsafe
code and transitive native dependencies.

## Architectural consequences

### Public types stay owned by Identus

Approved crates are implementation details. Public signatures must not expose
their types. Errors map to stable, redaction-safe Identus codes. Parsers retain
input, nesting and duplicate limits. Cryptographic secrets retain zeroizing
ownership and cannot gain `Debug`, serde or FFI exposure.

### Suites are decomposed at the seam

For mdoc, OAuth, storage and bindings, the integration issue starts with a
consumer-shaped spike. A successful spike identifies the smallest crate or
adapter boundary. It does not grant permission to import a framework's policy,
runtime, HTTP client, database, chain or custody choices.

### Rejections are maintained data

The [not-adopted ledger](not-adopted.md) distinguishes permanent semantic
mismatches from current packaging, draft or maintenance problems. A future
agent must meet the recorded reconsideration trigger and update the ADR before
opening a production integration PR.

### Research becomes a factory gate

Every new OpenSpec change now carries `research.md`. The full evidence matrix
is required for protocol, cryptography/security, foundational, storage and FFI
changes. Routine work may use a lightweight record. `scripts/factory
research-ready <change>` is the zero-blocker gate before implementation; it is
separate from the final `ready` gate.

## Delivery issues

| Order | Issue | Decision |
| ---: | --- | --- |
| 1 | [#152 — BIP-39 mechanics](https://github.com/hyperledger-identus/sdk-rust/issues/152) | `adopt`; correctness first |
| 2 | [#153 — BIP-32 mechanics](https://github.com/hyperledger-identus/sdk-rust/issues/153) | `retain-local`; correct with existing `k256` per ADR 0080 |
| 3 | [#170 — evidence-driven Rust policy](https://github.com/hyperledger-identus/sdk-rust/issues/170) | implement ADR 0064; retain 1.85 and evaluate 1.89 |
| 4 | [#155 — multihash](https://github.com/hyperledger-identus/sdk-rust/issues/155) | `conditional-adopt`; named DID-method consumer required |
| 5 | [#156 — multibase carriers](https://github.com/hyperledger-identus/sdk-rust/issues/156) | adopt narrow `bs58` plus existing Base64; reject broad `multibase` cone per ADR 0085 |
| 6 | [#157 — RFC 3986 engine](https://github.com/hyperledger-identus/sdk-rust/issues/157) | `adopt` with parser parity |
| 7 | [#158 — form encoding](https://github.com/hyperledger-identus/sdk-rust/issues/158) | `not-adopt`; retain the strict bounded local codec per ADR 0083 |
| 8 | [#159 — DID parser parity](https://github.com/hyperledger-identus/sdk-rust/issues/159) | `spike` |
| 9 | [#160 — OAuth mechanics](https://github.com/hyperledger-identus/sdk-rust/issues/160) | `spike` |
| 10 | [#161 — isomdl](https://github.com/hyperledger-identus/sdk-rust/issues/161) | `spike` |
| 11 | [#162 — Askar](https://github.com/hyperledger-identus/sdk-rust/issues/162) | `spike` |
| 12 | [#163 — UniFFI](https://github.com/hyperledger-identus/sdk-rust/issues/163) | `conditional-adopt` |
| 13 | [#164 — differential oracles](https://github.com/hyperledger-identus/sdk-rust/issues/164) | `oracle` |

## Claim-to-source ledger

| Claim | Primary evidence |
| --- | --- |
| BIP-39 requires 128–256-bit entropy, checksum validation and NFKD inputs. | [Bitcoin BIP-39](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki) |
| `bip39 2.2.2` validates entropy/checksum and normalizes UTF-8. | [`bip39` 2.2.2 source](https://docs.rs/crate/bip39/2.2.2/source/) and [`rust-bip39` assessed revision](https://github.com/rust-bitcoin/rust-bip39/tree/1a63bd457cf0643f7eee9b8768ce6ced13d01c18) |
| BIP-32 declares `IL >= n` or a zero child invalid. | [Bitcoin BIP-32](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki) |
| `bip32 0.5.3` is `no_std`-friendly and declares Rust 1.65, but its k256 backend rejects zero `IL` and its feature graph retains extended-key serialization packages. | [`bip32` 0.5.3 documentation](https://docs.rs/bip32/0.5.3/bip32/) and [exact release source](https://github.com/iqlusioninc/crates/tree/240679a2454945783acc4f9e7d3bae839359b0b7/bip32) |
| did:key fingerprints are multibase-encoded multicodec key types plus raw public-key bytes, not multihash. | [did:key Method v0.9 identifier syntax](https://w3c-ccg.github.io/did-key-spec/#did-key-identifier-syntax) |
| `multihash` is a bare structural codec and does not select hash algorithms. | [`multihash` 0.19.5 documentation](https://docs.rs/multihash/0.19.5/multihash/) and [exact release source](https://github.com/multiformats/rust-multihash/tree/e2044a2e3aa27c2a08d3bad492fccd4babf10310) |
| `fluent-uri` targets RFC 3986 and RFC 3987; this integration enables only the RFC 3986 URI/URI-reference parser. | [`fluent-uri` 0.4.1 documentation](https://docs.rs/fluent-uri/0.4.1/fluent_uri/) and [exact release source](https://github.com/yescallop/fluent-uri-rs/tree/d9a6a20614f34b00476837eb8904fb01ca3e54df) |
| `form_urlencoded 1.2.2` preserves malformed percent escapes and decodes invalid UTF-8 lossily; its serializer matches the assessed valid-output mechanics. | [`form_urlencoded` 1.2.2 packaged source](https://docs.rs/crate/form_urlencoded/1.2.2/source/src/lib.rs) and [recorded VCS revision](https://github.com/servo/rust-url/tree/91377f48bf35011d042aa5abef9e7f2a0a625aaa/form_urlencoded) |
| `slice::as_chunks`, used by resolved `base45 3.2.0`, stabilized in Rust 1.88. | [Rust standard library documentation](https://doc.rust-lang.org/stable/core/primitive.slice.html#method.as_chunks) |
| Rust releases stable trains every six weeks and supports only current stable upstream. | [The Rust release-channel model](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html) |
| Current Rust stable was 1.98.1 on the decision date. | [Rust 1.98.1 announcement](https://blog.rust-lang.org/2026/09/03/Rust-1.98.1/) |
| Spruce SSI 0.16 declares Rust 1.89 and is a broad multi-suite workspace. | [Spruce SSI manifest](https://github.com/spruceid/ssi/blob/89630368438c81b55335362b21621d3aadd48d93/Cargo.toml) |
| Spruce SSI is actively used and has a published 2022 review, so it is valuable as an oracle despite coupling. | [Spruce SSI repository](https://github.com/spruceid/ssi/tree/89630368438c81b55335362b21621d3aadd48d93) |
| `identity.rs` contains method-agnostic DID/VC crates but also method-specific IOTA components. | [IOTA Identity repository](https://github.com/iotaledger/identity/tree/7dd527087b96bf26ea97d486225eff8278c9a89c) |
| OWF `sd-jwt-rs` explicitly supports draft version 7. | [OWF SD-JWT Rust repository](https://github.com/openwallet-foundation-labs/sd-jwt-rust/tree/146546cc20682b6ddee0fcc270211aac9b0bc83b) |
| RFC 9901 is the SDK's stable SD-JWT target. | [RFC 9901](https://www.rfc-editor.org/rfc/rfc9901.html) |
| impierce tracks OID4VCI/OID4VP finals, but the assessed implementation is a git-coupled workspace. | [impierce/openid4vc assessed revision](https://github.com/impierce/openid4vc/tree/e9d99d211036f61d46f2c5c56e26197f74290929) |
| OpenID4VCI 1.0 became a Final Specification on 2025-09-16. | [OpenID Foundation final approval](https://openid.net/openid-for-verifiable-credential-issuance-1-final-specification-approved/) |
| Spruce `isomdl` implements ISO 18013-5 device and reader flows. | [Spruce isomdl assessed revision](https://github.com/spruceid/isomdl/tree/fcb49d15ad9d54afa028a12183ee7fab1e46a5dc) |
| `didcomm 0.4.1` was last released in 2023. | [didcomm-rust releases and limitations](https://github.com/sicpa-dlab/didcomm-rust/tree/4388350def84b6d7f6b65cf4a451607200035d8d) |
| Askar combines cryptographic primitives, storage, optional databases and FFI. | [OWF Askar assessed revision](https://github.com/openwallet-foundation/askar/tree/48a495920e2166771c6be1aca2fd057a8b0c8831) |
| UniFFI is a multi-language Rust binding generator. | [Mozilla UniFFI assessed revision](https://github.com/mozilla/uniffi-rs/tree/3a2d44a7786d0cb364e446bd96711a0d31dd9904) |
| Cargo can use declared MSRV during dependency resolution, but undeclared or inaccurate metadata remains possible. | [Rust RFC 3537](https://rust-lang.github.io/rfcs/3537-msrv-resolver.html) |
| Exact-version advisory queries returned zero entries on the assessment date. | [OSV API](https://google.github.io/osv.dev/api/) |

## Limits of this research

- ISO standards requiring paid text were not reproduced; `isomdl` remains a
  spike until licensed normative text and applicable test material are
  available to the implementer.
- Target probes were compile-only and did not run device, browser, database or
  FFI code.
- Upstream activity is a point-in-time observation. Revisions are pinned so a
  future refresh can identify drift.
- No external audit was re-performed. Published reviews and advisory databases
  are inputs, not a security approval.
- No dependency is integrated by this research change.
