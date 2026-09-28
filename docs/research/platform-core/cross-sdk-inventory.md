# Identus cross-SDK inventory

**Snapshot date:** 2026-09-29

**Program:** [issue #415](https://github.com/hyperledger-identus/sdk-rust/issues/415)

**Discussion:** [cross-SDK normalization #423](https://github.com/hyperledger-identus/sdk-rust/discussions/423)

**Decision:** preliminary discovery; not a parity or support claim

## Evidence basis

Fresh shallow clones of each public repository were inspected so local donor
worktrees and uncommitted changes were not touched.

| Repository | Immutable revision | Declared platform | Approximate authored source | Test files |
|---|---|---|---:|---:|
| [`sdk-ts`](https://github.com/hyperledger-identus/sdk-ts/tree/4bf86ebf69d5e96616a148e4c973f831f95fa38e) | `4bf86ebf69d5e96616a148e4c973f831f95fa38e` | Browser and Node.js | 20,463 TS lines in primary source | 84 SDK tests |
| [`sdk-swift`](https://github.com/hyperledger-identus/sdk-swift/tree/ebbfdb666a3b0c9905dd443589e03ccf2ae8087b) | `ebbfdb666a3b0c9905dd443589e03ccf2ae8087b` | iOS 15+, macOS 14+ | 30,533 Swift lines | 87 tests |
| [`sdk-kmp`](https://github.com/hyperledger-identus/sdk-kmp/tree/5a8fda770bbbca84192979b2c8e6090ee94790e3) | `5a8fda770bbbca84192979b2c8e6090ee94790e3` | Android/JVM; JS explicitly unsupported | 22,740 Kotlin lines | 63 tests |

Line and file counts scope the work only. They do not measure correctness,
coverage, maintainability, or migration difficulty.

## Historical module map

| Historical name | Intended responsibility | TS evidence | Swift evidence | KMP evidence | Preliminary Rust direction |
|---|---|---|---|---|---|
| Apollo | cryptography, keys and derivation | 14 source files | 18 source files plus `identus-apollo` and AnonCreds dependencies | 19 source files plus Apollo/BC dependencies | converge on `identus-crypto`; keep custody/platform handles outside |
| Castor | DID creation, management and resolution | 19 files; Prism and peer DID behavior | 18 files; PeerDID/Protobuf dependencies | 11 files; peer DID dependency | generic DID core in `identus-did`; method/ledger adapters separately owned |
| Pollux | credential and presentation behavior | 10 core files plus plugins; JWT, SD-JWT and AnonCreds | 94 files; SD-JWT, AnonCreds, JSON Schema | 17 files; SD-JWT, AnonCreds, JSON-LD/RDF | normalize formats and protocols; generic states in credentials/presentations; private engines where justified |
| Mercury | DIDComm messaging | 6 files | 11 files; `didcomm-swift` | 8 files; DIDComm dependency | research exact DIDComm v2 scope; `identus-messaging` is currently a placeholder |
| Pluto | storage, repositories, backup and migrations | 36 files | 82 files; CoreData | 9 files; SQLDelight | generic ports/wire migration contracts in Rust; platform databases remain adapters |
| Edge Agent | connections and protocol orchestration | 23 files plus plugins | 48 files plus builders/authentication | 47 files | reusable state machines/ports may move; product/runtime orchestration remains outside core |
| OpenID4VC/plugins | OIDC/OID4VC integration and extensibility | large plugin tree (99 files) | selected/deferred package integration | EUDI/HTTP dependencies | continue bounded `identus-oid4vci`/`identus-oid4vp`; retain language plugin hosts where cohesive |

Counts reflect directory structure and are not one-to-one feature counts.

## Repository dispositions

### sdk-ts

Target a thin, idiomatic TypeScript facade and browser/Node platform package.
Keep npm packaging, TypeScript ergonomics, runtime feature detection, browser
storage/networking, Node adapters, and the plugin host where those are
ecosystem responsibilities. Move portable crypto, DID, credential, protocol,
and reusable workflow semantics behind WASM or another deliberately selected
adapter. The existing `identus-wasm-did` package is the safest canary; it does
not yet prove npm publication, Node, bundlers, or production support.

### sdk-swift

Target a thin Swift facade and Apple platform shell over isolated UniFFI
packages. Keep SwiftPM/XCFramework distribution, async/throwing ergonomics,
Keychain/Secure Enclave, CoreData, networking, background lifecycle, and Apple
consent/UI integration. The existing experimental DID UniFFI surface is the
canary; secrets and generic trait objects are not the next binding surface.

### sdk-kmp

Target a thin Kotlin facade and Android/JVM platform shell. Keep Maven/Gradle
distribution, coroutines/Flow ergonomics, Ktor, SQLDelight, Android keystore,
lifecycle, and JVM integration. Reuse the experimental DID UniFFI surface for
the first canary, then prove Android device/JVM runtime and package compatibility
before broader migration.

### React Native

Treat React Native as a first-class mobile surface, not a synonym for browser
React or generated Swift/Kotlin. Issue #223 remains the qualification gate.
As of 2026-09-29, upstream `uniffi-bindgen-react-native` `0.31.0-6` still pins
UniFFI `=0.31`, while sdk-rust uses 0.32 for the native DID line. Do not mix
versions. Reassess on an exact compatible release and require iOS/Android New
Architecture runtime, threading, async/cancellation, packaging, and ownership
evidence.

## Preliminary capability disposition

| Capability family | Current Rust state | Disposition | Why / next evidence |
|---|---|---|---|
| crypto primitives and HD derivation | mature first release train | `move-to-rust` | expand consumer/vector evidence; keep secret custody out of value APIs |
| generic DID syntax/documents/resolution/registration | substantial | `move-to-rust` | normalize method-specific behavior and canary existing bindings |
| Prism/peer DID adapters | outside or incomplete | `defer` | decide generic adapter versus donor/product repository ownership |
| credential/presentation models and verification states | partial generic core | `move-to-rust` | inventory formats and authoritative vectors before engines |
| AnonCreds | no full owned implementation | `replace-upstream` candidate | assess maintained Rust engine behind Identus facade and exact target support |
| SD-JWT VC/JWT VC | incomplete | `defer` pending format inventory | normalize draft/final versions, fixtures, and upstream candidates |
| OID4VCI/OID4VP | active bounded implementation | `move-to-rust` | continue Final-spec slices; map language plugin/API compatibility |
| DIDComm v2 | placeholder/reference decisions | `defer` | inventory pack/unpack/routing/protocol scope and current engines |
| wallet storage contracts | ports and conformance exist | `move-to-rust` for contracts; `retain-platform` for adapters | map schema, migrations, encryption, concurrency and backup |
| edge-agent protocols/state machines | placeholder | `defer` | split reusable protocol state from product/runtime orchestration |
| browser/Node, Apple, Android/JVM integration | outside generic core | `retain-platform` | define narrow ports and package/runtime support matrices |
| legacy or draft-only features | not normalized | `deprecate`/`drop` candidate | require consumer and normative evidence before any port |

## Test normalization questions

Every inventory issue must answer:

1. Which fixtures come from official standards or published canonical vectors?
2. Which fixtures intentionally define an Identus profile?
3. Which tests reproduce released behavior used by a named consumer?
4. Which tests merely guard an implementation detail?
5. Which tests are contradictory, draft-bound, flaky, platform-dependent, or
   missing negative/boundary cases?
6. Which fixtures can become language-neutral bytes/JSON/error-code vectors in
   `identus-conformance`?
7. Which historical failures are bugs, and which are supported behavior?

The authority rules are defined by ADR 0164. No coverage percentage or passing
legacy suite by itself establishes source of truth.

## Known gaps before execution

- Exact public symbol/API inventories and package export maps.
- Published-version and downstream-consumer usage evidence.
- Test fixture provenance and contradiction reports.
- Persistence schemas, migration/backup compatibility, and data ownership.
- DIDComm and edge-agent protocol/version matrices.
- Concrete credential format and draft/final version matrices.
- Supported platform, runtime, package, performance, and resource baselines.
- Secret/custody and async/cancellation boundaries suitable for bindings.

These gaps become child issues; they are not filled by copying source into Rust.
