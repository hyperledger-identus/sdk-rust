# First language-binding slice research

Research class: storage-ffi
Research status: ready
Decision date: 2026-09-09
Source retrieval date: 2026-09-09
Research blockers: none

## Problem and existing implementation

The current implementation has no supported FFI. `identus-bindings` is a
quarantined marker and `SDK-LIM-002` forbids inferring native or JavaScript
support from compile-only target evidence. `identus-did` already owns bounded
2,048-byte DID and 4,096-byte DID URL parsing, exact-string round trips,
borrowed components and stable `did.invalid_did` / `did.invalid_did_url`
redacted errors. That deterministic value-only surface is useful to Swift and
Kotlin consumers and avoids secrets, ownership handles, callbacks, networking
and runtime selection.

## Normative sources

- UniFFI 0.32 user guide and supported-language contract:
  https://mozilla.github.io/uniffi-rs/latest/
- UniFFI foreign-language/library-mode guidance:
  https://mozilla.github.io/uniffi-rs/latest/tutorial/foreign_language_bindings.html
- UniFFI 0.32.0 release source and changelog, tag `v0.32.0`, commit
  `5c7b73906358e1a7acdc1bdc7bf5cd86fb27e44c`:
  https://github.com/mozilla/uniffi-rs/tree/v0.32.0
- W3C DID Core 1.0 DID syntax:
  https://www.w3.org/TR/did-core/#did-syntax
- React Native candidate source tag `0.31.0-5`, commit
  `0f7fc67e8dd98ad43af3787d45c3c570d469a145`:
  https://github.com/jhugman/uniffi-bindgen-react-native/tree/0.31.0-5
- wasm-bindgen guide for the separately generated browser boundary:
  https://wasm-bindgen.github.io/wasm-bindgen/

The stable native contract under test is UniFFI's direct Swift and Kotlin
support. React Native/TypeScript and browser JavaScript are third-party or
different generator/runtime contracts and are not normative evidence for
native UniFFI.

## Candidate decisions

| Candidate | Version/revision | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- | --- |
| UniFFI proc macros plus library-mode generation | 0.32.0 / `5c7b7390` | `conditional-adopt` | One Rust definition; exact double-generation plus Swift/Kotlin host runtime passed | Reconsider if production mobile/package, security or versioned-ABI gates fail |
| UDL-first generation | UniFFI 0.32.0 | `not-adopt` | Duplicates the Rust interface, and upstream deprecates single-UDL generation in favor of library mode | Reconsider for a consumer-owned ABI whose independent schema is intentionally authoritative |
| Hybrid UDL plus proc macros | UniFFI 0.32.0 | `not-adopt` | Adds two definition mechanisms without a need in the value-only slice | Reconsider when a required type or external ABI cannot be expressed with proc macros |
| SDK-owned value/error wrapper records | repository fixture | `retain-local` | Keeps `identus-did` UniFFI-free and prevents Rust/domain/dependency types crossing the ABI | Reconsider only through a versioned binding contract |
| `uniffi-bindgen-react-native` | 0.31.0-5 / `0f7fc67e` | `not-adopt` for this slice | Its current exact line uses UniFFI 0.31 and adds JSI/TurboModule/WASM packaging outside native proof | Reconsider after an exact UniFFI 0.32-compatible release and device/runtime evidence |
| `wasm-bindgen` browser adapter | separate future issue | `conditional-adopt` | It matches browser packaging/runtime semantics better than treating native FFI as a browser ABI | Reconsider with the first named browser consumer and bundler/runtime contract |

## Compatibility and dependency evidence

The production public and wire compatibility boundary remains unchanged: this
issue introduces no supported API. The fixture exposes only owned strings,
SDK-owned records and an SDK-owned error enum. It maps internal `Did`/`DidUrl`
and `IdentusError` values at the wrapper edge; no external or Rust domain type
crosses the ABI. A production facade can therefore version independently and
roll back by removing its isolated binding crate.

