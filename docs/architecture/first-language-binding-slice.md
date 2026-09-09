# First language-binding slice report

**Issue:** [#215](https://github.com/hyperledger-identus/sdk-rust/issues/215)

**Parent implementation:** [#163](https://github.com/hyperledger-identus/sdk-rust/issues/163)

**Decision:** [ADR 0097](../adr/0097-use-proc-macro-uniffi-for-the-first-native-did-slice.md)

**Follow-ups:** [native #222](https://github.com/hyperledger-identus/sdk-rust/issues/222),
[React Native #223](https://github.com/hyperledger-identus/sdk-rust/issues/223), and
[browser #224](https://github.com/hyperledger-identus/sdk-rust/issues/224)
**Assessment date:** 2026-09-09

## Outcome

Proceed with bounded DID/DID URL parsing as the first native Swift/Kotlin
production slice. Use UniFFI 0.32 proc macros in an isolated binding crate and
compiled-library generation. Keep `identus-did` UniFFI-free and map its values
and `IdentusError` codes into SDK-owned FFI records and stable redacted cases.

The research does not activate an FFI support claim. React Native and browser
React are separate decisions because their generators, runtimes and packaging
do not share the native Swift/Kotlin compatibility contract.

| Surface | Decision now | Evidence | Remaining gate |
| --- | --- | --- | --- |
| Swift | Go for production slice | Swift 6.3 host program linked the generated wrapper and passed valid, invalid and oversized cases | iOS simulator/device, XCFramework/SwiftPM, API/ABI versioning |
| Kotlin | Go for production slice | Kotlin 2.2.20/JVM 17 host proof plus a deterministic arm64-v8a AAR and API-35 arm64 emulator receipt using explicit JNA 5.18.1 | physical device, other ABI/runtime matrix, Maven publication and supported JVM/Android API contract |
| React Native | No-go on current dependency | `uniffi-bindgen-react-native` 0.31.0-5 uses UniFFI 0.31 and adds JSI/TurboModule/runtime coupling | Exact 0.32-compatible release, New Architecture iOS/Android runtime and packaging proof |
| Browser React | Experimental adapter implemented | exact wasm-bindgen runtime/CLI 0.2.121 browser-native ESM/TypeScript package is byte-reproducible and executes bounded DID behavior in Chrome; the weekly gate requires Chromium and Firefox | Named downstream/bundler adoption, browser support matrix and publication contract |
| Node | Deferred | Neither native host proof nor browser WASM defines a Node package/loading contract | Named consumer and N-API or WASM runtime decision |

## Interface comparison

| Strategy | Cohesion | Drift risk | Decision |
| --- | --- | --- | --- |
| Proc macros + library mode | Rust wrapper is the single source; metadata travels with compiled library | Generated output still changes with exact UniFFI version | Adopt for the first native slice |
| UDL-first | Independent schema can be useful, but duplicates Rust records/functions | Rust and UDL signatures can diverge; single-UDL generation is deprecated | Do not adopt now |
| Hybrid | Can bridge special legacy/external shapes | Two definition mechanisms and a larger review surface | Do not adopt without a concrete expressiveness need |

The generated APIs are value-oriented: Swift receives `DidView` /
`DidUrlView`, throwing `DidBindingError`; Kotlin receives data classes and
throws `DidBindingException`. Kotlin's generated exception message is empty and
Swift's description contains only the enum identity. Neither contains input.
Production should add stable error-code accessors because matching only a
generated language type is less durable for a long-lived SDK.

## Reproducibility and dependency evidence

The fixture's `scripts/verify.sh` generated all four native files twice and
`diff -ru` found no difference. Reviewed normalized snapshots record only the
public records, properties, cases and functions. The generated tree hashes
were:

| Artifact | SHA-256 |
| --- | --- |
| `IdentusDidSpike.swift` | `d09bcbd0c0bfe3d8b393ca941c47201011c3930976f2645bd5682d81f7e6478b` |
| `IdentusDidSpikeFFI.h` | `dea2f557dfd0c2680f107d0ae44536a589020794edd0e014cf664d3550e622a5` |
| `IdentusDidSpikeFFI.modulemap` | `a14ba05487ec0373265c604d205978e852ff641aae96e1da00b2a84cb21da56c` |
| Kotlin binding source | `12282a26f27978eede73cd24758bd7a8fc6259aa3365f627d1dfde3673beea05` |

The exact fixture lock hash is
`248c8414b01e937dfcce98f23bca85887a1775634e30639adbed972ab1a3d8b4`.
It resolves 85 packages; 54 unique package/version renderings are reachable in
the default normal/build graph and 79 when the in-fixture bindgen CLI feature
is enabled. Future production layout must split generator tooling from the
runtime crate so its template/parser/CLI packages do not ship accidentally.

No resolved package declares Cargo `links`. Native dynamic-library loading is
still inherent, and Kotlin uses JNA. UniFFI 0.32.0 is MPL-2.0, has no declared
MSRV and passes the fixture on the SDK's Rust 1.98.1 etalon. A source scan found
47 explicit unsafe-block lines in UniFFI runtime/macro source. This is accepted
as dependency evidence for research, not as a relaxation of first-party
`unsafe_code = "forbid"`.

## Security and limitation findings

- Inputs retain the SDK's 2,048-byte DID and 4,096-byte DID URL ceilings.
- The ABI contains only owned public strings; there are no borrowed pointers,
  secret values, handles, callbacks, futures, storage or network operations.
- Errors are closed/redacted and both language tests prove caller text is not
  reflected.
- Generated scaffolding performs its own contract/API checksum at load time.
- The installed `cargo-audit` failed while parsing a CVSS 4.0 advisory record;
  no clean advisory claim is made. Production adoption must use the
  repository-pinned supply-chain gate.
- Host execution proves neither mobile runtime nor packaging/certification.

## Evidence commands

Passed: fixture `cargo test --locked`, strict Clippy, release build, two exact
library-mode generations, full generated-tree diff, normalized API snapshot
diff, Swift compile/runtime, Kotlin compile/runtime, and root manifest/lock
invariance. The complete command is:

```bash
docs/research/uniffi-did-spike/scripts/verify.sh
```

Subsequent issues #228 and #230 now provide local arm64 iOS Simulator plus
arm64-v8a Android emulator package/runtime evidence. Still unrun by design:
physical-device and wider runtime/ABI tests, React
Native Metro/JSI/TurboModule runtime, Node execution, callbacks, async,
objects, cancellation, secrets, signing, storage, networking, publication and
certification.

Browser issue #224 is now implemented separately through `identus-wasm-did` and
[ADR 0101](../adr/0101-adopt-wasm-bindgen-for-browser-did-values.md). Its
browser-native ESM package and two-engine slow gate do not change native UniFFI
or activate supported FFI.