UniFFI 0.32.0 declares Rust edition 2021 and no package `rust-version`; the
spike passes the SDK's Rust 1.98.1 etalon rather than inferring an MSRV. Its
isolated lock contains 85 packages and hashes to
`248c8414b01e937dfcce98f23bca85887a1775634e30639adbed972ab1a3d8b4`.
The direct and resolved dependency cone in the default normal/build graph has 54 unique package/version
renderings including the SDK/DID graph; the bindgen CLI feature has 79. No
package declares Cargo `links`. Generator features belong to the fixture tool
path and must not leak into a future runtime-only crate. The root
`Cargo.toml` and `Cargo.lock` remain unchanged. Kotlin/JNA 5.18.1 and Kotlin
plugin 2.2.20 are recorded by the fixture's Gradle lock.

## Security, privacy and maintenance evidence

The wrapper accepts only bounded public identifier strings. Errors contain
stable enum cases/codes and no caller input. No raw secret, seed, signing key,
storage, native keystore, network, callback or object handle is reachable. The
fixture must demonstrate panic-free invalid-input mapping in both languages;
production still requires an ownership, memory, threading, cancellation and
panic-containment threat contract before `SDK-LIM-002` changes.

License and provenance: UniFFI 0.32.0 is MPL-2.0, was released 2026-06-30, and upstream describes the
project as production-ready but pre-1.0. Its normal runtime and generator cones
contain dependency-owned unsafe and native/JNA interaction; the spike records
that reach rather than weakening the SDK's first-party unsafe prohibition.
`uniffi-bindgen-react-native` 0.31.0-5 is also MPL-2.0 and actively maintained,
but its UniFFI 0.31 coupling is a current compatibility stop condition.
Protocol/draft currency is not applicable to the binding mechanism; DID syntax
remains W3C DID Core 1.0.

## Rejected or deferred candidates

Raw C/JNI/Swift hand-written bindings are not adopted because they multiply
memory and error-lowering code without evidence that UniFFI fails this slice.
Diplomat, CXX and wasm-pack are not equivalent direct Swift/Kotlin generators
for this bounded question. They remain candidates only if UniFFI fails a named
consumer requirement. React Native JSI/TurboModules and browser WASM are
deferred to separate issues so platform-specific coupling cannot reshape the
generic domain crate.

Supply-chain evidence is bounded to immutable upstream tags, crates.io package
metadata, the isolated lockfile and the resolved normal/build graphs. Every
Cargo package has a declared license expression. Source inspection found 47
explicit `unsafe {}` lines across UniFFI runtime/macro source; these are
dependency-owned and no authored fixture unsafe is permitted. Native dynamic
loading and JNA remain runtime boundaries even though no Cargo package declares
`links`. The installed `cargo-audit` failed to parse a current advisory-database
CVSS 4.0 record, so no clean advisory assertion is made; production #222 must
use the repository-pinned supply-chain gate. No package is admitted to the
production workspace by this decision.

## Open questions and blockers

No research blocker remains. The generated native call shapes are useful and
value-oriented. Stable enum cases are redacted, although production #222
should add machine-readable code access because Kotlin's generated message is
empty and Swift's default description is the enum identity. Exact output is
byte-reproducible on the host. Mobile packaging/runtime, panic containment for
arbitrary exports, object ownership, async/callback behavior and supported ABI
versioning deliberately remain #222 rather than being inferred from this
value-only proof.

## Evidence commands

Commands run: `git ls-remote` for both pinned tags, GitHub commit API
lookups, `cargo info uniffi@0.32.0`, direct manifest/license inspection,
`rustc --version`, `swiftc --version`, `java -version`, base
`scripts/factory doctor`, isolated locked Rust test/Clippy/release build,
`cargo tree`, `cargo metadata`, two library-mode Swift/Kotlin generations,
complete generated-tree and normalized API diffs, `swiftc` runtime execution,
Gradle Kotlin/JNA runtime execution and root manifest/lock diff. The committed
`scripts/verify.sh` reproduces the executable proof.

Intentionally unrun: iOS/Android device packaging,
React Native Metro/JSI/TurboModule execution, browser execution, Windows,
Python/Ruby, async/callback/object handles, fuzzing and release publication.
Those are outside this research-only value slice. Local `cargo-audit` was run
but failed on unsupported CVSS 4.0 advisory syntax before evaluating this lock;
it is an unpassed tool gate, not vulnerability evidence. Rollback is
documentation and fixture removal; no production dependency or consumer state
exists.
